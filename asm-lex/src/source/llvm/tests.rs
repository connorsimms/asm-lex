#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::cursor::Cursor;
use crate::source::llvm::{targets::*, Llvm, LlvmTarget};
use crate::source::Kind;
use crate::Span;

fn check_try_single_quoted<T: LlvmTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_single_quoted(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_single_quoted() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (br"'c'", Some(0..3), 3),
        (br"'\r'", Some(0..4), 4),
        (br"'\''", Some(0..4), 4),
        (br"'\\''", Some(0..4), 4),
        (br#"'\"'"#, Some(0..4), 4),
    ];
    check_try_single_quoted::<X86Elf>(cases);
    check_try_single_quoted::<X86Darwin>(cases);
    check_try_single_quoted::<ArmElf>(cases);
    check_try_single_quoted::<ArmDarwin>(cases);
    check_try_single_quoted::<Aarch64Elf>(cases);
    check_try_single_quoted::<Aarch64Darwin>(cases);
    check_try_single_quoted::<RiscvElf>(cases);
    check_try_single_quoted::<RiscvDarwin>(cases);
}

fn check_eat_double_quoted<T: LlvmTarget>(cases: &[(&[u8], Span, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::eat_double_quoted(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn eat_double_quoted() {
    let cases: &[(&[u8], Span, usize)] = &[
        (b"\"\"", 0..2, 2),
        (b"\"a\"", 0..3, 3),
        (b"\"a b\"", 0..5, 5),
        (b"\"a\nb\"", 0..5, 5),
        (b"\"a\tb\"", 0..5, 5),
        (b"\"a\\\"b\"", 0..6, 6),
    ];
    check_eat_double_quoted::<X86Elf>(cases);
    check_eat_double_quoted::<X86Darwin>(cases);
    check_eat_double_quoted::<ArmElf>(cases);
    check_eat_double_quoted::<ArmDarwin>(cases);
    check_eat_double_quoted::<Aarch64Elf>(cases);
    check_eat_double_quoted::<Aarch64Darwin>(cases);
    check_eat_double_quoted::<RiscvElf>(cases);
    check_eat_double_quoted::<RiscvDarwin>(cases);
}

fn check_eat_line_separator<T: LlvmTarget>(cases: &[(&[u8], bool, usize)]) {
    for (bytes, res, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::eat_line_separator(&mut cursor), *res);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn eat_semicolon_line_separator() {
    let cases: &[(&[u8], bool, usize)] = &[
        (b";", true, 1),
        (b"; ", true, 1),
        (b";\t", true, 1),
        (b";;", true, 1),
        (b"", false, 0),
        (b"#", false, 0),
        (b"\n", false, 0),
    ];
    check_eat_line_separator::<X86Elf>(cases);
    check_eat_line_separator::<X86Darwin>(cases);
    check_eat_line_separator::<ArmElf>(cases);
    check_eat_line_separator::<ArmDarwin>(cases);
    check_eat_line_separator::<Aarch64Elf>(cases);
    check_eat_line_separator::<RiscvElf>(cases);
}

#[test]
fn eat_double_percent_line_separator() {
    let cases: &[(&[u8], bool, usize)] = &[
        (b"%%", true, 2),
        (b"%% ", true, 2),
        (b"%%\t", true, 2),
        (b"%%%%", true, 2),
        (b"", false, 0),
        (b";", false, 0),
        (b"\n", false, 0),
    ];
    check_eat_line_separator::<Aarch64Darwin>(cases);
    check_eat_line_separator::<RiscvDarwin>(cases);
}

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

fn check_is_line_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_line_comment(&cursor), *res);
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

fn check_try_line_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_line_comment(&mut cursor, true), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
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

fn check_is_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_comment(&cursor), *res);
    }
}

#[test]
fn is_comment_x86_elf() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"// ...", true),
        (b"@ ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<X86Elf>(cases);
}

