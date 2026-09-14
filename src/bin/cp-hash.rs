use tmig::controlplane::ControlPlane;
use tmig::ledger::hex;
fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let plane = ControlPlane::derive(
        &std::fs::read_to_string("docs/DOCTRINE.md").unwrap_or_default(),
        &std::fs::read_to_string("catalog/local-models.json").unwrap_or_default(),
        &std::fs::read_to_string("src/skills.rs").unwrap_or_default(),
    );
    println!("{}", hex(&plane.hash()));
    eprintln!("  doctrine_hash: {}", hex(&plane.doctrine_hash));
    eprintln!("  catalog_hash:  {}", hex(&plane.catalog_hash));
    eprintln!("  skills_hash:   {}", hex(&plane.skills_hash));
    eprintln!("  thesis:        content_addressed={} human_authorized={} reversible={} consequence_classed={} audited={}",
        plane.thesis.content_addressed, plane.thesis.human_authorized,
        plane.thesis.reversible, plane.thesis.consequence_classed, plane.thesis.audited);
}
