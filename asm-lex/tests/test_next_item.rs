mod common;
use asm_lex::cursor::Cursor;
use asm_lex::source::Dialect;
use asm_lex::source::Kind;
use asm_lex::Span;

fn check_next_item<D: Dialect>(bytes: &[u8], expected: &[(Kind, Span, bool)]) {
    let mut cursor = Cursor::new(bytes);
    for (kind, span, starts_line) in expected {
        let item = D::next_item(&mut cursor).unwrap();
        assert_eq!(item.kind(), kind);
        assert_eq!(item.span(), span);
        assert_eq!(item.starts_line(), *starts_line);
    }
}

#[test]
fn next_item() {
    use asm_lex::source::gas::Gas;
    use common::targets::*;
    use Kind::*;

    let bytes: &[u8] = b"
insn arg, arg # Comment
.dir \"string\"
insn arg
# 100 \"file\"
Label:
    insn
    .dir
";
    let expected: &[(Kind, Span, bool)] = &[
        (
            Instruction {
                mnemonic: 1..5,
                args: Some(6..14),
            },
            1..14,
            true,
        ),
        (Comment, 15..24, false),
        (
            Directive {
                name: 25..29,
                args: Some(30..38),
            },
            25..38,
            true,
        ),
        (
            Instruction {
                mnemonic: 39..43,
                args: Some(44..47),
            },
            39..47,
            true,
        ),
        (Preprocessor, 48..60, true),
        (Label { name: 61..66 }, 61..67, true),
        (
            Instruction {
                mnemonic: 72..76,
                args: None,
            },
            72..76,
            true,
        ),
        (
            Directive {
                name: 81..85,
                args: None,
            },
            81..85,
            true,
        ),
    ];

    check_next_item::<Gas<X86_64LinuxElf>>(bytes, expected);
}
