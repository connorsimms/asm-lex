use crate::byte;
use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct X86GenericElf;
pub struct X86LinuxElf;
pub struct X86Darwin;
pub struct X86Pe;

// tc-i386 te-generic obj-elf
impl GasTarget for X86GenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386 te-linux obj-elf
impl GasTarget for X86LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386 te-generic obj-macho
impl GasTarget for X86Darwin {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const HAS_LOCAL_LABELS: bool = true;
}

// tc-i386 te-pe obj-coff
impl GasTarget for X86Pe {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
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
