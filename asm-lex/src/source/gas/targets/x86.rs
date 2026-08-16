use crate::byte::Set;
use crate::source::gas::GasTarget;

pub struct X86_64LinuxElf;

impl GasTarget for X86_64LinuxElf {
    const COMMENT_CHARS: Set = Set::from_bytes(b"#");
    const LINE_COMMENT_CHARS: Set = Set::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [[u8; 2]] = &[];
    const LINE_SEPARATOR_CHARS: Set = Set::from_bytes(b";");
}
