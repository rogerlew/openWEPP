# PERFDEEP08 Hard-Isolation Plan

Status: complete.
Evidence mode: Static/Ran.

## Candidate Plan Executed

| Candidate | Source | Expected effect | Result |
|---|---|---|---|
| Cache PERFDEEP02 roundtrip env lookup | `perfdeep02_frame_roundtrip.rs` | Avoid env lookup on every disabled hook call | Rejected: H2637 `691.93 s`. |
| Short-circuit inactive indexed-shadow hooks | `indexed_shadow_surface.rs` | Avoid thread-local borrow when report env var unset | Rejected with same candidate: H2637 `691.93 s`. |
| Hoist PERFDEEP02/03 flags outside scheduler phase loop | `scheduler.rs` | Avoid repeated `OnceLock` checks | Reverted before timing to avoid retaining 3000+ line touched file without closure plan. |

## Disposition

No candidate was retained. The measured hook-cache candidate did not close the
hold and was slower than the PERFDEEP07 retained point.
