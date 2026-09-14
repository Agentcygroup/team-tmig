pub fn is_connected(components: usize) -> bool {
    components == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn torus_is_one_component() { assert!(is_connected(1)); }
    #[test] fn two_tori_are_not() { assert!(!is_connected(2)); }
}
