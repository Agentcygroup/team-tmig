# Substrate

Every claim in this document cites a source. A claim without a source is
a claim without support. Every clause in the crate's declared set is
backed by one of the entries below.

The document has four sections: primitives, algebra, verification, and
attribution.

## 1. Primitives

Each primitive is one irreducible operation. Each has a fixed citation.

P1  SHA-256
    Source: NIST FIPS 180-4, §6.2
    Statement: SHA-256 maps an arbitrary-length byte string to a 256-bit
    digest. The digest is deterministic and collision-resistant under
    the standard assumptions.
    Crate location: Cargo.toml (sha2), src/level3_algorithms/hash.rs

P2  Ed25519
    Source: NIST FIPS 186-5, §7.1; RFC 8032
    Statement: Ed25519 is a deterministic signature scheme over Curve25519.
    Signatures are 64 bytes, public keys 32 bytes.
    Crate location: Cargo.toml (ed25519-dalek)

P3  RFC 3339 timestamps
    Source: IETF RFC 3339, §5.6
    Statement: A timestamp is a string of the form YYYY-MM-DDTHH:MM:SSZ.
    The Z suffix means UTC.
    Crate location: chrono crate, used in .audit/log/writes.jsonl

P4  RFC 8259 JSON
    Source: IETF RFC 8259, §2
    Statement: JSON is a text encoding of structured data. Values are
    null, boolean, number, string, array, or object.
    Crate location: serde_json crate, used in every log write

P5  IEEE 754 floats
    Source: IEEE 754-2019
    Statement: f32 and f64 are IEEE 754 binary32 and binary64 values.
    Arithmetic on them follows the standard's rules for rounding,
    subnormals, infinities, and NaN.
    Crate location: Rust primitive types, used in classifier functions

P6  SI units
    Source: BIPM SI Brochure, 9th edition (2019)
    Statement: The seven base units are second, metre, kilogram, ampere,
    kelvin, mole, candela. All derived units are products of these.
    Crate location: src/level1_physics/constants.rs

P7  CODATA 2022 constants
    Source: CODATA 2022 recommended values, NIST
    Statement: c = 299792458 m/s. h = 6.62607015e-34 J·s. k_B = 1.380649e-23
    J/K. e = 1.602176634e-19 C. N_A = 6.02214076e23 mol^-1.
    Crate location: src/level1_physics/constants.rs

## 2. Algebra

Each algebraic relation is one provable statement about the primitives.

A1  Composition
    Statement: If f: A → B and g: B → C, then g ∘ f: A → C.
    Citation: category theory, standard definition of morphism composition.

A2  Identity
    Statement: For every type T, there is a function id_T: T → T
    such that id_T ∘ f = f and g ∘ id_T = g.
    Citation: category theory, standard definition of identity morphism.

A3  Associativity of composition
    Statement: (h ∘ g) ∘ f = h ∘ (g ∘ f).
    Citation: category theory, associativity axiom.

A4  Result as coproduct
    Statement: Result<T, E> is the coproduct T + E. A value is either
    Ok(t) or Err(e), never both.
    Citation: Rust Reference, enum semantics.

A5  Option as pointed type
    Statement: Option<T> is T + {()}.
    Citation: Rust Reference, enum semantics.

A6  BTreeMap as ordered finite function
    Statement: BTreeMap<K, V> is a partial function from K to V with a
    total order on K. get(k) returns Some(v) iff k is in the domain.
    Citation: Rust std documentation, BTreeMap.

A7  Hash preimage resistance
    Statement: For a cryptographic hash H with output length n, finding
    a preimage for a given digest requires 2^n operations on average.
    Citation: Rogaway & Shrimpton, "Cryptographic Hash-Function Basics"
    (2004), definitions of preimage resistance.

A8  Hash collision resistance
    Statement: For a cryptographic hash H with output length n, finding
    two distinct inputs with the same digest requires 2^(n/2) operations
    on average (birthday bound).
    Citation: Bellare & Rogaway, "Random Oracles are Practical" (1993).

A9  Laplace smoothing
    Statement: Given k successes in n trials, the smoothed estimate of
    the success probability is (k+1)/(n+2). This avoids a zero estimate
    when k = 0.
    Citation: Laplace, "Théorie Analytique des Probabilités" (1812);
    Manning, Raghavan, Schütze, "Introduction to Information Retrieval"
    (2008), §13.5.

A10 Weighted majority
    Statement: Given weights w_i and answers a_i, the aggregate is the
    a that maximises sum of w_i over i where a_i = a. If no a receives
    at least half the total weight, the aggregate is undefined.
    Citation: Arrow, "Social Choice and Individual Values" (1951),
    theorem 1; standard definition of weighted plurality.

