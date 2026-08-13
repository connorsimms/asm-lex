#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::Span;

fn check_try_single_quoted<T: LlvmTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_single_quoted(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

fn check_eat_double_quoted<T: LlvmTarget>(cases: &[(&[u8], Span, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::eat_double_quoted(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_single_quoted() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (br"'c'", Some(0..3), 3),
        (br"'\r'", Some(0..4), 4),
        (br"'\''", Some(0..4), 4),
        (br"'\\''", Some(0..4), 4),
        (br#"'\"'"#, Some(0..4), 4),
    ];
    check_try_single_quoted::<X86Elf>(cases);
    check_try_single_quoted::<X86Darwin>(cases);
    check_try_single_quoted::<ArmElf>(cases);
    check_try_single_quoted::<ArmDarwin>(cases);
    check_try_single_quoted::<Aarch64Elf>(cases);
    check_try_single_quoted::<Aarch64Darwin>(cases);
    check_try_single_quoted::<RiscvElf>(cases);
    check_try_single_quoted::<RiscvDarwin>(cases);
}

#[test]
fn eat_double_quoted() {
    let cases: &[(&[u8], Span, usize)] = &[
        (b"\"\"", 0..2, 2),
        (b"\"a\"", 0..3, 3),
        (b"\"a b\"", 0..5, 5),
        (b"\"a\nb\"", 0..5, 5),
        (b"\"a\tb\"", 0..5, 5),
        (b"\"a\\\"b\"", 0..6, 6),
    ];
    check_eat_double_quoted::<X86Elf>(cases);
    check_eat_double_quoted::<X86Darwin>(cases);
    check_eat_double_quoted::<ArmElf>(cases);
    check_eat_double_quoted::<ArmDarwin>(cases);
    check_eat_double_quoted::<Aarch64Elf>(cases);
    check_eat_double_quoted::<Aarch64Darwin>(cases);
    check_eat_double_quoted::<RiscvElf>(cases);
    check_eat_double_quoted::<RiscvDarwin>(cases);
}
