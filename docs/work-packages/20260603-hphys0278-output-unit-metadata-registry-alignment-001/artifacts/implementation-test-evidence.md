# Implementation Test Evidence

Status: completed/HOLD
Evidence mode: ran

Static: implementation adds registry-backed output metadata validation without
changing output values or publication column names.

Ran:

- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`: pass,
  13 tests.
- `cargo test -p openwepp-hillslope-output -- --nocapture`: pass, 14 tests.
- `cargo test -p openwepp-watershed-output -- --nocapture`: pass, 4 tests.
- `tools/release/check_unit_registry.sh`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing duplicate-crate and unmatched-license
  warnings.
- `cargo test --workspace`: HOLD; two pre-existing SIMIMPL18/PL14S tests fail
  with `HKERNEL-WB11-ET-E-003`.

Clean `HEAD` reproduction:

- `cargo test --manifest-path /tmp/openwepp-head-hphys0278/Cargo.toml -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires -- --nocapture`
  failed on the same two SIMIMPL18 tests with the same `HKERNEL-WB11-ET-E-003`
  error.

Review-driven fixes validated:

- Dynamic row-level watershed `value`/`units` outputs are now modeled with
  `unit_source = "units"` metadata and explicit output-registry rows.
- Watershed writer errors now preserve typed `UnitMetadata` failures.
- Shared `validate_output_schema_unit(...)` centralizes output registry unit
  lookup and mismatch detection.
