# Review Agent A

Evidence mode: Static.

## Findings

No blocking findings.

## Checks

- Contract-first sequencing is present: v101 authority was added before code
  edits in the package history.
- The active default selector path maps absent/empty env values to
  `coe_liquid_holding_capacity_v1` and
  `physics_bulk_density_compaction_v1`.
- `legacy_coe` and `legacy_wepp` remain explicit rollback/test values.
- `physics_bulk_spring_densification_v1` is not accepted by the active density
  selector.
- `coe_winter_thaw_state_loss_v1` and `coe_shortwave_albedo_v1` are not
  accepted by the active 10.3.15 melt selector.
- New tests assert no parser/runfile/user CLI selector exposure.
- The diagnostic evidence uses real `openwepp-cli-hill` WAT output and direct
  snow trace rows.

## Residual Risk

`498/1415` paired snow-depth rows still fail. This is correctly carried as a
frost-attribution blocker and not hidden as a closure claim.
