# Six Degrees as Boundary

A wishlist has no fixed point when every item is within a few hops of every other item. In a diameter-6 feature graph, asking "what is missing?" only adds neighbors; it does not find an exterior.

Team TMIG closes that graph with two predicates imposed from outside the feature graph.

## P: Thesis / invariant predicate

Every consequential action must be proposed by a machine, approved by a human with a cryptographic signature, executed only within its consequence class, verified after execution, and recorded in a content-addressed ledger. Nothing is thrown away. Every state is reversible. Every decision is auditable.

## S: Scope / domain predicate

Developer tooling that operates on files, commits, tickets, and commands, in real time, at machine speed, with human authorization for consequential actions.

## Rule

```text
For any feature graph F with diam(F) <= 6:
    "what's missing?" has no fixed point in F.
    It has a fixed point only in F_P ∩ F_S,
    where P and S are predicates imposed from outside F.
```

## Roadmap intersection

Built or partially built:

1. Content-addressed code review
2. Cryptographic change approval
3. Auditable observability

Buildable backlog:

1. Reversible refactoring
2. Content-addressed CI/CD
3. Reversible feature flags
4. Signed supply chain (SLSA)
5. Reversible deployments
6. Content-addressed config
7. Human-authorized secrets rotation
8. Reversible schema migrations
9. Auditable API contracts
10. Human-authorized quotas
11. Content-addressed test results
12. Reversible log retention
13. Two-person merges for critical files
14. Policy-gated auto-merge
15. Consequence-classed runbooks
16. Signed incident timelines
17. Reversible permissions

Everything outside `F_P ∩ F_S` is rejected or parked as aspirational. This is how Team TMIG stops asking "what's missing?" and starts asking "what in the bounded backlog is next?"

The executable form of this boundary lives in `src/boundary.rs`.
