#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::targets::*;
use super::*;
use crate::source::Kind;

struct NoHashLineComment {}
struct NonSlashMultibyte {}
struct NoLineSeparator {}

impl GasTarget for NoHashLineComment {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NonSlashMultibyte {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[b"@@"];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

impl GasTarget for NoLineSeparator {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#/");
    const MULTI_COMMENT_CHARS: &'static [&'static [u8]] = &[];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b"");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
}

fn check_eat_string<T: GasTarget>(cases: &[(&[u8], Span, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::eat_string(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn test_eat_string() {
    let cases: &[(&[u8], Span, usize)] = &[
        (b"", 0..0, 0),
        (b"\"\"...", 0..2, 2),
        (b"\"Text\"...", 0..6, 6),
        (b"\"\n\"...", 0..3, 3),
        (b"\";\"...", 0..3, 3),
        (b"\"\\\"\"...", 0..4, 4),
        (b"\"\\\\\"...", 0..4, 4),
        (b"\"@#/**///\"...", 0..10, 10),
    ];
    check_eat_string::<X86_64LinuxElf>(&cases);
    check_eat_string::<Aarch64LinuxElf>(&cases);
    check_eat_string::<ArmLinuxEabi>(&cases);
    check_eat_string::<Riscv64Elf>(&cases);
    check_eat_string::<NoHashLineComment>(&cases);
    check_eat_string::<NonSlashMultibyte>(&cases);
    check_eat_string::<NoLineSeparator>(&cases);
}

fn check_lex_preamble<T: GasTarget>(cases: &[(&[u8], usize, bool, usize)]) {
    for (bytes, s_pos, starts_line, e_pos) in cases {
        let mut cursor = Cursor::new(bytes);
        cursor.advance(*s_pos);
        assert_eq!(Gas::<T>::lex_preamble(&mut cursor), *starts_line);
        assert_eq!(cursor.pos(), *e_pos);
    }
}

#[test]
fn lex_preamble_no_separators() {
    let cases: &[(&[u8], usize, bool, usize)] = &[
        (b"", 0, true, 0),
        (b"Item", 0, true, 0),
        (b"Item", 4, false, 4),
        (b" Item", 0, true, 1),
        (b"\tItem", 0, true, 1),
        (b"\nItem", 0, true, 1),
        (b"Item\nItem", 4, true, 5),
        (b"Item\n\nItem", 4, true, 6),
    ];
    check_lex_preamble::<X86_64LinuxElf>(cases);
    check_lex_preamble::<Aarch64LinuxElf>(cases);
    check_lex_preamble::<ArmLinuxEabi>(cases);
    check_lex_preamble::<Riscv64Elf>(cases);
    check_lex_preamble::<NoHashLineComment>(cases);
    check_lex_preamble::<NonSlashMultibyte>(cases);
    check_lex_preamble::<NoLineSeparator>(cases);
}

#[test]
fn lex_preamble_semicolon_separators() {
    let cases: &[(&[u8], usize, bool, usize)] = &[
        (b";Item", 0, true, 1),
        (b";;Item", 0, true, 2),
        (b"Item;Item", 4, false, 5),
        (b"Item;;Item", 4, false, 6),
        (b"Item\n;;Item", 4, true, 7),
        (b"Item;\n;Item", 4, true, 7),
        (b"Item;;\nItem", 4, true, 7),
    ];
    check_lex_preamble::<X86_64LinuxElf>(cases);
    check_lex_preamble::<Aarch64LinuxElf>(cases);
    check_lex_preamble::<ArmLinuxEabi>(cases);
    check_lex_preamble::<Riscv64Elf>(cases);
    check_lex_preamble::<NoHashLineComment>(cases);
    check_lex_preamble::<NonSlashMultibyte>(cases);
}

fn check_try_linemarker<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_linemarker(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_linemarker_with_hash_ln_comment() {
    use Kind::Preprocessor;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        // whitespace
        (b"# 100 \"file\"", Some(Preprocessor), 12),
        (b"#100 \"file\"", Some(Preprocessor), 11),
        (b"# 100\"file\"", Some(Preprocessor), 11),
        (b"#100\"file\"", Some(Preprocessor), 10),
        (b"#\t100\t\"file\"", Some(Preprocessor), 12),
        // inner string
        (b"# 100 \"\"", Some(Preprocessor), 8),
        (b"# 100 \"\n\"", Some(Preprocessor), 9),
        (b"# 100 \";@#//\"", Some(Preprocessor), 13),
        // flags
        (b"# 100 \"filename\" 1", Some(Preprocessor), 18),
        (b"# 100 \"filename\"1", Some(Preprocessor), 17),
        (b"# 100 \"filename\" 1 2 3", Some(Preprocessor), 22),
        // invalid
        (b"# junk 100 \"filename\"", None, 0),
        (b"# 100 junk \"filename\"", None, 0),
        (b"# 100 \"filename\" junk", None, 0),
        (b"# 100 \"filename\" 1 junk", None, 0),
    ];
    check_try_linemarker::<X86_64LinuxElf>(cases);
    check_try_linemarker::<Aarch64LinuxElf>(cases);
    check_try_linemarker::<ArmLinuxEabi>(cases);
    check_try_linemarker::<Riscv64Elf>(cases);
    check_try_linemarker::<NonSlashMultibyte>(cases);
    check_try_linemarker::<NoLineSeparator>(cases);
}

#[test]
fn try_linemarker_no_hash_ln_comment() {
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
    ];
    check_try_linemarker::<NoHashLineComment>(cases);
}

fn check_is_line_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ln_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_line_comment(&cursor), *is_ln_cmnt);
    }
}

