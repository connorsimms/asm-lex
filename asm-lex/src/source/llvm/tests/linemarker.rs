#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;

fn check_try_linemarker<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_linemarker(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_linemarker() {
    use Kind::Preprocessor;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# 100 \"file\"\n", Some(Preprocessor), 12),
        (b"# 100 \"file\"\r", Some(Preprocessor), 12),
        (b"# 100 \"file\"", Some(Preprocessor), 12),
        (b"#100 \"file\"", Some(Preprocessor), 11),
        (b"#100\"file\"", Some(Preprocessor), 10),
        (b"# 100 \"\"", Some(Preprocessor), 8),
        (b"# 100 \"file\" junk", Some(Preprocessor), 17),
        (b"# comment", None, 0),
        (b"# \"comment\"", None, 0),
        (b"# 100 comment", None, 0),
        (b"# comment \"file\"", None, 0),
    ];
    check_try_linemarker::<X86Elf>(cases);
    check_try_linemarker::<X86Darwin>(cases);
    check_try_linemarker::<ArmElf>(cases);
    check_try_linemarker::<ArmDarwin>(cases);
    check_try_linemarker::<Aarch64Elf>(cases);
    check_try_linemarker::<RiscvElf>(cases);
    check_try_linemarker::<Aarch64Darwin>(cases);
    check_try_linemarker::<RiscvDarwin>(cases);
}
