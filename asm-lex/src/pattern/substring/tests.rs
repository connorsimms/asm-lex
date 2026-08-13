#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::pattern::Pattern;

#[test]
fn substring_2() {
    let haystack = b"abcdefghijkl";
    assert_eq!(Substring(*b"ab").find(haystack), Some(0));
    assert_eq!(Substring(*b"cd").find(haystack), Some(2));
    assert_eq!(Substring(*b"dc").find(haystack), None);
    assert_eq!(Substring(*b"ef").find(haystack), Some(4));
    assert_eq!(Substring(*b"fe").find(haystack), None);
    assert_eq!(Substring(*b"xy").find(haystack), None);
}