#[test]
fn is_line_comment_with_hash_ln_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"## ...", true),
        (b"### ...", true),
        (b"#...", true),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_line_comment::<X86_64LinuxElf>(cases);
    check_is_line_comment::<Aarch64LinuxElf>(cases);
    check_is_line_comment::<ArmLinuxEabi>(cases);
    check_is_line_comment::<Riscv64Elf>(cases);
    check_is_line_comment::<NonSlashMultibyte>(cases);
    check_is_line_comment::<NoLineSeparator>(cases);
}

#[test]
fn is_line_comment_no_hash_ln_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", false),
        (b"## ...", false),
        (b"### ...", false),
        (b"#...", false),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_line_comment::<NoHashLineComment>(cases);
}

fn check_try_line_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_line_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_line_comment_with_hash_ln_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", Some(Comment), 5),
        (b"## ...", Some(Comment), 6),
        (b"### ...", Some(Comment), 7),
        (b"#...", Some(Comment), 4),
        (b"nop", None, 0),
        (b"nop#", None, 0),
    ];
    check_try_line_comment::<X86_64LinuxElf>(cases);
    check_try_line_comment::<Aarch64LinuxElf>(cases);
    check_try_line_comment::<ArmLinuxEabi>(cases);
    check_try_line_comment::<Riscv64Elf>(cases);
    check_try_line_comment::<NonSlashMultibyte>(cases);
    check_try_line_comment::<NoLineSeparator>(cases);
}

#[test]
fn try_line_comment_no_hash_ln_comment() {
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"# ...", None, 0),
        (b"## ...", None, 0),
        (b"### ...", None, 0),
        (b"#...", None, 0),
        (b"nop", None, 0),
        (b"nop#...", None, 0),
    ];
    check_try_line_comment::<NoHashLineComment>(cases);
}

fn check_is_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ln_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_comment(&cursor), *is_ln_cmnt);
    }
}

#[test]
fn is_comment_with_hash_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"# ...", true),
        (b"## ...", true),
        (b"### ...", true),
        (b"#...", true),
        (b"@ ...", false),
        (b"@@ ...", false),
        (b"@@@ ...", false),
        (b"@...", false),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_comment::<X86_64LinuxElf>(cases);
    check_is_comment::<Riscv64Elf>(cases);
    check_is_comment::<NonSlashMultibyte>(cases);
    check_is_comment::<NoLineSeparator>(cases);
    check_is_comment::<NoHashLineComment>(cases);
}

