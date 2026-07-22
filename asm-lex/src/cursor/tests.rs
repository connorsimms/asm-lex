use super::*;
#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};
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
    assert_eq!(cursor.seek(-1), None);
    assert_eq!(cursor.seek(0), Some(b'S'));
    assert_eq!(cursor.seek(5), Some(b's'));
    cursor.advance(5);
    assert_eq!(cursor.seek(-5), Some(b'S'));
    assert_eq!(cursor.seek(0), Some(b's'));
    assert_eq!(cursor.seek(6), None);
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
