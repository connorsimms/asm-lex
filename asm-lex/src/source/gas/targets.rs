use crate::byte;
use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct X86_64LinuxElf;
pub struct Aarch64LinuxElf;
pub struct ArmLinuxEabiElf;
pub struct Riscv64LinuxElf;

impl GasTarget for X86_64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
}

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

impl GasTarget for ArmLinuxEabiElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
}

impl GasTarget for Riscv64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$").with_set(&byte::ASCII_ALPHA);
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT);
    const LOCAL_LABELS_DOLLAR: bool = true;
}
