pub trait PatternType {}

impl<const N: usize> PatternType for crate::pattern::AnyOf<N> {}
