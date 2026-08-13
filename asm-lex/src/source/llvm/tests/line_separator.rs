#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};

fn check_eat_line_separator<T: LlvmTarget>(cases: &[(&[u8], bool, usize)]) {
    for (bytes, res, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::eat_line_separator(&mut cursor), *res);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn eat_semicolon_line_separator() {
    let cases: &[(&[u8], bool, usize)] = &[
        (b";", true, 1),
        (b"; ", true, 1),
        (b";\t", true, 1),
        (b";;", true, 1),
        (b"", false, 0),
        (b"#", false, 0),
        (b"\n", false, 0),
    ];
    check_eat_line_separator::<X86Elf>(cases);
    check_eat_line_separator::<X86Darwin>(cases);
    check_eat_line_separator::<ArmElf>(cases);
    check_eat_line_separator::<ArmDarwin>(cases);
    check_eat_line_separator::<Aarch64Elf>(cases);
    check_eat_line_separator::<RiscvElf>(cases);
}

#[test]
fn eat_double_percent_line_separator() {
    let cases: &[(&[u8], bool, usize)] = &[
        (b"%%", true, 2),
        (b"%% ", true, 2),
        (b"%%\t", true, 2),
        (b"%%%%", true, 2),
        (b"", false, 0),
        (b";", false, 0),
        (b"\n", false, 0),
    ];
    check_eat_line_separator::<Aarch64Darwin>(cases);
    check_eat_line_separator::<RiscvDarwin>(cases);
}
