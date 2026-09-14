use crate::level7_governance::control::{all_controls as controls, Control};

#[derive(Clone, Debug)]
pub struct AuditReport {
    pub total: usize,
    pub implemented: usize,
    pub gaps: Vec<Control>,
}

pub fn run() -> AuditReport {
    let c = controls();
    let gaps: Vec<Control> = c.iter().filter(|x| !x.implemented).cloned().collect();
    AuditReport {
        total: c.len(),
        implemented: c.iter().filter(|x| x.implemented).count(),
        gaps,
    }
}
