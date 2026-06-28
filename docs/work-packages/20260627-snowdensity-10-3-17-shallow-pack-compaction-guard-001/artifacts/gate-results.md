# Gate Results

Status: complete
Evidence mode: Static + Ran

| Gate | Command / Artifact | Result |
|---|---|---|
| Build real WAT runner | `cargo build -p openwepp-runner --bin openwepp-cli-hill` | pass |
| Python syntax | `.venv/bin/python -m py_compile tools/snowfreeze_observed/shallow_pack_compaction_guard.py` | pass |
| Coupled WAT/trace diagnostic | `.venv/bin/python tools/snowfreeze_observed/shallow_pack_compaction_guard.py` | ran; non-promotion |
| Focused contract test | `cargo test --test snowdensity10_3_17_shallow_pack_compaction_guard -- --nocapture` | pass, `4 passed` |
| Formatting | `cargo fmt --check` | pass after `cargo fmt` |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| Workspace tests | `cargo test --workspace` | pass |
| Dependency/license gate | `cargo deny check` | pass |

Current-scope candidate gates:

| Gate | Result | Evidence |
|---|---|---|
| Cut induced under-persistence tail, especially `harvard_hardwood` | fail | Global induced under-persistence `177 -> 176`; `harvard_hardwood` `73 -> 73`, recovered `0`. |
| Do not worsen over-persistence tail | fail | Over-persistence `264 -> 267`; `3` new over rows from non-over rows. |
| Shallow threshold authority-derived, not fixture tuned | pass | Threshold fixed at `0.25 m` from Marks/SNOBAL active layer authority; no fixture-tuned threshold search performed. |
| Whole-model conservation / no mass-term drift | fail | SWE-depth-density identity closed locally, but trace comparison recorded `max_abs_mass_term_delta_m = 3.3417423040965196e-3`; `snow_state_conservation_ok=false`. |
| Protected boundaries | pass | No default activation, cap, public schema, fixture, user CLI, compatibility runtime, Qwet/frzftp, or frost-attribution change. |

Disposition: `NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET`; activation remains
unauthorized.
