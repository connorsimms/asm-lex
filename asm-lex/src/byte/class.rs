#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Class(pub u16);

impl Class {
    /// # Panics
    /// Panics if `bit` is greater than or equal to `u16::BITS`
    pub const fn with_bit(bit: u32) -> Self {
        assert!(bit < u16::BITS, "bit should be less than 16");
        Self(1 << bit)
    }

    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}
