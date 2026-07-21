use super::*;

extern crate alloc;

#[test]
fn test_new() {
    const SET: ByteSet = ByteSet::new();
    for b in b'\x00'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_empty() {
    const SET: ByteSet = ByteSet::empty();
    for b in b'\x00'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_insert() {
    let mut set = ByteSet::empty();
    set.insert(b'\x00');
    assert!(set.contains(b'\x00'));
    set.insert(b'\xFF');
    assert!(set.contains(b'\xFF'));
}

#[test]
fn test_contains() {
    let bytes = b"ABC123!@#";
    let set = ByteSet::from_bytes(bytes);
    for b in 0..=255u8 {
        assert_eq!(bytes.contains(&b), set.contains(b));
    }
}

#[test]
fn test_from_bytes() {
    const SET: ByteSet = ByteSet::from_bytes(b"ABCDEF");
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    for b in b'A'..=b'F' {
        assert!(SET.contains(b));
    }
    for b in b'G'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_from_bytes_empty() {
    const SET: ByteSet = ByteSet::from_bytes(b"");
    for b in b'\x00'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_from_bytes_repeat() {
    const SET: ByteSet = ByteSet::from_bytes(b"AAABBCDD");
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    for b in b'A'..=b'D' {
        assert!(SET.contains(b));
    }
    for b in b'E'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_from_first_bytes() {
    const WORDS: &[&[u8]] = &[b"Apple", b"Banana", b"Cherry"];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    for b in b'A'..=b'C' {
        assert!(SET.contains(b));
    }
    for b in b'D'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_from_first_bytes_empty() {
    const WORDS: &[&[u8]] = &[];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in b'\x00'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_from_first_bytes_repeat() {
    const WORDS: &[&[u8]] = &[b"Apple", b"Banana", b"Broccoli"];
    const SET: ByteSet = ByteSet::from_first_bytes(WORDS);
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    for b in b'A'..=b'B' {
        assert!(SET.contains(b));
    }
    for b in b'C'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_with_range() {
    const SET: ByteSet = ByteSet::new().with_range(b'A', b'Y');
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    for b in b'A'..=b'Y' {
        assert!(SET.contains(b));
    }
    for b in b'Z'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
fn test_with_range_single() {
    const SET: ByteSet = ByteSet::new().with_range(b'A', b'A');
    for b in b'\x00'..b'A' {
        assert!(!SET.contains(b));
    }
    assert!(SET.contains(b'A'));
    for b in b'B'..=b'\xFF' {
        assert!(!SET.contains(b));
    }
}

#[test]
#[should_panic(expected = "Start must not be greater than end")]
fn test_with_range_invalid() {
    let _set = ByteSet::new().with_range(b'B', b'A');
}

#[test]
fn test_from() {
    let set = ByteSet::from(b"");
    for b in b'\x00'..=b'\xFF' {
        assert!(!set.contains(b));
    }

    let set = ByteSet::from(b"ABCDEF");
    for b in b'\x00'..b'A' {
        assert!(!set.contains(b));
    }
    for b in b'A'..=b'F' {
        assert!(set.contains(b));
    }
    for b in b'G'..=b'\xFF' {
        assert!(!set.contains(b));
    }

    let set = ByteSet::from(alloc::vec![b'A', b'B', b'C', b'D', b'E', b'F']);
    for b in b'\x00'..b'A' {
        assert!(!set.contains(b));
    }
    for b in b'A'..=b'F' {
        assert!(set.contains(b));
    }
    for b in b'G'..=b'\xFF' {
        assert!(!set.contains(b));
    }

    let set = ByteSet::from("ABCDEF");
    for b in b'\x00'..b'A' {
        assert!(!set.contains(b));
    }
    for b in b'A'..=b'F' {
        assert!(set.contains(b));
    }
    for b in b'G'..=b'\xFF' {
        assert!(!set.contains(b));
    }
}
