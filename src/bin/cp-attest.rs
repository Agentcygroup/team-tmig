use std::io::Read;
use tmig::controlplane::{ActionProof, Consequence, ControlPlane};
use tmig::ledger::hex;

#[derive(serde::Deserialize)]
struct InputProof {
    content_hash: String,
    target: String,
    consequence: Option<String>,
    #[serde(default)]
    approvals: Vec<String>,
    #[serde(default = "t")]
    reversible: bool,
    attestation_id: String,
}
fn t() -> bool {
    true
}

fn hx(s: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    let b = s.as_bytes();
    for i in 0..32 {
        if i * 2 + 1 >= b.len() {
            break;
        }
        let hi = (b[i * 2] as char).to_digit(16).unwrap_or(0) as u8;
        let lo = (b[i * 2 + 1] as char).to_digit(16).unwrap_or(0) as u8;
        out[i] = (hi << 4) | lo;
    }
    out
}

fn cons(s: &Option<String>) -> Option<Consequence> {
    s.as_ref().map(|v| match v.as_str() {
        "None" => Consequence::None,
        "Low" => Consequence::Low,
        "Moderate" => Consequence::Moderate,
        "High" => Consequence::High,
        "Legal" => Consequence::Legal,
        "Irreversible" => Consequence::Irreversible,
        _ => Consequence::None,
    })
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).expect("stdin");
    // bounded
    let input: InputProof = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{{\"error\":\"bad json\",\"detail\":\"{}\"}}", e);
            std::process::exit(2);
        }
    };
    let proof = ActionProof {
        content_hash: hx(&input.content_hash),
        target: input.target,
        consequence: cons(&input.consequence),
        approvals: input.approvals,
        reversible: input.reversible,
        attestation_id: hx(&input.attestation_id),
        primitives: Vec::new(),
        required_agent: None,
        thesis_ok: true,
        scope_ok: true,
    };
    let plane = ControlPlane::derive(
        &std::fs::read_to_string("docs/DOCTRINE.md").unwrap_or_default(),
        &std::fs::read_to_string("catalog/local-models.json").unwrap_or_default(),
        &std::fs::read_to_string("src/skills.rs").unwrap_or_default(),
    );
    match plane.handle(&proof) {
        Ok(att) => println!(
            "{}",
            serde_json::json!({
                "ok": true, "action_hash": hex(&att.action_hash),
                "thesis_ok": att.thesis_ok, "scope_ok": att.scope_ok,
                "polarity": format!("{:?}", att.polarity),
                "layer": format!("{:?}", att.layer),
                "consequence": format!("{:?}", att.consequence),
                "doctrine_hash": hex(&att.doctrine_hash),
            })
        ),
        Err(e) => {
            println!(
                "{}",
                serde_json::json!({"ok": false, "error": format!("{:?}", e)})
            );
            std::process::exit(1);
        }
    }
}
