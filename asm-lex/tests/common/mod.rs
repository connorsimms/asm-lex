pub mod invariants;
pub mod snapshot;
pub mod targets;

// prototype api design stuff

pub struct Lexer<'a, D: asm_lex::source::Dialect> {
    cursor: asm_lex::cursor::Cursor<'a>,
    _dialect: core::marker::PhantomData<D>,
}

impl<'a, D: asm_lex::source::Dialect> Lexer<'a, D> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            cursor: asm_lex::cursor::Cursor::new(bytes),
            _dialect: core::marker::PhantomData,
        }
    }
}

impl<'a, D: asm_lex::source::Dialect> Iterator for Lexer<'a, D> {
    type Item = asm_lex::source::Item;

    fn next(&mut self) -> Option<Self::Item> {
        D::next_item(&mut self.cursor)
    }
}

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
        Comment => {}
        Preprocessor => {}
        Unknown => {}
        _ => {
            panic!("Variant is not handled");
        }
    }
}
