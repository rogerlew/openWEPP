# Gate Results

Status: complete
Evidence mode: Ran

Pre-implementation gates:
- `cargo test -p openwepp --test infile_slope_parser_contract compatibility_mode_accepts_shared_geometry_multi_ofe_form -- --nocapture` -> expected fail (pre-impl)
- `cargo test -p openwepp --test infile_soil_parser_contract compatibility_accepts_quoted_7778_soil_header_form -- --nocapture` -> expected fail (pre-impl)
- `cargo test -p openwepp --test infile_soil_parser_contract compatibility_accepts_quoted_7778_with_per_ofe_restrictive_rows -- --nocapture` -> expected fail (pre-impl)

Validation gates:
- `cargo test -p openwepp --test infile_slope_parser_contract --test infile_soil_parser_contract` -> pass
- `cargo clippy -p openwepp-input-contract -- -D warnings` -> pass
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe07 --policy compat` -> fail at climate parse (`unsupported datver '5.323'`), slope/soil parse blockers no longer active.

Static:
- Workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` and
  `cargo deny check` were not run in MOFE07.
