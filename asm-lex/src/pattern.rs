#[cfg(test)]
mod tests;

#[cfg(not(feature = "memchr"))]
mod swar {
    pub const LO: u64 = 0x0101_0101_0101_0101;
    pub const HI: u64 = 0x8080_8080_8080_8080;
}

pub struct AnyOf<const N: usize>(pub [u8; N]);

pub struct Substring<const N: usize>(pub [u8; N]);

impl<const N: usize> AnyOf<N> {
    #[allow(unused)]
    const SUPPORTED: () = assert!(N >= 1 && N <= 3, "AnyOf supports 1 to 3 bytes");
}

impl<const N: usize> Substring<N> {
    #[allow(unused)]
    const SUPPORTED: () = assert!(N == 2, "Substring supports 2 bytes");
}

pub trait Pattern: crate::sealed::PatternType {
    fn find(&self, haystack: &[u8]) -> Option<usize>;
}

#[cfg(feature = "memchr")]
#[inline]
fn find_memchr<const N: usize>(needles: &[u8; N], haystack: &[u8]) -> Option<usize> {
    match N {
        1 => memchr::memchr(needles[0], haystack),
        2 => memchr::memchr2(needles[0], needles[1], haystack),
        3 => memchr::memchr3(needles[0], needles[1], needles[2], haystack),
        _ => panic!("{N} is not supported"),
    }
}

#[cfg(not(feature = "memchr"))]
#[inline]
fn check(mask: u64) -> u64 {
    mask.wrapping_sub(swar::LO) & !mask & swar::HI
}

#[cfg(not(feature = "memchr"))]
#[inline]
fn find_any<const N: usize>(bytes: &[u8; N], haystack: &[u8]) -> Option<usize> {
    let needles = bytes.map(|n| u64::from(n) * swar::LO);
    let mut chunks = haystack.chunks_exact(8);
    let mut offset: usize = 0;

    for chk in &mut chunks {
        let chk = u64::from_le_bytes(chk.try_into().unwrap());

        let mut hits: u64 = 0;
        for n in needles {
            hits |= check(chk ^ n);
        }

        if hits != 0 {
            return Some(offset + hits.trailing_zeros() as usize / 8);
        }
        offset += 8;
    }

    chunks
        .remainder()
        .iter()
        .position(|b| bytes.contains(b))
        .map(|i| offset + i)
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
        let hits = check(chk ^ n1) | check(shf ^ n2);
        if hits != 0 {
            return Some(offset + hits.trailing_zeros() as usize / 8);
        }
        offset += 8;
    }

    haystack[offset..]
        .windows(substr.len())
        .position(|sub| sub == substr)
}

impl<const N: usize> Pattern for AnyOf<N> {
    fn find(&self, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "memchr")]
        {
            find_memchr(&self.0, haystack)
        }
        #[cfg(not(feature = "memchr"))]
        {
            find_any(&self.0, haystack)
        }
    }
}

impl<const N: usize> Pattern for Substring<N> {
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
