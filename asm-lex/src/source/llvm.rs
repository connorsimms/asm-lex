pub mod targets;

use crate::byte;
use crate::byte::{ByteSet, Class, ClassTable};
use crate::cursor::Cursor;
use crate::source;
use crate::source::{Dialect, Item, Kind};
use crate::Span;

// https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/MC/MCAsmInfo.h
pub trait LlvmTarget {
    const SEPARATOR_STR: &'static [u8] = b";";
    const COMMENT_STR: &'static [u8] = b"#";
    const ALLOW_ADDITIONAL_COMMENTS: bool = true;
    const AT_IN_IDENTIFIER: bool = false;
    const QUESTION_STARTS_IDENTIFIER: bool = false;
    const DOLLAR_STARTS_IDENTIFIER: bool = false;
    const AT_STARTS_IDENTIFIER: bool = false;
}

pub struct Llvm<T: LlvmTarget> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: LlvmTarget> Llvm<T> {
    const LINE_END_CHARS: ByteSet = ByteSet::from_bytes(b"\r\n");

    const HSPACE_CHARS: ByteSet = ByteSet::from_bytes(b" \t\x00");

    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"_.")
        .with_set(&byte::ASCII_ALPHA)
        .with_byte_if(b'?', T::QUESTION_STARTS_IDENTIFIER)
        .with_byte_if(b'$', T::DOLLAR_STARTS_IDENTIFIER)
        .with_byte_if(b'@', T::AT_STARTS_IDENTIFIER);

    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"_.$?")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT)
        .with_byte_if(b'@', T::AT_IN_IDENTIFIER);

    const STATEMENT_END_CHARS: ByteSet = ByteSet::from_bytes(b"\r\n")
        .with_byte_if(T::SEPARATOR_STR[0], T::SEPARATOR_STR.len() == 1)
        .with_byte_if(T::COMMENT_STR[0], T::COMMENT_STR.len() == 1);

    const ARG_STOP_CHARS: ByteSet = ByteSet::new()
        .with_byte(b'/') // slash-star comments
        .with_byte(b'"') // double quotes
        .with_byte(b'\'') // single quotes
        .with_set(&Self::STATEMENT_END_CHARS);

    const SYMBOL_START: Class = Class::with_bit(0);
    const SYMBOL_CONTINUE: Class = Class::with_bit(1);
    const STATEMENT_END: Class = Class::with_bit(2);
    const ARG_STOP: Class = Class::with_bit(3);

    const TABLE: ClassTable = ClassTable::build(&[
        (Self::SYMBOL_START, &Self::SYMBOL_START_CHARS),
        (Self::SYMBOL_CONTINUE, &Self::SYMBOL_CONTINUE_CHARS),
        (Self::STATEMENT_END, &Self::STATEMENT_END_CHARS),
        (Self::ARG_STOP, &Self::ARG_STOP_CHARS),
    ]);

    #[inline]
    fn class(b: u8) -> Class {
        Self::TABLE.classify(b)
    }

    #[inline]
    fn is_horizontal_whitespace(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\x00')
    }

    #[inline]
    fn is_end_of_line(b: u8) -> bool {
        matches!(b, b'\r' | b'\n')
    }

    fn eat_single_quoted(cursor: &mut Cursor<'_>) -> Span {
        let start = cursor.pos();
        if cursor.eat(b'\'') {
            // Any character following a backslash is ignored
            while let Some(b) = cursor.bump() {
                match b {
                    b'\\' => {
                        cursor.bump();
                    }
                    b'\'' => break,
                    _ => {}
                }
            }
        }
        start..cursor.pos()
    }

    // Handles escape quotes.
    // Non-terminated strings go to EOF.
    fn eat_double_quoted(cursor: &mut Cursor<'_>) -> Span {
        let start = cursor.pos();
        if cursor.eat(b'"') {
            // Any character following a backslash is ignored
            while let Some(b) = cursor.bump() {
                match b {
                    b'\\' => {
                        cursor.bump();
                    }
                    b'"' => break,
                    _ => {}
                }
            }
        }
        start..cursor.pos()
    }

    fn trim_trailing_hspace(cursor: &Cursor) -> usize {
        let mut i = -1;
        while cursor
            .seek(i)
            .is_some_and(|b| Self::is_horizontal_whitespace(b))
        {
            i -= 1;
        }
        cursor.pos().wrapping_add_signed(i + 1)
    }

    // hopefully compiler does smart things here
    fn eat_line_separator(cursor: &mut Cursor<'_>) -> bool {
        match T::SEPARATOR_STR.len() {
            1 => cursor.eat(T::SEPARATOR_STR[0]),
            2 => {
                if cursor.peek() == Some(T::SEPARATOR_STR[0])
                    && cursor.seek(1) == Some(T::SEPARATOR_STR[2])
                {
                    cursor.advance(2);
                    true
                } else {
                    false
                }
            }
            len => {
                if cursor.starts_with(T::SEPARATOR_STR) {
                    cursor.advance(len);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn try_linemarker(cursor: &mut Cursor<'_>) -> Option<Kind> {
        if cursor.peek() != Some(b'#')
            || cursor
                .seek(-1)
                .is_some_and(|b| !Self::class(b).contains(Self::STATEMENT_END))
        {
            return None;
        }

        let save = cursor.pos();
        cursor.bump();

        cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        if cursor.eat_while(|b| b.is_ascii_digit()).is_empty() {
            cursor.restore(save);
            return None;
        }

        cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        if Self::eat_double_quoted(cursor).is_empty() {
            cursor.restore(save);
            return None;
        }

        cursor.eat_while(|b| !Self::is_end_of_line(b));
        cursor.restore(Self::trim_trailing_hspace(cursor));
        Some(Kind::Preprocessor)
    }

    // hopefully compiler does smart things here
    fn is_line_comment(cursor: &mut Cursor<'_>) -> bool {
        if T::ALLOW_ADDITIONAL_COMMENTS {
            if cursor.peek() == Some(b'#') {
                return true;
            }
            if cursor.peek() == Some(b'/') && cursor.seek(1) == Some(b'/') {
                return true;
            }
        }
        match T::COMMENT_STR.len() {
            1 => cursor.peek() == Some(T::COMMENT_STR[0]),
            2 => {
                cursor.peek() == Some(T::COMMENT_STR[0])
                    && cursor.seek(1) == Some(T::COMMENT_STR[1])
            }
            _ => cursor.starts_with(T::COMMENT_STR),
        }
    }

    fn try_line_comment(cursor: &mut Cursor<'_>) -> Option<Kind> {
        if !Self::is_line_comment(cursor) {
            return None;
        }
        cursor.eat_until(&crate::pattern::AnyOf(*b"\r\n"));
        cursor.restore(Self::trim_trailing_hspace(cursor));
        Some(Kind::Comment)
    }

    #[inline]
    fn is_slash_star_comment(cursor: &mut Cursor<'_>) -> bool {
        T::ALLOW_ADDITIONAL_COMMENTS && cursor.peek() != Some(b'/') && cursor.seek(1) != Some(b'*')
    }

    fn try_slash_star_comment(cursor: &mut Cursor<'_>) -> Option<Kind> {
        if !Self::is_slash_star_comment(cursor) {
            return None;
        }
        cursor.eat_until(&crate::pattern::Substring(*b"*/"));
        cursor.restore(Self::trim_trailing_hspace(cursor));
        Some(Kind::Comment)
    }

    fn lex_args(cursor: &mut Cursor<'_>) -> Option<Span> {
        let save = cursor.pos();
        let mut content: Option<Span> = None;
        cursor.eat_while(|b| Self::is_horizontal_whitespace(b));

        while let Some(b) = cursor.peek() {
            let class = Self::class(b);
            match b {
                _ if !class.contains(Self::ARG_STOP) => {
                    let span = content.get_or_insert(cursor.pos()..cursor.pos());
                    cursor.eat_while(|b| !Self::class(b).contains(Self::ARG_STOP));
                    span.end = cursor.pos();
                }
                _ if class.contains(Self::STATEMENT_END) => break,
                b'\'' => {
                    let span = content.get_or_insert(cursor.pos()..cursor.pos());
                    Self::eat_single_quoted(cursor);
                    span.end = cursor.pos();
                }
                b'"' => {
                    let span = content.get_or_insert(cursor.pos()..cursor.pos());
                    Self::eat_double_quoted(cursor);
                    span.end = cursor.pos();
                }
                _ if Self::is_line_comment(cursor) => break,
                _ if Self::try_slash_star_comment(cursor).is_some() => {}
                _ => {
                    let span = content.get_or_insert(cursor.pos()..cursor.pos());
                    cursor.bump();
                    cursor.eat_while(|b| !Self::class(b).contains(Self::ARG_STOP));
                    span.end = cursor.pos();
                }
            }
        }
        match content {
            None => {
                cursor.restore(save);
                None
            }
            Some(mut span) => {
                cursor.restore(span.end);
                span.end = Self::trim_trailing_hspace(cursor);
                cursor.restore(span.end);
                Some(span)
            }
        }
    }

    // Eats leading gap chars.
    // Returns true if next item is the first on its physical line.
    #[inline]
    fn lex_preamble(cursor: &mut Cursor<'_>) -> bool {
        let mut starts_line = cursor.pos() == 0;
        loop {
            if !cursor.eat_while(|b| Self::is_end_of_line(b)).is_empty() {
                starts_line = true;
                continue;
            }
            if !cursor
                .eat_while(|b| Self::is_horizontal_whitespace(b))
                .is_empty()
            {
                continue;
            }
            if Self::eat_line_separator(cursor) {
                continue;
            }
            break;
        }
        starts_line
    }

    fn try_symbol_kind(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        let symbol_start = cursor.pos();

        let _ = cursor.eat(b'@') || cursor.eat(b'$');

        match cursor.peek()? {
            b'0'..=b'9' => {
                let _ = cursor.eat_while(|b| b.is_ascii_digit());
                let symbol_end = cursor.pos();
                let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                if cursor.eat(b':') {
                    Some(source::Kind::Label {
                        name: symbol_start..symbol_end,
                    })
                } else {
                    let _ = Self::lex_args(cursor);
                    Some(source::Kind::Unknown)
                }
            }
            b'"' => {
                let string = Self::eat_double_quoted(cursor);
                let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                if cursor.eat(b':') {
                    // string is closed or ':' would've been swallowed
                    let (name_start, name_end) = (string.start + 1, string.end - 1);
                    Some(source::Kind::Label {
                        name: name_start..name_end,
                    })
                } else {
                    let _ = Self::lex_args(cursor);
                    Some(source::Kind::Unknown)
                }
            }
            b'\'' => {
                let string = Self::eat_single_quoted(cursor);
                let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                if cursor.eat(b':') {
                    // string is closed or ':' would've been swallowed
                    let (name_start, name_end) = (string.start + 1, string.end - 1);
                    Some(source::Kind::Label {
                        name: name_start..name_end,
                    })
                } else {
                    let _ = Self::lex_args(cursor);
                    Some(source::Kind::Unknown)
                }
            }
            b if Self::class(b).contains(Self::SYMBOL_START) => {
                cursor.bump();
                let _ = cursor.eat_while(|b| Self::class(b).contains(Self::SYMBOL_CONTINUE));
                let symbol_end = cursor.pos();
                let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                if cursor.eat(b':') {
                    return Some(source::Kind::Label {
                        name: symbol_start..symbol_end,
                    });
                }

                if cursor.peek().is_some_and(|b| b == b'=') {
                    let keyword_start = cursor.pos();
                    cursor.eat(b'=');
                    let keyword_end = cursor.pos();
                    let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                    let args = Self::lex_args(cursor);
                    return Some(source::Kind::Definition {
                        symbol: symbol_start..symbol_end,
                        keyword: keyword_start..keyword_end,
                        args,
                    });
                }

                cursor.restore(symbol_end);
                let args = Self::lex_args(cursor);

                if b == b'.' {
                    Some(source::Kind::Directive {
                        name: symbol_start..symbol_end,
                        args,
                    })
                } else {
                    Some(source::Kind::Instruction {
                        mnemonic: symbol_start..symbol_end,
                        args,
                    })
                }
            }
            _ => {
                Self::lex_args(cursor);
                Some(source::Kind::Unknown)
            }
        }
    }
}

impl<T: LlvmTarget> Dialect for Llvm<T> {
    fn next_item(cur: &mut Cursor<'_>) -> Option<Item> {
        let is_first = Self::lex_preamble(cur);

        let start = cur.pos();

        let kind = Self::try_linemarker(cur)
            .or_else(|| Self::try_slash_star_comment(cur))
            .or_else(|| Self::try_line_comment(cur))
            .or_else(|| Self::try_symbol_kind(cur))?;

        Some(Item {
            kind,
            span: start..cur.pos(),
            starts_line: is_first,
        })
    }
}
