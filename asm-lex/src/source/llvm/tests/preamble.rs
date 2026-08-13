#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};

fn check_lex_preamble<T: LlvmTarget>(cases: &[(&[u8], usize, (bool, bool), usize)]) {
    for (bytes, s_pos, res, e_pos) in cases {
        let mut cursor = Cursor::new(bytes);
        cursor.restore(*s_pos);
        assert_eq!(Llvm::<T>::lex_preamble(&mut cursor), *res);
        assert_eq!(cursor.pos(), *e_pos);
    }
}

#[test]
fn lex_preamble() {
    let cases: &[(&[u8], usize, (bool, bool), usize)] = &[
        (b" symbol", 0, (true, true), 1),
        (b" \tsymbol", 0, (true, true), 2),
        (b" \nsymbol", 0, (true, true), 2),
        (b" \rsymbol", 0, (true, true), 2),
        (b"Label: # ...", 6, (false, true), 7),
        (b"Label:\n# ...", 6, (true, true), 7),
        (b"Label:\r# ...", 6, (true, true), 7),
        (b"Label:\n\r# ...", 6, (true, true), 8),
    ];
    check_lex_preamble::<X86Elf>(cases);
    check_lex_preamble::<X86Darwin>(cases);
    check_lex_preamble::<ArmElf>(cases);
    check_lex_preamble::<ArmDarwin>(cases);
    check_lex_preamble::<Aarch64Elf>(cases);
    check_lex_preamble::<RiscvElf>(cases);
    check_lex_preamble::<Aarch64Darwin>(cases);
    check_lex_preamble::<RiscvDarwin>(cases);
}

#[test]
fn lex_preamble_semicolon_separator() {
    let cases: &[(&[u8], usize, (bool, bool), usize)] = &[
        (b";\nsymbol", 0, (true, true), 2),
        (b"\n;symbol", 0, (true, true), 2),
        (b"Label:;# ...", 6, (false, true), 7),
    ];
    check_lex_preamble::<X86Elf>(cases);
    check_lex_preamble::<X86Darwin>(cases);
    check_lex_preamble::<ArmElf>(cases);
    check_lex_preamble::<ArmDarwin>(cases);
    check_lex_preamble::<Aarch64Elf>(cases);
    check_lex_preamble::<RiscvElf>(cases);
}

#[test]
fn lex_preamble_double_percent_separator() {
    let cases: &[(&[u8], usize, (bool, bool), usize)] = &[
        (b"%%\nsymbol", 0, (true, true), 3),
        (b"\n%%symbol", 0, (true, true), 3),
        (b"Label:%%# ...", 6, (false, true), 8),
    ];
    check_lex_preamble::<Aarch64Darwin>(cases);
    check_lex_preamble::<RiscvDarwin>(cases);
}
