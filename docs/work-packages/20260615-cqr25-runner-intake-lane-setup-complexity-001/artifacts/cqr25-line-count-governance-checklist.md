# CQR25 Line-Count Governance Checklist

Status: complete.

Ran: before line counts:

| File | Lines |
| --- | ---: |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1667 |
| `docs/work-packages/README.md` | 632 |
| `docs/work-packages/cqr-burndown-execplan.md` | 716 |

Ran: after production refactor, before final artifact edits:

| File | Lines |
| --- | ---: |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2360 |
| `docs/work-packages/README.md` | 637 |
| `docs/work-packages/cqr-burndown-execplan.md` | 716 |

Static: target production file remains below the `3000` line threshold from
`docs/decisions/0021-module-coverage-closure-thresholds.md`.

Static: no non-exempt touched Rust file is at or above `3000` lines.
