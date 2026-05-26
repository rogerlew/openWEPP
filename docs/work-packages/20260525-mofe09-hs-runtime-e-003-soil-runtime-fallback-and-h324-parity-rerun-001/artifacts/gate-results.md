# Gate Results

Status: complete
Evidence mode: Ran

## Contract-first gates
- `cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_uses_measured_theta_fallback_for_7778 -- --nocapture` -> fail (expected pre-implementation gate)
- `cargo test --test parser_runtime_seam_integration parser_to_hillslope_runtime_surface_7778_measured_theta_fallback_closure -- --nocapture` -> fail (expected pre-implementation gate)

## Post-implementation validation
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crate versions + unmatched license allowances)
- `cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_uses_measured_theta_fallback_for_7778 -- --nocapture` -> pass
- `cargo test --test parser_runtime_seam_integration parser_to_hillslope_runtime_surface_7778_measured_theta_fallback_closure -- --nocapture` -> pass
- `cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_ -- --nocapture` -> pass
- `cargo test --test parser_runtime_seam_integration parser_to_hillslope_runtime_surface_ -- --nocapture` -> pass

## Parity lane execution
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe09 --policy compat` -> fail (`HS-RUNTIME-E-050`)
