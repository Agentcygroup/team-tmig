pub fn merge_sort<T: Ord + Clone>(v: &[T]) -> Vec<T> {
    if v.len() <= 1 {
        return v.to_vec();
    }
    let m = v.len() / 2;
    let a = merge_sort(&v[..m]);
    let b = merge_sort(&v[m..]);
    let mut o = Vec::with_capacity(v.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            o.push(a[i].clone());
            i += 1
        } else {
            o.push(b[j].clone());
            j += 1
        }
    }
    o.extend_from_slice(&a[i..]);
    o.extend_from_slice(&b[j..]);
    o
}
