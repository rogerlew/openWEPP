# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Static + Ran.

## Added Test

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
  - `fq4_contract_default_frost_controls_activate_without_frost_sidecar_presence`

The test starts from a surface with `frost_file_present=0`, sets
`frost.options.wintRed=1`, and requires:

- `frost.runtime_dfrost > 0`
- `frost.runtime_ws_frz > 0`
- `frost.runtime_infcap_frz < ssc`

## Adjusted Tests

- `seeded_clim06_surface(false)` now sets `wintRed=0` for intentional inactive
  frost paths. Inactive frost is controlled by the frost control, not by
  file-presence provenance.
- HPHYS0319 and HPHYS0320 contract-version assertions were updated from
  `SC-SNOWFREEZE-001` v52 to v53.

## Ran

- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
  passed (`12 passed`).
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture`
  passed (`5 passed`).
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture`
  passed (`3 passed`).