#[test]
fn is_comment_with_at_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"@...", true),
        (b"@ ...", true),
        (b"@@ ...", true),
        (b"@@@ ...", true),
        (b"#...", false),
        (b"# ...", false),
        (b"## ...", false),
        (b"### ...", false),
        (b"nop", false),
        (b"nop #", false),
    ];
    check_is_comment::<ArmLinuxEabi>(cases);
}

fn check_try_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_comment_with_hash_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"#...", Some(Comment), 4),
        (b"# ...", Some(Comment), 5),
        (b"## ...", Some(Comment), 6),
        (b"### ...", Some(Comment), 7),
        (b"@...", None, 0),
        (b"@ ...", None, 0),
        (b"@@ ...", None, 0),
        (b"@@@ ...", None, 0),
        (b"nop", None, 0),
        (b"nop #", None, 0),
    ];
    check_try_comment::<X86_64LinuxElf>(cases);
    check_try_comment::<Riscv64Elf>(cases);
    check_try_comment::<NonSlashMultibyte>(cases);
    check_try_comment::<NoLineSeparator>(cases);
    check_try_comment::<NoHashLineComment>(cases);
}

#[test]
fn try_comment_with_at_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@...", Some(Comment), 4),
        (b"@ ...", Some(Comment), 5),
        (b"@@ ...", Some(Comment), 6),
        (b"@@@ ...", Some(Comment), 7),
        (b"#...", None, 0),
        (b"# ...", None, 0),
        (b"## ...", None, 0),
        (b"### ...", None, 0),
        (b"nop", None, 0),
        (b"nop #", None, 0),
    ];
    check_try_comment::<ArmLinuxEabi>(cases);
}

fn check_is_slash_star_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ss_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_slash_star_comment(&cursor), *is_ss_cmnt);
    }
}

#[test]
fn is_slash_star_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"/**/", true),
        (b"/*\n*/", true),
        (b"/***/", true),
        (b"/*\"\"*/", true),
        (b"/* ... */", true),
        (b"/* ...", true),
        (b"#/* ... */", false),
        (b"@/* ... */", false),
    ];
    check_is_slash_star_comment::<X86_64LinuxElf>(&cases);
    check_is_slash_star_comment::<Aarch64LinuxElf>(&cases);
    check_is_slash_star_comment::<ArmLinuxEabi>(&cases);
    check_is_slash_star_comment::<Riscv64Elf>(&cases);
    check_is_slash_star_comment::<NoHashLineComment>(&cases);
    check_is_slash_star_comment::<NonSlashMultibyte>(&cases);
    check_is_slash_star_comment::<NoLineSeparator>(&cases);
}

fn check_try_slash_star_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_slash_star_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_slash_star_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"/**/", Some(Comment), 4),
        (b"/*\n*/", Some(Comment), 5),
        (b"/***/", Some(Comment), 5),
        (b"/*\"\"*/", Some(Comment), 6),
        (b"/* ... */", Some(Comment), 9),
        (b"/* ...", Some(Comment), 6),
        (b"#/* ... */", None, 0),
        (b"@/* ... */", None, 0),
    ];
    check_try_slash_star_comment::<X86_64LinuxElf>(&cases);
    check_try_slash_star_comment::<Aarch64LinuxElf>(&cases);
    check_try_slash_star_comment::<ArmLinuxEabi>(&cases);
    check_try_slash_star_comment::<Riscv64Elf>(&cases);
    check_try_slash_star_comment::<NoHashLineComment>(&cases);
    check_try_slash_star_comment::<NonSlashMultibyte>(&cases);
    check_try_slash_star_comment::<NoLineSeparator>(&cases);
}

fn check_is_multibyte_comment<T: GasTarget>(cases: &[(&[u8], bool)]) {
    for (bytes, is_ln_cmnt) in cases {
        let cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::is_multibyte_comment(&cursor), *is_ln_cmnt);
    }
}

