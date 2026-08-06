#[cfg(test)]
mod tests;

use crate::Span;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// # Panics
    /// Panics if `pos` is greater than `bytes.len()`.
    #[inline]
    pub fn restore(&mut self, pos: usize) {
        debug_assert!(
            pos <= self.bytes.len(),
            "Attempted to restore out of bounds"
        );
        self.pos = pos;
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        self.pos = core::cmp::min(self.bytes.len(), self.pos.saturating_add(n));
    }

    #[inline]
    pub fn at_line_start(&self) -> bool {
        self.pos() == 0 || self.seek(-1).is_some_and(|b| b == b'\n')
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    #[inline]
    pub fn seek(&self, offset: isize) -> Option<u8> {
        self.bytes
            .get(self.pos.checked_add_signed(offset)?)
            .copied()
    }

    #[inline]
    pub fn starts_with(&self, bytes: &[u8]) -> bool {
        self.bytes
            .get(self.pos()..)
            .is_some_and(|slice| slice.starts_with(bytes))
    }

    #[inline]
    pub fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    #[inline]
    pub fn eat(&mut self, byte: u8) -> bool {
        if Some(byte) == self.peek() {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    #[inline]
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

    pub fn eat_until(&mut self, pattern: &impl crate::pattern::Pattern) -> Span {
        let start = self.pos();
        let haystack = &self.bytes()[start..];
        let offset = pattern.find(haystack);
        let end = match offset {
            Some(offset) => start + offset,
            None => self.bytes().len(),
        };
        self.restore(end);
        start..end
    }
}
