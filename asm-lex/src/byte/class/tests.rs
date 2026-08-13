use super::*;

#[test]
#[should_panic(expected = "bit should be less than 16")]
fn with_bit_invalid() {
    let _class = Class::with_bit(17);
}

#[test]
fn contains() {
    let mask_b = (1 << 2) | (1 << 4) | (1 << 6);
    let mask_c = (1 << 3) | (1 << 6) | (1 << 9);

    let a = Class::with_bit(2);
    let b = Class(mask_b);
    let c = Class(mask_c);

    assert!(a.contains(b));
    assert!(b.contains(a));
    assert!(!c.contains(a));
    assert!(!a.contains(c));
}