#[test]
fn is_slash_multibyte_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"//...", true),
        (b"// ...", true),
        (b"/// ...", true),
        (b"//// ...", true),
        (b"# ...", false),
        (b"@ ...", false),
        (b"nop ...", false),
        (b"nop//...", false),
    ];
    check_is_multibyte_comment::<Aarch64LinuxElf>(&cases);
    check_is_multibyte_comment::<ArmLinuxEabi>(&cases);
}

#[test]
fn is_nonslash_multibyte_comment() {
    let cases: &[(&[u8], bool)] = &[
        (b"@@...", true),
        (b"@@ ...", true),
        (b"@@@ ...", true),
        (b"@@@@ ...", true),
        (b"// ...", false),
        (b"# ...", false),
        (b"@ ...", false),
        (b"nop ...", false),
        (b"nop@@...", false),
    ];
    check_is_multibyte_comment::<NonSlashMultibyte>(&cases);
}

fn check_try_multibyte_comment<T: GasTarget>(cases: &[(&[u8], Option<Kind>, usize)]) {
    for (bytes, kind, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::try_multibyte_comment(&mut cursor), *kind);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn try_slash_multibyte_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"//...", Some(Comment), 5),
        (b"// ...", Some(Comment), 6),
        (b"/// ...", Some(Comment), 7),
        (b"//// ...", Some(Comment), 8),
        (b"# ...", None, 0),
        (b"@ ...", None, 0),
        (b"nop ...", None, 0),
        (b"nop//...", None, 0),
    ];
    check_try_multibyte_comment::<Aarch64LinuxElf>(&cases);
    check_try_multibyte_comment::<ArmLinuxEabi>(&cases);
}

#[test]
fn try_nonslash_multibyte_comment() {
    use Kind::Comment;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"@@...", Some(Comment), 5),
        (b"@@ ...", Some(Comment), 6),
        (b"@@@ ...", Some(Comment), 7),
        (b"@@@@ ...", Some(Comment), 8),
        (b"// ...", None, 0),
        (b"# ...", None, 0),
        (b"@ ...", None, 0),
        (b"nop ...", None, 0),
        (b"nop@@...", None, 0),
    ];
    check_try_multibyte_comment::<NonSlashMultibyte>(&cases);
}

fn check_lex_args<T: GasTarget>(cases: &[(&[u8], Option<Span>, usize)]) {
    for (bytes, span, pos) in cases {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<T>::lex_args(&mut cursor), *span);
        assert_eq!(cursor.pos(), *pos);
    }
}

#[test]
fn lex_args_slash_star_comments() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        // no args
        (b"", None, 0),
        (b"/*...*/", None, 0),
        // whitespace
        (b"arg", Some(0..3), 3),
        (b" arg", Some(1..4), 4),
        (b"arg ", Some(0..3), 3),
        (b" arg ", Some(1..4), 4),
        // quoted
        (b"\"arg\"", Some(0..5), 5),
        (b"\"arg\" \"arg\"", Some(0..11), 11),
        (b"\"a\ng\" \"a\ng\"", Some(0..11), 11),
        // slash star
        (b"/*...*/arg", Some(7..10), 10),
        (b"arg/*...*/", Some(0..3), 3),
        (b"arg/*...*/arg", Some(0..13), 13),
        (b"arg/*...*/arg/*...*/", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(&cases);
    check_lex_args::<Aarch64LinuxElf>(&cases);
    check_lex_args::<ArmLinuxEabi>(&cases);
    check_lex_args::<Riscv64Elf>(&cases);
    check_lex_args::<NoHashLineComment>(&cases);
    check_lex_args::<NonSlashMultibyte>(&cases);
    check_lex_args::<NoLineSeparator>(&cases);
}

