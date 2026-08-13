#[cfg(test)]
mod tests;

#[cfg(not(feature = "memchr"))]
use crate::pattern::swar;

pub struct Substring<const N: usize>(pub [u8; N]);

impl<const N: usize> Substring<N> {
    #[allow(unused)]
    const SUPPORTED: () = assert!(N == 2, "Substring supports 2 bytes");
}

#[cfg(feature = "memchr")]
#[inline]
fn find_memmem<const N: usize>(needle: &[u8; N], haystack: &[u8]) -> Option<usize> {
    memchr::memmem::find(haystack, needle)
}

#[cfg(not(feature = "memchr"))]
#[inline]
fn find_substr<const N: usize>(substr: &[u8; N], haystack: &[u8]) -> Option<usize> {
    let shifted = &haystack[substr.len() - 1..];
    let n1 = u64::from(*substr.first()?) * swar::LO;
    let n2 = u64::from(*substr.last()?) * swar::LO;
    let chunks = haystack.chunks_exact(8);
    let shifted_chunks = shifted.chunks_exact(8);
    let mut offset: usize = 0;

    for (chk, shf) in chunks.zip(shifted_chunks) {
        let chk = u64::from_le_bytes(chk.try_into().unwrap());
        let shf = u64::from_le_bytes(shf.try_into().unwrap());
        let hits = swar::check(chk ^ n1) & swar::check(shf ^ n2);
        if hits != 0 {
            return Some(offset + hits.trailing_zeros() as usize / 8);
        }
        offset += 8;
    }

    haystack[offset..]
        .windows(substr.len())
        .position(|sub| sub == substr)
}

impl<const N: usize> crate::pattern::Pattern for Substring<N> {
    fn find(&self, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "memchr")]
        {
            find_memmem(&self.0, haystack)
        }
        #[cfg(not(feature = "memchr"))]
        {
            find_substr(&self.0, haystack)
        }
    }
}
