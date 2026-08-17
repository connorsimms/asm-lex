use crate::byte;
use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct Aarch64GenericElf;
pub struct Aarch64LinuxElf;
pub struct Aarch64Pe;

// tc-aarch64 | te-generic | obj-elf
impl GasTarget for Aarch64GenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
    const HAS_DOLLAR_LOCAL_LABELS: bool = true;
}

// tc-aarch64 | te-linux | obj-elf
impl GasTarget for Aarch64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const HAS_DOUBLESLASH_COMMENTS: bool = true;
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-aarch64 | te-pepaarch64 | obj-coff
impl GasTarget for Aarch64Pe {
    const COMMENT_CHARS: Set = Set::from_bytes(b"");
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
