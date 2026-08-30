# Line-count governance

Status: HOLD — exact-current pre-existing `open_snow.rs` threshold breach

Evidence mode: `Ran + Static`. Touched 2000+ `.rs` files retain WARN rationale;
the exact-current `open_snow.rs` breach is recorded below and prevents a PASS.

## 2026-08-29 profiling reconciliation

Static: the profiling increment initially pushed
`snow_stage3_v11_attachment.rs` above 3,000 lines. Its flat diagnostic fields
were mechanically consolidated into `snow_stage3_v11_profile.rs`; terminal
counts are `2,981` and `58` lines respectively, so that new breach is closed.

Static: exact-current reconciliation also found `v11_covered/open_snow.rs` at
`3,228` lines (`3,219` before the nine profiling timer lines). This supersedes
the earlier statement that no 3,000+ path remained. The profiling-only owner
direction does not authorize an unrelated 229-line mechanical split, and the
package remains HOLD rather than claiming line-count closure. The detailed
timers add no new nested state or control flow beyond result-blind timing calls.

## Terminal mechanical splits

Four exact-current parents crossed the mandatory threshold during correctness
work and were split by include-only moves without API, digest, or behavior
changes:

| Parent | Final lines | Extracted helper | Lines |
|---|---:|---|---:|
| `snow_stage3_v11_terminal_execution.rs` | 2,997 | `snow_stage3_v11_terminal_receiver_topology.rs` | 52 |
| `stage3_solver.rs` | 2,940 | `stage3_solver_cumulative_order_tests.rs` | 99 |
| `open_snow.rs` | 2,985 | `open_snow_physical_reconstruction_helpers.rs` | 32 |
| `stage3_committed_publication.rs` | 2,974 | `stage3_committed_publication_event_helpers.rs` | 36 |

Ran: workspace rustfmt, diff hygiene, persisted-restart orchestrator check,
the extracted solver test, interlayer-reconstruction poison, and terminal
physical-ledger poison all pass. A terminal `find`/`wc -l` scan reports no
3,000+ `.rs` path.

The remaining touched 2,000--2,999-line parents are WARN debt. They remain
bounded modules with established include seams; further mechanical splitting
is not required for this owner-amended numerical objective and would expand
the terminal diff without changing behavior.

## Restart V3 disposition

The restart implementation and its fixture tests were split mechanically after
the terminal failure-cluster rerun. Current counts are 1,366 lines for
`snow_stage3_v11.rs` and 1,752 lines for `snow_stage3_v11_tests.rs`; neither is
WARN or BLOCK under the 2,000/3,000-line thresholds. Current-source test
compilation, rustfmt, and diff hygiene passed after the split.
