#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::gas::{targets::*, Gas, GasTarget};
use crate::source::Kind;
use Kind::Preprocessor;

fn check_try_inline_asm_marker<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_inline_asm_marker(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn hash_line_comments() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"#APP", Some(Preprocessor), 4),
        (b"#NO_APP", Some(Preprocessor), 7),
        (b"#APP\n", Some(Preprocessor), 4),
        (b"#NO_APP\n", Some(Preprocessor), 7),
        (b"#APP ", None, 0),
        (b"#NO_APP ", None, 0),
        (b"# APP", None, 0),
        (b"# NO_APP", None, 0),
    ];
    check_try_inline_asm_marker::<X86LinuxElf>(cases);
    check_try_inline_asm_marker::<X86Darwin>(cases);
    check_try_inline_asm_marker::<X86Pe>(cases);
}

#[test]
fn slash_line_comments() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"/APP", Some(Preprocessor), 4),
        (b"/NO_APP", Some(Preprocessor), 7),
        (b"/APP\n", Some(Preprocessor), 4),
        (b"/NO_APP\n", Some(Preprocessor), 7),
        (b"/APP ", None, 0),
        (b"/NO_APP ", None, 0),
        (b"/ APP", None, 0),
        (b"/ NO_APP", None, 0),
    ];
    check_try_inline_asm_marker::<X86LinuxElf>(cases);
    check_try_inline_asm_marker::<X86Darwin>(cases);
    check_try_inline_asm_marker::<X86Pe>(cases);
}
