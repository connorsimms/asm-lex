#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};

fn check_lex_preamble<T: GasTarget>(cases: &[(&[u8], usize, bool, usize)]) {
    for (bytes, s_pos, starts_line, e_pos) in cases {
        let mut cursor = Cursor::new(bytes);
        cursor.advance(*s_pos);
        assert_eq!(Gas::<T>::lex_preamble(&mut cursor), *starts_line);
        assert_eq!(cursor.pos(), *e_pos);
    }
}

#[test]
fn no_separators() {
    let cases: &[(&[u8], usize, bool, usize)] = &[
        (b"", 0, true, 0),
        (b"Item", 0, true, 0),
        (b"Item", 4, false, 4),
        (b" Item", 0, true, 1),
        (b"\tItem", 0, true, 1),
        (b"\nItem", 0, true, 1),
        (b"Item\nItem", 4, true, 5),
        (b"Item\n\nItem", 4, true, 6),
    ];
    check_lex_preamble::<X86LinuxElf>(cases);
    check_lex_preamble::<Aarch64LinuxElf>(cases);
    check_lex_preamble::<ArmLinuxEabiElf>(cases);
    check_lex_preamble::<RiscvGenericElf>(cases);
    check_lex_preamble::<NoHashLineComment>(cases);
    check_lex_preamble::<NoLineSeparator>(cases);
}

#[test]
fn semicolon_separators() {
    let cases: &[(&[u8], usize, bool, usize)] = &[
        (b";Item", 0, true, 1),
        (b";;Item", 0, true, 2),
        (b"Item;Item", 4, false, 5),
        (b"Item;;Item", 4, false, 6),
        (b"Item\n;;Item", 4, true, 7),
        (b"Item;\n;Item", 4, true, 7),
        (b"Item;;\nItem", 4, true, 7),
    ];
    check_lex_preamble::<X86LinuxElf>(cases);
    check_lex_preamble::<Aarch64LinuxElf>(cases);
    check_lex_preamble::<ArmLinuxEabiElf>(cases);
    check_lex_preamble::<RiscvGenericElf>(cases);
    check_lex_preamble::<NoHashLineComment>(cases);
}
