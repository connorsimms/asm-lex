#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;

fn check_is_line_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_line_comment(&cursor), *res);
    }
}

fn check_try_line_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_line_comment(&mut cursor, true), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn is_line_comment_x86_elf() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ", true),
        (b"## ", true),
        (b"// ", true),
        (b"@@ ", false),
        (b";; ", false),
    ];
    check_is_line_comment::<X86Elf>(cases);
}

#[test]
// https://github.com/llvm/llvm-project/blob/main/llvm/test/MC/AsmParser/comments-x86-darwin.s
fn is_line_comment_x86_darwin() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ", true),
        (b"## ", true),
        (b"// ", true),
        (b"@@ ", false),
        (b";; ", false),
    ];
    check_is_line_comment::<X86Darwin>(cases);
}

#[test]
fn try_line_comment_x86_elf() {
    use Kind::{Comment, Preprocessor};
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"# 100 \"file\"\n", Some(Preprocessor), 12),
        (b"# 100 \"file\"\r", Some(Preprocessor), 12),
        (b"# 100 \"file\"", Some(Preprocessor), 12),
        (b"#100 \"file\"", Some(Preprocessor), 11),
        (b"#100\"file\"", Some(Preprocessor), 10),
        (b"# 100 \"\"", Some(Preprocessor), 8),
        (b"# 100 \"file\" junk", Some(Preprocessor), 17),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
        (b"! ...", None, 0),
    ];
    check_try_line_comment::<X86Elf>(cases);
}

#[test]
fn try_line_comment_x86_darwin() {
    use Kind::{Comment, Preprocessor};
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"## ...", Some(Comment), 6),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"# 100 \"file\"\n", Some(Preprocessor), 12),
        (b"# 100 \"file\"\r", Some(Preprocessor), 12),
        (b"# 100 \"file\"", Some(Preprocessor), 12),
        (b"#100 \"file\"", Some(Preprocessor), 11),
        (b"#100\"file\"", Some(Preprocessor), 10),
        (b"# 100 \"\"", Some(Preprocessor), 8),
        (b"# 100 \"file\" junk", Some(Preprocessor), 17),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
        (b"! ...", None, 0),
    ];
    check_try_line_comment::<X86Darwin>(cases);
}
