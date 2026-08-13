#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::source::Kind;

fn check_is_slash_star_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ss_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_slash_star_comment(&cursor), *is_ss_cmnt);
    }
}

fn check_try_slash_star_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_slash_star_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn is_slash_star_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"/**/", true),
        (b"/*\n*/", true),
        (b"/***/", true),
        (b"/*\"\"*/", true),
        (b"/* ... */", true),
        (b"/* ...", true),
        (b"#/* ... */", false),
        (b"@/* ... */", false),
    ];
    check_is_slash_star_comment::<X86_64LinuxElf>(cases);
    check_is_slash_star_comment::<Aarch64LinuxElf>(cases);
    check_is_slash_star_comment::<ArmLinuxEabiElf>(cases);
    check_is_slash_star_comment::<Riscv64LinuxElf>(cases);
    check_is_slash_star_comment::<NoHashLineComment>(cases);
    check_is_slash_star_comment::<NonSlashMultibyte>(cases);
    check_is_slash_star_comment::<NoLineSeparator>(cases);
}

#[test]
fn try_slash_star_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"/**/", Some(Comment), 4),
        (b"/*\n*/", Some(Comment), 5),
        (b"/***/", Some(Comment), 5),
        (b"/*\"\"*/", Some(Comment), 6),
        (b"/* ... */", Some(Comment), 9),
        (b"/* ...", Some(Comment), 6),
        (b"#/* ... */", None, 0),
        (b"@/* ... */", None, 0),
    ];
    check_try_slash_star_comment::<X86_64LinuxElf>(cases);
    check_try_slash_star_comment::<Aarch64LinuxElf>(cases);
    check_try_slash_star_comment::<ArmLinuxEabiElf>(cases);
    check_try_slash_star_comment::<Riscv64LinuxElf>(cases);
    check_try_slash_star_comment::<NoHashLineComment>(cases);
    check_try_slash_star_comment::<NonSlashMultibyte>(cases);
    check_try_slash_star_comment::<NoLineSeparator>(cases);
}
