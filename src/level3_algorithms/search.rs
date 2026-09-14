pub fn binary_search<T: Ord>(v: &[T], target: &T) -> Option<usize> {
    v.binary_search(target).ok()
}

pub fn linear_search<T: PartialEq>(v: &[T], target: &T) -> Option<usize> {
    v.iter().position(|x| x == target)
}
