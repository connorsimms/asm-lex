#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ByteSet {
    bytes: [u64; 4],
}

impl ByteSet {
    pub const fn new() -> Self {
        Self::empty()
    }

    pub const fn empty() -> Self {
        Self { bytes: [0u64; 4] }
    }

    pub const fn insert(&mut self, b: u8) {
        let idx = (b >> 6) as usize;
        let val = b & 63;
        self.bytes[idx] |= 1 << val;
    }

    pub const fn contains(&self, b: u8) -> bool {
        let idx = (b >> 6) as usize;
        let val = b & 63;
        self.bytes[idx] & (1 << val) != 0
    }

    #[must_use]
    pub const fn from_bytes(input: &[u8]) -> Self {
        let mut set = Self::empty();
        let mut i = 0usize;
        while i < input.len() {
            let b = input[i];
            set.insert(b);
            i += 1;
        }
        set
    }

    #[must_use]
    pub const fn from_first_bytes(input: &[&[u8]]) -> Self {
        let mut set = Self::empty();
        let mut i = 0usize;
        while i < input.len() {
            if let Some(b) = input[i].first().copied() {
                set.insert(b);
            }
            i += 1;
        }
        set
    }

    /// # Panics
    /// Panics if `start` is greater than `end`.
    #[must_use]
    pub const fn with_range(mut self, start: u8, end: u8) -> Self {
        assert!(start <= end, "Start must not be greater than end");
        let mut b = start;
        loop {
            self.insert(b);
            if b == end {
                break;
            }
            b += 1;
        }
        self
    }

    #[must_use]
    pub const fn union(mut self, rhs: &Self) -> Self {
        let mut i = 0usize;
        while i < 4 {
            self.bytes[i] |= rhs.bytes[i];
            i += 1;
        }
        self
    }
}

impl<T: AsRef<[u8]>> From<T> for ByteSet {
    fn from(input: T) -> Self {
        Self::from_bytes(input.as_ref())
    }
}

impl core::ops::BitOr<Self> for ByteSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(&rhs)
    }
}

impl core::ops::BitOr<&Self> for ByteSet {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        self.union(rhs)
    }
}
