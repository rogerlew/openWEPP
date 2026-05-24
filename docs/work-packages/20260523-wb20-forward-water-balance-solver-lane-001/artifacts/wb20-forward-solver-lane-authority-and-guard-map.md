# WB20 Forward Solver Lane Authority And Guard Map

Status: `completed`
Evidence mode: `Static`

## Scope
Record canonical WB20 forward-solver lane authority and runtime guard behavior
for observed-target exclusion semantics.

## Canonical Authority
- `SC-WATBAL-001` v24:
  - `INV-WATBAL-016` and WB12/WB14 addendum updates define lane-scoped closure
    semantics and observed-target exclusion in forward lanes.
- `SC-RUNOFFPART-001` v15:
  - `INV-RUNOFFPART-011` defines WB12 runoff closure-delta lane semantics under
    runoff-partition authority.
- `SC-SYSTEM-001` v10:
  - `INV-SYSTEM-016` defines parity-lane governance requirement for explicit
    lane manifest and no-substitution evidence.

## Lane and Guard Map
| Lane mode | Selector | Acceptance closure semantics | Observed-target role | Guard behavior |
|---|---|---|---|---|
| Forward-solver lane | `wb20_forward_solver_lane_enabled = 1` | `wb12_*_closure_delta` computed from solver residual identities | Excluded from acceptance-driving inputs | Missing/non-finite/domain-invalid required symbols hard-fail; non-residual closure over tolerance hard-fail |
| Compatibility lane | `wb20_forward_solver_lane_enabled = 0` or selector absent | `wb12_*_closure_delta` computed against observed targets | Retained as compatibility closure diagnostics | Missing/non-finite/domain-invalid symbols hard-fail; closure-delta overflow hard-fail |

## Typed Failure Surfaces
- Runoff reconciliation lane/domain failures: `HKERNEL-WB14-RUNOFF-E-001..003`
- Storage reconciliation lane/domain failures: `HKERNEL-WB12-STORAGE-E-001..003`

## Runtime Implementation Anchor
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - lane selector constant: line ~147
  - selector resolver helper: line ~1930
  - runoff branch semantics: `run_runoff_reconciliation` line ~4895
  - storage branch semantics: `run_storage_reconciliation` line ~5295

## No-Silent-Default Posture
Forward-lane acceptance semantics explicitly exclude observed-target
substitution. Compatibility lane remains explicit and does not silently coerce
closure semantics.
