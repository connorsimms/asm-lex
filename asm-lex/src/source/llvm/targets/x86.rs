// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/X86/MCTargetDesc/X86MCAsmInfo.cpp

use crate::source::llvm::LlvmTarget;

pub struct X86Darwin;
pub struct X86Elf;
pub struct X86Microsoft;
pub struct X86GnuCoff;

impl LlvmTarget for X86Darwin {
    const COMMENT_STR: &'static [u8] = b"##";
    const USE_AT_FOR_SPECIFIER: bool = true;
    const INLINE_ASM_START: &'static [u8] = b" InlineAsm Start";
    const INLINE_ASM_END: &'static [u8] = b" InlineAsm End";
}

impl LlvmTarget for X86Elf {
    const USE_AT_FOR_SPECIFIER: bool = true;
}

impl LlvmTarget for X86Microsoft {
    const USE_AT_FOR_SPECIFIER: bool = true;
}

impl LlvmTarget for X86GnuCoff {
    const USE_AT_FOR_SPECIFIER: bool = true;
}
