use tmig::corpus::Engine;

fn main() -> std::io::Result<()> {
    let engine = Engine::new("corpus/provenance/manifest.csv", "corpus/spectral")?;
    println!("manifest_sources\t{}", engine.manifest.len());
    println!("manifest_classes\t{}", engine.manifest.classes());
    println!(
        "manifest_population\t{}",
        engine.manifest.total_population()
    );
    let mut pass = 0;
    let mut fail = 0;
    for (name, ok) in engine.run_self_tests() {
        println!("test\t{}\t{}", name, if ok { "PASS" } else { "FAIL" });
        if ok {
            pass += 1
        } else {
            fail += 1
        }
    }
    let n = engine.run_all_crosscuts()?;
    println!("crosscuts_written\t{}", n);
    println!("self_tests\t{}_pass_{}_fail", pass, fail);
    Ok(())
}
