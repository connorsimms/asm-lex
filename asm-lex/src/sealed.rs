pub trait PatternType {}

impl<const N: usize> PatternType for crate::pattern::AnyOf<N> {}
impl<const N: usize> PatternType for crate::pattern::Substring<N> {}
