# R3B Owned File Manifest

Status: complete.
Evidence mode: Static + Ran.

Touched Rust files:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: R3B constants,
  ledger state, operands, shadow projection, execution, validation, and counter
  aggregation.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public exports for R3B
  constants and types.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`:
  R3B identity, invalid-input, signed-residual, and aggregate-counter tests.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: explicit opt-in/default
  counter assertions updated for R3A+R3B.

Touched docs:

- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, or science-contract file was
touched.

