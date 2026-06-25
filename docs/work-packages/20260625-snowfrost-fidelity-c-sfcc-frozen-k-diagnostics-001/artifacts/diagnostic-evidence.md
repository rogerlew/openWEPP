# Diagnostic Evidence

Evidence mode: Ran.

## Tool Output

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/frozen_k_diagnostics.py \
  --output-json target/snowfrost_fidelity_c/diagnostics.json \
  --output-md target/snowfrost_fidelity_c/diagnostics.md
```

Result: passed after correcting the diagnostic Mualem helper to use
`math.sqrt(se)`.

Generated payload highlights:

- schema: `snowfrost-fidelity-c-frozen-k-diagnostics-v1`
- promotion status: `diagnostic_only_not_runtime_authority`
- runtime coupling: `none`
- Qwet authority: `not_authorized`
- soils: `coarse_diagnostic_fixture`, `medium_diagnostic_fixture`,
  `fine_diagnostic_fixture`
- temperature grid: `0.0, -0.1, -0.5, -1.0, -2.0, -5.0, -10.0`
- references retained in output: Dun 2010, Kurylyk/Watanabe 2013,
  Watanabe/Flury 2008, Azmatch 2012, Ming 2020, Cheng 2023, Amankwah 2021,
  Devoie 2022

The generated Markdown report explicitly states that the outputs are diagnostic
comparison surfaces only and are not texture defaults, runtime authority, field
calibration, or Qwet authorization.

## Candidate Curves

The diagnostic surface emits:

- Clapeyron-derived pressure head for subzero liquid-water screening;
- diagnostic van Genuchten effective saturation and liquid-water content;
- SFCC-Mualem relative frozen conductivity;
- Watanabe/Flury-style capillary-bundle screening ratio;
- Cheng-style impedance-scaled comparison values;
- Amankwah-style salinity freezing-point-depression sensitivity.

All parameter sets are marked `diagnostic_fixture_not_texture_default`.

## Execution Defects Found and Fixed

- Initial CLI run failed because `mualem_relative_conductivity` called
  `.sqrt()` on a Python float. Fixed to `math.sqrt(se)`.
- Initial focused Rust contract run exposed a parallel test isolation race: two
  tests wrote/read the same diagnostic JSON path. Fixed by giving each helper
  call a unique `target/snowfrost_fidelity_c_contract_<label>` directory.
