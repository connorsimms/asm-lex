// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/RISCV/MCTargetDesc/RISCVMCAsmInfo.cpp

use crate::source::llvm::LlvmTarget;

pub struct RiscvElf;
pub struct RiscvDarwin;

impl LlvmTarget for RiscvElf {}

impl LlvmTarget for RiscvDarwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
}
