use super::*;

pub struct X86_64LinuxElf;
pub struct Aarch64LinuxElf;
pub struct ArmLinuxEabiElf;
pub struct Riscv64LinuxElf;

impl GasTarget for X86_64LinuxElf {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
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

impl GasTarget for Aarch64LinuxElf {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[b"//"];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for ArmLinuxEabiElf {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"@");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[b"//"];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for Riscv64LinuxElf {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
    const LOCAL_LABELS_DOLLAR: bool = true;
}
