#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::Span;

fn check_lex_args<T: LlvmTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::lex_args(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_lex_args_x86_elf_and_darwin() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"a b c", Some(0..5), 5),
        (b"a b c ", Some(0..5), 5),
        (b" a b c", Some(1..6), 6),
        (b"a b c\n", Some(0..5), 5),
        (b"a b c\r", Some(0..5), 5),
        (b"a b c;", Some(0..5), 5),
        (b"a b c#", Some(0..5), 5),
        (b"a b c//", Some(0..5), 5),
        (b"a / b c", Some(0..7), 7),
        (b"'a' b c", Some(0..7), 7),
        (b"a 'b' c", Some(0..7), 7),
        (b"a '\n' c", Some(0..7), 7),
        (b"\"a\" b c", Some(0..7), 7),
        (b"a \"b\" c", Some(0..7), 7),
        (b"a \"\n\" c", Some(0..7), 7),
        (b"a /*...*/ b c", Some(0..13), 13),
        (b"a b c /*...*/", Some(0..5), 5),
        (b"", None, 0),
        (b"/*...*/", None, 0),
        (b"//...", None, 0),
    ];
    check_lex_args::<X86Elf>(cases);
    check_lex_args::<X86Darwin>(cases);
}

#[test]
fn try_lex_args_arm_elf_and_darwin() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"a b c", Some(0..5), 5),
        (b"a b c ", Some(0..5), 5),
        (b" a b c", Some(1..6), 6),
        (b"a b c\n", Some(0..5), 5),
        (b"a b c\r", Some(0..5), 5),
        (b"a b c@", Some(0..5), 5),
        (b"a b c;", Some(0..5), 5),
        (b"a b c//", Some(0..5), 5),
        (b"a # c#", Some(0..6), 6),
        (b"a / b c", Some(0..7), 7),
        (b"'a' b c", Some(0..7), 7),
        (b"a 'b' c", Some(0..7), 7),
        (b"a '\n' c", Some(0..7), 7),
        (b"\"a\" b c", Some(0..7), 7),
        (b"a \"b\" c", Some(0..7), 7),
        (b"a \"\n\" c", Some(0..7), 7),
        (b"a /*...*/ b c", Some(0..13), 13),
        (b"a b c /*...*/", Some(0..5), 5),
        (b"", None, 0),
        (b"/*...*/", None, 0),
        (b"//...", None, 0),
    ];
    check_lex_args::<ArmElf>(cases);
    check_lex_args::<ArmDarwin>(cases);
}
