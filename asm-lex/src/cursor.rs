#[cfg(test)]
mod tests;

use crate::Span;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    #[cfg(test)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// # Panics
    pub fn restore(&mut self, pos: usize) {
        assert!(
            pos <= self.bytes.len(),
            "Attempted to restore out of bounds"
        );
        self.pos = pos;
    }

    pub fn advance(&mut self, n: usize) {
        self.pos = core::cmp::min(self.bytes.len(), self.pos.saturating_add(n));
    }

    pub fn is_eof(&self) -> bool {
        self.pos == self.bytes.len()
    }

    pub fn at_line_start(&self) -> bool {
        self.pos() == 0 || self.seek(-1).is_some_and(|b| b == b'\n')
    }

    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    pub fn seek(&self, offset: isize) -> Option<u8> {
        self.bytes
            .get(self.pos.checked_add_signed(offset)?)
            .copied()
    }

    pub fn starts_with(&self, seq: &[u8]) -> bool {
        self.bytes
            .get(self.pos()..)
            .is_some_and(|slice| slice.starts_with(seq))
    }

    pub fn bump(&mut self) -> Option<u8> {
        if self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }

    pub fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(u8) -> bool) -> Span {
        let start = self.pos();
        while let Some(b) = self.peek() {
            if predicate(b) {
                self.pos += 1;
            } else {
                break;
            }
        }
        start..self.pos()
    }
}
