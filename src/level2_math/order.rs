pub fn min_of<T: Ord + Copy>(xs: &[T]) -> Option<T> {
    xs.iter().copied().min()
}
pub fn max_of<T: Ord + Copy>(xs: &[T]) -> Option<T> {
    xs.iter().copied().max()
}
pub fn is_sorted<T: Ord>(xs: &[T]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}
