#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::source::Kind;

fn check_is_multibyte_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ln_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_multibyte_comment(&cursor), *is_ln_cmnt);
    }
}

#[test]
fn is_slash_multibyte_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"//...", true),
        (b"// ...", true),
        (b"/// ...", true),
        (b"//// ...", true),
        (b"# ...", false),
        (b"@ ...", false),
        (b"nop ...", false),
        (b"nop//...", false),
    ];
    check_is_multibyte_comment::<Aarch64LinuxElf>(cases);
    check_is_multibyte_comment::<ArmLinuxEabiElf>(cases);
}

#[test]
fn is_nonslash_multibyte_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"@@...", true),
        (b"@@ ...", true),
        (b"@@@ ...", true),
        (b"@@@@ ...", true),
        (b"// ...", false),
        (b"# ...", false),
        (b"@ ...", false),
        (b"nop ...", false),
        (b"nop@@...", false),
    ];
    check_is_multibyte_comment::<NonSlashMultibyte>(cases);
}

fn check_try_multibyte_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_multibyte_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_slash_multibyte_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"//...", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"/// ...", Some(Comment), 7),
        (b"//// ...", Some(Comment), 8),
        (b"# ...", None, 0),
        (b"@ ...", None, 0),
        (b"nop ...", None, 0),
        (b"nop//...", None, 0),
    ];
    check_try_multibyte_comment::<Aarch64LinuxElf>(cases);
    check_try_multibyte_comment::<ArmLinuxEabiElf>(cases);
}

#[test]
fn try_nonslash_multibyte_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@@...", Some(Comment), 5),
        (b"@@ ...", Some(Comment), 6),
        (b"@@@ ...", Some(Comment), 7),
        (b"@@@@ ...", Some(Comment), 8),
        (b"// ...", None, 0),
        (b"# ...", None, 0),
        (b"@ ...", None, 0),
        (b"nop ...", None, 0),
        (b"nop@@...", None, 0),
    ];
    check_try_multibyte_comment::<NonSlashMultibyte>(cases);
}
