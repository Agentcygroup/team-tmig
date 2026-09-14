use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    Content,
    Structure,
    Metadata,
    History,
    Access,
}
