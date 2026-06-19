# PERFDEEP07 Gate Results

Status: HOLD.
Evidence mode: Static/Ran.

## Required Gates

| Gate | Result | Evidence |
|---|---|---|
| Required-reading map populated | PASS | `required-reading-map.md` updated during execution. |
| Owned-file manifest complete | PASS | `owned-file-manifest.md` updated during execution. |
| Disabled-path audit complete | PASS | `perfdeep07-disabled-path-audit.md`. |
| Disabled-path H2637 identity | PASS | Retained `hash-hot` output: HBP/WAT byte-identical; PASS/WAT Arrow equal; plot/loss run-name-only difference. |
| Disabled-path timing median `<= 676.67 s` | FAIL | Best viable single run `685.85 s`; P0 median gate not completed because no viable candidate reached threshold. |
| Static zero-cost-disabled proof | FAIL | Dense-first tax reduced but endpoint remains above gate. |
| Direct-frame implementation plan complete | NOT RUN | Blocked by P0 failure. |
| Direct-frame identity fixtures | NOT RUN | Blocked by P0 failure. |
| H2637 opt-in output identity | NOT RUN | Blocked by P0 failure. |
| H2637 opt-in endpoint/RSS | NOT RUN | Blocked by P0 failure. |
| No-hot-loop-map proof | NOT RUN | Blocked by P0 failure. |
| Layout/allocation evidence | NOT RUN | Blocked by P0 failure. |
| Line-count governance | PASS | `line-count-governance.md`; touched WARN files recorded, no touched file at or above `3000` lines. |
| `cargo fmt --check` | PASS | Ran after retained edits. |
| Focused Rust tests | PASS | `cargo test -p openwepp-kernel-contract indexed_request_without_dense_slots_keeps_dense_surface_absent`; `cargo test -p openwepp-hillslope-orchestrator writeback`; `cargo test -p openwepp-runner`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | Blocked by P0 HOLD; not a completion claim. |
| `cargo test --workspace` | NOT RUN | Blocked by P0 HOLD; not a completion claim. |
| `cargo deny check` | NOT RUN | Blocked by P0 HOLD; not a completion claim. |
| Markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001 --format json`: `28` files scanned, `0` errors, `0` warnings. |
| Diff whitespace check | PASS | `git diff --check`. |
| Dual review complete | NOT RUN | Blocked by P0 HOLD. |
| Dual verification complete | NOT RUN | Blocked by P0 HOLD. |
| Finding disposition complete | PASS | `disposition.md`. |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` prevents complete disposition.
