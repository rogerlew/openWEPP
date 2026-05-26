# MOFE10 Implementation and Test Evidence

Status: complete
Evidence mode: Ran

Ran:
1. `cargo test --test parser_runtime_seam_integration climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path -- --nocapture`
- Result: pass

2. `cargo test --test parser_runtime_seam_integration climate_parser_to_hillslope_runtime_surface_closure -- --nocapture`
- Result: pass

3. `cargo fmt --check`
- Result: pass

4. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: pass

5. `cargo test --workspace`
- Result: pass

6. `cargo deny check`
- Result: pass (warnings only: duplicate crates + unmatched license allowances)

7. `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe10 --policy compat`
- Result: fail at new typed blocker `HS-RUNTIME-E-050` (`oratea` domain)
