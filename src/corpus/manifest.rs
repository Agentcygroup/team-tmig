use std::path::Path;

#[derive(Clone, Debug)]
pub struct Source {
    pub class_: String,
    pub authority: String,
    pub url: String,
    pub license: String,
    pub size: String,
    pub scope: String,
    pub dtype: String,
    pub cadence: String,
    pub access: String,
    pub domain: String,
    pub population: u64,
}

pub struct Manifest {
    pub sources: Vec<Source>,
}

impl Manifest {
    pub fn load(p: impl AsRef<Path>) -> std::io::Result<Self> {
        let t = std::fs::read_to_string(p)?;
        let mut lines = t.lines();
        let header = lines.next().unwrap_or("");
        let cols: Vec<&str> = header.split(',').collect();
        let idx = |k: &str| cols.iter().position(|c| *c == k).unwrap_or(0);
        let i_class = idx("class_");
        let i_auth = idx("authority");
        let i_url = idx("url");
        let i_lic = idx("license");
        let i_size = idx("size");
        let i_scope = idx("scope");
        let i_dt = idx("dtype");
        let i_cad = idx("cadence");
        let i_acc = idx("access");
        let i_dom = idx("domain");
        let i_pop = idx("population");
        let mut sources = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let p: Vec<&str> = line.split(',').collect();
            let g = |i: usize| {
                p.get(i)
                    .map(|s| s.trim_matches('"').to_string())
                    .unwrap_or_default()
            };
            sources.push(Source {
                class_: g(i_class),
                authority: g(i_auth),
                url: g(i_url),
                license: g(i_lic),
                size: g(i_size),
                scope: g(i_scope),
                dtype: g(i_dt),
                cadence: g(i_cad),
                access: g(i_acc),
                domain: g(i_dom),
                population: g(i_pop).parse().unwrap_or(0),
            });
        }
        Ok(Self { sources })
    }
    pub fn len(&self) -> usize {
        self.sources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
    pub fn total_population(&self) -> u64 {
        self.sources.iter().map(|s| s.population).sum()
    }
    pub fn classes(&self) -> usize {
        self.sources
            .iter()
            .map(|s| s.class_.clone())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    }
}
