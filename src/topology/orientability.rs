pub fn is_orientable(euler: i64) -> bool {
    (2 - euler) % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sphere_orientable() { assert!(is_orientable(2)); }
    #[test] fn torus_orientable() { assert!(is_orientable(0)); }
    #[test] fn double_torus_orientable() { assert!(is_orientable(-2)); }
}
