# Erod15 runtime route integration plan

Status: complete
Evidence mode: mixed

## Static
- Integration surface implemented in runner crate:
  - binary target `openwepp-cli-watershed`
  - source `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  - delegated writer crate `crates/openwepp-watershed-output/`
- Invocation contract:
  - required args: `--run-dir`, `--run-file`, `--output-dir`
  - optional args: `--policy strict|compat`, `--legacy-sidecar-discovery`
  - required `.run` inputs: `inputs.pw0_str`, `inputs.pw0_chn`,
    `inputs.pw0_imp`, `inputs.pw0_man`, `inputs.pw0_slp`, `inputs.pw0_cli`,
    `inputs.pw0_sol`, `inputs.hillslopes_block`
  - optional `.run` sidecars: `inputs.chaninp`, `inputs.tcr`
  - required `.run` outputs:
    `outputs.ebe_pw0`, `outputs.chan_out`, `outputs.chanwb`,
    `outputs.chnwb`, `outputs.soil_pw0`, `outputs.totalwatsed3`,
    `outputs.loss_hill`, `outputs.loss_chn`, `outputs.loss_out`,
    `outputs.loss_class_data`, `outputs.loss_all_years_hill`,
    `outputs.loss_all_years_chn`, `outputs.loss_all_years_out`,
    `outputs.loss_all_years_class_data`
- Execution sequence:
  1. parse and validate watershed `.run` file and path surfaces,
  2. parse/validate topology + watershed inputs + hillslope HBP payload
     boundaries,
  3. seed runtime state/flux surfaces (including optional sidecar overlays),
  4. dispatch `Ws10ChannelImpoundmentKernel`,
  5. emit interchange parquet outputs through
     `openwepp-watershed-output::writers`.
  6. current writer behavior is typed hard-fail (`OWSOUT-E-004`) until
     data-backed watershed row publication is implemented.

## Ran
- Coverage validation via integration suites:
  - `cargo test --test cli01_runner_hillslope_integration --test cli03_runner_contract_derived_tests` -> PASS.
  - `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> PASS.
