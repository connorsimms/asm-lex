use super::*;
#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};
use proptest::collection::vec;
use proptest::prelude::*;

#[test]
fn test_new() {
    let bytes = b"Some string";
    let cursor = Cursor::new(bytes);
    assert_eq!(cursor.pos(), 0);
    assert_eq!(cursor.bytes(), bytes);
}

#[test]
fn test_restore() {
    let bytes = b"Some string";
    let mut cursor = Cursor::new(bytes);
    for i in 0..=bytes.len() {
        cursor.restore(i);
        assert_eq!(cursor.pos(), i);
    }
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_restore_invalid() {
    let bytes = b"Some string";
    let mut cursor = Cursor::new(bytes);
    cursor.restore(bytes.len() + 1);
}

#[test]
fn test_advance() {
    let bytes = &[b'a'; 100];
    let mut cursor = Cursor::new(bytes);
    let mut prev = 0;
    for i in 0..=5 {
        cursor.advance(i);
        assert_eq!(cursor.pos(), prev + i);
        prev = cursor.pos();
    }
    cursor.advance(usize::MAX);
    assert_eq!(cursor.pos(), bytes.len());
}

proptest! {
    #[test]
    fn prop_test_advance(bytes in vec(any::<u8>(), 0..1000), steps in vec(any::<usize>(), 0..100)) {
        let mut cursor = Cursor::new(&bytes);
        for step in steps {
            cursor.advance(step);
            prop_assert!(cursor.pos() <= cursor.bytes().len());
        }
    }
}

#[test]
fn test_at_line_start() {
    let mut cursor = Cursor::new(b"\nSome\nstring\n");
    assert!(cursor.at_line_start());
    cursor.advance(1);
    assert!(cursor.at_line_start());
    cursor.advance(1);
    assert!(!cursor.at_line_start());
    cursor.advance(4);
    assert!(cursor.at_line_start());
    cursor.advance(usize::MAX);
    assert!(cursor.at_line_start());
    let mut cursor = Cursor::new(b"\nSome\nstring");
    cursor.advance(usize::MAX);
    assert!(!cursor.at_line_start());
}

#[test]
fn test_peek() {
    let mut cursor = Cursor::new(b"Some string");
    for i in 0..cursor.bytes().len() {
        assert_eq!(cursor.peek(), Some(cursor.bytes()[i]));
        cursor.advance(1);
    }
    assert_eq!(cursor.peek(), None);
}

#[test]
fn test_seek() {
    let mut cursor = Cursor::new(b"Some string");
    assert_eq!(cursor.seek(isize::MIN), None);
    assert_eq!(cursor.seek(-1), None);
    assert_eq!(cursor.seek(0), Some(b'S'));
    assert_eq!(cursor.seek(5), Some(b's'));
    assert_eq!(cursor.seek(isize::MAX), None);
    cursor.restore(5);
    assert_eq!(cursor.seek(isize::MIN), None);
    assert_eq!(cursor.seek(-5), Some(b'S'));
    assert_eq!(cursor.seek(0), Some(b's'));
    assert_eq!(cursor.seek(6), None);
    assert_eq!(cursor.seek(isize::MAX), None);
}

#[test]
fn test_starts_with() {
    let mut cursor = Cursor::new(b"Some string");
    assert!(cursor.starts_with(b""));
    assert!(cursor.starts_with(b"S"));
    assert!(cursor.starts_with(b"Some"));
    assert!(cursor.starts_with(b"Some string"));
    assert!(cursor.starts_with(b"Some s"));
    assert!(!cursor.starts_with(b"Somestringg"));
    assert!(!cursor.starts_with(b"Some stringg"));
    assert!(!cursor.starts_with(b"Some string "));
    assert!(!cursor.starts_with(b"Some string\n"));
    cursor.advance(usize::MAX);
    assert!(cursor.starts_with(b""));
    assert!(!cursor.starts_with(b"g"));
}

proptest! {
    #[test]
    fn prop_test_starts_with(bytes in vec(any::<u8>(), 0..100), needle in vec(any::<u8>(),0..200)) {
        let cursor = Cursor::new(&bytes);
        prop_assert_eq!(cursor.starts_with(needle.as_ref()), bytes.starts_with(needle.as_ref()));
    }
}

#[test]
fn test_bump() {
    let mut cursor = Cursor::new(b"Some string");
    assert_eq!(cursor.bump(), Some(b'S'));
    assert_eq!(cursor.pos(), 1);
    assert_eq!(cursor.bump(), Some(b'o'));
    assert_eq!(cursor.pos(), 2);
    cursor.advance(8);
    assert_eq!(cursor.bump(), Some(b'g'));
    assert_eq!(cursor.pos(), 11);
    assert_eq!(cursor.bump(), None);
    assert_eq!(cursor.pos(), 11);
}

proptest! {
    #[test]
    fn prop_test_bump(bytes in vec(any::<u8>(), 0..100)) {
        let mut cursor = Cursor::new(&bytes);
        for _ in 0..=100 {
            let pos = cursor.pos();
            if cursor.bump().is_some() {
                prop_assert_eq!(cursor.pos(), pos + 1);
            } else {
                prop_assert_eq!(cursor.pos(), pos);
                prop_assert_eq!(cursor.pos(), bytes.len());
            }
        }
    }

    #[test]
    fn prop_test_bump_peek(bytes in vec(any::<u8>(), 0..100)) {
        let mut cursor = Cursor::new(&bytes);
        for _ in 0..=100 {
            prop_assert_eq!(cursor.peek(), cursor.bump());
        }
    }

    #[test]
    fn prop_test_bump_seek(bytes in vec(any::<u8>(), 0..100)) {
        let mut cursor = Cursor::new(&bytes);
        for _ in 0..bytes.len() {
            prop_assert_eq!(cursor.bump(), cursor.seek(-1));
        }
        prop_assert_eq!(cursor.bump(), None);
    }
}

#[test]
fn test_eat() {
    let mut cursor = Cursor::new(b"Some string");
    assert!(cursor.eat(b'S'));
    assert_eq!(cursor.pos(), 1);
    assert!(!cursor.eat(b'S'));
    assert_eq!(cursor.pos(), 1);
    assert!(cursor.eat(b'o'));
    assert_eq!(cursor.pos(), 2);
    cursor.advance(3);
    assert!(cursor.eat(b's'));
    assert_eq!(cursor.pos(), 6);
    assert!(!cursor.eat(b's'));
    assert_eq!(cursor.pos(), 6);
    cursor.advance(5);
    assert!(!cursor.eat(b'g'));
    assert_eq!(cursor.pos(), 11);
}

proptest! {
    #[test]
    fn prop_test_eat_peek(bytes in vec(any::<u8>(), 0..100), attempts in vec(any::<u8>(), 0..200)) {
        let mut cursor = Cursor::new(&bytes);
        for attempt in attempts {
            if Some(attempt) == cursor.peek() {
                prop_assert!(cursor.eat(attempt));
            } else {
                prop_assert!(!cursor.eat(attempt));
            }
        }
    }

    #[test]
    fn prop_test_eat_seek(bytes in vec(any::<u8>(), 0..100), attempts in vec(any::<u8>(), 0..200)) {
        let mut cursor = Cursor::new(&bytes);
        for attempt in attempts {
            if cursor.eat(attempt) {
                prop_assert_eq!(cursor.seek(-1), Some(attempt));
            } else {
                prop_assert_ne!(cursor.peek(), Some(attempt));
            }
        }
    }
}

#[test]
fn test_eat_while() {
    let mut cursor = Cursor::new(b"Some string");
    assert_eq!(cursor.eat_while(|b| b == b'S' || b == b'o'), 0..2);
    assert_eq!(cursor.pos(), 2);
    assert_eq!(cursor.eat_while(|b| b != b's'), 2..5);
    assert_eq!(cursor.pos(), 5);
    assert_eq!(cursor.eat_while(|b| b != b's'), 5..5);
    assert_eq!(cursor.pos(), 5);
    assert_eq!(cursor.eat_while(|b| b != b'z'), 5..11);
    assert_eq!(cursor.pos(), 11);
    assert_eq!(cursor.eat_while(|_| true), 11..11);
}

proptest! {
    #[test]
    fn prop_test_eat_while(bytes in vec(any::<u8>(), 0..100), set in vec(any::<u8>(), 0..100)) {
        let mut cursor = Cursor::new(&bytes);
        let span = cursor.eat_while(|b| set.contains(&b));
        for b in &bytes[span] {
            prop_assert!(set.contains(b));
        }
        prop_assert!(!cursor.peek().is_some_and(|b| set.contains(&b)));
    }
}
