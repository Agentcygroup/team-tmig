//! gloss — parser, schema registry, referee slots for the GLOSS file format.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Coverage {
    Provided,
    Partial,
    Absent,
    OutOfScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Accept,
    Reject,
    Abstain,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub id: String,
    pub name: String,
    pub discipline: String,
    pub coverage: Coverage,
    pub note: String,
}
#[derive(Debug, Clone)]
pub struct NonClaim {
    pub id: String,
    pub statement: String,
    pub rationale: String,
}
#[derive(Debug, Clone)]
pub struct Referee {
    pub term_id: String,
    pub institution: String,
    pub standard: String,
    pub verdict: Verdict,
    pub reason: String,
}
#[derive(Debug, Default, Clone)]
pub struct GlossFile {
    pub terms: Vec<Term>,
    pub non_claims: Vec<NonClaim>,
    pub referees: Vec<Referee>,
}
#[derive(Debug)]
pub enum GlossError {
    Parse(String),
    MissingMandatory(Vec<String>),
    Unknown(String),
}

const MANDATORY: &[&str] = &[
    "no-mastery",
    "no-accreditation",
    "no-coherence-proof",
    "no-completeness",
];

fn unquote(s: &str) -> String {
    let t = s.trim();
    let t = t.strip_prefix('"').unwrap_or(t);
    let t = t.strip_suffix('"').unwrap_or(t);
    t.replace("\\\"", "\"").replace("\\\\", "\\")
}
fn cov(s: &str) -> Result<Coverage, GlossError> {
    Ok(match s.trim() {
        "provided" => Coverage::Provided,
        "partial" => Coverage::Partial,
        "absent" => Coverage::Absent,
        "out-of-scope" => Coverage::OutOfScope,
        o => return Err(GlossError::Unknown(format!("coverage: {o}"))),
    })
}
fn vd(s: &str) -> Result<Verdict, GlossError> {
    Ok(match s.trim() {
        "accept" => Verdict::Accept,
        "reject" => Verdict::Reject,
        "abstain" => Verdict::Abstain,
        o => return Err(GlossError::Unknown(format!("verdict: {o}"))),
    })
}

/// Hand-rolled scanner. The pest grammar in `format/gloss.pest` is the
/// normative description; this scanner implements it without a proc-macro
/// dependency, so the crate stays offline-buildable.
pub fn parse(src: &str) -> Result<GlossFile, GlossError> {
    let mut out = GlossFile::default();
    for (n, raw) in src.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let toks: Vec<&str> = split_tokens(line).map_err(GlossError::Parse)?;
        match toks.first().copied() {
            Some("term") if toks.len() == 6 => {
                out.terms.push(Term {
                    id: unquote(toks[1]),
                    name: unquote(toks[2]),
                    discipline: toks[3].to_string(),
                    coverage: cov(toks[4])?,
                    note: unquote(toks[5]),
                });
            }
            Some("non_claim") if toks.len() == 4 => {
                out.non_claims.push(NonClaim {
                    id: unquote(toks[1]),
                    statement: unquote(toks[2]),
                    rationale: unquote(toks[3]),
                });
            }
            Some("referee") if toks.len() == 6 => {
                out.referees.push(Referee {
                    term_id: unquote(toks[1]),
                    institution: unquote(toks[2]),
                    standard: unquote(toks[3]),
                    verdict: vd(toks[4])?,
                    reason: unquote(toks[5]),
                });
            }
            Some(k) => {
                return Err(GlossError::Parse(format!(
                    "line {}: bad record `{k}`",
                    n + 1
                )));
            }
            None => {}
        }
    }
    let present: std::collections::BTreeSet<&str> =
        out.non_claims.iter().map(|n| n.id.as_str()).collect();
    let missing: Vec<String> = MANDATORY
        .iter()
        .filter(|m| !present.contains(*m))
        .map(|m| (*m).to_string())
        .collect();
    if !missing.is_empty() {
        return Err(GlossError::MissingMandatory(missing));
    }
    Ok(out)
}

fn split_tokens(line: &str) -> Result<Vec<&str>, String> {
    let mut out = Vec::new();
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() {
        while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        if b[i] == b'"' {
            let start = i;
            i += 1;
            while i < b.len() {
                if b[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if b[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push(&line[start..i]);
        } else {
            let start = i;
            while i < b.len() && b[i] != b' ' && b[i] != b'\t' {
                i += 1;
            }
            out.push(&line[start..i]);
        }
    }
    Ok(out)
}

pub fn digest(g: &GlossFile) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    for t in &g.terms {
        let _ = writeln!(s, "term {} {} {:?}", t.id, t.discipline, t.coverage);
    }
    for n in &g.non_claims {
        let _ = writeln!(s, "non_claim {} {}", n.id, n.statement);
    }
    for r in &g.referees {
        let _ = writeln!(s, "referee {} {}", r.term_id, r.institution);
    }
    crate::compose::Id::of(s.as_bytes()).hex()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MIN: &str = r#"
term "a-star" "A* search" search absent "compose is a substrate"
non_claim "no-mastery" "GLOSS does not confer mastery over any discipline." "Mastery is a property of a mind or system producing novel correct results, not of a file format."
non_claim "no-accreditation" "GLOSS does not confer institutional accreditation." "Accreditation is a human act by a named body under a named standard for a named scope."
non_claim "no-coherence-proof" "GLOSS does not prove that its source glossary is internally coherent." "Grammar parses; it does not adjudicate semantic contradiction."
non_claim "no-completeness" "GLOSS does not claim to enumerate every term of any field." "The set of terms is open and moves; a file is a snapshot."
"#;
    #[test]
    fn parses_minimal() {
        let g = parse(MIN).unwrap();
        assert_eq!(g.terms.len(), 1);
    }
    #[test]
    fn refuses_missing_mandatory() {
        assert!(matches!(
            parse("term \"x\" \"X\" d absent \"n\""),
            Err(GlossError::MissingMandatory(_))
        ));
    }
    #[test]
    fn refuses_bad_coverage() {
        assert!(parse(&MIN.replace("absent", "nope")).is_err());
    }
    #[test]
    fn referee_slots() {
        let s = format!(
            "{MIN}\nreferee \"a-star\" \"Example University\" \"CS-2024\" accept \"matches field\""
        );
        assert_eq!(parse(&s).unwrap().referees.len(), 1);
    }
    #[test]
    fn digest_stable() {
        assert_eq!(digest(&parse(MIN).unwrap()), digest(&parse(MIN).unwrap()));
    }
}
