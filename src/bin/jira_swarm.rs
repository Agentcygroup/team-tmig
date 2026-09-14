use clap::Parser;
use serde::{Deserialize, Serialize};

const TICKETS_PER_SWARM: usize = 333;
const DUNBAR: usize = 150;
const TICKETS_PER_FLOCK: usize = TICKETS_PER_SWARM * DUNBAR;
const FLOCK_BYTES: usize = TICKETS_PER_FLOCK;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "1000")]
    tickets: usize,
    #[arg(long, default_value = "10")]
    ticks: u64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum TicketState {
    Open,
    InProgress,
    Review,
    Done,
    Blocked,
}

impl TicketState {
    fn idx(self) -> usize {
        match self {
            TicketState::Open => 0,
            TicketState::InProgress => 1,
            TicketState::Review => 2,
            TicketState::Done => 3,
            TicketState::Blocked => 4,
        }
    }
}

#[derive(Clone, Copy)]
struct TicketAgent {
    state: TicketState,
    energy: u8,
}

impl TicketAgent {
    fn new(i: usize) -> Self {
        let state = match i % 5 {
            0 => TicketState::Open,
            1 => TicketState::InProgress,
            2 => TicketState::Review,
            3 => TicketState::Done,
            _ => TicketState::Blocked,
        };
        Self {
            state,
            energy: ((i * 37) & 0xff) as u8,
        }
    }

    fn step(&mut self, neighbors: &[TicketAgent; 8]) {
        let done_neighbors = neighbors
            .iter()
            .filter(|n| matches!(n.state, TicketState::Done))
            .count();
        let blocked_neighbors = neighbors
            .iter()
            .filter(|n| matches!(n.state, TicketState::Blocked))
            .count();
        self.energy = self
            .energy
            .wrapping_add((done_neighbors as u8).saturating_add(1));
        self.state = match self.state {
            TicketState::Open if self.energy % 3 == 0 => TicketState::InProgress,
            TicketState::InProgress if blocked_neighbors > 2 => TicketState::Blocked,
            TicketState::InProgress if self.energy % 4 == 0 => TicketState::Review,
            TicketState::Review if done_neighbors >= 2 || self.energy % 5 == 0 => TicketState::Done,
            TicketState::Blocked if blocked_neighbors <= 2 && self.energy % 7 == 0 => {
                TicketState::Open
            }
            s => s,
        };
    }
}

fn coherence(tickets: &[TicketAgent]) -> f32 {
    if tickets.is_empty() {
        return 0.0;
    }
    let done = tickets
        .iter()
        .filter(|t| matches!(t.state, TicketState::Done))
        .count() as f32;
    let blocked = tickets
        .iter()
        .filter(|t| matches!(t.state, TicketState::Blocked))
        .count() as f32;
    ((done + 1.0) / (tickets.len() as f32 + blocked + 1.0)).clamp(0.0, 1.0)
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let flocks = cli.tickets.div_ceil(TICKETS_PER_FLOCK).max(1);
    let ir_bytes = flocks * FLOCK_BYTES;
    let mut tickets: Vec<_> = (0..cli.tickets).map(TicketAgent::new).collect();

    println!("════════════════════════════════════════════════════════════════");
    println!("                    JIRA SWARM");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("jira-swarm");
    println!("  tickets:   {}", cli.tickets);
    println!("  flocks:    {}", flocks);
    println!();
    println!("  law:");
    println!("    333 tickets per swarm");
    println!("    333 × Dunbar 150 = 49,950 tickets per flock");
    println!("    flocks bounded by IR");
    println!("    IR: {} bytes", ir_bytes);
    println!();

    let t0 = std::time::Instant::now();
    for _ in 0..cli.ticks {
        let prev = tickets.clone();
        for i in 0..tickets.len() {
            let n = [
                prev[(i + tickets.len() - 1) % tickets.len()],
                prev[(i + 1) % tickets.len()],
                prev[(i + 7) % tickets.len()],
                prev[(i + 47) % tickets.len()],
                prev[(i + 113) % tickets.len()],
                prev[(i + 251) % tickets.len()],
                prev[(i + 331) % tickets.len()],
                prev[(i + 997) % tickets.len()],
            ];
            tickets[i].step(&n);
        }
    }
    let elapsed = t0.elapsed();
    let mut counts = [0usize; 5];
    for t in &tickets {
        counts[t.state.idx()] += 1;
    }

    println!("after {} ticks", cli.ticks);
    println!("  elapsed:         {:?}", elapsed);
    println!("  mean coherence:  {:.4}", coherence(&tickets));
    println!();
    println!("state distribution:");
    println!("  Open:        {}", counts[0]);
    println!("  In Progress: {}", counts[1]);
    println!("  Review:      {}", counts[2]);
    println!("  Done:        {}", counts[3]);
    println!("  Blocked:     {}", counts[4]);
}
