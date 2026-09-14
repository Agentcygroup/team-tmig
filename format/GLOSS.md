# GLOSS — a refereeable file format for discipline glossaries

GLOSS is line-oriented, grammar-checked, and content-addressed. It carries three
record kinds: `term`, `non_claim`, `referee`. Four non-claims are mandatory in
every file; a GLOSS file that omits any of them is rejected by the parser.

GLOSS does not confer mastery, accreditation, coherence, or completeness.
Those are the mandatory non-claims.

    term "<id>" "<name>" <discipline> <coverage> "<note>"
    non_claim "<id>" "<statement>" "<rationale>"
    referee "<term-id>" "<institution>" "<standard>" <verdict> "<reason>"

coverage ∈ { provided, partial, absent, out-of-scope }
verdict  ∈ { accept, reject, abstain }
