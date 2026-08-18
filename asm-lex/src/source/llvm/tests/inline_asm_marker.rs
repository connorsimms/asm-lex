#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;
use Kind::Preprocessor;

fn check_try_inline_asm_marker<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_inline_asm_marker(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn hash_comments() {
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
    check_try_inline_asm_marker::<X86Elf>(cases);
    check_try_inline_asm_marker::<RiscvElf>(cases);
}

#[test]
fn doubleslash_comments() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"//APP", Some(Preprocessor), 5),
        (b"//NO_APP", Some(Preprocessor), 8),
        (b"//APP\n", Some(Preprocessor), 5),
        (b"//NO_APP\n", Some(Preprocessor), 8),
        (b"//APP ", None, 0),
        (b"//NO_APP ", None, 0),
        (b"// APP", None, 0),
        (b"// NO_APP", None, 0),
    ];
    check_try_inline_asm_marker::<Aarch64Elf>(cases);
    check_try_inline_asm_marker::<Aarch64MicrosoftCoff>(cases);
}

#[test]
fn at_comments() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@APP", Some(Preprocessor), 4),
        (b"@NO_APP", Some(Preprocessor), 7),
        (b"@APP\n", Some(Preprocessor), 4),
        (b"@NO_APP\n", Some(Preprocessor), 7),
        (b"@APP ", None, 0),
        (b"@NO_APP ", None, 0),
        (b"@ APP", None, 0),
        (b"@ NO_APP", None, 0),
    ];
    check_try_inline_asm_marker::<ArmElf>(cases);
    check_try_inline_asm_marker::<ArmMicrosoftCoff>(cases);
}

#[test]
fn x86_darwin() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"## InlineAsm Start", Some(Preprocessor), 18),
        (b"## InlineAsm End", Some(Preprocessor), 16),
        (b"## InlineAsm Start\n", Some(Preprocessor), 18),
        (b"## InlineAsm End\n", Some(Preprocessor), 16),
        (b"## InlineAsm Start ", None, 0),
        (b"## InlineAsm End ", None, 0),
    ];
    check_try_inline_asm_marker::<X86Darwin>(cases);
}

#[test]
fn aarch64_darwin() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"; InlineAsm Start", Some(Preprocessor), 17),
        (b"; InlineAsm End", Some(Preprocessor), 15),
        (b"; InlineAsm Start\n", Some(Preprocessor), 17),
        (b"; InlineAsm End\n", Some(Preprocessor), 15),
        (b"; InlineAsm Start ", None, 0),
        (b"; InlineAsm End ", None, 0),
    ];
    check_try_inline_asm_marker::<Aarch64Darwin>(cases);
}
