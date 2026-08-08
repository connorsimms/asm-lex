pub mod targets;

use crate::byte;
use crate::byte::ByteSet;

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

pub struct Llvm<T: LlvmTarget> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: LlvmTarget> Llvm<T> {
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"_.")
        .with_set(&byte::ASCII_ALPHA)
        .with_byte_if(b'?', T::QUESTION_STARTS_IDENTIFIER)
        .with_byte_if(b'$', T::DOLLAR_STARTS_IDENTIFIER)
        .with_byte_if(b'@', T::AT_STARTS_IDENTIFIER);

    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"_.$?")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT)
        .with_byte_if(b'@', T::AT_IN_IDENTIFIER);
}
