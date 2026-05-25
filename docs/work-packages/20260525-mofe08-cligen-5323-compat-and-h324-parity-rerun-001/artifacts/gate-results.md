# gate-results

Status: complete
Evidence mode: Ran

Pre-implementation gate:
- `cargo test -p openwepp --test infile_climate_parser_contract strict_mode_accepts_datver_5_323_and_canonicalizes_to_5_3 -- --nocapture` -> expected fail (pre-impl)

Validation gates:
- `cargo test -p openwepp --test infile_climate_parser_contract` -> pass
- `cargo test -p openwepp --test infile_climate_parser_contract --test infile_slope_parser_contract --test infile_soil_parser_contract` -> pass
- `cargo clippy -p openwepp-input-contract -- -D warnings` -> pass
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe08 --policy compat` -> fail (`HS-RUNTIME-E-003`)

Static:
- Workspace-wide `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` were not run in MOFE08.
