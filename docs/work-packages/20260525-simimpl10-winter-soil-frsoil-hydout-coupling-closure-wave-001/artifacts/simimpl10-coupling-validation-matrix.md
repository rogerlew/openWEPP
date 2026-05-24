# simimpl10-coupling-validation-matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
| Check ID | Coupling vector | Rule | Result |
|---|---|---|---|
| `SIMCOUP-V-001` | winter | `newsnw > 0`, `ssd > 0`, `newsnw <= ssd`, `runtime_swe >= 0` | pass |
| `SIMCOUP-V-002` | frsoil | `dfrost,dthaw in [0,0.20]`, `nft >= 0`, `ws_frz >= 0` | pass |
| `SIMCOUP-V-003` | soil/frsoil | `0 <= infcap_frz <= ssc` | pass |
| `SIMCOUP-V-004` | hydout-equivalent | `SoilWaterTotal = Total-Soil + frozwt` within `1e-6` | pass |
| `SIMCOUP-V-005` | publication | coupling vector manifest surface emitted with `guard_id=HS-SIMCOUP-E-001` | pass |

## Ran
- Validation exercised by:
  - `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract`
  - `cargo test --workspace`
