# Gate Results

Status: `execution and reconstruction PASS / scientific disposition HOLD`.

Evidence mode: `Ran`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Exact result-blind v3 admission | `PASS` | Independent science/Rust/consumer `PASS/PASS/PASS` at `5ebfc5135`. |
| Four-site 12-lane execution | `PASS` | Release binary completed control, same-state, and sequential lanes at all sites. |
| Retained replay verification | `PASS` | `--verify-existing`; `143/143` retained artifacts. |
| Reconstruction and endpoint closure | `PASS` | All primitive, total, mass, cold, continuity, join, and support guards. |
| Protected production outputs | `PASS` | HBP/PASS/WAT exact at all four sites; CoE authority unchanged. |
| Frozen scientific classifier | `PASS` | Emitted `PREDECESSOR_NOT_REPRODUCED` and projection difference without overclaim. |
| Assurance source adoption | `PASS` | Typed transaction `31798778...`; DRAFT preserved, no authority invalidated. |
| Critical terminal validation | `PENDING` | Exact closure-head heavy gate to be appended. |
| Dual terminal verification | `PENDING` | Exact closure-head independent verification to be appended. |

Scientific reconciliation remains `HOLD` because the exact predecessor bridge
fails. This does not invalidate the package's completed operator-mechanics
characterization.
