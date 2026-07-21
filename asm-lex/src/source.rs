pub mod gas;

use crate::Span;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Kind {
    Label {
        name: Span,
    },
    Directive {
        name: Span,
        args: Option<Span>,
    },
    Instruction {
        mnemonic: Span,
        args: Option<Span>,
    },
    Definition {
        symbol: Span,
        keyword: Span,
        args: Option<Span>,
    },
    Comment,
    Preprocessor,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    kind: Kind,
    span: Span,
    starts_line: bool,
}

pub trait Dialect {
    fn next_item(cur: &mut crate::cursor::Cursor<'_>) -> Option<Item>;
}
