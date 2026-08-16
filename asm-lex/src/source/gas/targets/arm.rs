use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct ArmGenericElf;
pub struct ArmLinuxElf;
pub struct ArmLinuxEabiElf;

// tc-arm
// te-generic
// obj-elf
impl GasTarget for ArmGenericElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
    const LOCAL_LABELS_DOLLAR: bool = true;
}

// tc-arm
// te-linux
// obj-elf
impl GasTarget for ArmLinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
}

// tc-arm
// te-armlinuxeabi
// obj-elf
impl GasTarget for ArmLinuxEabiElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
    const LOCAL_LABELS: bool = true;
}
