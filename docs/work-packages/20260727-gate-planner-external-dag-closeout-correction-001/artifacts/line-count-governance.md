# Line-Count Governance

Status: `PASS WITH WARNINGS`

Evidence class: `Ran`

No touched production Rust file exceeds 3,000 lines.

| File | Lines | Disposition |
|---|---:|---|
| `external_dag.rs` | 1,639 | PASS |
| `external_dag/audit.rs` | 806 | PASS |
| `external_dag/custody.rs` | 1,027 | PASS |
| `external_dag/tests.rs` | 1,976 | PASS |
| `external_outputs.rs` | 761 | unchanged |
| `pre_heavy.rs` | 2,052 | WARN: narrow lifecycle/proof hooks pushed the existing module across 2,000 but remain below 3,000 |
| `publication.rs` | 2,978 | WARN: descriptor-relative recovery plus deterministic race and interruption fixtures remain below 3,000 |

`pre_heavy.rs` must receive decomposition review during implementation review.
The next non-critical structural increment should move its coverage fixtures
into a child test module without changing policy behavior.

`publication.rs` is within 22 lines of the mandatory threshold and may not grow
further in this package. The next non-critical structural increment must move
publication fixtures and adversarial tests to `publication/tests.rs` before
any production growth. Current recovery code and races remain together for
this critical correctness review.
