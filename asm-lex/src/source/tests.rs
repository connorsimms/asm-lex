use super::*;
use proptest::prelude::*;

#[test]
fn kind_accessor() {
    let kinds = [
        Kind::Label { name: 1..3 },
        Kind::Directive {
            name: 2..5,
            args: Some(1..2),
        },
        Kind::Instruction {
            mnemonic: 3..7,
            args: None,
        },
        Kind::Definition {
            symbol: 0..5,
            keyword: 2..7,
            args: Some(1..2),
        },
        Kind::Comment,
        Kind::Preprocessor,
        Kind::Unknown,
    ];
    for kind in &kinds {
        let item = Item {
            kind: kind.clone(),
            span: 0..0,
            starts_line: false,
        };
        assert_eq!(item.kind(), kind);
    }
}

#[test]
fn span_accessor() {
    let spans = [0..2, 2..5, 5..13, 7..19];
    for span in &spans {
        let item = Item {
            kind: Kind::Unknown,
            span: span.clone(),
            starts_line: false,
        };
        assert_eq!(item.span(), span);
    }
}

proptest! {
    #[test]
    fn prop_span_accessor(span: Span) {
        let item = Item {
            kind: Kind::Unknown,
            span: span.clone(),
            starts_line: false,
        };
        assert_eq!(item.span(), &span);
    }
}

#[test]
fn starts_line_accessor() {
    let starts = [true, false];
    for starts_line in starts {
        let item = Item {
            kind: Kind::Unknown,
            span: 0..0,
            starts_line,
        };
        assert_eq!(item.starts_line(), starts_line);
    }
}
