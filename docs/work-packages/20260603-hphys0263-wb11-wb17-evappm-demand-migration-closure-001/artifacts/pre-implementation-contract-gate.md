# Pre-Implementation Contract Gate

Status: completed

Evidence mode: ran

Ran:

- `cargo test -p openwepp-runner hphys0263_wb11_seed_uses_evappm_branch_when_pmetpara_selects_pmet -- --nocapture`

Observed red gate:

- The test failed before production migration because the PMET branch
  intermediate `pmet.etorc_mm` was missing.
- Failure surfaced at
  `crates/openwepp-runner/src/hillslope/mod.rs:7338` in the pre-migration
  line map.
- This confirmed the contract-derived test was exercising the missing
  `evappm.for` lineage instead of passing through the existing
  Priestley-Taylor seed path.

Truthfulness note:

- The exact red-gate run occurred before the production edits in this package
  execution. Later line numbers shifted after helper insertion and formatting.
