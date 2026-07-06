# Line-Count Governance (D10B)

Status: executed
Evidence mode: Ran (`wc -l`, 2026-07-06)

| File | Lines | Status |
|---|---:|---|
| `ofe_routing/kinematic_wave.rs` | 1594 | OK (< 2000) |
| `ofe_routing/infiltration.rs` | 752 | OK |
| `ofe_routing/iwagaki_oracle.rs` | 708 | OK |
| `ofe_routing/cascade.rs` | 567 | OK |
| `ofe_routing/dval.rs` | 528 | OK |
| `ofe_routing/d10b_reconciliation_tests.rs` | 467 | OK |
| `ofe_routing/friction.rs` | 401 | OK |
| `ofe_routing/seam.rs` | 352 | OK |
| examples (3) | <= 202 | OK |

(Refreshed post-review-response, Codex Low-5; earlier 1472/256-line counts
were pre-review values.)

No file at WARN (2000+) or refactor-required (3000+) thresholds; no
exceptions needed.
