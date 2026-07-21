#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ByteSet {
    bytes: [u64; 4],
}

impl ByteSet {
    pub const fn empty() -> Self {
        Self { bytes: [0u64; 4] }
    }

    #[must_use]
    pub const fn from_bytes(input: &[u8]) -> Self {
        let mut bytes = [0u64; 4];

        let mut i = 0usize;
        while i < input.len() {
            let b = input[i];
            let idx = b >> 6;
            let val = b & 63;
            bytes[idx as usize] |= 1 << val;
            i += 1;
        }

        Self { bytes }
    }

    #[must_use]
    pub const fn from_multibyte_start_chars(input: &[&[u8]]) -> Self {
        let mut bytes = [0u64; 4];

        let mut i = 0usize;
        while i < input.len() {
            if !input[i].is_empty() {
                let b = input[i][0];
                let idx = b >> 6;
                let val = b & 63;
                bytes[idx as usize] |= 1 << val;
            }
            i += 1;
        }

        Self { bytes }
    }

    /// # Panics
    #[must_use]
    pub const fn with_range(mut self, start: u8, end: u8) -> Self {
        assert!(start <= end, "Start must not be greater than end");
        // widening to usize avoids overflow
        let mut b = start as usize;
        while b <= end as usize {
            let idx = b >> 6;
            let val = b & 63;
            self.bytes[idx] |= 1 << val;
            b += 1;
        }
        self
    }

    #[must_use]
    pub const fn union(mut self, byte_set: &Self) -> Self {
        let mut i = 0;
        while i < 256 {
            let idx = i >> 6;
            let val = i & 63;
            if (byte_set.bytes[idx] & (1 << val)) != 0 {
                self.bytes[idx] |= 1 << val;
            }
            i += 1;
        }
        self
    }

    pub fn contains(&self, b: u8) -> bool {
        let idx = (b >> 6) as usize;
        let val = b & 63;
        self.bytes[idx] & (1 << val) != 0
    }
}
