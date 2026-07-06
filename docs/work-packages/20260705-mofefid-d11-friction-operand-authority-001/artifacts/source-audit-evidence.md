# Source Audit Evidence

Status: executed-hold-source-authority
Evidence mode: Static

## Summary

The audit found partial source authority, not enough to implement a real builder:

- `I`: source candidate exists from typed climate hyetograph intensity and from
  binned hourly rainfall depth with an explicit `/3600 s` conversion.
- `LAI`: source candidate exists from plant growth state.
- `h_c`: candidate symbols exist, but D11 did not ratify Lane D timing/source
  binding.
- `k_o`, `C_d`, `D_r`, `lambda`: no WEPP-runtime source/default mapping was
  found. Chapter-10 hydraulics and D-val fixture constants are adjacent
  evidence only.

## Inspected Sources

| Source | Finding |
|---|---|
| `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | Current shadow uses `LANED_SHADOW_KO = 500`, `CellParameters::bare(...)`, and an `I=0` intensity closure. |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | `CellParameters` supports all required operands and validates finite/nonnegative domains, but the shadow constructs only bare cells. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` | `wb14_hourly_rainfall_m` is a local-rainfall hourly depth surface, explicitly separate from runon/excess. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs` | `direct_production_hyetograph` validates `intsty_m_s` as finite nonnegative `m s^-1`. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` and builder helpers | Growth state carries `leaf_area_index`; builder reads `lai`. |
| `SC-CLIMATE-001`, `SC-PLANT-001`, `SC-HYDRAULICS-001`, `SC-RESIDUE-001` | Climate/plant support `I` and `LAI`/`Hc` candidates. Hydraulics/residue do not directly alias to Papanicolaou `k_o`, `C_d`, `D_r`, or `lambda`. |

## Subagent Outputs

- Explorer operand audit: recommended `HOLD` overall; `I` and `LAI` are
  `PASS-source / HOLD-wiring`; `k_o`, `C_d`, `D_r`, `lambda`, and `h_c` are
  held.
- Explorer consumer audit: confirmed no current friction builder exists; the
  real shadow consumer reads live publication rows but not friction operands.

Rejected aliases:

- D-val constants such as `k_o=500`, `D_r=0.06 m`, and `lambda=0.2` are fixture
  values, not WEPP runtime defaults.
- `SC-HYDRAULICS-001` Chapter-10 friction/cover terms are not direct
  Papanicolaou roughness-element aliases without a new reconciliation package.
- Residue depth / random roughness candidates do not define `D_r` or `lambda`
  by themselves.
