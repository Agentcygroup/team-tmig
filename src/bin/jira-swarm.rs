use clap::Parser;
use tmig::jira::{Board, JiraSwarm, Priority, Ticket, TicketState};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "1000")]
    tickets: usize,
    #[arg(long, default_value = "10")]
    ticks: u64,
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let mut board = Board::new();
    for i in 0..cli.tickets {
        let mut t = Ticket::new(format!("PROJ-{}", i), "reporter@tmig.io");
        t.priority = match i % 4 {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            _ => Priority::Critical,
        };
        board.add(t);
    }
    let mut swarm = JiraSwarm::new(board);
    let (n0, f0, _) = swarm.stats();
    println!("jira-swarm");
    println!("  tickets: {}", n0);
    println!("  flocks:  {}", f0);
    println!();
    println!("  law: 333 tickets/swarm × Dunbar 150 = 49,950 tickets/flock");
    println!();
    let t0 = std::time::Instant::now();
    for _ in 0..cli.ticks {
        swarm.step();
    }
    let e = t0.elapsed();
    let (n, f, c) = swarm.stats();
    println!("after {} ticks", cli.ticks);
    println!("  elapsed:        {:?}", e);
    println!("  tickets:        {}", n);
    println!("  flocks:         {}", f);
    println!("  mean coherence: {:.4}", c);
    println!("  per-tick:       {:?}", e / cli.ticks as u32);
    let mut counts = [0usize; 5];
    for t in swarm.board.tickets.values() {
        match t.state {
            TicketState::Open => counts[0] += 1,
            TicketState::InProgress => counts[1] += 1,
            TicketState::Review => counts[2] += 1,
            TicketState::Done => counts[3] += 1,
            TicketState::Blocked => counts[4] += 1,
        }
    }
    println!();
    println!("state distribution:");
    println!("  Open:        {}", counts[0]);
    println!("  In Progress: {}", counts[1]);
    println!("  Review:      {}", counts[2]);
    println!("  Done:        {}", counts[3]);
    println!("  Blocked:     {}", counts[4]);
}
