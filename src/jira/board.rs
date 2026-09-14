use crate::jira::ticket::{Ticket, TicketState};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Board {
    pub tickets: BTreeMap<String, Ticket>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, t: Ticket) {
        self.tickets.insert(t.id.clone(), t);
    }
    pub fn insert(&mut self, t: Ticket) {
        self.add(t);
    }
    pub fn get(&self, id: &str) -> Option<&Ticket> {
        self.tickets.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Ticket> {
        self.tickets.get_mut(id)
    }
    pub fn len(&self) -> usize {
        self.tickets.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tickets.is_empty()
    }
    pub fn counts(&self) -> [usize; 5] {
        let mut c = [0usize; 5];
        for t in self.tickets.values() {
            c[match t.state {
                TicketState::Open => 0,
                TicketState::InProgress => 1,
                TicketState::Review => 2,
                TicketState::Done => 3,
                TicketState::Blocked => 4,
            }] += 1;
        }
        c
    }
}
