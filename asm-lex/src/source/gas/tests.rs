mod args;
mod comment;
mod line_comment;
mod multibyte_comment;
mod preamble;
mod slash_star;
mod string;
mod symbol_kind;

use super::*;
use crate::source::Kind;

// Synthetic targets for testing
struct NoHashLineComment {}
struct NonSlashMultibyte {}
struct NoLineSeparator {}

impl GasTarget for NoHashLineComment {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&crate::byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&crate::byte::ASCII_ALPHA)
        .with_set(&crate::byte::ASCII_DIGIT);
}

impl GasTarget for NonSlashMultibyte {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"@@"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&crate::byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&crate::byte::ASCII_ALPHA)
        .with_set(&crate::byte::ASCII_DIGIT);
}

impl GasTarget for NoLineSeparator {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b"");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&crate::byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&crate::byte::ASCII_ALPHA)
        .with_set(&crate::byte::ASCII_DIGIT);
}
