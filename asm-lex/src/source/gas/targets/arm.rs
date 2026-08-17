use crate::byte;
use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct ArmGenericElf;
pub struct ArmLinuxElf;
pub struct ArmLinuxEabiElf;
pub struct ArmPe;

// tc-arm | te-generic | obj-elf
impl GasTarget for ArmGenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
    const HAS_DOLLAR_LOCAL_LABELS: bool = true;
}

// tc-arm | te-linux | obj-elf
impl GasTarget for ArmLinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-arm | te-armlinuxeabi | obj-elf
impl GasTarget for ArmLinuxEabiElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-arm | te-pe | obj-coff
impl GasTarget for ArmPe {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const SYMBOL_START_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_EXTENDED)
        .with_byte(b'@');
    const SYMBOL_CONTINUE_CHARS: Set = Set::from_bytes(b"._$")
        .with_set(&byte::ASCII_ALPHA)
        .with_set(&byte::ASCII_DIGIT)
        .with_set(&byte::ASCII_EXTENDED)
        .with_byte(b'@');
    const HAS_LOCAL_LABELS: bool = true;
}
