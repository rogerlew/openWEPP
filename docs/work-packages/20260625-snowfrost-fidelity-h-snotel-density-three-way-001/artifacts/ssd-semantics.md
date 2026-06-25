# SSD Semantics

Evidence mode: Static.

Phase-0 gate passed.

- Pinned baseline `/home/workdir/wepp-forest_260430_baseline/src/infile.for`
  reads `snow.txt` as `rst`, `newsnw`, `ssd`; default `ssd = 250.0`.
- Pinned baseline `/home/workdir/wepp-forest_260430_baseline/src/snowd.for`
  uses `ssd` as the cold-settling density threshold: when snowpack density is
  greater than `ssd`, the settling factor is clamped to `1`.
- Pinned baseline `snowd.for` carries `snodpy`/`densg` into `snodep`/`densgt`
  and writes the evolved density back.
- openWEPP parses the same three `snow.txt` fields in
  `crates/openwepp-input-contract/src/parsers/snow.rs` and seeds
  `snow.options.ssd` from runtime input projection.
- openWEPP snow settlement uses `inputs.ssd_kg_m3` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`.

Conclusion: field 3 is a legitimate snow-settling-density / settling-threshold
arm for H. The observed-density arm remains a site-characterization arm, not a
residual-fit calibration.
