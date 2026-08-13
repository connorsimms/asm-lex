#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;
use crate::pattern::Pattern;

#[test]
fn any_of_1() {
    let haystack = b"abcdefghijklmnopqrstuvwxyz";
    for b in 0..=255u8 {
        assert_eq!(
            AnyOf([b]).find(haystack),
            haystack.iter().position(|x| *x == b)
        );
    }
}

#[test]
fn any_of_2() {
    use core::cmp::min;
    let haystack = b"abcdefghijklmnopqrstuvwxyz";
    for b1 in 0..=255u8 {
        for b2 in 0..=255u8 {
            let p1 = haystack.iter().position(|x| *x == b1);
            let p2 = haystack.iter().position(|x| *x == b2);
            if p1.is_none() && p2.is_none() {
                assert_eq!(AnyOf([b1, b2]).find(haystack), None);
            } else {
                assert_eq!(
                    AnyOf([b1, b2]).find(haystack),
                    min(p1.or(Some(usize::MAX)), p2.or(Some(usize::MAX)))
                );
            }
        }
    }
}

#[test]
fn any_of_3() {
    use core::cmp::min;
    let haystack = b"abcdefghijkl";
    for b1 in 0..=128u8 {
        let p1 = haystack.iter().position(|x| *x == b1);
        for b2 in 0..=128u8 {
            let p2 = haystack.iter().position(|x| *x == b2);
            for b3 in 0..=128u8 {
                let p3 = haystack.iter().position(|x| *x == b3);
                if p1.is_none() && p2.is_none() && p3.is_none() {
                    assert_eq!(AnyOf([b1, b2, b3]).find(haystack), None);
                } else {
                    assert_eq!(
                        AnyOf([b1, b2, b3]).find(haystack),
                        min(
                            p1.or(Some(usize::MAX)),
                            min(p2.or(Some(usize::MAX)), p3.or(Some(usize::MAX)))
                        )
                    );
                }
            }
        }
    }
}
