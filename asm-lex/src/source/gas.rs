#[cfg(test)]
mod tests;

pub mod targets;

use crate::cursor::Cursor;
use crate::pattern::{ByteSet, Class, ClassTable};
use crate::source;
use crate::source::{Dialect, Item};
use crate::Span;

pub trait GasTarget {
    // Anything from byte to newline is a comment, can be placed anywhere
    const COMMENT_CHARS: ByteSet;

    // Anything from byte to newline is a comment, must be first character
    const LINE_COMMENT_CHARS: ByteSet;

    // Anything from byte sequence to newline is a comment, can be placed anywhere
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]];

    // The starting bytes of multi-byte comment sequence
    const MULTI_COMMENT_START: ByteSet = ByteSet::from_first_bytes(Self::MULTI_COMMENT_CHARS);

    // A statement ends at a newline or line separator
    const LINE_SEPARATOR_CHARS: ByteSet;

    // Characters that symbols can start with
    const SYMBOL_START_CHARS: ByteSet;

    // Characters that may follow
    const SYMBOL_CONTINUE_CHARS: ByteSet;

    // Whether (55$:) is a valid label or not
    const LOCAL_LABELS_DOLLAR: bool = false;

    // Whether linemarker preprocessor directives are enabled
    const HAS_LINEMARKERS: bool = Self::LINE_COMMENT_CHARS.contains(b'#');
}

pub struct Gas<T: GasTarget> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: GasTarget> Gas<T> {
    // Characters not included in the spans of Items
    const GAP_CHARS: ByteSet = ByteSet::from_bytes(b" \t\r\n").with_set(&T::LINE_SEPARATOR_CHARS);

    // Horizontal whitespace characters
    const HSPACE_CHARS: ByteSet = ByteSet::from_bytes(b" \t\r");

    // Characters at which lex_args may be interrupted
    const ARG_STOP_CHARS: ByteSet = ByteSet::from_bytes(b"\n\"/")
        .with_set(&Self::HSPACE_CHARS) // will be removed
        .with_set(&T::LINE_SEPARATOR_CHARS)
        .with_set(&T::COMMENT_CHARS)
        .with_set(&T::MULTI_COMMENT_START);

    // Characters that comments or linemarkers start with
    const TRIVIA_START_CHARS: ByteSet = ByteSet::from_bytes(b"/")
        .with_set(&T::MULTI_COMMENT_START)
        .with_set(&T::LINE_COMMENT_CHARS)
        .with_set(&T::COMMENT_CHARS);

    const COMMENT: Class = Class::with_bit(0);
    const LINE_COMMENT: Class = Class::with_bit(1);
    const MULTI_START: Class = Class::with_bit(2);
    const LINE_SEPARATOR: Class = Class::with_bit(3);
    const SYMBOL_START: Class = Class::with_bit(4);
    const SYMBOL_CONTINUE: Class = Class::with_bit(5);
    const GAP: Class = Class::with_bit(6);
    const HSPACE: Class = Class::with_bit(7);
    const ARG_STOP: Class = Class::with_bit(8);
    const TRIVIA_START: Class = Class::with_bit(9);

    const TABLE: ClassTable = ClassTable::build(&[
        (Self::COMMENT, &T::COMMENT_CHARS),
        (Self::LINE_COMMENT, &T::LINE_COMMENT_CHARS),
        (Self::MULTI_START, &T::MULTI_COMMENT_START),
        (Self::LINE_SEPARATOR, &T::LINE_SEPARATOR_CHARS),
        (Self::SYMBOL_START, &T::SYMBOL_START_CHARS),
        (Self::SYMBOL_CONTINUE, &T::SYMBOL_CONTINUE_CHARS),
        (Self::GAP, &Self::GAP_CHARS),
        (Self::HSPACE, &Self::HSPACE_CHARS),
        (Self::ARG_STOP, &Self::ARG_STOP_CHARS),
        (Self::TRIVIA_START, &Self::TRIVIA_START_CHARS),
    ]);

    #[inline]
    fn class(b: u8) -> Class {
        Self::TABLE.classify(b)
    }

    #[inline]
    fn is_horizontal_whitespace(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\r')
    }

    // return position of last non-whitespace byte
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

