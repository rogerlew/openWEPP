# Required Reading Map

Status: EXECUTED-READ. Evidence mode: Static.

All Core entries were read before removal edits. `SC-OFEROUTE-002.md` was
read before deletion and converted into `artifacts/strip-inventory.md`.
Conditional entries were consulted as needed for gate selection and unit /
contract checks; on-demand Rust surfaces were read while applying the strip.

| Path | Tier | Purpose | Bytes |
|---|---|---|---:|
| `AGENTS.md` | Core | Repo conventions, validation gates. | 10269 |
| `docs/work-packages/AGENTS.md` | Core | Package execution/closure rules. | 16364 |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Contract amendment/withdrawal rules. | 5599 |
| `crates/AGENTS.md` | Core | Rust crate conventions, line-count governance. | 5171 |
| `docs/decisions/0037-abandon-hybrid-implicit-stepping.md` | Core | The decision this package executes: grounds, keep-list, knowledge-extraction items. | ~12000 |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` | Core | READ BEFORE DELETION: guard map / test-vector obligations / BEI are the authoritative strip inventory. | 45851 |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Core | Hybrid pointer rows to remove; all other surfaces protected. | 127733 |
| `docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/artifacts/review-claude.md` | Core | CL-M3 test-retirement obligation discharged here. | 12196 |
| `docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/artifacts/selector-policy.md` | Core | The selector surface being removed. | 1425 |
| `docs/standards/local-ci-gate-selection.md` | Conditional | Narrowed iteration gates during the strip. | - |
| `docs/specifications/unit-governance.md` | Conditional | Only if removal exceeds pure deletion on a runtime symbol surface. | - |
| `crates/.../ofe_routing/{implicit_recession,cascade,kinematic_wave,profile,friction,dval}.rs` | On-demand | Strip surface, guided by `artifacts/strip-inventory.md`. | - |
| `crates/.../direct_runtime/laned_active.rs`, `crates/openwepp-runner/src/hillslope/{laned_active,00_runner_intake_and_lane_setup,05_runner_execution_and_outputs}.rs` | On-demand | Selector/counter plumbing strip surface. | - |

Budget: local_required_bytes_total ~240000; threshold_outcome OK
(`<=400000`).
