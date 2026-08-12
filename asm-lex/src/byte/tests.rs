use super::*;

#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};
use proptest::collection::vec;
use proptest::prelude::*;

extern crate alloc;

#[test]
fn new() {
    const SET: ByteSet = ByteSet::new();
    for b in 0u8..=255 {
        assert!(!SET.contains(b));
    }
}

#[test]
fn from_bytes() {
    const BYTES: &[u8] = b"ABCDEF";
    const SET: ByteSet = ByteSet::from_bytes(BYTES);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), BYTES.contains(&b));
    }
}

#[test]
fn from_bytes_empty() {
    const SET: ByteSet = ByteSet::from_bytes(b"");
    for b in 0u8..=255 {
        assert!(!SET.contains(b));
    }
}

#[test]
fn from_bytes_repeat() {
    const BYTES: &[u8] = b"AAABBCDD";
    const SET: ByteSet = ByteSet::from_bytes(BYTES);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), BYTES.contains(&b));
    }
}

proptest! {
    #[test]
    fn prop_from_bytes(bytes in vec(any::<u8>(), 0..300)) {
        let set = ByteSet::from_bytes(&bytes);
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), bytes.contains(&b));
        }
    }
}

#[test]
fn from_first_bytes() {
    const WORDS: &[[u8; 2]] = &[*b"AA", *b"BB", *b"CC"];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), (b'A'..=b'C').contains(&b));
    }
}

#[test]
fn from_first_bytes_empty() {
    const WORDS: &[[u8; 2]] = &[];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in 0u8..=255 {
        assert!(!SET.contains(b));
    }
}

#[test]
fn from_first_bytes_repeat() {
    const WORDS: &[[u8; 2]] = &[*b"AA", *b"BB", *b"BB"];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), (b'A'..=b'B').contains(&b));
    }
}

proptest! {
    #[test]
    fn prop_from_first_bytes(bytes in vec(proptest::array::uniform2(1u8..), 0..300)) {
        let set = ByteSet::from_first_bytes(&bytes);
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), bytes.iter().any(|w| w.first() == Some(&b)));
        }
    }
}

#[test]
fn from_range() {
    const SET: ByteSet = ByteSet::from_range(b'a', b'y');
    for b in 0u8..b'a' {
        assert!(!SET.contains(b));
    }
    for b in b'a'..=b'y' {
        assert!(SET.contains(b));
    }
    for b in b'z'..=255 {
        assert!(!SET.contains(b));
    }
}

#[test]
fn contains() {
    const BYTES: &[u8] = b"ABC123!@#";
    const SET: ByteSet = ByteSet::from_bytes(BYTES);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), BYTES.contains(&b));
    }
}

#[test]
fn insert() {
    let mut set = ByteSet::new();
    set.insert(0u8);
    assert!(set.contains(0u8));
    set.insert(255u8);
    assert!(set.contains(255u8));
}

proptest! {
    #[test]
    fn prop_insert_byte(b: u8) {
        let mut set = ByteSet::new();
        set.insert(b);

        for x in 0u8..=255 {
            prop_assert_eq!(set.contains(x), b == x);
        }
    }

    #[test]
    fn prop_insert_bytes(bytes in vec(any::<u8>(), 0..300)) {
        let mut set = ByteSet::new();
        for b in &bytes {
            set.insert(*b);
        }

        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), bytes.contains(&b));
        }
    }
}

#[test]
fn union() {
    let bytes1 = b"ABC";
    let bytes2 = b"XYZ";
    let mut set1 = ByteSet::from_bytes(bytes1);
    let set2 = ByteSet::from_bytes(bytes2);
    set1.union(&set2);
    for b in 0u8..=255 {
        assert_eq!(set1.contains(b), bytes1.contains(&b) || bytes2.contains(&b));
    }
}

proptest! {
    #[test]
    fn prop_union(bytes1 in vec(any::<u8>(), 0..100), bytes2 in vec(any::<u8>(), 0..100)) {
        let mut set1 = ByteSet::from_bytes(&bytes1);
        let set2 = ByteSet::from_bytes(&bytes2);
        set1.union(&set2);
        for b in 0u8..=255 {
            prop_assert_eq!(set1.contains(b), bytes1.contains(&b) || bytes2.contains(&b));
        }
    }
}

#[test]
fn with_byte() {
    const SET1: ByteSet = ByteSet::new().with_byte(0);
    const SET2: ByteSet = ByteSet::new().with_byte(255);
    for b in 0u8..=255 {
        assert_eq!(SET1.contains(b), b == 0);
    }
    for b in 0u8..=255 {
        assert_eq!(SET2.contains(b), b == 255);
    }
}

proptest! {
    #[test]
    fn prop_with_byte(byte: u8) {
        let set = ByteSet::new().with_byte(byte);
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), b == byte);
        }
    }
}

