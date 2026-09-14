# Referee protocol for GLOSS

A referee adjudicates one term at a time, at a named revision.

1. Parse: `cargo test --release --lib gloss::`
2. Read a term: `grep '^term ' declarations/ai-glossary.gloss`
3. Check coverage tag against the artifact at the named revision.
4. Append a referee record:
   `referee "<term-id>" "<institution>" "<standard>" <accept|reject|abstain> "<reason>"`
5. Re-run the check. Digest changes; the artifact revises.

Verdicts mean:
- accept  — the coverage tag matches the artifact at the named revision.
- reject  — the tag is wrong; the next revision must change it.
- abstain — the referee's standard does not cover this term.

Verdicts do not confer mastery, accreditation, coherence, or completeness.
