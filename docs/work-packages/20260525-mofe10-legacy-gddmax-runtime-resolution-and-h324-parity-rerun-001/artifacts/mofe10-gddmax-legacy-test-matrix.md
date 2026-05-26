# MOFE10 Legacy `gddmax` Test Matrix

Status: complete
Evidence mode: Ran

| Test command | Pre-implementation | Post-implementation |
| --- | --- | --- |
| `cargo test -p openwepp-hillslope-orchestrator gddmax -- --nocapture` | fail (`HS-RUNTIME-E-050` sentinel rejection) | pass |
| `cargo test --test parser_runtime_seam_integration climate_parser_to_hillslope_runtime_surface_closure -- --nocapture` | fail (missing monthly projection symbols) | pass |
| `cargo test --test parser_runtime_seam_integration climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path -- --nocapture` | fail (missing `hs*_obmaxt_*` parity symbols) | pass |
