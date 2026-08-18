// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/AArch64/MCTargetDesc/AArch64MCAsmInfo.cpp

use crate::source::llvm::LlvmTarget;

pub struct Aarch64Darwin;
pub struct Aarch64Elf;
pub struct Aarch64MicrosoftCoff;
pub struct Aarch64GnuCoff;

impl LlvmTarget for Aarch64Darwin {
    const SEPARATOR_STR: &'static [u8] = b"%%";
    const COMMENT_STR: &'static [u8] = b";";
    const INLINE_ASM_START: &'static [u8] = b" InlineAsm Start";
    const INLINE_ASM_END: &'static [u8] = b" InlineAsm End";
}

impl LlvmTarget for Aarch64Elf {
    const COMMENT_STR: &'static [u8] = b"//";
}

impl LlvmTarget for Aarch64MicrosoftCoff {
    const COMMENT_STR: &'static [u8] = b"//";
    const USE_AT_FOR_SPECIFIER: bool = true;
}

impl LlvmTarget for Aarch64GnuCoff {
    const COMMENT_STR: &'static [u8] = b"//";
    const USE_AT_FOR_SPECIFIER: bool = true;
}
