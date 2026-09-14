use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TicketState {
    Open,
    InProgress,
    Review,
    Done,
    Blocked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub state: TicketState,
    pub priority: Priority,
    pub assignee: Option<String>,
    pub reporter: String,
    pub history: Vec<TicketState>,
    pub comments: Vec<String>,
    pub tick: u64,
}

impl Ticket {
    pub fn new(id: impl Into<String>, reporter: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: String::new(),
            state: TicketState::Open,
            priority: Priority::Medium,
            assignee: None,
            reporter: reporter.into(),
            history: Vec::new(),
            comments: Vec::new(),
            tick: 0,
        }
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn as_byte(&self) -> u8 {
        self.id.bytes().fold(0u8, |a, b| a.wrapping_add(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_ticket_open() {
        let t = Ticket::new("PROJ-1", "x@y.z");
        assert_eq!(t.state, TicketState::Open);
        assert_eq!(t.priority, Priority::Medium);
    }
    #[test]
    fn with_title_sets_title() {
        let t = Ticket::new("PROJ-1", "x@y.z").with_title("hello");
        assert_eq!(t.title, "hello");
    }
    #[test]
    fn as_byte_stable() {
        let a = Ticket::new("PROJ-1", "x").as_byte();
        let b = Ticket::new("PROJ-1", "x").as_byte();
        assert_eq!(a, b);
    }
}
