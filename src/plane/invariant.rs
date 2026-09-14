use crate::plane::terminal::CommandEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvariantKind {
    Safety,
    Reversibility,
    BoundedResources,
    ContentAddressing,
    Attribution,
}

#[derive(Clone, Debug)]
pub struct Invariant {
    pub kind: InvariantKind,
}

impl Invariant {
    pub fn all() -> Vec<Self> {
        use InvariantKind::*;
        vec![
            Safety,
            Reversibility,
            BoundedResources,
            ContentAddressing,
            Attribution,
        ]
        .into_iter()
        .map(|kind| Self { kind })
        .collect()
    }

    pub fn check_command(&self, command: &str) -> bool {
        match self.kind {
            InvariantKind::Safety => is_safe(command),
            InvariantKind::BoundedResources => command.len() <= 4096,
            _ => true,
        }
    }

    pub fn check_event(&self, event: &CommandEvent) -> bool {
        match self.kind {
            InvariantKind::Reversibility => event.parent.is_some() || event.tick == 0,
            InvariantKind::ContentAddressing => event.hash == event.stable_hash(),
            InvariantKind::Attribution => !event.command.is_empty() && event.terminal_id < u64::MAX,
            InvariantKind::BoundedResources => event.stdout_hash.len() == 64,
            InvariantKind::Safety => is_safe(&event.command),
        }
    }
}

fn is_safe(command: &str) -> bool {
    let c = command.trim();
    if c.is_empty() {
        return false;
    }
    let banned = [
        "rm", "mv", "cp", "dd", "mkfs", "shutdown", "reboot", "sudo", "su", "chmod", "chown",
        "kill", "curl", "wget", "ssh", "scp", "rsync", ">", "|", ";", "&&", "||", "`", "$(",
    ];
    !banned
        .iter()
        .any(|b| c == *b || c.starts_with(&format!("{} ", b)) || c.contains(b))
}
