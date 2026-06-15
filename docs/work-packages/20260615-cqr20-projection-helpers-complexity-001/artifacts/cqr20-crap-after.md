# CQR20 CRAP After

Status: complete.

Ran: after CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/crap_after.json
```

Ran: ranked after rows for
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`:

```text
growth_equation_parameter_values                line 274  CC 22.0  coverage 78.57142857142857  CRAP 26.762390670553938
project_perennial_grazing_cycle_symbols         line 852  CC 11.0  coverage 100.0              CRAP 11.0
validate_slope_points                           line 112  CC 6.0   coverage 52.77777777777778  CRAP 9.790895061728396
derive_avgslp                                   line 154  CC 8.0   coverage 72.3404255319149   CRAP 9.354304922801305
project_annual_extension_controls               line 584  CC 9.0   coverage 100.0              CRAP 9.0
annual_extension_variant_name                   line 557  CC 7.0   coverage 100.0              CRAP 7.0
project_annual_extension_symbols                line 747  CC 6.0   coverage 100.0              CRAP 6.0
project_primary_annual_extension_aliases        line 792  CC 6.0   coverage 100.0              CRAP 6.0
validate_projection_day                         line 485  CC 5.0   coverage 100.0              CRAP 5.0
project_burn_annual_extension_controls          line 632  CC 4.0   coverage 100.0              CRAP 4.0
project_cut_annual_extension_controls           line 683  CC 3.0   coverage 100.0              CRAP 3.0
project_remove_annual_extension_controls        line 705  CC 3.0   coverage 100.0              CRAP 3.0
project_herbicide_annual_extension_controls     line 611  CC 2.0   coverage 100.0              CRAP 2.0
project_silage_annual_extension_controls        line 662  CC 2.0   coverage 100.0              CRAP 2.0
project_no_annual_extension_controls            line 728  CC 2.0   coverage 100.0              CRAP 2.0
AnnualExtensionProjection::zeroed               line 470  CC 1.0   coverage 100.0              CRAP 1.0
annual_extension_mismatch                       line 568  CC 1.0   coverage 100.0              CRAP 1.0
```

Static: final scoped target `project_annual_extension_controls` CRAP is `9.0`.
All newly extracted annual extension helpers are CRAP `<= 4.0`, below the
`30` threshold.

Static: `cargo crap` emitted duplicate rows for some functions and the same
repo-wide LCOV warning pattern seen before; the duplicated target rows reported
identical CRAP values.
