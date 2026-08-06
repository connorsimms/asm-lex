#[cfg(test)]
mod tests;

pub trait Pattern: crate::sealed::PatternType {
    fn find(&self, haystack: &[u8]) -> Option<usize>;
}

impl Pattern for u8 {
    fn find(&self, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "memchr")]
        {
            memchr::memchr(*self, haystack)
        }
        #[cfg(not(feature = "memchr"))]
        {
            let mut offset = 0usize;
            let mut chunks = haystack.chunks_exact(8);
            let remainder = chunks.remainder();
            let mut needle = *self as u64;
            needle |= needle << 8;
            needle |= needle << 16;
            needle |= needle << 32;
            for chunk in chunks {
                let hay = u64::from_le_bytes(chunk.try_into().unwrap());
                let mut hits = !(hay ^ needle);
                hits &= hits >> 1;
                hits &= hits >> 2;
                hits &= hits >> 4;
                hits &= 0x01010101_01010101;
                if hits != 0 {
                    let ctz = hits.trailing_zeros() as usize;
                    return Some(offset + (ctz / 8));
                }
                offset += 8;
            }
            for b in remainder {
                if b == self {
                    return Some(offset);
                }
                offset += 1;
            }
            None
        }
    }
}
