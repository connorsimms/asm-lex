use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct Aarch64Elf;
pub struct Aarch64LinuxElf;
pub struct Aarch64GnuElf;

// tc-aarch64
// te-generic
// obj-elf
impl GasTarget for Aarch64Elf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS_DOLLAR: bool = true;
}

// tc-aarch64
// te-linux
// obj-elf
impl GasTarget for Aarch64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
}

// tc-aarch64
// te-gnu
// obj-elf
impl GasTarget for Aarch64GnuElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
}
