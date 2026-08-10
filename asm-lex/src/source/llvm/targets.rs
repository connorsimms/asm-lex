use super::LlvmTarget;

// not sure if <arch><os><obj> is ideal
pub struct X86LinuxElf {}
pub struct X86Darwin {}
pub struct ArmLinuxElf {}
pub struct ArmDarwin {}
pub struct Aarch64LinuxElf {}
pub struct Aarch64Darwin {}
pub struct RiscvLinuxElf {}
pub struct RiscvDarwin {}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/X86/MCTargetDesc/X86MCAsmInfo.cpp
impl LlvmTarget for X86LinuxElf {}
impl LlvmTarget for X86Darwin {
    const COMMENT_STR: &'static [u8] = b"##";
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/ARM/MCTargetDesc/ARMMCAsmInfo.cpp
impl LlvmTarget for ArmLinuxElf {
    const COMMENT_STR: &'static [u8] = b"@";
}
impl LlvmTarget for ArmDarwin {
    const COMMENT_STR: &'static [u8] = b"@";
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/AArch64/MCTargetDesc/AArch64MCAsmInfo.cpp
impl LlvmTarget for Aarch64LinuxElf {
    const COMMENT_STR: &'static [u8] = b"//";
}
impl LlvmTarget for Aarch64Darwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
}

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/RISCV/MCTargetDesc/RISCVMCAsmInfo.cpp
impl LlvmTarget for RiscvLinuxElf {}
impl LlvmTarget for RiscvDarwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
}
