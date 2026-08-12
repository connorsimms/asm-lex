use asm_lex::byte::Set;
use asm_lex::source::gas::GasTarget;

#[allow(unused)]
struct NoHashLineComment {}
#[allow(unused)]
struct NonSlashMultibyte {}
#[allow(unused)]
struct NoLineSeparator {}

impl GasTarget for NoHashLineComment {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NonSlashMultibyte {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"@@"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NoLineSeparator {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b"");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}
