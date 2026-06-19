# PERFDEEP07 Line-Count Governance

Status: HOLD.
Evidence mode: Ran.

## Requirement

Files at or above `2000` lines are `WARN`; files at or above `3000`
non-exempt lines require refactor before implementation closure or an explicit
package exception with a sunset plan.

## Touched Rust Files

Ran `wc -l` on the retained Rust write set:

| File | Lines | Result |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | `2212` | WARN |
| `crates/openwepp-kernel-contract/src/lib.rs` | `686` | OK |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | `27` | OK |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` | `1147` | OK |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` | `1154` | OK |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | `2435` | WARN |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs` | `127` | OK |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs` | `856` | OK |
| `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs` | `158` | OK |

`crates/openwepp-hillslope-orchestrator/src/scheduler.rs` was not edited in
the retained HOLD patch. No touched file is at or above `3000` lines, so no
mandatory split is required for this HOLD disposition. The two WARN files
remain follow-up refactor candidates before any larger implementation closure.
