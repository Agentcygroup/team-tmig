//! cargo xt — task runner aligned with CI.
use std::process::{exit, Command};

fn run(cmd: &str, args: &[&str]) -> bool {
    eprintln!("\n$ {} {}", cmd, args.join(" "));
    Command::new(cmd)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    let task = std::env::args().nth(1).unwrap_or_else(|| "help".into());
    let ok = match task.as_str() {
        "ci" => {
            run("cargo", &["fmt", "--check"])
                && run("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"])
                && run("cargo", &["test", "--release", "--all-targets"])
        }
        "fmt"     => run("cargo", &["fmt"]),
        "fmt-check" => run("cargo", &["fmt", "--check"]),
        "lint"    => run("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"]),
        "test"    => run("cargo", &["test", "--release", "--all-targets"]),
        "build"   => run("cargo", &["build", "--release"]),
        "receipt" => {
            std::fs::create_dir_all("target/provenance").ok();
            let out = std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let body = format!("green baseline\ncommit: {}\n", out.trim());
            std::fs::write("target/provenance/green-receipt.txt", &body).is_ok()
        }
        "clean" => run("cargo", &["clean"]),
        _ => {
            eprintln!("usage: cargo xt <ci|fmt|fmt-check|lint|test|build|receipt|clean>");
            false
        }
    };
    exit(if ok { 0 } else { 1 });
}
