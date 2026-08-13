#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;

fn check_is_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_comment(&cursor), *res);
    }
}

fn check_try_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn is_comment_x86_elf() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"// ...", true),
        (b"@ ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<X86Elf>(cases);
}

#[test]
fn is_comment_x86_darwin() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"// ...", true),
        (b"@ ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<X86Darwin>(cases);
}

#[test]
fn is_comment_arm_elf() {
    let cases: &[(&[u8], bool)] = &[
        (b"// ...", true),
        (b"@ ...", true),
        (b"# ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<ArmElf>(cases);
}

#[test]
fn try_comment_x86_elf() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<X86Elf>(cases);
}

#[test]
fn try_comment_x86_darwin() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<X86Darwin>(cases);
}

#[test]
fn try_comment_arm_elf() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@ ...", Some(Comment), 5),
        (b"@ ...\n", Some(Comment), 5),
        (b"@ ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"# ...", None, 0),
        (b"# ...\n", None, 0),
        (b"# ...\r", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<ArmElf>(cases);
}
