use crate::jira::board::Board;
use crate::jira::ticket::TicketState;

pub const DUNBAR: usize = 150;
pub const TICKETS_PER_SWARM: usize = 333;
pub const TICKETS_PER_FLOCK: usize = TICKETS_PER_SWARM * DUNBAR;

pub struct JiraSwarm {
    pub board: Board,
    pub ticks: u64,
}

impl JiraSwarm {
    pub fn new(board: Board) -> Self {
        Self { board, ticks: 0 }
    }
    pub fn with_capacity(tickets: usize) -> Self {
        let mut board = Board::new();
        for i in 0..tickets {
            let id = format!("PROJ-{}", i);
            let mut t = crate::jira::ticket::Ticket::new(id, "reporter@tmig.io");
            t.title = format!("ticket {}", i);
            board.add(t);
        }
        Self { board, ticks: 0 }
    }
    pub fn step(&mut self) {
        self.tick();
    }
    pub fn tick(&mut self) {
        self.ticks += 1;
        for t in self.board.tickets.values_mut() {
            if t.state == TicketState::Open && (t.as_byte() as u64 + self.ticks).is_multiple_of(3) {
                t.state = TicketState::InProgress;
                t.history.push(TicketState::InProgress);
            }
        }
    }
    pub fn stats(&self) -> (usize, usize, f32) {
        let n = self.board.len();
        let flocks = n.div_ceil(TICKETS_PER_FLOCK);
        let c = self.board.counts();
        (n, flocks.max(1), c[3] as f32 / (n.max(1) as f32))
    }
}
