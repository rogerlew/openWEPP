# Disposition

Status: complete

Evidence mode: static + ran

## Final Disposition

Status: executed-hold.

HPHYS0299 corrected the HPHYS0298 diagnostic unit/provenance seam. Canonical
`hrsnow` is snowfall depth from pinned-baseline `stmtim.for`, and the openWEPP
comparison surface is `snow_hourly_snowfall_depth_sum_m`, not
`snow_hourly_snowfall_water_equiv_sum_m`.

## Corrected Result

- Baseline observe identity: H1/H7/H39 pass; release, observe-off, and
  observe-on WAT outputs are bit-identical for target lanes.
- Corrected partition ledger rows: `9`.
- Verdicts: `OPENWEPP-DEFECTIVE=9`.
- First cut-points:
  - `raw-hourly-melt=7`
  - `negative-melt-correction=1`
  - `hourly-forcing=1`
- Canonical `hrsnow` provenance: all rows map to
  `snow_hourly_snowfall_depth_sum_m`; no canonical `hrsnow` row maps to
  `snow_hourly_snowfall_water_equiv_sum_m`.

## Interpretation

HPHYS0298's all-window `hourly-forcing` verdict is superseded. Seven windows
now route to raw hourly melt, one H7 first-2013 row routes to post-raw
routed-melt/negative-melt handling without legacy-defective acceptance, and
only H39 first-2013 remains a corrected depth-vs-depth hourly-forcing producer
defect.

## Continuation Recommendation

Open a follow-on package focused on baseline-authoritative raw hourly melt
lineage and post-raw routed-melt handling after corrected precipitation-depth
forcing is closed. That package should prioritize:

1. H1/H7/H39 windows at `raw-hourly-melt` (`7` windows).
2. H7 first-2013 post-raw routed-melt/negative-melt handling (`1` window),
   explicitly not accepted as legacy-defective authority.
3. H39 first-2013 corrected depth-vs-depth hourly-forcing divergence (`1`
   window).

No WB17/WB18/WB19/WB13 downstream compensation is authorized.

## Validation

Ran and passed:

- `cargo fmt --check`
- `cargo test --test hphys0299_hourly_snow_partition_unit_provenance_contract`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `markdown-doc lint --path ... --format json`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`

`cargo deny check` retained existing duplicate dependency and unmatched license
allowance warnings while reporting advisories, bans, licenses, and sources
`ok`.
