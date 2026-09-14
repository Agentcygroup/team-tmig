use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            "rs" | "py" | "go" | "ts" | "tsx" | "js" | "jsx" | "c" | "cc" | "cpp" | "h" | "hpp"
            | "java" | "kt" | "swift" | "rb" | "sh" | "zsh" | "bash" => Self::Source,
            "toml" | "yaml" | "yml" | "json" | "ini" | "conf" | "cfg" | "env" => Self::Config,
            "md" | "txt" | "rst" | "adoc" | "tex" => Self::Doc,
            "csv" | "tsv" | "parquet" | "arrow" | "sqlite" | "db" => Self::Data,
            "bin" | "so" | "dylib" | "dll" | "exe" | "o" | "a" => Self::Binary,
            _ => Self::Unknown,
        }
    }
}
