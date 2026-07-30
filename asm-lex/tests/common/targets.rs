use asm_lex::pattern::ByteSet;
use asm_lex::source::gas::GasTarget;

#[allow(unused)]
struct NoHashLineComment {}
#[allow(unused)]
struct NonSlashMultibyte {}
#[allow(unused)]
struct NoLineSeparator {}

impl GasTarget for NoHashLineComment {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NonSlashMultibyte {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[b"@@"];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NoLineSeparator {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b"");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}
