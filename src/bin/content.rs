use tmig::content::{derive_all, Federation, Render};

fn main() {
    let idx = derive_all("tmig", b"seed-content");
    println!("content_kinds\t{}", idx.kinds());
    println!("content_entries\t{}", idx.len());
    println!("content_bytes\t{}", idx.total_bytes);
    let fed = Federation::omega();
    println!("federation_sources\t{}", fed.len());
    std::fs::create_dir_all("data/content").ok();
    std::fs::write("data/content/index.tsv", Render::tsv(&idx)).ok();
    std::fs::write("data/content/index.json", Render::json(&idx)).ok();
    println!("wrote data/content/index.tsv and index.json");
}
