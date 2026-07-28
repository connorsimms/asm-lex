#[cfg(test)]
mod tests;

pub mod targets;

use crate::cursor::Cursor;
use crate::pattern::ByteSet;
use crate::source;
use crate::source::{Dialect, Item};
use crate::Span;

pub trait GasTarget {
    // Anything from byte to newline is a comment, can be placed anywhere
    const COMMENT_CHARS: ByteSet;

    // Anything from byte to newline is a comment, must be first character
    const LINE_COMMENT_CHARS: ByteSet;

    // Anything from byte sequence to newline is a comment, can be placed anywhere
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]];

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

    // Characters not included in the spans of Items
    const GAP_CHARS: ByteSet =
        ByteSet::from_bytes(b" \t\r\n").with_set(&Self::LINE_SEPARATOR_CHARS);
}

pub struct Gas<T: GasTarget> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: GasTarget> Gas<T> {
    fn is_horizontal_whitespace(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\r')
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
            if !T::GAP_CHARS.contains(b) {
                break;
            }
            if b == b'\n' {
                starts_line = true;
            }
            cursor.bump();
        }
        starts_line
    }

    fn try_linemarker(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        let save = cursor.pos();
        if !cursor.at_line_start() {
            cursor.restore(save);
            return None;
        }
        if !cursor.eat(b'#') {
            cursor.restore(save);
            return None;
        }
        if !T::LINE_COMMENT_CHARS.contains(b'#') {
            cursor.restore(save);
            return None;
        }
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        if cursor.eat_while(|b| b.is_ascii_digit()).is_empty() {
            cursor.restore(save);
            return None;
        }
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        if Self::eat_string(cursor).is_empty() {
            cursor.restore(save);
            return None;
        }
        let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        while !cursor.eat_while(|b| b.is_ascii_digit()).is_empty() {
            let _ = cursor.eat_while(|b| Self::is_horizontal_whitespace(b));
        }
        if matches!(cursor.peek(), Some(b'\n') | None) {
            Some(source::Kind::Preprocessor)
        } else {
            cursor.restore(save);
            None
        }
    }

    fn is_line_comment(cursor: &Cursor<'_>) -> bool {
        cursor
            .peek()
            .is_some_and(|b| T::LINE_COMMENT_CHARS.contains(b))
    }

    fn try_line_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if Self::is_line_comment(cursor) {
            let _trivia = cursor.eat_while(|b| b != b'\n');
            Some(source::Kind::Comment)
        } else {
            None
        }
    }

    fn is_comment(cursor: &Cursor<'_>) -> bool {
        cursor.peek().is_some_and(|b| T::COMMENT_CHARS.contains(b))
    }

    fn try_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if !Self::is_comment(cursor) {
            return None;
        }
        let _trivia = cursor.eat_while(|b| b != b'\n');
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
            .is_some_and(|b| T::MULTI_COMMENT_START.contains(b))
        {
            for pattern in T::MULTI_COMMENT_CHARS {
                if cursor.starts_with(pattern) {
                    return true;
                }
            }
        }
        false
    }

    fn try_multibyte_comment(cursor: &mut Cursor<'_>) -> Option<source::Kind> {
        if cursor
            .peek()
            .is_some_and(|b| T::MULTI_COMMENT_START.contains(b))
        {
            for pattern in T::MULTI_COMMENT_CHARS {
                if cursor.starts_with(pattern) {
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
                _ if T::LINE_SEPARATOR_CHARS.contains(b) => {
                    break;
                }
                _ if T::COMMENT_CHARS.contains(b) => {
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
                    cursor.eat_while(|b| {
                        !matches!(b, b'\n' | b'"' | b'/')
                            && !Self::is_horizontal_whitespace(b)
                            && !T::LINE_SEPARATOR_CHARS.contains(b)
                            && !T::COMMENT_CHARS.contains(b)
                            && !T::MULTI_COMMENT_START.contains(b)
                    });
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
                let _ = cursor.eat_while(|b| T::SYMBOL_CONTINUE_CHARS.contains(b));
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

        let kind = Self::try_linemarker(cur)
            .or_else(|| Self::try_slash_star_comment(cur))
            .or_else(|| Self::try_multibyte_comment(cur))
            .or_else(|| Self::try_line_comment(cur))
            .or_else(|| Self::try_comment(cur))
            .or_else(|| Self::try_symbol_kind(cur))?;

        Some(Item {
            kind,
            span: start..cur.pos(),
            starts_line: is_first,
        })
    }
}
