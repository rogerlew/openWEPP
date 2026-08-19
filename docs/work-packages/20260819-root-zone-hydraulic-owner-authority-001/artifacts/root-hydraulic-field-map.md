# Root Hydraulic Field Map

Historical HOLD intake evidence. Its blocker dispositions are superseded by
`SC-ROOTZONEHYDRAULICS-001@1.0.0-rc1`; the source census remains retained.

Evidence class: `Static`

| V10 operand | Current repository source | Disposition |
|---|---|---|
| `soil_psi_mm` | Caller/template `SoilLayerForcing.matric_potential_mm`; no live owner | Candidate derivation from live `theta_m/depth_m`, porosity, `psi_sat`, B; retention parameters are not currently configured. |
| `lateral_root_length_mm` / `dxroot` | `VegetationConfiguration.strata[].root_layers[].lateral_root_length_m` | Existing immutable authority; retain unchanged. |
| `root_path_length_mm` / `z3` | Caller/template and tests only | **BLOCKED**: no admitted input or builder. Must not alias `lateral_root_length_m`, layer depth, plant height, or a fixture constant. |
| `gravity_root_mm` | Caller/template only | Derivable from ordered live/configured layer thickness after exact topology binding. |
| `soil_conductivity_mm_s` | Caller/template; proposed source `DirectSubsurfaceLayerState.conductivity_m_s` | **BLOCKED**: current field is base/saturated conductivity, not current unsaturated soil-root conductivity. |
| `accessible` | Caller/template | Derivable from exact occupancy root membership and live hydrology topology. |
| `frozen` | Caller/template | Derivable from current staged winter/hydrology state; partial-frozen posture needs the existing typed policy. |

Repository search found production `z3_m` only where caller
`root_path_length_mm` is projected. Positive root paths elsewhere are tests,
diagnostic fixtures, or persisted validation-cache fields.
