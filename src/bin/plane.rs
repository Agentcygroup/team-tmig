use clap::Parser;
use std::fs;
use tmig::plane::Plane;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "1000")]
    terminals: usize,
    #[arg(long, default_value = "/tmp/alphabet.txt")]
    alphabet_file: String,
    #[arg(long, default_value = "100")]
    ticks: u64,
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let contents = fs::read_to_string(&cli.alphabet_file).unwrap_or_default();
    let alphabet: Vec<String> = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(String::from)
        .collect();
    println!("plane");
    println!("  terminals: {}", cli.terminals);
    println!("  alphabet:  {} commands", alphabet.len());
    println!("  ticks:     {}", cli.ticks);
    println!();
    println!("  every command is an agent");
    println!("  every terminal is a plane");
    println!("  every plane is a search");
    println!();
    let mut p = Plane::new_full(cli.terminals, alphabet, 0xDEADBEEF);
    let t0 = std::time::Instant::now();
    p.run(cli.ticks);
    let e = t0.elapsed();
    let s = p.stats();
    println!("after {} ticks", cli.ticks);
    println!("  elapsed:        {:?}", e);
    println!("  total commands: {}", s.total_commands);
    println!("  per-tick:       {:?}", e / cli.ticks as u32);
    println!(
        "  throughput:     {} commands/sec",
        s.total_commands as u128 * 1000 / e.as_millis().max(1)
    );
}
