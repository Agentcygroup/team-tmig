use crate::accelerator::{CacheEngine, MemoEngine, SimdEngine};

pub fn run() -> (usize, u64, usize) {
    let s = SimdEngine::new();
    let x = s.xor_blocks(&[1, 2, 3, 4, 5, 6, 7, 8], &[8, 7, 6, 5, 4, 3, 2, 1]);

    let mut c = CacheEngine::new();
    c.put("k", b"v".to_vec());
    let _ = c.get("k");
    let _ = c.get("missing");

    let mut m: MemoEngine<u32, u32> = MemoEngine::new();
    let _ = m.memoized(1, || 10);
    let _ = m.memoized(1, || 20);

    (x.len(), c.hits, m.store.len())
}
