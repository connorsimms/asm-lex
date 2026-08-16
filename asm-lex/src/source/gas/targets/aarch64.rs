use crate::byte;
use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct Aarch64Elf;
pub struct Aarch64LinuxElf;
pub struct Aarch64GnuElf;

// tc-aarch64
// te-generic
// obj-elf
impl GasTarget for Aarch64Elf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
    const LOCAL_LABELS_DOLLAR: bool = true;
}

// tc-aarch64
// te-linux
// obj-elf
impl GasTarget for Aarch64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
}

// tc-aarch64
// te-gnu
// obj-elf
impl GasTarget for Aarch64GnuElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
}
