#[allow(unused)]
use pretty_assertions::{assert_eq, assert_ne};

use super::*;

#[test]
fn test_new() {
    let bytes = b"Some string";
    let cursor = Cursor::new(bytes);
    assert_eq!(cursor.pos(), 0);
    assert_eq!(cursor.bytes(), bytes);
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
fn test_is_eof() {
    let mut cursor = Cursor::new(b"Some string");
    assert!(!cursor.is_eof());
    cursor.advance(5);
    assert!(!cursor.is_eof());
    cursor.advance(6);
    assert!(cursor.is_eof());
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