#[test]
fn with_byte_if() {
    const SET1: ByteSet = ByteSet::new().with_byte_if(0, true);
    const SET2: ByteSet = ByteSet::new().with_byte_if(255, false);
    for b in 0u8..=255 {
        assert_eq!(SET1.contains(b), b == 0);
    }
    for b in 0u8..=255 {
        assert!(!SET2.contains(b));
    }
}

proptest! {
    #[test]
    fn prop_with_byte_if(byte: u8, cond: bool)
    {
        let set = ByteSet::new().with_byte_if(byte,cond);
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), b == byte && cond);
        }
    }
}

#[test]
fn with_bytes() {
    const BYTES: &[u8] = b"ABC";
    const SET: ByteSet = ByteSet::new().with_bytes(BYTES);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), BYTES.contains(&b));
    }
}

#[test]
fn with_bytes_empty() {
    const BYTES: &[u8] = b"";
    const SET: ByteSet = ByteSet::new().with_bytes(BYTES);
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), BYTES.contains(&b));
    }
}

proptest! {
    #[test]
    fn prop_with_bytes(bytes in vec(any::<u8>(), 0..300)) {
        let set = ByteSet::new().with_bytes(&bytes);
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), bytes.contains(&b));
        }
    }
}

#[test]
fn with_range() {
    const SET: ByteSet = ByteSet::new().with_range(b'A', b'Y');
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), (b'A'..=b'Y').contains(&b));
    }
}

#[test]
fn with_range_single() {
    const SET: ByteSet = ByteSet::new().with_range(b'A', b'A');
    for b in 0u8..=255 {
        assert_eq!(SET.contains(b), b == b'A');
    }
}

#[test]
#[should_panic(expected = "Start must not be greater than end")]
fn with_range_invalid() {
    let _set = ByteSet::new().with_range(b'B', b'A');
}

proptest! {
    #[test]
    fn prop_with_range(range in any::<core::ops::RangeInclusive<u8>>()) {
        let set = ByteSet::new().with_range(*range.start(), *range.end());
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), range.contains(&b));
        }
    }

    #[test]
    fn prop_with_ranges(ranges in any::<(core::ops::RangeInclusive<u8>, core::ops::RangeInclusive<u8>)>()) {
        let (range1, range2) = ranges;
        let mut set = ByteSet::new().with_range(*range1.start(), *range1.end());
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), range1.contains(&b));
        }
        set = set.with_range(*range2.start(), *range2.end());
        for b in 0u8..=255 {
            prop_assert_eq!(set.contains(b), range1.contains(&b) || range2.contains(&b));
        }
    }
}

#[test]
fn with_set() {
    const BYTES1: &[u8] = b"ABCDEF123";
    const BYTES2: &[u8] = b"XYZ";
    const SET1: ByteSet = ByteSet::from_bytes(BYTES1);
    const SET2: ByteSet = ByteSet::from_bytes(BYTES2).with_set(&SET1);
    for b in 0u8..=255 {
        if BYTES2.contains(&b) {
            assert!(SET2.contains(b));
        }
        if SET1.contains(b) {
            assert!(SET2.contains(b));
        }
    }
}

proptest! {
    #[test]
    fn prop_with_set(bytes1 in vec(any::<u8>(), 0..100), bytes2 in vec(any::<u8>(), 0..100)) {
        let set1 = ByteSet::from_bytes(&bytes1);
        let set2 = ByteSet::from_bytes(&bytes2).with_set(&set1);

        for b in 0u8..=255 {
            prop_assert_eq!(set2.contains(b), bytes1.contains(&b) || bytes2.contains(&b));
        }
    }
}

#[test]
fn from() {
    let bytes = b"";
    let set = ByteSet::from(bytes);
    for b in 0u8..=255 {
        assert_eq!(set.contains(b), bytes.contains(&b));
    }

    let bytes: &[u8] = b"ABCDEF";
    let set = ByteSet::from(bytes);
    for b in 0u8..=255 {
        assert_eq!(set.contains(b), bytes.contains(&b));
    }

    let bytes: &[u8; 4] = b"ABCD";
    let set = ByteSet::from(bytes);
    for b in 0u8..=255 {
        assert_eq!(set.contains(b), bytes.contains(&b));
    }
}

#[test]
fn bitor() {
    let bytes1 = b"ABC";
    let bytes2 = b"XYZ";
    let set1 = ByteSet::from_bytes(bytes1);
    let set2 = ByteSet::from_bytes(bytes2);
    let set3 = set1 | set2;
    for b in 0u8..=255 {
        assert_eq!(set3.contains(b), bytes1.contains(&b) || bytes2.contains(&b));
    }
}

#[test]
fn bitor_assign() {
    let bytes1 = b"ABC";
    let bytes2 = b"XYZ";
    let mut set1 = ByteSet::from_bytes(bytes1);
    let set2 = ByteSet::from_bytes(bytes2);
    set1 |= set2;
    for b in 0u8..=255 {
        assert_eq!(set1.contains(b), bytes1.contains(&b) || bytes2.contains(&b));
    }
}
