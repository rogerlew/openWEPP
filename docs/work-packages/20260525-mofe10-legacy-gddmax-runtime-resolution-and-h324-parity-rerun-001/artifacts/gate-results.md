# Gate Results

Status: complete
Evidence mode: Ran

## Contract-first gates
- `cargo test -p openwepp-hillslope-orchestrator gddmax -- --nocapture` -> fail (expected pre-implementation gate)
- `cargo test --test parser_runtime_seam_integration climate_parser_to_hillslope_runtime_surface_closure -- --nocapture` -> fail (expected pre-implementation gate)

## Post-implementation validation
- `cargo test --test parser_runtime_seam_integration climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path -- --nocapture` -> pass
- `cargo test --test parser_runtime_seam_integration climate_parser_to_hillslope_runtime_surface_closure -- --nocapture` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crate versions + unmatched license allowances)

## Parity lane execution
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe10 --policy compat` -> fail (`HS-RUNTIME-E-050` on `oratea`)
