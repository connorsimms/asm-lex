#![allow(unused)]

#[cfg(test)]
mod tests;

mod swar {
    #![allow(unused)]
    pub const LO: u64 = 0x0101_0101_0101_0101;
    pub const HI: u64 = 0x8080_8080_8080_8080;
}

pub struct AnyOf<const N: usize>(pub [u8; N]);

impl<const N: usize> AnyOf<N> {
    #[allow(unused)]
    const SUPPORTED: () = assert!(N >= 1 && N <= 3, "AnyOf supports 1 to 3 bytes");
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

#[inline]
fn check(mask: u64) -> u64 {
    mask.wrapping_sub(swar::LO) & !mask & swar::HI
}

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
