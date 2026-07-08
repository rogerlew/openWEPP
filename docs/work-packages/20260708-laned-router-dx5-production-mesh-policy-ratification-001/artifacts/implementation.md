# Implementation

Status: `EXECUTED-COMPLETE`
Evidence mode: Static + Ran.

## Code Changes

The active production mesh default now uses `target_dx_m = 5.0`:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
  adds `LANED_ACTIVE_PRODUCTION_TARGET_DX_M = 5.0` and makes
  `DirectLanedActiveMeshPolicy::production_default()` return `TargetDx` with
  the retained floor/cap.
- `DirectLanedActiveMeshPolicySummary::default()` now serializes the active
  default as `mode = target_dx`, `target_dx_m = 5.0`.
- The mesh-policy unit test now pins the production default on a 300 m OFE to
  `60` cells and retains the 26 m floor at `10` cells.
- `crates/openwepp-runner/src/hillslope/laned_active.rs` keeps the diagnostic
  selector explicit and fail-closed; absent
  `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` returns the production default.

## Runtime Evidence

`artifacts/run_default_dx5_evidence.py` built the release runner and executed
four modes for every selected real-cohort member:

- active no-env production default;
- active explicit `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=5.0` control;
- subsystem-off default;
- subsystem-off with the mesh env present.

Result: `artifacts/default-dx5-evidence.md` is `PASS`.

Release binary SHA256:
`3f60d8bd064a11c514edd1558951051782f2e757f4ce71ce4b2e7be292c9524b`.

Active no-env versus explicit `dx5` output identity: `PASS` for
`mn_corn_h4`, `n_idaho_forest_h1`, and `wa_cascades_forest_h1` across HBP,
loss JSON, pass parquet, WAT parquet, and trace JSONL.

Subsystem-off default versus subsystem-off mesh-env-control identity: `PASS`
for all three real-cohort members across HBP, loss JSON, pass parquet, and WAT
parquet.

## Closure Evidence

Active default closure assertions passed for all selected real-cohort members.
Observed maxima on the no-env active runs:

| Member | Max cascade rel | Max seam rel | Max identity rel | Clamp/source |
|---|---:|---:|---:|---:|
| `mn_corn_h4` | `1.0388136044916924e-14` | `5.092454815097145e-15` | `8.752483860221787e-14` | `0.0` |
| `n_idaho_forest_h1` | `9.435478910085368e-15` | `6.427411888066303e-15` | `5.254588131978787e-14` | `0.0` |
| `wa_cascades_forest_h1` | `4.705058001136025e-14` | `4.832475752036399e-14` | `5.933273356520313e-14` | `8.44135663405994e-19` |

WA total clamp remains the rev-41 roundoff-scale class
(`7.305156020320419e-13 m3` on `865400.7095073939 m3` source), and the live
runtime clamp-source hard-fail did not trip.

## Shadow Decision

Shadow mesh is explicitly out of scope and unchanged. The diagnostic shadow
still owns `LANED_SHADOW_CELLS = 10`, `LANED_SHADOW_SAMPLE_DT_S = 900`, and
`LANED_SHADOW_MAX_DT_S = 300`. The rev45 default applies only to
`OPENWEPP_LANED_ACTIVE=1`.
