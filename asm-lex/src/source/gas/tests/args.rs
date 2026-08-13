#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::source::gas::{targets::*, GasTarget};

fn check_lex_args<T: GasTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::lex_args(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn lex_args() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        // no args
        (b"", None, 0),
        (b"/*...*/", None, 0),
        // whitespace
        (b"arg", Some(0..3), 3),
        (b" arg", Some(1..4), 4),
        (b"arg ", Some(0..3), 3),
        (b" arg ", Some(1..4), 4),
        // quoted
        (b"\"arg\"", Some(0..5), 5),
        (b"\"arg\" \"arg\"", Some(0..11), 11),
        (b"\"a\ng\" \"a\ng\"", Some(0..11), 11),
        // slash star
        (b"/*...*/arg", Some(7..10), 10),
        (b"arg/*...*/", Some(0..3), 3),
        (b"arg/*...*/arg", Some(0..13), 13),
        (b"arg/*...*/arg/*...*/", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(cases);
    check_lex_args::<Aarch64LinuxElf>(cases);
    check_lex_args::<ArmLinuxEabiElf>(cases);
    check_lex_args::<Riscv64LinuxElf>(cases);
    check_lex_args::<NoHashLineComment>(cases);
    check_lex_args::<NonSlashMultibyte>(cases);
    check_lex_args::<NoLineSeparator>(cases);
}

#[test]
fn lex_args_hash_comments() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"#...", None, 0),
        (b"arg#", Some(0..3), 3),
        (b" arg#", Some(1..4), 4),
        (b"arg #", Some(0..3), 3),
        (b" arg #", Some(1..4), 4),
        (b"arg arg#", Some(0..7), 7),
        (b"arg/*.#.*/arg #", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(cases);
    check_lex_args::<Riscv64LinuxElf>(cases);
    check_lex_args::<NoHashLineComment>(cases);
    check_lex_args::<NonSlashMultibyte>(cases);
    check_lex_args::<NoLineSeparator>(cases);
}

#[test]
fn lex_args_line_separators() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b";...", None, 0),
        (b"arg;", Some(0..3), 3),
        (b" arg;", Some(1..4), 4),
        (b"arg ;", Some(0..3), 3),
        (b" arg ;", Some(1..4), 4),
        (b"arg arg;", Some(0..7), 7),
        (b"arg/*.;.*/arg ;", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(cases);
    check_lex_args::<Aarch64LinuxElf>(cases);
    check_lex_args::<ArmLinuxEabiElf>(cases);
    check_lex_args::<Riscv64LinuxElf>(cases);
    check_lex_args::<NoHashLineComment>(cases);
    check_lex_args::<NonSlashMultibyte>(cases);
}

#[test]
fn lex_args_slash_multibyte_comment() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"//...", None, 0),
        (b"arg//", Some(0..3), 3),
        (b" arg//", Some(1..4), 4),
        (b"arg //", Some(0..3), 3),
        (b" arg //", Some(1..4), 4),
        (b"arg arg//", Some(0..7), 7),
        (b"arg/*.//.*/arg //", Some(0..14), 14),
    ];
    check_lex_args::<Aarch64LinuxElf>(cases);
    check_lex_args::<ArmLinuxEabiElf>(cases);
}

#[test]
fn lex_args_hash_line_comment() {
    // Line comment chars are not comments in arguments
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"#...", Some(0..4), 4),
        (b"arg#", Some(0..4), 4),
        (b" arg#", Some(1..5), 5),
        (b"arg #", Some(0..5), 5),
        (b" arg #", Some(1..6), 6),
        (b"arg arg#", Some(0..8), 8),
        (b"arg/*.#.*/arg #", Some(0..15), 15),
    ];
    check_lex_args::<Aarch64LinuxElf>(cases);
    check_lex_args::<ArmLinuxEabiElf>(cases);
}
