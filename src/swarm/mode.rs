use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum Mode {
    #[default]
    Adaptive,
    Reactive,
    Planned,
    Coordinated,
    Swarm,
    Propagate,
    Migrate,
    Mimic,
    Linger,
    Veil,
    Observe,
    Seal,
    Watch,
    Inject,
    Carry,
    Drop,
    Dwell,
    Wait,
    Tick,
    Morph,
    Rewrite,
    Obscure,
    Wrap,
    Fold,
    Blur,
    Pulse,
    Orchestrate,
    Siphon,
    Anchor,
    Slide,
    Rise,
    Scan,
    Open,
    Act,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Mode> {
        Some(match s {
            "adaptive" => Mode::Adaptive,
            "reactive" => Mode::Reactive,
            "planned" => Mode::Planned,
            "coordinated" => Mode::Coordinated,
            "swarm" => Mode::Swarm,
            "propagate" => Mode::Propagate,
            "migrate" => Mode::Migrate,
            "mimic" => Mode::Mimic,
            "linger" => Mode::Linger,
            "veil" => Mode::Veil,
            "observe" => Mode::Observe,
            "seal" => Mode::Seal,
            "watch" => Mode::Watch,
            "inject" => Mode::Inject,
            "carry" => Mode::Carry,
            "drop" => Mode::Drop,
            "dwell" => Mode::Dwell,
            "wait" => Mode::Wait,
            "tick" => Mode::Tick,
            "morph" => Mode::Morph,
            "rewrite" => Mode::Rewrite,
            "obscure" => Mode::Obscure,
            "wrap" => Mode::Wrap,
            "fold" => Mode::Fold,
            "blur" => Mode::Blur,
            "pulse" => Mode::Pulse,
            "orchestrate" => Mode::Orchestrate,
            "siphon" => Mode::Siphon,
            "anchor" => Mode::Anchor,
            "slide" => Mode::Slide,
            "rise" => Mode::Rise,
            "scan" => Mode::Scan,
            "open" => Mode::Open,
            "act" => Mode::Act,
            _ => return None,
        })
    }
}

impl std::str::FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Mode::parse(s).ok_or_else(|| format!("unknown mode: {}", s))
    }
}
