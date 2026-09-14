use clap::Parser;
use tmig::ledger::{hash_bytes, hex};
use tmig::model_boundary::{
    strict_developer_survivors, MODEL_P, MODEL_S, NGC_CATALOG_SIZE_ESTIMATE,
};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    json: bool,
}

fn main() -> anyhow::Result<()> {
    let survivors = strict_developer_survivors();
    let manifest = serde_json::json!({
        "model_p": MODEL_P,
        "model_s": MODEL_S,
        "source_catalog_estimate": NGC_CATALOG_SIZE_ESTIMATE,
        "survivor_count": survivors.len(),
        "survivors": survivors,
    });
    let bytes = serde_json::to_vec_pretty(&manifest)?;
    let hash = hex(&hash_bytes(&bytes));

    if Cli::parse().json {
        println!("{}", String::from_utf8(bytes)?);
    } else {
        println!("tmig model boundary");
        println!("  source catalog estimate: {}", NGC_CATALOG_SIZE_ESTIMATE);
        println!("  strict survivors:        {}", manifest["survivor_count"]);
        println!("  manifest hash:           {}", hash);
        println!("  primary:                 mlx-community/Qwen2.5-Coder-7B-Instruct-4bit");
        println!("  local provider:          http://localhost:11434/v1 (ollama)");
    }
    Ok(())
}
