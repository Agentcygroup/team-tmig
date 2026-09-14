#[test]
fn c1_builds() {
    let out = std::process::Command::new("cargo")
        .args(["build", "--release", "--quiet"])
        .output()
        .expect("cargo");
    assert!(out.status.success());
}

#[test]
fn c3_modules_declared() {
    let n = std::fs::read_dir("src")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .count();
    assert!(n >= 18);
}

#[test]
fn c4_no_unsafe_outside_ffi() {
    let out = std::process::Command::new("sh")
        .args([
            "-c",
            "grep -rn 'unsafe' src --include='*.rs' | grep -v src/ffi || true",
        ])
        .output()
        .expect("grep");
    assert!(out.stdout.is_empty());
}
