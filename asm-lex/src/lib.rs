#![no_std]
#![allow(clippy::must_use_candidate)]

mod cursor;
mod pattern;
mod source;

pub type Span = core::ops::Range<usize>;
