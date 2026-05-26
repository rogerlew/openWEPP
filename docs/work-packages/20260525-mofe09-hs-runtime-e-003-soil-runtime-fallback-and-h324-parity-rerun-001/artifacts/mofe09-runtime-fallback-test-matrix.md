# MOFE09 Runtime Fallback Test Matrix

Status: complete
Evidence mode: Ran

| Test command | Pre-implementation | Post-implementation |
| --- | --- | --- |
| `cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_uses_measured_theta_fallback_for_7778 -- --nocapture` | fail (`MissingThetaResidual`) | pass |
| `cargo test --test parser_runtime_seam_integration parser_to_hillslope_runtime_surface_7778_measured_theta_fallback_closure -- --nocapture` | fail (`MissingThetaResidual`) | pass |
| `cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_ -- --nocapture` | not run | pass (4/4) |
| `cargo test --test parser_runtime_seam_integration parser_to_hillslope_runtime_surface_ -- --nocapture` | not run | pass (6/6) |
