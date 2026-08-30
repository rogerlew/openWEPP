# Line-count governance

Status: PASS

Evidence mode: `Ran + Static`. Touched 2000+ `.rs` files retain WARN rationale;
no non-generated `.rs` file is at or above the 3,000-line closure threshold.

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
