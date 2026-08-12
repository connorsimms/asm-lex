pub mod class;
pub mod set;
pub mod table;

pub use class::Class;
pub use set::Set;
pub use table::Table;

pub const ASCII_ALPHA_LOWER: Set = Set::from_range(b'a', b'z');
pub const ASCII_ALPHA_UPPER: Set = Set::from_range(b'A', b'Z');
pub const ASCII_ALPHA: Set = ASCII_ALPHA_LOWER.with_set(&ASCII_ALPHA_UPPER);
pub const ASCII_DIGIT: Set = Set::from_range(b'0', b'9');
