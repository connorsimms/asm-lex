// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/ARM/MCTargetDesc/ARMMCAsmInfo.cpp

use crate::source::llvm::LlvmTarget;

pub struct ArmDarwin;
pub struct ArmElf;
pub struct ArmMicrosoftCoff;
pub struct ArmGnuCoff;

impl LlvmTarget for ArmDarwin {
    const COMMENT_STR: &'static [u8] = b"@";
    const USE_AT_FOR_SPECIFIER: bool = true;
    const INLINE_ASM_START: &'static [u8] = b" InlineAsm Start";
    const INLINE_ASM_END: &'static [u8] = b" InlineAsm End";
}

impl LlvmTarget for ArmElf {
    const COMMENT_STR: &'static [u8] = b"@";
}

impl LlvmTarget for ArmMicrosoftCoff {
    const COMMENT_STR: &'static [u8] = b"@";
    const USE_AT_FOR_SPECIFIER: bool = true;
}

impl LlvmTarget for ArmGnuCoff {
    const COMMENT_STR: &'static [u8] = b"@";
}
