use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct Aarch64GenericElf;
pub struct Aarch64LinuxElf;
pub struct Aarch64GnuElf;

// tc-aarch64
// te-generic
// obj-elf
impl GasTarget for Aarch64GenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
    const HAS_DOLLAR_LOCAL_LABELS: bool = true;
}

// tc-aarch64
// te-linux
// obj-elf
impl GasTarget for Aarch64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-aarch64
// te-gnu
// obj-elf
impl GasTarget for Aarch64GnuElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}