#[test]
fn is_comment_x86_darwin() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"// ...", true),
        (b"@ ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<X86Darwin>(cases);
}

#[test]
fn is_comment_arm_elf() {
    let cases: &[(&[u8], bool)] = &[
        (b"// ...", true),
        (b"@ ...", true),
        (b"# ...", false),
        (b"; ...", false),
    ];
    check_is_comment::<ArmElf>(cases);
}

fn check_try_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_comment_x86_elf() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<X86Elf>(cases);
}

#[test]
fn try_comment_x86_darwin() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"# ...\n", Some(Comment), 5),
        (b"# ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"@ ...", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<X86Darwin>(cases);
}

#[test]
fn try_comment_arm_elf() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@ ...", Some(Comment), 5),
        (b"@ ...\n", Some(Comment), 5),
        (b"@ ...\r", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"// ...\n", Some(Comment), 6),
        (b"// ...\r", Some(Comment), 6),
        (b"# ...", None, 0),
        (b"# ...\n", None, 0),
        (b"# ...\r", None, 0),
        (b"; ...", None, 0),
    ];
    check_try_comment::<ArmElf>(cases);
}

fn check_is_slash_star_comment<T: LlvmTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, res) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::is_slash_star_comment(&cursor), *res);
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

fn check_try_slash_star_comment<T: LlvmTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::try_slash_star_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
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

fn check_lex_args<T: LlvmTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Llvm::<T>::lex_args(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_lex_args_x86_elf_and_darwin() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"a b c", Some(0..5), 5),
        (b"a b c ", Some(0..5), 5),
        (b" a b c", Some(1..6), 6),
        (b"a b c\n", Some(0..5), 5),
        (b"a b c\r", Some(0..5), 5),
        (b"a b c;", Some(0..5), 5),
        (b"a b c#", Some(0..5), 5),
        (b"a b c//", Some(0..5), 5),
        (b"a / b c", Some(0..7), 7),
        (b"'a' b c", Some(0..7), 7),
        (b"a 'b' c", Some(0..7), 7),
        (b"a '\n' c", Some(0..7), 7),
        (b"\"a\" b c", Some(0..7), 7),
        (b"a \"b\" c", Some(0..7), 7),
        (b"a \"\n\" c", Some(0..7), 7),
        (b"a /*...*/ b c", Some(0..13), 13),
        (b"a b c /*...*/", Some(0..5), 5),
        (b"", None, 0),
        (b"/*...*/", None, 0),
        (b"//...", None, 0),
    ];
    check_lex_args::<X86Elf>(cases);
    check_lex_args::<X86Darwin>(cases);
}

#[test]
fn try_lex_args_arm_elf_and_darwin() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"a b c", Some(0..5), 5),
        (b"a b c ", Some(0..5), 5),
        (b" a b c", Some(1..6), 6),
        (b"a b c\n", Some(0..5), 5),
        (b"a b c\r", Some(0..5), 5),
        (b"a b c@", Some(0..5), 5),
        (b"a b c;", Some(0..5), 5),
        (b"a b c//", Some(0..5), 5),
        (b"a # c#", Some(0..6), 6),
        (b"a / b c", Some(0..7), 7),
        (b"'a' b c", Some(0..7), 7),
        (b"a 'b' c", Some(0..7), 7),
        (b"a '\n' c", Some(0..7), 7),
        (b"\"a\" b c", Some(0..7), 7),
        (b"a \"b\" c", Some(0..7), 7),
        (b"a \"\n\" c", Some(0..7), 7),
        (b"a /*...*/ b c", Some(0..13), 13),
        (b"a b c /*...*/", Some(0..5), 5),
        (b"", None, 0),
        (b"/*...*/", None, 0),
        (b"//...", None, 0),
    ];
    check_lex_args::<ArmElf>(cases);
    check_lex_args::<ArmDarwin>(cases);
}

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
