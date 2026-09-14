# Novel Harness Control Plane

The Team TMIG control plane is not a scheduler. It is a stateless conformance engine with exactly three responsibilities:

1. **Admit** — check an `ActionProof` against thesis `P` and scope `S`.
2. **Route** — deterministically map consequence class to `(flock polarity, Dunbar layer)`.
3. **Attest** — emit a content-addressed attestation tied to the doctrine hash.

It has no planner, queue, retry policy, learning loop, or model router. Agents propose; the control plane checks conformance.

## Derivation inputs

The plane is derived from three content-addressed inputs:

- `docs/DOCTRINE.md`
- `catalog/local-models.json`
- `src/skills.rs` / runtime `SkillSet`

Changing any input changes the plane hash. Prior attestations remain valid against the doctrine hash they carry.

## Formal shape

```text
ControlPlane : (Doctrine, Catalog, Skills) -> Plan

Plan = {
  admit  : ActionProof -> Result<(), AdmitError>,
  route  : Consequence -> (FlockPolarity, DunbarLayer),
  attest : (ActionProof, DoctrineHash) -> Attestation,
}
```

## Consequence routing

| Consequence | Polarity | Dunbar layer |
|---|---|---|
| None | White | Org |
| Low | White | Repo |
| Moderate | White | Service |
| High | White | Subsystem |
| Legal | White | Module |
| Irreversible | Black | Module |

The implementation lives in `src/controlplane/mod.rs`.
