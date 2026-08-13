use super::LlvmTarget;

pub struct X86Elf {}
pub struct X86Darwin {}
pub struct ArmElf {}
pub struct ArmDarwin {}
pub struct Aarch64Elf {}
pub struct Aarch64Darwin {}
pub struct RiscvElf {}
pub struct RiscvDarwin {}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/X86/MCTargetDesc/X86MCAsmInfo.cpp
impl LlvmTarget for X86Elf {
    const USE_AT_FOR_SPECIFIER: bool = true;
}
impl LlvmTarget for X86Darwin {
    const COMMENT_STR: &'static [u8] = b"##";
    const USE_AT_FOR_SPECIFIER: bool = true;
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/ARM/MCTargetDesc/ARMMCAsmInfo.cpp
impl LlvmTarget for ArmElf {
    const COMMENT_STR: &'static [u8] = b"@";
    const USE_AT_FOR_SPECIFIER: bool = true;
}
impl LlvmTarget for ArmDarwin {
    const COMMENT_STR: &'static [u8] = b"@";
    const USE_AT_FOR_SPECIFIER: bool = true;
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/AArch64/MCTargetDesc/AArch64MCAsmInfo.cpp
impl LlvmTarget for Aarch64Elf {
    const COMMENT_STR: &'static [u8] = b"//";
}
impl LlvmTarget for Aarch64Darwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/RISCV/MCTargetDesc/RISCVMCAsmInfo.cpp
impl LlvmTarget for RiscvElf {}
impl LlvmTarget for RiscvDarwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
}
