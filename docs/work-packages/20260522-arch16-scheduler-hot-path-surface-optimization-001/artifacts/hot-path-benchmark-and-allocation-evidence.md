# Hot-Path Benchmark and Allocation Evidence

Static: source diff and hot-path call-surface inspection complete.
Ran: targeted and workspace tests plus gate commands executed.
Status: complete.

## Before vs After Clone/Allocation Surface

| Path | Before (pre-ARCH16) | After (ARCH16) |
|---|---|---|
| Hillslope scheduler kernel request | `state_surface.clone()` + `flux_surface.clone()` per phase call (2 full-map clones/phase) | Borrowed `&state_surface` + `&flux_surface` (0 full-map clones/phase) |
| Watershed scheduler kernel request | `state_surface.clone()` + `flux_surface.clone()` + `contributor_hillslopes.clone()` per step call | Borrowed `&state_surface` + `&flux_surface` + `&contributor_hillslopes` |

Static evidence anchor (post-change):
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:592-599`
- `crates/openwepp-watershed-orchestrator/src/lib.rs:337-352`
- `crates/openwepp-kernel-contract/src/lib.rs:225-294`

## Dynamic Evidence

1. Pointer-stability tests prove scheduler now lends stable writeback surfaces instead of constructing per-call full-map copies:
- `openwepp_hillslope_orchestrator::tests::execute_with_kernel_lends_stable_surface_references`
  - 9 phase calls observed; state/flux surface pointers stable across all calls.
- `openwepp_watershed_orchestrator::tests::execute_with_kernel_lends_stable_surface_references`
  - 3 dispatch calls observed; state/flux surface pointers stable across all calls.

2. Clone-site sweep command on scheduler files reports no remaining full-map clone call sites:
- Command:
  - `rg -n "state_surface\.clone\(|flux_surface\.clone\(|contributor_hillslopes\.clone\(" crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-watershed-orchestrator/src/lib.rs`
- Result:
  - no matches

## Notes

- ARCH16 evidence is allocation-surface focused. No separate timing harness existed in this package baseline, so this artifact records deterministic clone-site removal plus runtime pointer-stability validation.
