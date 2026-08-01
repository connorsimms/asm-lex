use crate::cursor::Cursor;
use crate::source::Dialect;
use crate::source::Item;
use core::marker::PhantomData;

pub struct Lexer<'a, D: Dialect> {
    cursor: Cursor<'a>,
    _marker: PhantomData<D>,
}

impl<'a, D: Dialect> Lexer<'a, D> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            _marker: PhantomData,
        }
    }
}

impl<'a, D: Dialect> Iterator for Lexer<'a, D> {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        D::next_item(&mut self.cursor)
    }
}
