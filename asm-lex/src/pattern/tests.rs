use super::*;

#[test]
fn test_from_bytes() {
    let set = ByteSet::from_bytes(b"ABC123\x00\xFF ");
    assert!(set.contains(b'A'));
    assert!(set.contains(b' '));
    assert!(!set.contains(b'a'));
    assert!(set.contains(b'\x00'));
    assert!(set.contains(b'\xFF'));
}

#[test]
fn test_contains() {
    let set = ByteSet::from_bytes(b"ABC123\n\t\r ");
    assert!(set.contains(b'A'));
    assert!(set.contains(b'B'));
    assert!(set.contains(b'C'));
    assert!(set.contains(b'\n'));
    assert!(set.contains(b'\r'));
    assert!(!set.contains(b'\0'));
    assert!(!set.contains(b'Z'));
}

#[test]
fn test_with_range() {
    let set = ByteSet::empty().with_range(b'a', b'z');
    assert!(set.contains(b'a'));
    assert!(set.contains(b'e'));
    assert!(set.contains(b'z'));
    assert!(!set.contains(b'A'));
    assert!(!set.contains(b'1'));
    let set = ByteSet::empty().with_range(0, 255);
    assert!(set.contains(b'a'));
}

#[test]
#[should_panic(expected = "Start must not be greater than end")]
fn test_with_range_invalid() {
    let _ = ByteSet::empty().with_range(255, 0);
}
