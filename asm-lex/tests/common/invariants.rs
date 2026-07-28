#[allow(unused)]
use asm_lex::cursor::Cursor;
#[allow(unused)]
use asm_lex::source::Dialect;

#[allow(unused)]
fn assert_coverage<D: Dialect>(bytes: &[u8]) {
    let mut cursor = Cursor::new(bytes);
    while let Some(item) = D::next_item(&mut cursor) {
        todo!();
    }
}
