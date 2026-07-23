use super::*;

pub struct X86_64LinuxElf;

impl GasTarget for X86_64LinuxElf {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &[&[u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}
