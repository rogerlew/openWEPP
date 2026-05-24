# CLI03 Preimplementation Contract Gate

Status: pass
Evidence mode: Static + Ran

## Static
Gate objective: verify CLI03 contract-first sequencing before runner/CLI
production implementation edits.

Checklist:
- [x] CLI03 canonical authority references are available and contract-derived test
      expectations are encoded.
- [x] Contract-derived tests are implemented and wired in workspace metadata.
- [x] Dedicated output crate test surface exists under
      `crates/openwepp-hillslope-output/` with crate-owned tests.
- [x] Pre-implementation test execution evidence is captured before any
      production runner/CLI implementation edits for CLI03 behavior.

## Ran
- Command:
  - `cargo test -p openwepp-hillslope-output`
- Observed:
  - pass (`11 passed; 0 failed`).

- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - fail (`4 passed; 3 failed`).
- Failure signatures (expected pre-implementation signal):
  - missing runner wiring to `openwepp-hillslope-output`,
  - `.run` non-metric unit and unresolved required input path cases currently
    execute successfully instead of hard-failing at the contract boundary.

Gate decision:
- `PASS` for completion of CLI03 Phase B sequencing gate.
- Phase C production implementation is required to close the three failing
  CLI03 contract-derived tests.
