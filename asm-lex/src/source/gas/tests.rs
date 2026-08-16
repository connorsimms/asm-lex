mod args;
mod comment;
mod line_comment;
mod linemarker;
mod multibyte_comment;
mod preamble;
mod slash_star;
mod string;
mod symbol_kind;

use crate::byte::Set;
use crate::source::gas::GasTarget;

// Synthetic targets for testing
struct NoHashLineComment {}
struct NonSlashMultibyte {}
struct NoLineSeparator {}

impl GasTarget for NoHashLineComment {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
}

impl GasTarget for NonSlashMultibyte {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"@@"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
}

impl GasTarget for NoLineSeparator {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b"");
    const LOCAL_LABELS: bool = true;
}
