#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;

fn check_try_symbol_kind<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_symbol_kind(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_symbol_kind_label_x86_elf() {
    use Kind::Label;
    let label = |name| Label { name };
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"a:", Some(label(0..1)), 2),
        (b"@a:", Some(label(0..2)), 3),
        (b"$a:", Some(label(0..2)), 3),
        (b"'a':", Some(label(0..3)), 4),
        (b"'\n':", Some(label(0..3)), 4),
        (b"@'a':", Some(label(0..4)), 5),
        (b"$'a':", Some(label(0..4)), 5),
        (b"\"b\":", Some(label(1..2)), 4),
        (b"\"a\n$\":", Some(label(1..4)), 6),
    ];
    check_try_symbol_kind::<X86Elf>(cases);
}

#[test]
fn try_symbol_kind_directive_x86_elf() {
    use Kind::Directive;
    let dir = |name, args| Directive { name, args };
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b".dir", Some(dir(0..4, None)), 4),
        (b".dir ", Some(dir(0..4, None)), 4),
        (b".dir\n", Some(dir(0..4, None)), 4),
        (b".dir\r", Some(dir(0..4, None)), 4),
        (b".dir a", Some(dir(0..4, Some(5..6))), 6),
        (b".dir a b", Some(dir(0..4, Some(5..8))), 8),
        (b".dir \"a\"", Some(dir(0..4, Some(5..8))), 8),
        (b".dir\"a\"", Some(dir(0..4, Some(4..7))), 7),
        (b".dir \"\n\"", Some(dir(0..4, Some(5..8))), 8),
        (b".dir a /*...*/ b", Some(dir(0..4, Some(5..16))), 16),
    ];
    check_try_symbol_kind::<X86Elf>(cases);
}

#[test]
fn try_symbol_kind_instruction_x86_elf() {
    use Kind::Instruction;
    let ins = |mnemonic, args| Instruction { mnemonic, args };
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"insn", Some(ins(0..4, None)), 4),
        (b"insn ", Some(ins(0..4, None)), 4),
        (b"insn\n", Some(ins(0..4, None)), 4),
        (b"insn\r", Some(ins(0..4, None)), 4),
        (b"insn a", Some(ins(0..4, Some(5..6))), 6),
        (b"insn a, b", Some(ins(0..4, Some(5..9))), 9),
        (b"insn a / b", Some(ins(0..4, Some(5..10))), 10),
        (b"insn a, /*...*/ b", Some(ins(0..4, Some(5..17))), 17),
    ];
    check_try_symbol_kind::<X86Elf>(cases);
}

#[test]
fn try_symbol_kind_definition_x86_elf() {
    use Kind::Definition;
    let sym = |symbol, keyword, args| Definition {
        symbol,
        keyword,
        args,
    };
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"sym = val", Some(sym(0..3, 4..5, Some(6..9))), 9),
        (b"sym= val", Some(sym(0..3, 3..4, Some(5..8))), 8),
        (b"sym =val", Some(sym(0..3, 4..5, Some(5..8))), 8),
        (b"sym=val", Some(sym(0..3, 3..4, Some(4..7))), 7),
        (b". = .+4", Some(sym(0..1, 2..3, Some(4..7))), 7),
    ];
    check_try_symbol_kind::<X86Elf>(cases);
}

#[test]
fn try_symbol_kind_unknown_x86_elf() {
    use Kind::Unknown;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[(b"@\"b\":", Some(Unknown), 5)];
    check_try_symbol_kind::<X86Elf>(cases);
}
