# HPHYS0241 Gate Results

Status: complete
Evidence mode: ran

Ran:

- `cargo fmt --check` — pass.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `cargo test --workspace` — pass.
- `cargo deny check` — pass. Output included existing warnings for duplicate
  crate versions (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license
  allowances (`ISC`, `Unicode-DFS-2016`); advisories, bans, licenses, and
  sources all reported `ok`.
- `bash tools/release/check_authority_suite_antievasion.sh` — pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` — pass.

Ran targeted pre-full-suite checks:

- `cargo test --test wb11_hydrology_kernel_contract hphys0241` — pass.
- `cargo test --test cli03_runner_contract_derived_tests hphys0241` — pass.
- `cargo test --test mofe04_publication_contract_authority_closure_contract` — pass.
- `cargo test --test mofe05_watershed_contributor_metadata_contract_authority_closure_contract` — pass.
- `cargo test --test cli03_runner_contract_derived_tests mofe04` — pass.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_mofe05` — pass.

Static: all commands were executed locally from `/home/workdir/openWEPP`.
