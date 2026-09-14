use clap::Parser;
use tmig::dunbar::Murmuration;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "10")]
    mb: usize,
    #[arg(long, default_value = "10")]
    ticks: u64,
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let mut m = Murmuration::new(cli.mb * 1024 * 1024);
    while m.spawn_flock().is_some() {}
    let s0 = m.stats();
    println!("murmuration");
    println!(
        "  IR:      {} / {} bytes ({} MB)",
        s0.ir_used, s0.ir_total, cli.mb
    );
    println!("  flocks:  {}", s0.flocks);
    println!("  swarms:  {}", s0.swarms);
    println!("  agents:  {}", s0.agents);
    println!();
    println!("  law: 1 / 0.0030 × 150 × IR");
    println!();
    let t0 = std::time::Instant::now();
    for _ in 0..cli.ticks {
        m.step();
    }
    let e = t0.elapsed();
    let s = m.stats();
    println!("after {} ticks", cli.ticks);
    println!("  elapsed:        {:?}", e);
    println!("  mean coherence: {:.4}", s.mean_coherence);
    println!("  per-tick:       {:?}", e / cli.ticks as u32);
}
