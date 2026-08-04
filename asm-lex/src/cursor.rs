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

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// # Panics
    /// Panics if `pos` is greater than `bytes.len()`.
    pub fn restore(&mut self, pos: usize) {
        debug_assert!(
            pos <= self.bytes.len(),
            "Attempted to restore out of bounds"
        );
        self.pos = pos;
    }

    pub fn advance(&mut self, n: usize) {
        self.pos = core::cmp::min(self.bytes.len(), self.pos.saturating_add(n));
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

    pub fn starts_with(&self, bytes: &[u8]) -> bool {
        self.bytes
            .get(self.pos()..)
            .is_some_and(|slice| slice.starts_with(bytes))
    }

    pub fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    pub fn eat(&mut self, byte: u8) -> bool {
        if Some(byte) == self.peek() {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(u8) -> bool) -> Span {
        let bytes = self.bytes();
        let len = bytes.len();
        let start = self.pos();
        let mut i = start;
        while i < len && predicate(bytes[i]) {
            i += 1;
        }
        self.restore(i);
        start..self.pos()
    }
}
