#![allow(dead_code)]

pub mod invariants;
pub mod proptest;
pub mod snapshot;

pub fn for_each_span(
    kind: &asm_lex::source::Kind,
    mut f: impl FnMut(&'static str, &asm_lex::Span),
) {
    use asm_lex::source::Kind::*;

    match kind {
        Label { name } => f("name", name),
        Directive { name, args } => {
            f("name", name);
            if let Some(a) = args {
                f("args", a);
            }
        }
        Instruction { mnemonic, args } => {
            f("mnemonic", mnemonic);
            if let Some(a) = args {
                f("args", a);
            }
        }
        Definition {
            symbol,
            keyword,
            args,
        } => {
            f("symbol", symbol);
            f("keyword", keyword);
            if let Some(a) = args {
                f("args", a);
            }
        }
        Comment | Preprocessor | Unknown => {}
        _ => {
            panic!("Variant is not handled");
        }
    }
}
