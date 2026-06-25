# Benchmark Evidence

Evidence mode: Ran.

## Implemented Gate Surface

Added
`tests/integration/clim06_frost_frozen_soil_kernel_contract/benchmark.rs`
and registered it in
`tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`.

The new `snowfrost_b_*` integration tests cover:

- Kurylyk/Stefan-style one-dimensional latent-only freezing-front upper bound;
- independent reconstruction of snow + residue + frozen-soil series
  resistance from published `surface_temp_c` and `Qsrf`;
- snowpack insulation raising resistance and suppressing freezing flux against
  the controlled bare-soil fixture;
- lower-front dry heat offsetting marginal surface freeze without a migration
  heat term;
- fine-layer ice/front mutation bounded by independently integrated positive
  surface freezing energy and volumetric latent heat.

## No-Migration Source Scan

Ran:

- `rg -n "qwet|Qwet|frzftp" crates -S || true`
- `rg -n "qwet|Qwet|frzftp" tests/integration/clim06_frost_frozen_soil_kernel_contract tools/snowfreeze_observed -S || true`

Result:

- `crates/`: no hits.
- Non-production expected hits only:
  - `tools/snowfreeze_observed/classify_residuals.py` warning text from
    SNOWFROST-FIDELITY-A;
  - the new CLIM06 B assertion text describing the no-`Qwet` fixture.

Disposition: no production Rust `Qwet`, `qwet`, or `frzftp` implementation is
present after this package.
