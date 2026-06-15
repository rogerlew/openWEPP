# CQR20 CRAP Before

Status: complete.

Ran: before CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/crap_before.json
```

Ran: ranked before rows for
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`:

```text
project_annual_extension_controls              line 585  CC 29.0  coverage 25.0               CRAP 383.796875
annual_extension_variant_name                  line 557  CC 7.0   coverage 0.0                CRAP 56.0
growth_equation_parameter_values               line 274  CC 22.0  coverage 78.57142857142857  CRAP 26.762390670553938
project_perennial_grazing_cycle_symbols        line 817  CC 11.0  coverage 100.0              CRAP 11.0
validate_slope_points                          line 112  CC 6.0   coverage 52.77777777777778  CRAP 9.790895061728396
derive_avgslp                                  line 154  CC 8.0   coverage 72.3404255319149   CRAP 9.354304922801305
validate_projection_day                        line 485  CC 5.0   coverage 62.5               CRAP 6.318359375
project_annual_extension_symbols               line 712  CC 6.0   coverage 100.0              CRAP 6.0
project_primary_annual_extension_aliases       line 757  CC 6.0   coverage 100.0              CRAP 6.0
insert_monthly_climate_symbols                 line 21   CC 5.0   coverage 100.0              CRAP 5.0
```

Ran: target-file before LCOV:

```text
lines 599/796 75.25%
functions 44/48 91.67%
```

Static: `cargo crap` emitted the repo-wide warning that 126 source files had no
matching LCOV entries. The target file had matching LCOV entries, matching prior
CQR package evidence posture.
