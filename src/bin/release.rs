use tmig::release::{
    verify_release, Artifact, ArtifactKind, Attestation, Distributor, KeyPair, Provenance,
    ReleaseIndex, Sbom, Signature, Target,
};

fn main() {
    let kp = KeyPair::from_bytes(b"tmig-master-key");

    let payloads: &[(&str, ArtifactKind, &[u8])] = &[
        ("tmig", ArtifactKind::Binary, b"binary-payload-1"),
        ("libtmig.rlib", ArtifactKind::Library, b"lib-payload"),
        ("tmig-cli", ArtifactKind::Binary, b"cli-payload"),
        ("tmig-doc", ArtifactKind::Doc, b"doc-payload"),
        ("schema.json", ArtifactKind::Schema, b"schema-payload"),
    ];

    let mut idx = ReleaseIndex::new("0.1.0");
    let mut pairs: Vec<(Artifact, Signature)> = Vec::new();

    for (name, kind, bytes) in payloads {
        let art = Artifact::new(name, *kind, bytes);
        let sig = Signature::sign(&kp, bytes);
        idx.add(art.clone());
        pairs.push((art, sig));
    }

    pairs.sort_by(|a, b| a.0.name.cmp(&b.0.name));

    let arts: Vec<Artifact> = pairs.iter().map(|(a, _)| a.clone()).collect();
    let sigs: Vec<Signature> = pairs.iter().map(|(_, s)| s.clone()).collect();
    let (pass, fail) = verify_release(&arts, &sigs);

    let p = Provenance::new(
        b"binary-payload-1",
        b"src-tree",
        b"Cargo.toml",
        "rustc-1.98.0",
        "2026-09-14T00:00:00Z",
    );
    let s = Sbom::cyclonedx(vec![
        "sha2".into(),
        "serde".into(),
        "chrono".into(),
        "thiserror".into(),
    ]);
    let sig = Signature::sign(&kp, b"binary-payload-1");
    let att = Attestation::build(p, s, sig);

    let mut dist = Distributor::new();
    for t in [
        Target::GitHubReleases,
        Target::CratesIo,
        Target::DocsRs,
        Target::Homebrew,
        Target::Docker,
        Target::Nix,
    ] {
        dist.add(t);
    }

    println!("release_index\t{}", idx.len());
    println!("release_bytes\t{}", idx.total_size());
    println!("signatures\t{}_pass_{}_fail", pass, fail);
    println!("multi_target\t{}", dist.is_multi_target());
    println!("sbom_format\tCycloneDX-1.5");
    println!("slsa_level\t{}", 3);
    println!("attestation\t{}", att.to_json());
    println!("index_json\t{}", idx.to_json());
    println!("key_pub\t{}", kp.public_hex());
}
