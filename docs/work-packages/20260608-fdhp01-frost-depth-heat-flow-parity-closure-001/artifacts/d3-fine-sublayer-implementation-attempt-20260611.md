# D3 Fine-Sublayer Implementation Attempt — Backed Out

Evidence mode: Static + Ran.

## Scope

Codex attempted the D3 implementation pass described by
`d3-fine-sublayer-port-scope.md`: runtime fine-layer frost state
(`fgfrst`, `slfsd`, `slsic`, `slsw`, `sltime`, `yst`, `nwfrzz`), hour-1
`frwatc(1)` ingress, `frzflg` diagnostics, and fine-state aggregation back to
the existing WB18 frozen-depth/frozen-water publication path.

The implementation was not retained. Production, contract, and test edits were
backed out after the current-tree cohort showed a hard D2 conservation
regression.

## Full Cohort Result

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh current-tree 43-prefix cohort:
  `/tmp/fdhp01_d3_fine_sublayer_20260611T160601Z`.
- Compact reports copied into this package:
  - `fdhp01_d3_fine_sublayer_attempt_run_status_20260611.tsv`
  - `fdhp01_d3_fine_sublayer_attempt_execution_summary_20260611.json`
  - `fdhp01_d3_fine_sublayer_attempt_activation_summary_20260611.csv`
  - `fdhp01_d3_fine_sublayer_attempt_annual_closure_residuals_20260611.csv`
  - `fdhp01_d3_fine_sublayer_attempt_summary_20260611.json`
  - `fdhp01_d3_fine_sublayer_attempt_depth_metrics_20260611.csv`
  - `fdhp01_d3_fine_sublayer_attempt_frozwt_frdp_ratio_20260611.csv`

Result:

- Execution: `43/43` prefixes clean; WAT emitted for all 43.
- D2 additive identity regressed: max abs annual `Total-Soil + frozwt`
  residual `70.27250390582333 mm`; years 2-6 max is the same value.
- Depth stayed effectively at the D2 pinned-bound failure: open max-depth mean
  `1782.0386969356455 mm` versus legacy mean `414.22093023255815 mm`.
- Duration did not improve: mean frozen-days delta open-minus-legacy
  `-520.953488372093`, median `-520`.
- Depth correlation stayed poor: median `-0.275982943044058`.
- `frozwt/frdp` moved back toward scalar-depth behavior: median correlation
  `0.9987984851020072`.

Worst closure row:

- prefix `p22`, year `4`
- residual `70.27250390582333 mm`
- inputs `1122.8617566132743 mm`, outputs `1086.4227521379805 mm`
- `Delta(Total-Soil + frozwt) = -33.833499430529514 mm`

## Debug Probes

Ran:

- Mass-coupled helper probe on p22:
  `/tmp/fdhp01_d3_mass_probe_20260611T162003Z`.
  - Max abs annual residual `86.995866585106 mm`.
  - Max abs years 2-6 residual `86.995866585106 mm`.
- Aggregate exchange probe on p22:
  `/tmp/fdhp01_d3_exchange_probe_20260611T162312Z`.
  - Same residuals as the mass-coupled probe.
- Direct fine-liquid egress variant:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
  failed `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost`; the thaw
  diagnostic expected `8.109464696602291` and observed `0.011923545603720697`.

Disposition of probes:

- None restored the D2 additive identity.
- The direct fine-liquid egress variant also violated existing focused
  contract diagnostics.
- The failed implementation was treated as evidence, not production progress.

## Backout Verification

Static:

- Production Rust, contract, and test edits from the failed implementation
  attempt were backed out. No D3 fine-sublayer production behavior is landed by
  this artifact.

Ran:

- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`:
  pass, `19/19`, after backout.

## Disposition

FDHP01 remains `executed-hold` on D3. The next implementation attempt must
prove fine-state storage conservation first, at the internal seam:

```text
Delta(fine liquid + nwfrzz + slsic) == WB storage delta - external fluxes
```

Only after that proof should the port resume depth-resistance and thaw-front
work. A fine-layer shape that emits plausible `frdp` while regressing the
years 2-6 `Total-Soil + frozwt` identity is a hard stop.
