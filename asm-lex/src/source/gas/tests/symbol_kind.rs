#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::source::gas::{targets::*, GasTarget};

fn check_try_symbol_kind<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_symbol_kind(&mut cursor), *kind, "{:?}", bytes);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_symbol_kind_label() {
    use Kind::Label;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"Label:", Some(Label { name: 0..5 }), 6),
        // whitespace
        (b"Label :", Some(Label { name: 0..5 }), 7),
        (b"Label \t:", Some(Label { name: 0..5 }), 8),
        // start chars
        (b".Label:", Some(Label { name: 0..6 }), 7),
        (b"_Label:", Some(Label { name: 0..6 }), 7),
        (b"$Label:", Some(Label { name: 0..6 }), 7),
        // quoted
        (b"\"Label\":", Some(Label { name: 1..6 }), 8),
        (b"\"Label\" :", Some(Label { name: 1..6 }), 9),
        (b"\"Label\" \t:", Some(Label { name: 1..6 }), 10),
        (b"\"!@#$%\":", Some(Label { name: 1..6 }), 8),
        (b"\"1234$\":", Some(Label { name: 1..6 }), 8),
        (b"\"\":", Some(Label { name: 1..1 }), 3),
        (b"\"\n\":", Some(Label { name: 1..2 }), 4),
        (b"\"\t\":", Some(Label { name: 1..2 }), 4),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

// It seems nearly all targets use local labels, but
// this is separated from above just in case.
#[test]
fn try_symbol_kind_local_label() {
    use Kind::Label;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"1:", Some(Label { name: 0..1 }), 2),
        (b"22:", Some(Label { name: 0..2 }), 3),
        (b"333:", Some(Label { name: 0..3 }), 4),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_local_dollar_label() {
    use Kind::Label;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"1$:", Some(Label { name: 0..2 }), 3),
        (b"22$:", Some(Label { name: 0..3 }), 4),
        (b"333$:", Some(Label { name: 0..4 }), 5),
    ];
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
}

#[test]
fn try_symbol_kind_no_local_dollar_label() {
    use Kind::Unknown;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"1$:", Some(Unknown), 3),
        (b"22$:", Some(Unknown), 4),
        (b"333$:", Some(Unknown), 5),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_directive() {
    use Kind::Directive;

    let directive = |name, args| Some(Directive { name, args });

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        // no args
        (b".", directive(0..1, None), 1),
        (b".dir", directive(0..4, None), 4),
        // no args + trivia
        (b".dir ", directive(0..4, None), 4),
        (b".dir \t", directive(0..4, None), 4),
        (b".dir \n", directive(0..4, None), 4),
        (b".dir \n ...", directive(0..4, None), 4),
        (b".dir /*...*/", directive(0..4, None), 4),
        // arg
        (b". arg", directive(0..1, Some(2..5)), 5),
        (b".dir arg", directive(0..4, Some(5..8)), 8),
        (b". \"a\"", directive(0..1, Some(2..5)), 5),
        (b". \"\n\"", directive(0..1, Some(2..5)), 5),
        (b".dir \"a\"", directive(0..4, Some(5..8)), 8),
        (b".dir \"a", directive(0..4, Some(5..7)), 7),
        (b".dir \"\n\"", directive(0..4, Some(5..8)), 8),
        // arg + trivia
        (b".dir arg ", directive(0..4, Some(5..8)), 8),
        (b".dir arg \t", directive(0..4, Some(5..8)), 8),
        (b".dir arg \n", directive(0..4, Some(5..8)), 8),
        (b".dir arg \n ...", directive(0..4, Some(5..8)), 8),
        (b".dir arg /*...*/", directive(0..4, Some(5..8)), 8),
        // args
        (b". a a", directive(0..1, Some(2..5)), 5),
        (b".dir a a", directive(0..4, Some(5..8)), 8),
        // args + trivia
        (b".dir a a ", directive(0..4, Some(5..8)), 8),
        (b".dir a \"a\"", directive(0..4, Some(5..10)), 10),
        (b".dir a a \t", directive(0..4, Some(5..8)), 8),
        (b".dir a a \n", directive(0..4, Some(5..8)), 8),
        (b".dir a a \n ...", directive(0..4, Some(5..8)), 8),
        (b".dir a a /*...*/", directive(0..4, Some(5..8)), 8),
        (b".dir a /*...*/a/*...*/", directive(0..4, Some(5..15)), 15),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_directive_line_separator() {
    use Kind::Directive;

    let directive = |name, args| Some(Directive { name, args });

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b".dir ;", directive(0..4, None), 4),
        (b".dir arg ;", directive(0..4, Some(5..8)), 8),
        (b".dir arg ; ...", directive(0..4, Some(5..8)), 8),
        (b".dir a a ;", directive(0..4, Some(5..8)), 8),
        (b".dir a a ; ...", directive(0..4, Some(5..8)), 8),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
}

