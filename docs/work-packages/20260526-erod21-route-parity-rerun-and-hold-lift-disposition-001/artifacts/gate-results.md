# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `cargo fmt --check`
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`

## Result
- `cargo fmt --check`: pass (`cargo_fmt_check.exit_code=0`)
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`: pass (`cargo_clippy.exit_code=0`)
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`: pass (`cargo_test_erod14.exit_code=0`)
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`: pass (`cargo_test_cli03_mofe03.exit_code=0`)

## Evidence bundle
- `artifacts/gates-20260526T210655Z/`
