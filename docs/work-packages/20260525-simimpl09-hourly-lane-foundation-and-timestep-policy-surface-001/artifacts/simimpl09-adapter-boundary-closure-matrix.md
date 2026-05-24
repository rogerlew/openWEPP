# simimpl09 adapter-boundary-closure-matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
| Surface family | SIMIMPL08 classification | SIMIMPL09 status | Closure behavior |
|---|---|---|---|
| `watbal_process_types` timestep/mode structure | adopt | integrated (typed runner policy surfaces) | requested/effective/lane context is typed and manifest-published |
| `watbal_daily_adapter` structural lane boundary | adopt | integrated | daily lane context resolves via typed policy with explicit scheduler mode |
| `watbal_hourly_adapter` structural lane boundary | adopt | integrated | hourly lane context resolves via typed policy with explicit scheduler mode |
| `watbal_closure_guard` typed residual/guard posture | adopt | integrated (typed guard semantics) | policy/boundary mismatch surfaces hard-fail with typed guard IDs |
| `wbk01..wbk08` bounded kernel family | adopt | maintained | scheduler-kernel execution remains canonical runtime path |
| `wbk09_hourly_qcap_policy` overlays | reject | excluded | not integrated; no qcap policy toggles surfaced |
| env identity/probe toggles | reject | excluded | no env-driven bypass/probe controls integrated |
| `wbk19a_*`, route/imp, pass adapter | defer | excluded | no deferred surfaces integrated into SIMIMPL09 runtime path |

## Manifest closure surface
- `/adapter_boundary/adopt_profile = "SIMIMPL08-adopt-only"`
- `/adapter_boundary/reject_surfaces_excluded = true`
- `/adapter_boundary/defer_surfaces_excluded = true`
- `/adapter_boundary/guard_id = "HS-SIMCONS-E-001"`

## Ran
- Verified adapter-boundary manifest assertions in
  `simimpl04_wepp_ui_mode_closure_contract`.
