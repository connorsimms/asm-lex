use super::*;

#[test]
fn find_u8() {
    let haystack = b"abcdefghijklmnop";
    for b in 0..=255u8 {
        assert_eq!(
            b.find(haystack),
            haystack.iter().position(|x| b == *x),
            "{}",
            b as char
        );
    }
}
