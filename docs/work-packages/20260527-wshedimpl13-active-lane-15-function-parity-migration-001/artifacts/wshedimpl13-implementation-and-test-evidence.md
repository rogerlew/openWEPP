# WSHEDIMPL13 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented runtime projection of WS12 outflow function families
  (`ws10_impoundment_{id}_f01..f15_{a,b,c,d,e,ha}`) from parser-exported
  active `.imp` branch payloads in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`.
- Implemented production kernel WS12 outflow composition using
  `min(qo1,qo2,qo3) + min(qo4,qo5,qo6) + min(qo7,qo8,qo9) + qo10 + qo11 + qo12 + min(qo13,qo14,qo15)`
  in `crates/openwepp-watershed-orchestrator/src/lib.rs`.
- Added integration vector validating `qo` equality against full function
  composition in
  `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`.
- Resolved strict `clippy -D warnings` failures introduced by the migration by:
  - replacing large coefficient-struct pass-by-value with borrowed references
    in WS12 helper call paths,
  - normalizing lint-sensitive naming/domain-reporting expressions,
  - adding targeted lint-allow annotations only where mathematically explicit
    test/solver form is intentional.

## Ran
- `cargo fmt --all`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_projects_active_structure_coefficients -- --nocapture`
- `cargo test --workspace wshed13_contract_ws12_vector_uses_full_min_controller_outflow_composition -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
