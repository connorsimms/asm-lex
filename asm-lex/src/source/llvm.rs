pub mod targets;

// https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/MC/MCAsmInfo.h
pub trait LlvmTarget {
    const SEPARATOR_STR: &'static [u8] = b";";
    const COMMENT_STR: &'static [u8] = b"#";
    const ALLOW_ADDITIONAL_COMMENTS: bool = true;
    const AT_IN_IDENTIFIER: bool = false;
    const QUESTION_STARTS_IDENTIFIER: bool = false;
    const DOLLAR_STARTS_IDENTIFIER: bool = false;
    const AT_STARTS_IDENTIFIER: bool = false;
}
