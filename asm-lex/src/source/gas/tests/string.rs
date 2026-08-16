#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::Span;

fn check_eat_string<T: GasTarget>(cases: &[(&[u8], Span, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::eat_string(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn eat_string() {
    let cases: &[(&[u8], Span, usize)] = &[
        (b"", 0..0, 0),
        (b"\"\"...", 0..2, 2),
        (b"\"Text\"...", 0..6, 6),
        (b"\"\n\"...", 0..3, 3),
        (b"\";\"...", 0..3, 3),
        (b"\"\\\"\"...", 0..4, 4),
        (b"\"\\\\\"...", 0..4, 4),
        (b"\"@#/**///\"...", 0..10, 10),
    ];
    check_eat_string::<X86LinuxElf>(cases);
    check_eat_string::<Aarch64LinuxElf>(cases);
    check_eat_string::<ArmLinuxEabiElf>(cases);
    check_eat_string::<RiscvGenericElf>(cases);
    check_eat_string::<NoHashLineComment>(cases);
    check_eat_string::<NonSlashMultibyte>(cases);
    check_eat_string::<NoLineSeparator>(cases);
}
