# Canonical Tags

Every tag used in a code review, issue tracker, ledger event, or doctrine section collapses to one of eleven canonical tag sequences.

A tag sequence has three parts:

```text
domain / facet / unit
```

The eleven sequences are:

| # | Sequence | Meaning |
|---|---|---|
| 1 | `identity/role/actor` | who or what is acting |
| 2 | `artifact/path/subject` | what artifact is touched |
| 3 | `action/verb/operation` | what happens |
| 4 | `consequence/risk/class` | how dangerous it is |
| 5 | `state/lifecycle/transition` | where it is in a lifecycle |
| 6 | `evidence/provenance/hash` | why it is believed |
| 7 | `consent/approval/signature` | human authorization |
| 8 | `security/privacy/control` | defensive control |
| 9 | `compliance/legal/regime` | external regime or license |
| 10 | `observability/incident/signal` | runtime signal or incident trail |
| 11 | `boundary/doctrine/predicate` | why it belongs under P/S |

The executable classifier lives in `src/tags.rs`. Unknown tags default to `boundary/doctrine/predicate`, because an unclassified tag is a boundary question until proven otherwise.
