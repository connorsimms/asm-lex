mod common;
use asm_lex::cursor::Cursor;
use asm_lex::source::Dialect;
use asm_lex::source::Kind;
use asm_lex::Span;

#[allow(unused)]
fn check_next_item<D: Dialect>(bytes: &[u8], expected: &[(Kind, Span, bool)]) {
    let mut cursor = Cursor::new(bytes);
    for (kind, span, starts_line) in expected {
        let item = D::next_item(&mut cursor).unwrap();
        assert_eq!(item.kind(), kind);
        assert_eq!(item.span(), span);
        assert_eq!(item.starts_line(), *starts_line);
    }
}
