use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct ArmElf;
pub struct ArmLinuxElf;
pub struct ArmLinuxEabiElf;

impl GasTarget for ArmLinuxEabiElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"@");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[*b"//"];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
}
