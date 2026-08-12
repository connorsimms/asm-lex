#![cfg_attr(not(test), no_std)]
#![allow(clippy::must_use_candidate)]

pub mod byte;
pub mod cursor;
pub mod listing;
pub mod pattern;
mod sealed;
pub mod source;

pub type Span = core::ops::Range<usize>;