    // Handles escape quotes.
    // Non-terminated strings go to EOF.
    fn eat_string(cursor: &mut Cursor<'_>) -> Span {
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

    // Eats leading gap chars.
    // Returns true if next item is the first on its physical line.
    fn lex_preamble(cursor: &mut Cursor<'_>) -> bool {
        let mut starts_line = cursor.pos() == 0;
        while let Some(b) = cursor.peek() {
            if !Self::class(b).contains(Self::GAP) {
                break;
            }
            if b == b'\n' {
                starts_line = true;
            }
            cursor.bump();
        }
        starts_line
    }

    fn try_line_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        let b = cursor.peek()?;

        if !Self::class(b).contains(Self::LINE_COMMENT) {
            return None;
        }

        let mut is_linemarker = b == b'#' && cursor.at_line_start();
        cursor.bump();
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        is_linemarker &= !cursor.eat_while(|b| b.is_ascii_digit()).is_empty();
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        is_linemarker &= !Self::eat_string(cursor).is_empty();
        let mut linemarker_end = cursor.pos();
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));

        while !cursor.eat_while(|b| b.is_ascii_digit()).is_empty() {
            linemarker_end = cursor.pos();
            let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        }

        if is_linemarker && matches!(cursor.peek(), Some(b'\n') | None) {
            cursor.restore(linemarker_end);
            Some(source::Kind::Preprocessor)
        } else {
            let _ = cursor.eat_while(|b| b != b'\n');
            cursor.restore(Self::trim_trailing_hspace(cursor));
            Some(source::Kind::Comment)
        }
    }

    fn is_comment(cursor: &Cursor<'_>) -> bool {
        cursor
            .peek()
            .is_some_and(|b| Self::class(b).contains(Self::COMMENT))
    }

    fn try_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if !Self::is_comment(cursor) {
            return None;
        }

        let mut comment_end = cursor.pos();
        while let Some(b) = cursor.peek() {
            match b {
                b'\n' => break,
                b if Self::is_horizontal_whitespace(b) => {
                    let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                }
                _ => {
                    let _ = cursor.eat_while(|b| b != b'\n' && !Self::is_horizontal_whitespace(b));
                    comment_end = cursor.pos();
                }
            }
        }
        cursor.restore(comment_end);
        Some(source::Kind::Comment)
    }

    // assumption: /* runs to eof if not closed
    fn is_slash_star_comment(cursor: &Cursor<'_>) -> bool {
        cursor.starts_with(b"/*")
    }

    fn try_slash_star_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if !Self::is_slash_star_comment(cursor) {
            return None;
        }
        while let Some(b) = cursor.bump() {
            if b == b'*' && cursor.eat(b'/') {
                break;
            }
        }
        Some(source::Kind::Comment)
    }

    fn is_multibyte_comment(cursor: &Cursor<'_>) -> bool {
        if cursor
            .peek()
            .is_some_and(|b| Self::class(b).contains(Self::MULTI_START))
        {
            for pattern in T::MULTI_COMMENT_CHARS {
                if cursor.peek() == Some(pattern[0]) && cursor.seek(1) == Some(pattern[1]) {
                    return true;
                }
            }
        }
        false
    }

    fn try_multibyte_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if cursor
            .peek()
            .is_some_and(|b| Self::class(b).contains(Self::MULTI_START))
        {
            for pattern in T::MULTI_COMMENT_CHARS {
                if cursor.peek() == Some(pattern[0]) && cursor.seek(1) == Some(pattern[1]) {
                    cursor.eat_while(|b| b != b'\n');
                    return Some(source::Kind::Comment);
                }
            }
        }
        None
    }

    fn lex_args(cursor: &mut Cursor<'_>) -> Option<Span> {
        let save = cursor.pos();
        let mut content: Option<Span> = None;

        while let Some(b) = cursor.peek() {
            match b {
                b'\n' => {
                    break;
                }
                _ if Self::class(b).contains(Self::LINE_SEPARATOR) => {
                    break;
                }
                _ if Self::class(b).contains(Self::COMMENT) => {
                    break;
                }
                _ if Self::is_multibyte_comment(cursor) => {
                    break;
                }
                b'"' => {
                    let span = content.get_or_insert(cursor.pos()..cursor.pos());
                    Self::eat_string(cursor);
                    span.end = cursor.pos();
                }
                _ if Self::is_slash_star_comment(cursor) => {
                    Self::try_slash_star_comment(cursor);
                }
                _ if Self::is_horizontal_whitespace(b) => {
                    cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
                }
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
            Some(span) => {
                cursor.restore(span.end);
                Some(span)
            }
        }
    }

    fn try_symbol_kind(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        let symbol_start = cursor.pos();

        match cursor.peek()? {
            b'0'..=b'9' => {
                let _ = cursor.eat_while(|b| b.is_ascii_digit());
                if T::LOCAL_LABELS_DOLLAR {
                    cursor.eat(b'$');
                }
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
                let string = Self::eat_string(cursor);
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
            b if T::SYMBOL_START_CHARS.contains(b) => {
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
                    cursor.eat(b'='); // '==' for .eqv
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

impl<T: GasTarget> Dialect for Gas<T> {
    fn next_item(cur: &mut Cursor<'_>) -> Option<Item> {
        let is_first = Self::lex_preamble(cur);

        let start = cur.pos();

        let kind = Self::try_slash_star_comment(cur)
            .or_else(|| Self::try_line_comment(cur))
            .or_else(|| Self::try_multibyte_comment(cur))
            .or_else(|| Self::try_comment(cur))
            .or_else(|| Self::try_symbol_kind(cur))?;

        Some(Item {
            kind,
            span: start..cur.pos(),
            starts_line: is_first,
        })
    }
}
