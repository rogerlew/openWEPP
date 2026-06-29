# Snow Conductivity Provenance

Status: `RECORDED`

This artifact records the authority and implementation mapping for the
snow-density-to-conductivity relation used by the layered insulation integral.

Authority:

- `SC-SNOWFREEZE-001` v109 `REF-SNOWFREEZE-PARADIGM2-STAGE2`
- Sturm et al. 1997, `Journal of Glaciology` 43(143), 26-41,
  DOI `10.3189/S0022143000002781`
- Legacy WEPP frozen surface heat path:
  `/workdir/wepp-forest_260430_baseline/src/tmpadj.for:274-287` and
  `/workdir/wepp-forest_260430_baseline/src/frostn.for:483-498`, pinned baseline
  commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

Implementation mapping:

- Code: `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- Function: `sturm1997_snow_conductivity_w_m_k`
- Relation:
  - for `rho < 0.156 g cm^-3`, `k = 0.023 + 0.234 rho`
  - otherwise, `k = 0.138 - 1.01 rho + 3.233 rho^2`
  - `rho` is `g cm^-3`, `k` is `W m^-1 K^-1`, and openWEPP multiplies by the
    existing frost control `ksnowf`
- Layer resistance: `sum(layer_thickness_m / k(layer_density_kg_m3))`
- Frost handoff: invert the same relation to an internal
  `snow_frost_effective_density` whose existing bulk `snow_depth / k(rho)`
  reproduces the layer-stack resistance.

No constants were fitted to openWEPP fixtures or observations.
