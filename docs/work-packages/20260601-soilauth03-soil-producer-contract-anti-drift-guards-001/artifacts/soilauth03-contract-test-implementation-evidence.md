# SOILAUTH03 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
Contract-derived anti-drift tests for required `.sol` obligations and fixture
integrity.

## Added/Updated Test Surfaces
Static:
- `tests/integration/soilauth03_soil_producer_contract_drift_guards_contract.rs`
  - validates required suite registration and obligation-map presence;
  - validates required symbol coverage across spec + parser contract;
  - validates canonical datver structure/order/arity envelopes from
    machine-readable obligations;
  - validates required-case bindings and hard-fail registry posture;
  - validates fixture lock/provenance schema posture;
  - executes injected red-state drift checks:
    - invalid header-arity obligation injection,
    - tampered `fixtures.sha256` lock entry.
- `Cargo.toml`
  - added explicit integration test target:
    `soilauth03_soil_producer_contract_drift_guards_contract`.

Ran:
- `cargo test --test soilauth03_soil_producer_contract_drift_guards_contract`
  - pass.