A11 Pareto dominance
    Statement: A target t is dominated by u if u is at least as good as t
    on every axis and strictly better on at least one. The Pareto set is
    the set of undominated targets.
    Citation: Pareto, "Manuale di Economia Politica" (1906).

A12 Jaccard similarity
    Statement: The similarity of two sets A and B is |A ∩ B| / |A ∪ B|.
    Citation: Jaccard, "Étude comparative de la distribution florale"
    (1901).

A13 Landauer's limit
    Statement: Erasing one bit of information requires at least k_B T ln 2
    joules of energy.
    Citation: Landauer, "Irreversibility and Heat Generation in the
    Computing Process", IBM Journal (1961).

A14 Grover's bound
    Statement: A quantum search over N unordered items requires O(√N)
    queries. Applied to a hash with output length n, this halves the
    effective security level.
    Citation: Grover, "A Fast Quantum Mechanical Algorithm for Database
    Search", STOC (1996).

## 3. Verification

Each verification rule names the method and the source.

V1  Unit test
    Method: A function is called with a fixed input and its output is
    compared to an expected value.
    Citation: IEEE 1008-1987, "Standard for Software Unit Testing".

V2  Property test
    Method: A predicate is checked on many randomly generated inputs.
    Citation: Claessen & Hughes, "QuickCheck: A Lightweight Tool for
    Random Testing of Haskell Programs", ICFP (2000).

V3  Fuzz test
    Method: A program is run on many randomly mutated inputs, monitoring
    for crashes and undefined behavior.
    Citation: Miller, Fredriksen, So, "An Empirical Study of the
    Reliability of UNIX Utilities", CACM (1990).

V4  Type check
    Method: The compiler verifies that every operation is applied to
    values of the correct type.
    Citation: Pierce, "Types and Programming Languages" (2002).

V5  Borrow check
    Method: The compiler verifies that no mutable reference aliases
    another reference, and that no reference outlives its referent.
    Citation: Matsakis & Klock, "The Rust Programming Language"
    (2014), ownership and borrowing.

V6  Clause conformance
    Method: Each declared clause has a script that returns success or
    failure. The clause set is satisfied iff every script returns
    success.
    Citation: ISO/IEC/IEEE 12207:2017, §7.2.2 (verification process).

V7  Audit trail
    Method: Every write to the tree is recorded with timestamp, actor,
    before-hash, and after-hash.
    Citation: ISO/IEC 27001:2022, Annex A.8.15 (logging).

V8  Interface documentation
    Method: The public API is enumerated and published as a document.
    Citation: ISO/IEC/IEEE 15288:2023, §6.4.7 (integration process).

## 4. Attribution

Every claim in sections 1–3 is traceable to a body, a publication, or
both. The bodies are: BIPM, CODATA, IETF, IEEE, ISO/IEC, NIST. The
publications are: the SI Brochure, CODATA recommended values, the RFC
series, the IEEE standards series, the ISO/IEC standards series, and
the NIST FIPS and SP series.

Peer-reviewed research cited: Bellare & Rogaway (1993), Claessen &
Hughes (2000), Grover (1996), Jaccard (1901), Landauer (1961), Laplace
(1812), Manning et al. (2008), Matsakis & Klock (2014), Miller et al.
(1990), Pareto (1906), Pierce (2002), Rogaway & Shrimpton (2004).

Standards cited: RFC 3339, RFC 8032, RFC 8259, IEEE 754-2019,
IEEE 1008-1987, ISO/IEC 27001:2022, ISO/IEC/IEEE 12207:2017,
ISO/IEC/IEEE 15288:2023, NIST FIPS 180-4, NIST FIPS 186-5.

## 5. What this document does not claim

This document does not claim that the crate is compliant with any
regime. Compliance requires a management system, an audit by an external
body, and a certificate. None of those exist.

This document does not claim that any primitive is unconditionally
secure. FIPS 180-4 declares SHA-256; Grover's bound declares that its
effective security under a quantum adversary is 128 bits, not 256.

This document does not claim that any verification rule proves
correctness. Tests prove presence of behavior on sampled inputs. Types
prove well-typedness. Borrow checks prove memory safety. None proves
semantic correctness.

This document does not claim completeness. The set of primitives,
algebraic relations, and verification rules is finite and declared.
The set of everything that could be claimed is not.

## 6. Attribution footer

Every line above is either a citation or a statement backed by a
citation. The crate's clause set (docs/CLAUSES.md) references this
document by identifier. A clause whose backing is not in this document
is a clause without support.

Body count: 6. Publication count: 10. Peer-reviewed paper count: 12.
Total attribution entries: 28.
