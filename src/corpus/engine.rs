use crate::corpus::manifest::Manifest;
use crate::corpus::spectral::{Axis, Spectral};
use std::path::Path;

pub struct Engine {
    pub manifest: Manifest,
    pub out_dir: std::path::PathBuf,
}

impl Engine {
    pub fn new(mp: impl AsRef<Path>, od: impl AsRef<Path>) -> std::io::Result<Self> {
        let manifest = Manifest::load(mp)?;
        std::fs::create_dir_all(od.as_ref())?;
        Ok(Self {
            manifest,
            out_dir: od.as_ref().to_path_buf(),
        })
    }
    pub fn run_all_crosscuts(&self) -> std::io::Result<usize> {
        let axes = Axis::all();
        let mut n = 0;
        for a in axes {
            for b in axes {
                let s = Spectral::compute(&self.manifest, *a, *b);
                let fname = format!("cross_{}_{}.tsv", a.name(), b.name());
                std::fs::write(self.out_dir.join(&fname), s.to_tsv())?;
                n += 1;
            }
        }
        Ok(n)
    }
    pub fn run_self_tests(&self) -> Vec<(String, bool)> {
        let v = vec![
            ("manifest_non_empty".into(), !self.manifest.is_empty()),
            ("classes_at_least_one".into(), self.manifest.classes() >= 1),
        ];
        v
    }
}
