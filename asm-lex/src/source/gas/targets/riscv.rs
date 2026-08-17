use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct RiscvGenericElf;

// tc-riscv te-generic obj-elf
impl GasTarget for RiscvGenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
    const HAS_DOLLAR_LOCAL_LABELS: bool = true;
}
