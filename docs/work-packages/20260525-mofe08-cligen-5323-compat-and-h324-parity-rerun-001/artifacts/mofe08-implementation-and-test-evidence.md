# MOFE08 Implementation and Test Evidence

Status: complete
Evidence mode: Ran

Commands executed:
1. Pre-implementation gate:
- `cargo test -p openwepp --test infile_climate_parser_contract strict_mode_accepts_datver_5_323_and_canonicalizes_to_5_3 -- --nocapture`
- Result: expected fail (`UnsupportedDatver 5.323`).

2. Post-implementation climate target:
- `cargo test -p openwepp --test infile_climate_parser_contract`
- Result: pass (`16 passed`).

3. Regression parser targets:
- `cargo test -p openwepp --test infile_climate_parser_contract --test infile_slope_parser_contract --test infile_soil_parser_contract`
- Result: pass (`16 + 18 + 11`).

4. Lint gate on touched parser crate:
- `cargo clippy -p openwepp-input-contract -- -D warnings`
- Result: pass.

5. MOFE parity lane rerun:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe08 --policy compat`
- Result: fail at downstream runtime soil surface requirement (`HS-RUNTIME-E-003`).