#[test]
fn try_symbol_kind_directive_hash_comment() {
    use Kind::Directive;

    let directive = |name, args| Some(Directive { name, args });

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b".dir #", directive(0..4, None), 4),
        (b".dir arg #", directive(0..4, Some(5..8)), 8),
        (b".dir arg # ...", directive(0..4, Some(5..8)), 8),
        (b".dir a a #", directive(0..4, Some(5..8)), 8),
        (b".dir a a # ...", directive(0..4, Some(5..8)), 8),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_directive_slash_multibyte_comment() {
    use Kind::Directive;

    let directive = |name, args| Some(Directive { name, args });

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b".dir //", directive(0..4, None), 4),
        (b".dir arg //", directive(0..4, Some(5..8)), 8),
        (b".dir arg // ...", directive(0..4, Some(5..8)), 8),
        (b".dir a a //", directive(0..4, Some(5..8)), 8),
        (b".dir a a // ...", directive(0..4, Some(5..8)), 8),
    ];
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
}

#[test]
fn try_symbol_kind_instruction() {
    use Kind::Instruction;

    let insn = |mnemonic, args| Some(Instruction { mnemonic, args });

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        // no args
        (b"op", insn(0..2, None), 2),
        // no args + trivia
        (b"op ", insn(0..2, None), 2),
        (b"op\t", insn(0..2, None), 2),
        (b"op\n", insn(0..2, None), 2),
        (b"op \t", insn(0..2, None), 2),
        (b"op \n", insn(0..2, None), 2),
        // arg
        (b"op arg", insn(0..2, Some(3..6)), 6),
        // arg + trivia
        (b"op arg ", insn(0..2, Some(3..6)), 6),
        (b"op arg\t", insn(0..2, Some(3..6)), 6),
        (b"op arg\n", insn(0..2, Some(3..6)), 6),
        (b"op arg \t", insn(0..2, Some(3..6)), 6),
        (b"op arg \n", insn(0..2, Some(3..6)), 6),
        // args
        (b"op a a", insn(0..2, Some(3..6)), 6),
        // args + trivia
        (b"op a a ", insn(0..2, Some(3..6)), 6),
        (b"op a a\t", insn(0..2, Some(3..6)), 6),
        (b"op a a\n", insn(0..2, Some(3..6)), 6),
        (b"op a a \t", insn(0..2, Some(3..6)), 6),
        (b"op a a \n", insn(0..2, Some(3..6)), 6),
        // real examples
        (b"movb $'A', %al", insn(0..4, Some(5..14)), 14),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_definition() {
    use Kind::Definition;

    let defn = |symbol, keyword, args| {
        Some(Definition {
            symbol,
            keyword,
            args,
        })
    };

    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"Sym=Val", defn(0..3, 3..4, Some(4..7)), 7),
        (b"Sym= Val", defn(0..3, 3..4, Some(5..8)), 8),
        (b"Sym =Val", defn(0..3, 4..5, Some(5..8)), 8),
        (b"Sym = Val", defn(0..3, 4..5, Some(6..9)), 9),
        (b"Sym==Val", defn(0..3, 3..5, Some(5..8)), 8),
        (b"Sym== Val", defn(0..3, 3..5, Some(6..9)), 9),
        (b"Sym ==Val", defn(0..3, 4..6, Some(6..9)), 9),
        (b"Sym == Val", defn(0..3, 4..6, Some(7..10)), 10),
        // real examples
        (b". = .+4", defn(0..1, 2..3, Some(4..7)), 7),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}

#[test]
fn try_symbol_kind_unknown() {
    use Kind::Unknown;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"123Label:", Some(Unknown), 9),
        (b"123.:", Some(Unknown), 5),
        (b"123_:", Some(Unknown), 5),
        (b"\"...\"", Some(Unknown), 5),
        (b"^^^:", Some(Unknown), 4),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(cases);
    check_try_symbol_kind::<ArmLinuxEabiElf>(cases);
    check_try_symbol_kind::<Riscv64LinuxElf>(cases);
    check_try_symbol_kind::<NoHashLineComment>(cases);
    check_try_symbol_kind::<NonSlashMultibyte>(cases);
    check_try_symbol_kind::<NoLineSeparator>(cases);
}
