use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct X86GenericElf;
pub struct X86LinuxElf;
pub struct X86GnuElf;
pub struct X86Darwin;

// tc-i386
// te-generic
// obj-elf
impl GasTarget for X86GenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386
// te-linux
// obj-elf
impl GasTarget for X86LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386
// te-gnu
// obj-elf
impl GasTarget for X86GnuElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386
// te-generic
// obj-macho
impl GasTarget for X86Darwin {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}
