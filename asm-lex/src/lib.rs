#![no_std]
#![allow(clippy::must_use_candidate)]

pub mod cursor;
mod listing;
pub mod pattern;
pub mod source;

pub type Span = core::ops::Range<usize>;