#[test]
fn lex_args_hash_comments() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"#...", None, 0),
        (b"arg#", Some(0..3), 3),
        (b" arg#", Some(1..4), 4),
        (b"arg #", Some(0..3), 3),
        (b" arg #", Some(1..4), 4),
        (b"arg arg#", Some(0..7), 7),
        (b"arg/*.#.*/arg #", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(&cases);
    check_lex_args::<Riscv64Elf>(&cases);
    check_lex_args::<NoHashLineComment>(&cases);
    check_lex_args::<NonSlashMultibyte>(&cases);
    check_lex_args::<NoLineSeparator>(&cases);
}

#[test]
fn lex_args_line_separators() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b";...", None, 0),
        (b"arg;", Some(0..3), 3),
        (b" arg;", Some(1..4), 4),
        (b"arg ;", Some(0..3), 3),
        (b" arg ;", Some(1..4), 4),
        (b"arg arg;", Some(0..7), 7),
        (b"arg/*.;.*/arg ;", Some(0..13), 13),
    ];
    check_lex_args::<X86_64LinuxElf>(&cases);
    check_lex_args::<Aarch64LinuxElf>(&cases);
    check_lex_args::<ArmLinuxEabi>(&cases);
    check_lex_args::<Riscv64Elf>(&cases);
    check_lex_args::<NoHashLineComment>(&cases);
    check_lex_args::<NonSlashMultibyte>(&cases);
}

#[test]
fn lex_args_slash_multibyte_comment() {
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"//...", None, 0),
        (b"arg//", Some(0..3), 3),
        (b" arg//", Some(1..4), 4),
        (b"arg //", Some(0..3), 3),
        (b" arg //", Some(1..4), 4),
        (b"arg arg//", Some(0..7), 7),
        (b"arg/*.//.*/arg //", Some(0..14), 14),
    ];
    check_lex_args::<Aarch64LinuxElf>(&cases);
    check_lex_args::<ArmLinuxEabi>(&cases);
}

#[test]
fn lex_args_hash_line_comment() {
    // Line comment chars are not comments in arguments
    let cases: &[(&[u8], Option<Span>, usize)] = &[
        (b"#...", Some(0..4), 4),
        (b"arg#", Some(0..4), 4),
        (b" arg#", Some(1..5), 5),
        (b"arg #", Some(0..5), 5),
        (b" arg #", Some(1..6), 6),
        (b"arg arg#", Some(0..8), 8),
        (b"arg/*.#.*/arg #", Some(0..15), 15),
    ];
    check_lex_args::<Aarch64LinuxElf>(&cases);
    check_lex_args::<ArmLinuxEabi>(&cases);
}

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
    use Kind::Unknown;
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
        // invalid
        (b"123Label:", Some(Unknown), 9),
        (b"123.:", Some(Unknown), 5),
        (b"123_:", Some(Unknown), 5),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
}

#[test]
fn try_symbol_kind_local_dollar_label() {
    use Kind::Label;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"1$:", Some(Label { name: 0..2 }), 3),
        (b"22$:", Some(Label { name: 0..3 }), 4),
        (b"333$:", Some(Label { name: 0..4 }), 5),
    ];
    check_try_symbol_kind::<Riscv64Elf>(&cases);
}

#[test]
fn try_symbol_kind_no_local_dollar_label() {
    use Kind::Unknown;
    let cases: &[(&[u8], Option<Kind>, usize)] = &[
        (b"1$:", Some(Unknown), 3),
        (b"22$:", Some(Unknown), 4),
        (b"333$:", Some(Unknown), 5),
    ];
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
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
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
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
    check_try_symbol_kind::<X86_64LinuxElf>(&cases);
    check_try_symbol_kind::<Aarch64LinuxElf>(&cases);
    check_try_symbol_kind::<ArmLinuxEabi>(&cases);
    check_try_symbol_kind::<Riscv64Elf>(&cases);
    check_try_symbol_kind::<NoHashLineComment>(&cases);
    check_try_symbol_kind::<NonSlashMultibyte>(&cases);
    check_try_symbol_kind::<NoLineSeparator>(&cases);
}
