# INT10 Kernel Profile Compliance Checklist

Status: `complete`
Evidence mode: `Static + Ran`

## Procedure/Profile Compliance

1. Canonical `SC-*` authority updated: `met`
- Evidence: `SC-PLANT-001 v8`, `SC-RESIDUE-001 v6`,
  `SC-WATBAL-001 v7`, `SC-SYSTEM-001 v3`.

2. Required algorithm/guard/invariant sections amended for changed behavior: `met`
- Evidence: INT10 invariants/guard rows and test-vector obligations in all
  affected canonical contracts.

3. Contract-derived INT10 tests implemented before any production INT10
   integration source edits: `met`
- Evidence: `tests/integration/int10_plant_water_coupling_validation_contract.rs`
  and `Cargo.toml` test registration; no production integration source edits.

4. Pre-implementation contract gate executed and recorded: `met`
- Evidence: `artifacts/int10-preimplementation-contract-gate.md`.

5. Typed failure mapping for ordering violations/missing symbols/non-finite
   coupled values is explicit and tested: `met`
- Evidence: `HS-GROWTH-E-001` and `HS-DECOMP-E-002` vectors in INT10 tests.

6. Required repository gates executed: `met`
- Evidence:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
