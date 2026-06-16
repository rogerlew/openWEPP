# CQR29 Line-Count Governance Checklist

Ran: line counts.

| Path | Before | After | Delta |
| --- | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs` | 330 | 383 | +53 |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs` | not scoped | 1248 | characterization |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | not scoped | 57 | import only |
| `docs/work-packages/README.md` | 654 | 658 | +4 |
| `docs/work-packages/cqr-burndown-execplan.md` | 742 | 742 | 0 before tracker update |

Static: no touched non-exempt Rust source file is at or above `3000` lines.

Ran: suppression census for the target file found one pre-existing
`#[allow(clippy::too_many_lines)]` before refactor and no target-file Clippy
complexity/line-length suppressions after refactor.
