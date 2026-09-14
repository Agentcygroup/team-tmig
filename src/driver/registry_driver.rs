use crate::registry::{ContentRegistry, IndexRegistry, VersionRegistry};

pub fn run() -> (usize, usize, usize) {
    let mut c = ContentRegistry::new();
    let mut v = VersionRegistry::new();
    let mut i = IndexRegistry::new();

    c.put([1u8; 32], b"alpha".to_vec());
    c.put([2u8; 32], b"beta".to_vec());
    v.publish("tmig", "0.1.0");
    v.publish("tmig", "0.2.0");
    i.bind("alpha", [1u8; 32]);
    i.bind("beta", [2u8; 32]);

    (
        c.len(),
        v.all("tmig").map(<[String]>::len).unwrap_or(0),
        i.count(),
    )
}
