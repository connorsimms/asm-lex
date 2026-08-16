use super::*;
use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::source::Kind;

#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

fn check_try_linemarker<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_linemarker(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn with_hash_ln_comment() {
    use Kind::Preprocessor;
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
        (b"#", None, 0),
        (b"# 100", None, 0),
        (b"# junk", None, 0),
        (b"# junk 100 \"filename\"", None, 0),
        (b"# 100 junk \"filename\"", None, 0),
        (b"# 100 \"filename\" junk", None, 0),
        (b"# 100 \"filename\" 1 junk", None, 0),
        (b"# ...", None, 0),
        (b"# ... ", None, 0),
        (b"# ...\t ", None, 0),
        (b"# ...\n...", None, 0),
    ];
    check_try_linemarker::<X86LinuxElf>(cases);
    check_try_linemarker::<Aarch64LinuxElf>(cases);
    check_try_linemarker::<ArmLinuxEabiElf>(cases);
    check_try_linemarker::<RiscvGenericElf>(cases);
    check_try_linemarker::<NonSlashMultibyte>(cases);
    check_try_linemarker::<NoLineSeparator>(cases);
}
