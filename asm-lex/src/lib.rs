#![allow(clippy::must_use_candidate)]

pub type Span = core::ops::Range<usize>;

pub mod cursor;
pub mod pattern;
