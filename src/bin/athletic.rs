use clap::Parser;
use tmig::swarm::{AthleticAgent, Constraints, Mode};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "adaptive")]
    mode: String,
    #[arg(long, default_value = "1000")]
    agents: usize,
    #[arg(long, default_value = "10")]
    duration: u64,
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let mode = match cli.mode.as_str() {
        "reactive" => Mode::Reactive,
        "planned" => Mode::Planned,
        "coordinated" => Mode::Coordinated,
        "adaptive" => Mode::Adaptive,
        "propagate" => Mode::Propagate,
        "swarm" => Mode::Swarm,
        _ => Mode::Adaptive,
    };
    let c = Constraints::for_mode_str(cli.mode.as_str());
    let mut agents: Vec<_> = (0..cli.agents)
        .map(|i| AthleticAgent::new(i as u64, mode))
        .collect();
    let t0 = std::time::Instant::now();
    for _ in 0..cli.duration {
        for a in &mut agents {
            let mut n = [0u8; 8];
            for (i, slot) in n.iter_mut().enumerate() {
                *slot = ((a.id + i as u64) & 0xFF) as u8;
            }
            a.cycle(&n, &c);
        }
    }
    let faults: u32 = agents.iter().map(|a| a.faults).sum();
    let tick_mean = agents.iter().map(|a| a.tick).sum::<u64>() / agents.len().max(1) as u64;
    println!("athletic-swarm");
    println!("  mode:     {:?}", mode);
    println!("  agents:   {}", cli.agents);
    println!("  ticks:    {} per agent", tick_mean);
    println!("  elapsed:  {:?}", t0.elapsed());
    println!("  faults:   {}", faults);
}
