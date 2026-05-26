# MOFE11 Legacy `oratea/orater` Test Matrix

Status: complete
Evidence mode: Ran

| Test command | Pre-implementation | Post-implementation |
| --- | --- | --- |
| `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay` | fail (`PlProjectionFieldOutOfDomain`, allowed `>0.0`) | pass |
| `cargo test -p openwepp --test parser_runtime_seam_integration pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero` | fail (decomposition transition not successful) | pass |
| `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_rejects_negative_oratea_projection_field` | pass | pass |
| `cargo test -p openwepp-hillslope-orchestrator decomposition_boundary_rejects_negative_oratea_with_typed_failure` | pass | pass |
