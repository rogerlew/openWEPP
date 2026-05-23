# INT10 Verification Agent A

Status: `complete`
Evidence mode: `Ran`

Commands executed:

```bash
cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture
cargo test --test int10_plant_water_coupling_validation_contract
cargo test -p openwepp-hillslope-orchestrator annual_growth_phase_emits_typed_growth_context -- --nocapture
cargo test -p openwepp-hillslope-orchestrator perennial_growth_phase_emits_typed_growth_context -- --nocapture
cargo test --test parser_runtime_seam_integration management_rotation_fixture_projects_schedule_growth_and_decomp_runtime_surface_families -- --nocapture
```

Result:

- INT10 target: `3 passed`, `0 failed`.
- Targeted growth typed-context checks: `1 passed` each.
- Targeted runtime projection coupling check: `1 passed`.
