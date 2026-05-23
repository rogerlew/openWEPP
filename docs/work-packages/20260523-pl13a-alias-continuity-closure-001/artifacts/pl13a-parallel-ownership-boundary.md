# PL13A Parallel Ownership Boundary

Status: `complete`
Evidence mode: `Static`

## Authorized PL13A-Owned Surfaces

- `docs/specifications/science-contracts/symbol-alias-registry.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` alias-map
  sections
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `tests/integration/sim_contract_symbol_alias_registry.rs`
- `docs/work-packages/20260523-pl13a-alias-continuity-closure-001/**`

## PL13-Owned Surfaces Explicitly Not Edited

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`

Static evidence:
- `git diff --name-only -- crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-kernel-contract/src/lib.rs`
- Output: no changed files for those paths.

## Boundary Conclusion

PL13A and PL13 ownership boundaries remained disjoint for this execution.
