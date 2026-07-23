#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ByteSet {
    bytes: [u64; 4],
}

impl ByteSet {
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: [0u64; 4] }
    }

    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self::new().with_bytes(bytes)
    }

    #[must_use]
    pub const fn from_first_bytes(input: &[&[u8]]) -> Self {
        let mut set = Self::new();
        let mut i = 0usize;
        while i < input.len() {
            if let Some(&b) = input[i].first() {
                set = set.with_byte(b);
            }
            i += 1;
        }
        set
    }

    pub const fn contains(&self, byte: u8) -> bool {
        self.bytes[(byte >> 6) as usize] & (1 << (byte & 63)) != 0
    }

    pub fn insert(&mut self, byte: u8) {
        self.bytes[(byte >> 6) as usize] |= 1 << (byte & 63);
    }

    pub fn union(&mut self, set: &ByteSet) {
        let mut i = 0usize;
        while i < 4 {
            self.bytes[i] |= set.bytes[i];
            i += 1;
        }
    }

    #[must_use]
    pub const fn with_byte(mut self, byte: u8) -> Self {
        self.bytes[(byte >> 6) as usize] |= 1 << (byte & 63);
        self
    }

    #[must_use]
    pub const fn with_bytes(mut self, bytes: &[u8]) -> Self {
        let mut i = 0usize;
        while i < bytes.len() {
            self = self.with_byte(bytes[i]);
            i += 1;
        }
        self
    }

    /// # Panics
    /// Panics if `start` is greater than `end`.
    #[must_use]
    pub const fn with_range(mut self, start: u8, end: u8) -> Self {
        assert!(start <= end, "Start must not be greater than end");
        let mut b = start;
        loop {
            let idx = (b >> 6) as usize;
            let val = b & 63;
            self.bytes[idx] |= 1 << val;
            if b == end {
                break;
            }
            b += 1;
        }
        self
    }

    #[must_use]
    pub const fn with_set(mut self, rhs: &Self) -> Self {
        let mut i = 0usize;
        while i < 4 {
            self.bytes[i] |= rhs.bytes[i];
            i += 1;
        }
        self
    }
}

impl From<&[u8]> for ByteSet {
    fn from(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl<const N: usize> From<&[u8; N]> for ByteSet {
    fn from(bytes: &[u8; N]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl core::ops::BitOr<Self> for ByteSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new().with_set(&self).with_set(&rhs)
    }
}

impl core::ops::BitOr<&Self> for ByteSet {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        Self::new().with_set(&self).with_set(rhs)
    }
}

impl core::ops::BitOrAssign<Self> for ByteSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.union(&rhs);
    }
}

impl core::ops::BitOrAssign<&Self> for ByteSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.union(rhs);
    }
}
