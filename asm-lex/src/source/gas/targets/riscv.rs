use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct RiscvGenericElf;

// tc-riscv
// te-generic
// obj-elf
impl GasTarget for RiscvGenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
    const LOCAL_LABELS_DOLLAR: bool = true;
}
