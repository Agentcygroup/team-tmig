pub fn total<T, E, F>(f: F, x: T) -> Result<E, ()>
where
    F: FnOnce(T) -> Option<E>,
{
    f(x).ok_or(())
}

pub fn total_or<T, E>(fallback: E, f: impl FnOnce() -> Option<E>) -> E {
    f().unwrap_or(fallback)
}
