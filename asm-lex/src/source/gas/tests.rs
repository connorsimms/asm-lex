use super::*;
use crate::cursor;
use crate::source;

struct TestTarget {}
impl GasTarget for TestTarget {
    const COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"#");
    const LINE_COMMENT_CHARS: ByteSet = ByteSet::from_bytes(b"/");
    const MULTI_COMMENT_CHARS: &[&[u8]] = &[b"//"];
    const LINE_SEPARATOR_CHARS: ByteSet = ByteSet::from_bytes(b";");
    const SYMBOL_START_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z');
    const SYMBOL_CONTINUE_CHARS: ByteSet = ByteSet::from_bytes(b"._$")
        .with_range(b'a', b'z')
        .with_range(b'A', b'Z')
        .with_range(b'0', b'9');
    const LOCAL_LABELS_DOLLAR: bool = true;
}

#[test]
fn test_eat_string() {
    let check = |bytes, range, pos| {
        let mut cursor = Cursor::new(bytes);
        let span = Gas::<TestTarget>::eat_string(&mut cursor);
        assert_eq!(span, range);
        assert_eq!(cursor.pos(), pos);
    };

    check(b"Non-quoted string", 0..0, 0);
    check(b"\"Quoted string\"", 0..15, 15);
    check(b"\"Quoted string\\\\\" not here", 0..17, 17);
    check(b"Some \"Quoted string\"", 0..0, 0);
    check(b"\"Escape \\\" quote\"", 0..17, 17);
    check(b"\"Non-terminated", 0..15, 15);
    check(b"\"Non-terminated escape\\\"", 0..24, 24);
    check(b"\"\\\" still here", 0..14, 14);
}

#[test]
fn test_lex_preamble() {
    let mut cursor = Cursor::new(b"First Item");
    assert!(Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"\nFirst Item");
    assert!(Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 1);

    let mut cursor = Cursor::new(b"\t ; \t \nFirst Item");
    assert!(Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 7);

    let mut cursor = Cursor::new(b"; \n;\t \n;;;\nFirst Item");
    assert!(Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 11);

    let mut cursor = Cursor::new(b";Not First Item");
    assert!(!Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 1);

    let mut cursor = Cursor::new(b"\t\n\n;Not First Item");
    assert!(!Gas::<TestTarget>::lex_preamble(&mut cursor));
    assert_eq!(cursor.pos(), 4);
}

