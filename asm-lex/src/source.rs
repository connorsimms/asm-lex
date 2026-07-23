#[cfg(test)]
pub mod tests;

pub mod gas;

use crate::Span;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    kind: Kind,
    span: Span,
    starts_line: bool,
}

impl Item {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn starts_line(&self) -> bool {
        self.starts_line
    }
}

pub trait Dialect {
    fn next_item(cur: &mut crate::cursor::Cursor<'_>) -> Option<Item>;
}
