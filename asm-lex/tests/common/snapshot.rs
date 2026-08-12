#![allow(dead_code)]
#![allow(clippy::incompatible_msrv)]

pub struct Escaped(String);

impl std::fmt::Debug for Escaped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

pub struct SnapSpan {
    text: Escaped,
    span: asm_lex::Span,
}

impl std::fmt::Debug for SnapSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{} {:?}", self.span.start, self.span.end, self.text)
    }
}

impl SnapSpan {
    pub fn from_span(span: asm_lex::Span, bytes: &[u8]) -> Self {
        Self {
            text: Escaped(bytes[span.clone()].escape_ascii().to_string()),
            span,
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Kind {
    Label {
        name: SnapSpan,
    },
    Directive {
        name: SnapSpan,
        args: Option<SnapSpan>,
    },
    Instruction {
        mnemonic: SnapSpan,
        args: Option<SnapSpan>,
    },
    Definition {
        symbol: SnapSpan,
        keyword: SnapSpan,
        args: Option<SnapSpan>,
    },
    Comment,
    Preprocessor,
    Unknown,
}

impl Kind {
    pub fn from_source_kind(kind: &asm_lex::source::Kind, bytes: &[u8]) -> Self {
        use self::Kind as snap;
        use asm_lex::source::Kind as source;

        match kind.clone() {
            source::Label { name } => snap::Label {
                name: SnapSpan::from_span(name, bytes),
            },
            source::Directive { name, args } => snap::Directive {
                name: SnapSpan::from_span(name, bytes),
                args: args.map(|a| SnapSpan::from_span(a, bytes)),
            },
            source::Instruction { mnemonic, args } => snap::Instruction {
                mnemonic: SnapSpan::from_span(mnemonic, bytes),
                args: args.map(|a| SnapSpan::from_span(a, bytes)),
            },
            source::Definition {
                symbol,
                keyword,
                args,
            } => snap::Definition {
                symbol: SnapSpan::from_span(symbol, bytes),
                keyword: SnapSpan::from_span(keyword, bytes),
                args: args.map(|a| SnapSpan::from_span(a, bytes)),
            },
            source::Comment => snap::Comment,
            source::Preprocessor => snap::Preprocessor,
            source::Unknown => snap::Unknown,
            _ => {
                panic!("This variant is not handled")
            }
        }
    }
}

#[derive(Debug)]
pub struct Item {
    span: SnapSpan,
    kind: Kind,
    starts_line: bool,
}

impl Item {
    pub fn from_source_item(item: &asm_lex::source::Item, bytes: &[u8]) -> Self {
        Self {
            span: SnapSpan::from_span(item.span().clone(), bytes),
            kind: Kind::from_source_kind(item.kind(), bytes),
            starts_line: item.starts_line(),
        }
    }
}
