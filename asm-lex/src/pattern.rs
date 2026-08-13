mod any_of;
mod substring;
mod swar;

pub use any_of::AnyOf;
pub use substring::Substring;

pub trait Pattern: crate::sealed::PatternType {
    fn find(&self, haystack: &[u8]) -> Option<usize>;
}
