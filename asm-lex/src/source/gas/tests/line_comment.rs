#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::source::gas::{targets::*, GasTarget};

fn check<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_line_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn with_hash_ln_comment() {
    use Kind::{Comment, Preprocessor};
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        // whitespace
        (b"# 100 \"file\"", Some(Preprocessor), 12),
        (b"#100 \"file\"", Some(Preprocessor), 11),
        (b"# 100\"file\"", Some(Preprocessor), 11),
        (b"#100\"file\"", Some(Preprocessor), 10),
        (b"#\t100\t\"file\"", Some(Preprocessor), 12),
        (b"# 100 \"file\"  ", Some(Preprocessor), 12),
        (b"# 100 \"file\"\t ", Some(Preprocessor), 12),
        // inner string
        (b"# 100 \"\"", Some(Preprocessor), 8),
        (b"# 100 \"\n\"", Some(Preprocessor), 9),
        (b"# 100 \";@#//\"", Some(Preprocessor), 13),
        // flags
        (b"# 100 \"filename\" 1", Some(Preprocessor), 18),
        (b"# 100 \"filename\"1", Some(Preprocessor), 17),
        (b"# 100 \"filename\" 1 2 3", Some(Preprocessor), 22),
        // comments
        (b"#", Some(Comment), 1),
        (b"# 100", Some(Comment), 5),
        (b"# junk", Some(Comment), 6),
        (b"# junk 100 \"filename\"", Some(Comment), 21),
        (b"# 100 junk \"filename\"", Some(Comment), 21),
        (b"# 100 \"filename\" junk", Some(Comment), 21),
        (b"# 100 \"filename\" 1 junk", Some(Comment), 23),
        (b"# ...", Some(Comment), 5),
        (b"# ... ", Some(Comment), 5),
        (b"# ...\t ", Some(Comment), 5),
        (b"# ...\n...", Some(Comment), 5),
        (b"## ...", Some(Comment), 6),
        (b"## ... ", Some(Comment), 6),
        (b"## ... \t", Some(Comment), 6),
        (b"## ...\n...", Some(Comment), 6),
        (b"### ...", Some(Comment), 7),
        (b"### ... ", Some(Comment), 7),
        (b"### ... \t", Some(Comment), 7),
        (b"### ... \n...", Some(Comment), 7),
        (b"#...", Some(Comment), 4),
        (b"#... ", Some(Comment), 4),
        (b"#... \t", Some(Comment), 4),
        (b"#... \n...", Some(Comment), 4),
        (b"", None, 0),
        (b"nop", None, 0),
        (b"nop#", None, 0),
    ];
    check::<X86_64LinuxElf>(cases);
    check::<Aarch64LinuxElf>(cases);
    check::<ArmLinuxEabiElf>(cases);
    check::<Riscv64LinuxElf>(cases);
    check::<NonSlashMultibyte>(cases);
    check::<NoLineSeparator>(cases);
}

#[test]
fn no_hash_ln_comment() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# 100 \"file\"", None, 0),
        (b"#100 \"file\"", None, 0),
        (b"# 100\"file\"", None, 0),
        (b"#100\"file\"", None, 0),
        (b"#\t100\t\"file\"", None, 0),
        (b"# 100 \"\"", None, 0),
        (b"# 100 \"\n\"", None, 0),
        (b"# 100 \";@#//\"", None, 0),
        (b"# 100 \"filename\" 1", None, 0),
        (b"# 100 \"filename\"1", None, 0),
        (b"# 100 \"filename\" 1 2 3", None, 0),
        (b"# 1000 \"filename\"", None, 0),
        (b"# ...", None, 0),
        (b"## ...", None, 0),
        (b"### ...", None, 0),
        (b"#...", None, 0),
        (b"nop", None, 0),
        (b"nop#...", None, 0),
    ];
    check::<NoHashLineComment>(cases);
}
