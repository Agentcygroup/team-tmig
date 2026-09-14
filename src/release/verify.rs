use crate::release::{Artifact, Signature};

pub fn verify_release(arts: &[Artifact], sigs: &[Signature]) -> (usize, usize) {
    let mut p = 0;
    let mut f = 0;
    for (a, s) in arts.iter().zip(sigs.iter()) {
        if a.hash == s.artifact_hash {
            p += 1
        } else {
            f += 1
        }
    }
    (p, f)
}
