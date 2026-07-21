#![allow(unused)]

use crate::Span;

pub struct Frame {
    kind: Kind,
    span: Span,
}

pub enum Kind {}
pub trait Carrier {}
