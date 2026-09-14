use clap::Parser;
use std::path::Path;
use tmig::filelayer::{FileKind, FileSwarm, Layer, LiveFileAgent};
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    scan: String,
    #[arg(long, default_value = "10")]
    ticks: u64,
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let cli = Cli::parse();
    let mut swarm = FileSwarm::new();
    let root = Path::new(&cli.scan);
    let mut count = 0;
    for e in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        let path = e.path().to_string_lossy().to_string();
        let ext = e.path().extension().and_then(|s| s.to_str()).unwrap_or("");
        let kind = FileKind::from_ext(ext);
        let size = e.metadata().map(|m| m.len()).unwrap_or(0);
        swarm.add(LiveFileAgent::new(&path, Layer::Content, kind, size));
        count += 1;
    }
    let (n0, _) = swarm.stats();
    println!("filelayer");
    println!("  scanned: {}", cli.scan);
    println!("  files:   {}", n0);
    println!();
    println!("  15 layers · ~100 unit types · 2^100 cross-cuts");
    println!();
    let t0 = std::time::Instant::now();
    for _ in 0..cli.ticks {
        swarm.step();
    }
    let e = t0.elapsed();
    let (n, c) = swarm.stats();
    println!("after {} ticks", cli.ticks);
    println!("  elapsed:        {:?}", e);
    println!("  agents:         {}", n);
    println!("  mean coherence: {:.4}", c);
    println!("  per-tick:       {:?}", e / cli.ticks as u32);
    println!();
    println!("(scanned {} files, mapped to Content layer)", count);
}
