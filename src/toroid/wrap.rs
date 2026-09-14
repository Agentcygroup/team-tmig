pub fn wrap_i64(x: i64, m: i64) -> i64 {
    if m == 0 {
        0
    } else {
        ((x % m) + m) % m
    }
}
pub fn wrap_u64(x: u64, m: u64) -> u64 {
    if m == 0 {
        0
    } else {
        x % m
    }
}
