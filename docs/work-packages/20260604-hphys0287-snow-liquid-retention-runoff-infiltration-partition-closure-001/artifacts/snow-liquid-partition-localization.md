# Snow Liquid Partition Localization

Status: complete
Evidence mode: Static + Ran

Static:
- Root cause was not a valid-run snow liquid retention magnitude correction. The proven defect was a fail-open runtime snow-state seam: material negative projected snow state could be classified as inactive stale snow and zeroed before WB12 same-pass infiltration and WB14 runoff reconciliation.
- `resolve_active_snow_coupling`, `compute_same_pass_wb14_infiltration_lineage`, and `run_runoff_reconciliation` now share `validate_runtime_snow_state_domains`.
- The shared validator fails closed for missing projected snow vector members, non-finite values, material negative SWE/depth/density/settle count, and density above the snow density cap.
- The validator returns explicit no-snow only when no snow option/control/runtime projection exists at all.

Ran:
- Expanded HPHYS0287 contract tests pass for direct-rain partition, dry-cold inactive fallback, partial vector failure, no-projection compatibility, and bounded SWE roundoff.
- Adjacent snow/frost/storage tests pass after completing the `clim06` fixture snow-state vector.

Disposition:
- HPHYS0287 closes the fail-closed guard issue.
- Valid-run residuals remain unchanged; continuation should target baseline-authoritative rain-on-snow retention/release magnitude and melt/runoff partition lineage.
