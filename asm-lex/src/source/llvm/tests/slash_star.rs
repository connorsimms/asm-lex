#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;

fn check_is_slash_star_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_slash_star_comment(&cursor), *res);
    }
}

fn check_try_slash_star_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_slash_star_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn is_slash_star_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"/* ...", true),
        (b"/ ...", false),
        (b"// ...", false),
        (b"# ...", false),
        (b"@ ...", false),
        (b"* ...", false),
    ];
    check_is_slash_star_comment::<X86Elf>(cases);
    check_is_slash_star_comment::<X86Darwin>(cases);
    check_is_slash_star_comment::<ArmElf>(cases);
}

#[test]
fn try_slash_star_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"/* ... */", Some(Comment), 9),
        (b"/* ... */ ", Some(Comment), 9),
        (b"/* ... */\t", Some(Comment), 9),
        (b"/* ... *//", Some(Comment), 9),
        (b"/* ... */#", Some(Comment), 9),
        (b"/* ... ", Some(Comment), 7),
        (b"// ... ", None, 0),
        (b"## ... ", None, 0),
        (b"@@ ... ", None, 0),
    ];
    check_try_slash_star_comment::<X86Elf>(cases);
    check_try_slash_star_comment::<X86Darwin>(cases);
    check_try_slash_star_comment::<ArmElf>(cases);
}
