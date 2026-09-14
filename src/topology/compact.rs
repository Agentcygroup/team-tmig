pub fn is_compact(has_boundary: bool, finite: bool) -> bool {
    finite && !has_boundary
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn closed_torus_is_compact() { assert!(is_compact(false, true)); }
    #[test] fn infinite_plane_is_not() { assert!(!is_compact(false, false)); }
}
