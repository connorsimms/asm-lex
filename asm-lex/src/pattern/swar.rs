#[cfg(not(feature = "memchr"))]
pub const LO: u64 = 0x0101_0101_0101_0101;

#[cfg(not(feature = "memchr"))]
pub const HI: u64 = 0x8080_8080_8080_8080;

#[cfg(not(feature = "memchr"))]
#[inline]
pub fn check(mask: u64) -> u64 {
    mask.wrapping_sub(LO) & !mask & HI
}
