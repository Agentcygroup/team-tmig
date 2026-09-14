#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileKind {
    Source,
    Config,
    Doc,
    Data,
    Binary,
    Unknown,
}

impl FileKind {
    pub fn from_ext(ext: &str) -> Self {
        match ext {
            "rs" | "py" | "go" | "ts" | "tsx" | "js" | "c" | "cc" | "cpp" | "h" | "hpp" => {
                Self::Source
            }
            "toml" | "yaml" | "yml" | "json" | "ini" | "conf" | "cfg" | "env" => Self::Config,
            "md" | "txt" | "rst" | "adoc" => Self::Doc,
            "csv" | "tsv" | "parquet" | "arrow" => Self::Data,
            "bin" | "so" | "dylib" | "dll" | "exe" | "o" => Self::Binary,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Layer {
    Content,
    Structure,
    Metadata,
    History,
    Access,
}

#[derive(Clone, Debug)]
pub struct LiveFileAgent {
    pub path: String,
    pub kind: FileKind,
    pub layer: Layer,
    pub size: u64,
    pub coherence: f32,
    pub tick: u64,
}

impl LiveFileAgent {
    pub fn new(path: &str, layer: Layer, kind: FileKind, size: u64) -> Self {
        Self {
            path: path.into(),
            kind,
            layer,
            size,
            coherence: 0.5,
            tick: 0,
        }
    }
    pub fn tick(&mut self) {
        self.tick += 1;
        self.coherence = (self.coherence * 0.99 + 0.01).min(1.0);
    }
}
