#[cfg(test)]
mod tests;

use crate::byte::{Class, Set};

pub struct Table {
    table: [u16; 256],
}

impl Table {
    pub const fn build(entries: &[(Class, &Set)]) -> Self {
        let mut table = [0u16; 256];
        let mut i = 0;
        while i < entries.len() {
            let (class, set) = entries[i];
            let mut b = 0u8;
            loop {
                if set.contains(b) {
                    table[b as usize] |= class.0;
                }
                if b == 255 {
                    break;
                }
                b += 1;
            }
            i += 1;
        }
        Self { table }
    }

    #[inline]
    pub fn classify(&self, b: u8) -> Class {
        Class(self.table[b as usize])
    }
}
