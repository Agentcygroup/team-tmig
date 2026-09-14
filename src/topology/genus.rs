pub fn genus(euler: i64, orientable: bool) -> Option<i64> {
    if orientable {
        let num = 2 - euler;
        if num % 2 == 0 { Some(num / 2) } else { None }
    } else {
        let num = 2 - euler;
        if num >= 0 { Some(num) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn torus_is_genus_one() {
        assert_eq!(genus(0, true), Some(1));
    }
    #[test] fn sphere_is_genus_zero() {
        assert_eq!(genus(2, true), Some(0));
    }
    #[test] fn klein_bottle_is_non_orientable() {
        assert_eq!(genus(0, false), Some(2));
    }
}