#[test]
fn test_try_linemarker() {
    let mut cursor = Cursor::new(b"# 1000 \"filename\"");
    assert_eq!(
        Gas::<TestTarget>::try_linemarker(&mut cursor),
        Some(source::Kind::Preprocessor)
    );
    assert_eq!(cursor.pos(), 17);

    let mut cursor = Cursor::new(b"#1000\"filename\"");
    assert_eq!(
        Gas::<TestTarget>::try_linemarker(&mut cursor),
        Some(source::Kind::Preprocessor)
    );
    assert_eq!(cursor.pos(), 15);

    let mut cursor = Cursor::new(b"# \r \t1000 \t\r \"filename\"");
    assert_eq!(
        Gas::<TestTarget>::try_linemarker(&mut cursor),
        Some(source::Kind::Preprocessor)
    );
    assert_eq!(cursor.pos(), 23);

    let mut cursor = Cursor::new(b"# 1000 \"\"");
    assert_eq!(
        Gas::<TestTarget>::try_linemarker(&mut cursor),
        Some(source::Kind::Preprocessor)
    );
    assert_eq!(cursor.pos(), 9);

    let mut cursor = Cursor::new(b"# 1000 \"filename\" 1 2 3 100");
    assert_eq!(
        Gas::<TestTarget>::try_linemarker(&mut cursor),
        Some(source::Kind::Preprocessor)
    );
    assert_eq!(cursor.pos(), 27);

    let mut cursor = Cursor::new(b"# 1000 \"filename\" junk");
    assert_eq!(Gas::<TestTarget>::try_linemarker(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"# 1000 junk");
    assert_eq!(Gas::<TestTarget>::try_linemarker(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"# 1000");
    assert_eq!(Gas::<TestTarget>::try_linemarker(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);
}

#[test]
fn test_is_line_comment() {
    let cursor = Cursor::new(b"/ This is a line comment");
    assert!(Gas::<TestTarget>::is_line_comment(&cursor));

    let cursor = Cursor::new(b"This is not");
    assert!(!Gas::<TestTarget>::is_line_comment(&cursor));

    let cursor = Cursor::new(b"# This is not");
    assert!(!Gas::<TestTarget>::is_line_comment(&cursor));

    let cursor = Cursor::new(b"; This is not");
    assert!(!Gas::<TestTarget>::is_line_comment(&cursor));
}

#[test]
fn test_try_line_comment() {
    let mut cursor = Cursor::new(b"/ This is a line comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_line_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 24);

    let mut cursor = Cursor::new(b"/ This ; is a ; line comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_line_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 28);

    let mut cursor = Cursor::new(b"This is not");
    assert_eq!(Gas::<TestTarget>::try_line_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"# This is not");
    assert_eq!(Gas::<TestTarget>::try_line_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"; This is not");
    assert_eq!(Gas::<TestTarget>::try_line_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);
}

#[test]
fn test_is_comment() {
    let cursor = Cursor::new(b"# This is a comment");
    assert!(Gas::<TestTarget>::is_comment(&cursor));

    let cursor = Cursor::new(b"This is not");
    assert!(!Gas::<TestTarget>::is_comment(&cursor));

    let cursor = Cursor::new(b"/ This is not");
    assert!(!Gas::<TestTarget>::is_comment(&cursor));

    let cursor = Cursor::new(b"; This is not");
    assert!(!Gas::<TestTarget>::is_comment(&cursor));
}

#[test]
fn test_try_comment() {
    let mut cursor = Cursor::new(b"# This is a comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 19);

    let mut cursor = Cursor::new(b"# This ; is a # comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 23);

    let mut cursor = Cursor::new(b"This is not");
    assert_eq!(Gas::<TestTarget>::try_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"/ This is not");
    assert_eq!(Gas::<TestTarget>::try_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"; This is not");
    assert_eq!(Gas::<TestTarget>::try_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);
}

#[test]
fn test_is_slash_star_comment() {
    let cursor = Cursor::new(b"/* This is a ss comment */");
    assert!(Gas::<TestTarget>::is_slash_star_comment(&cursor));

    let cursor = Cursor::new(b"/* This is a ss comment");
    assert!(Gas::<TestTarget>::is_slash_star_comment(&cursor));

    let cursor = Cursor::new(b"# /* This is not");
    assert!(!Gas::<TestTarget>::is_slash_star_comment(&cursor));

    let cursor = Cursor::new(b"/ /* This is not");
    assert!(!Gas::<TestTarget>::is_slash_star_comment(&cursor));

    let cursor = Cursor::new(b"// /* This is not");
    assert!(!Gas::<TestTarget>::is_slash_star_comment(&cursor));
}

#[test]
fn test_try_slash_star_comment() {
    let mut cursor = Cursor::new(b"/* This is a ss comment */");
    assert_eq!(
        Gas::<TestTarget>::try_slash_star_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 26);

    let mut cursor = Cursor::new(b"/* This is a ss comment");
    assert_eq!(
        Gas::<TestTarget>::try_slash_star_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 23);

    let mut cursor = Cursor::new(b"/* # This * / / is * a ss * / * comment*/");
    assert_eq!(
        Gas::<TestTarget>::try_slash_star_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 41);

    let mut cursor = Cursor::new(b"/ * # This * / / is * not */");
    assert_eq!(Gas::<TestTarget>::try_slash_star_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);
}

#[test]
fn test_is_multibyte_comment() {
    let cursor = Cursor::new(b"// This is a mb comment\n");
    assert!(Gas::<TestTarget>::is_multibyte_comment(&cursor),);

    let cursor = Cursor::new(b"/* This is not a mb comment\n");
    assert!(!Gas::<TestTarget>::is_multibyte_comment(&cursor),);

    let cursor = Cursor::new(b"# This is not a mb comment\n");
    assert!(!Gas::<TestTarget>::is_multibyte_comment(&cursor),);
}

#[test]
fn test_try_multibyte_comment() {
    let mut cursor = Cursor::new(b"// This is a mb comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_multibyte_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 23);

    let mut cursor = Cursor::new(b"// This /* */ \"\" is a # mb comment\n");
    assert_eq!(
        Gas::<TestTarget>::try_multibyte_comment(&mut cursor),
        Some(source::Kind::Comment)
    );
    assert_eq!(cursor.pos(), 34);

    let mut cursor = Cursor::new(b"/* This not a mb comment\n");
    assert_eq!(Gas::<TestTarget>::try_multibyte_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);

    let mut cursor = Cursor::new(b"## This not a mb comment\n");
    assert_eq!(Gas::<TestTarget>::try_multibyte_comment(&mut cursor), None);
    assert_eq!(cursor.pos(), 0);
}

#[test]
fn test_lex_args() {
    let check = |bytes, span, pos| {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<TestTarget>::lex_args(&mut cursor), span);
        assert_eq!(cursor.pos(), pos);
    };
    check(b"eax, edx", Some(0..8), 8);
    check(b"eax, edx\n", Some(0..8), 8);
    check(b"eax, edx;", Some(0..8), 8);
    check(b"eax, edx#", Some(0..8), 8);
    check(b"eax, edx//", Some(0..8), 8);
    check(b" eax, edx", Some(1..9), 9);
    check(b"eax, /* cmnt */ edx", Some(0..19), 19);
    check(b"eax, edx /* cmnt */#", Some(0..19), 19);
    check(b"eax, edx    ", Some(0..8), 12);
    check(b" 1 / 3", Some(1..6), 6);
    check(b" 1 / 3 \n", Some(1..6), 7);
    check(b";1 / 3 \n", None, 0);
    check(b"", None, 0);
    check(b"\t\r \t\r", None, 5);
}

#[test]
fn test_try_symbol_kind() {
    use source::Kind::*;

    let check = |bytes, kind, pos| {
        let mut cursor = Cursor::new(bytes);
        assert_eq!(Gas::<TestTarget>::try_symbol_kind(&mut cursor), kind);
        assert_eq!(cursor.pos(), pos);
    };

    check(b"", None, 0);

    /* Labels */
    let label = |name| Some(Label { name });
    check(b"Label:", label(0..5), 6);
    check(b"Label: ", label(0..5), 6);
    check(b"Label :", label(0..5), 7);
    check(b"\"!@#$%\":", label(1..6), 8);
    check(b"\"!@#$%\" :", label(1..6), 9);
    check(b"55:", label(0..2), 3);
    check(b"55 :", label(0..2), 4);
    check(b"55$:", label(0..3), 4);
    check(b"55$ :", label(0..3), 5);

    /* Directives */
    let dir = |name, args| Some(Directive { name, args });
    check(b".dir", dir(0..4, None), 4);
    check(b".dir;", dir(0..4, None), 4);
    check(b".dir#", dir(0..4, None), 4);
    check(b".dir\n", dir(0..4, None), 4);
    check(b".dir//", dir(0..4, None), 4);
    check(b".dir ", dir(0..4, None), 5);
    check(b".dir .", dir(0..4, Some(5..6)), 6);
    check(b".dir/**/", dir(0..4, Some(4..8)), 8);
    check(b".dir \"arg\"", dir(0..4, Some(5..10)), 10);
    check(b".dir \"arg\"  ", dir(0..4, Some(5..10)), 12);
    check(b".dir /**/ \"arg\"", dir(0..4, Some(5..15)), 15);
    check(b".dir /*\n\n*/ \"arg\"", dir(0..4, Some(5..17)), 17);
    check(b".dir \"a\ng\"  ", dir(0..4, Some(5..10)), 12);

    /* Instructions */
    let insn = |mnemonic, args| Some(Instruction { mnemonic, args });
    check(b"nop", insn(0..3, None), 3);
    check(b"nop;", insn(0..3, None), 3);
    check(b"nop#", insn(0..3, None), 3);
    check(b"nop\n", insn(0..3, None), 3);
    check(b"nop//", insn(0..3, None), 3);
    check(b"nop ", insn(0..3, None), 4);
    check(b"nop/**/", insn(0..3, Some(3..7)), 7);
    check(b"nop/*\n*/", insn(0..3, Some(3..8)), 8);
    check(b"mov eax, edx", insn(0..3, Some(4..12)), 12);
    check(b"mov eax,/*\n*/ edx", insn(0..3, Some(4..17)), 17);

    /* Definition */
    let def = |symbol, keyword, args| {
        Some(Definition {
            symbol,
            keyword,
            args,
        })
    };
    check(b"Symbol = Value", def(0..6, 7..8, Some(9..14)), 14);
    check(b"Symbol= Value", def(0..6, 6..7, Some(8..13)), 13);
    check(b"Symbol =Value", def(0..6, 7..8, Some(8..13)), 13);
    check(b"Symbol=Value", def(0..6, 6..7, Some(7..12)), 12);
    check(b"Symbol==Value", def(0..6, 6..8, Some(8..13)), 13);
    check(b"Symbol ==Value", def(0..6, 7..9, Some(9..14)), 14);
    check(b"Symbol== Value", def(0..6, 6..8, Some(9..14)), 14);
    check(b"Symbol == Value", def(0..6, 7..9, Some(10..15)), 15);
    check(b"Symbol =; Value", def(0..6, 7..8, None), 8);
    check(b"Symbol =\n Value", def(0..6, 7..8, None), 8);
    check(b"Symbol =# Value", def(0..6, 7..8, None), 8);
    check(b".=.+4", def(0..1, 1..2, Some(2..5)), 5);
}

fn assert_coverage(bytes: &[u8]) {
    let gap_bytes = ByteSet::from_bytes(b" ;\t\r");
    let mut cursor = Cursor::new(bytes);
    let mut i = 0usize;
    while let Some(item) = Gas::<TestTarget>::next_item(&mut cursor) {
        assert!(item.span.start <= item.span.end);
        assert!(item.span.end <= bytes.len());
        while i < item.span.start {
            assert!(bytes.get(i).copied().is_some_and(|b| gap_bytes.contains(b)));
            i += 1;
        }
        while i < item.span.end {
            assert!(
                bytes
                    .get(i)
                    .copied()
                    .is_some_and(|b| !gap_bytes.contains(b))
            );
            i += 1;
        }
    }
}

#[test]
fn test_next_item_preprocessor() {
    let mut cursor = cursor::Cursor::new(
        b"
# 123 \"filename\"
#123\"filename\" 1 2 3
#123\"filename\" junk
",
    );

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Preprocessor,
            span: 1..17,
            starts_line: true,
        })
    );
    assert_eq!(cursor.pos(), 17);

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Preprocessor,
            span: 18..38,
            starts_line: true,
        })
    );
    assert_eq!(cursor.pos(), 38);

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Comment,
            span: 39..58,
            starts_line: true,
        })
    );
    assert_eq!(cursor.pos(), 58);
}

#[test]
fn test_next_item_comment() {
    let mut cursor = cursor::Cursor::new(
        b"
/* I am a slash star comment */ / I am a line comment
/ So am I
# I am a regular comment
",
    );

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Comment,
            span: 1..32,
            starts_line: true
        })
    );
    assert_eq!(cursor.pos(), 32);

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Comment,
            span: 33..54,
            starts_line: false
        })
    );
    assert_eq!(cursor.pos(), 54);

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Comment,
            span: 55..64,
            starts_line: true
        })
    );
    assert_eq!(cursor.pos(), 64);

    assert_eq!(
        Gas::<TestTarget>::next_item(&mut cursor),
        Some(Item {
            kind: source::Kind::Comment,
            span: 65..89,
            starts_line: true
        })
    );
    assert_eq!(cursor.pos(), 89);
}
