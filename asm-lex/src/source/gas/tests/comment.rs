#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::source::Kind;

fn check_is_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ln_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_comment(&cursor), *is_ln_cmnt);
    }
}

fn check_try_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn is_comment_with_hash_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"## ...", true),
        (b"### ...", true),
        (b"#...", true),
        (b"@ ...", false),
        (b"@@ ...", false),
        (b"@@@ ...", false),
        (b"@...", false),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_comment::<X86LinuxElf>(cases);
    check_is_comment::<RiscvGenericElf>(cases);
    check_is_comment::<NoLineSeparator>(cases);
    check_is_comment::<NoHashLineComment>(cases);
}

#[test]
fn is_comment_with_at_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"@...", true),
        (b"@ ...", true),
        (b"@@ ...", true),
        (b"@@@ ...", true),
        (b"#...", false),
        (b"# ...", false),
        (b"## ...", false),
        (b"### ...", false),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_comment::<ArmLinuxEabiElf>(cases);
}

#[test]
fn try_comment_with_hash_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"#...", Some(Comment), 4),
        (b"#... ", Some(Comment), 4),
        (b"#... \t", Some(Comment), 4),
        (b"#... \n...", Some(Comment), 4),
        (b"# ...", Some(Comment), 5),
        (b"# ... ", Some(Comment), 5),
        (b"# ... \t", Some(Comment), 5),
        (b"# ... \n...", Some(Comment), 5),
        (b"## ...", Some(Comment), 6),
        (b"## ... ", Some(Comment), 6),
        (b"## ... \t", Some(Comment), 6),
        (b"## ... \n...", Some(Comment), 6),
        (b"### ...", Some(Comment), 7),
        (b"### ... ", Some(Comment), 7),
        (b"### ... \t", Some(Comment), 7),
        (b"### ... \n...", Some(Comment), 7),
        (b"@...", None, 0),
        (b"@ ...", None, 0),
        (b"@@ ...", None, 0),
        (b"@@@ ...", None, 0),
        (b"nop", None, 0),
        (b"nop #", None, 0),
    ];
    check_try_comment::<X86LinuxElf>(cases);
    check_try_comment::<RiscvGenericElf>(cases);
    check_try_comment::<NoLineSeparator>(cases);
    check_try_comment::<NoHashLineComment>(cases);
}

#[test]
fn try_comment_with_at_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@...", Some(Comment), 4),
        (b"@ ...", Some(Comment), 5),
        (b"@@ ...", Some(Comment), 6),
        (b"@@@ ...", Some(Comment), 7),
        (b"#...", None, 0),
        (b"# ...", None, 0),
        (b"## ...", None, 0),
        (b"### ...", None, 0),
        (b"nop", None, 0),
        (b"nop #", None, 0),
    ];
    check_try_comment::<ArmLinuxEabiElf>(cases);
}
