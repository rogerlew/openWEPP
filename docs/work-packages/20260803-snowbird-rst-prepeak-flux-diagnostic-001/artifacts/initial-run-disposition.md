# Initial Arm Disposition

Status: `VALID NO-RESPONSE EVIDENCE / EXTENSION REQUIRED`

The first 36-cell arm completed with the active `harder_pomeroy_hourly` phase
model. Every response and flux metric was exactly invariant over `rst = 0..4
deg C` at all four sites. This is not a failed consumer-path test: copied
`snow.txt` hashes differ, manifests report each requested `rst`, and the real
direct-production consumer ran every cell.

Static inspection explains the invariance. `HarderPomeroyHourly` partitions
precipitation from hourly air temperature and relative humidity through the
hydrometeor-temperature solver; only `LegacyRst` executes `hrtemp_c > rst`.
The initial arm is retained as valid evidence that `rst` is inert under the
active phase model. A separately frozen existing-selector extension measures
the requested threshold stress response without changing production code.

An earlier single Mica cell completed before analysis failed closed because the
first tool version expected `observed_swe_m` instead of the actual
`observed_swe_mm` column. It wrote no receipt or result. That incomplete target
is preserved under `target/snowbird_rst_prepeak_flux_diagnostic_rejected_schema_mismatch/`
and is excluded from all terminal evidence.
