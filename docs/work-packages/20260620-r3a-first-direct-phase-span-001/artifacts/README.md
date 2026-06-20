# R3A Artifacts

Status: complete.
Evidence mode: Static + Ran.

R3A implemented the first complete direct-runtime phase span:
`DirectPhaseKind::Normalization -> DirectPhaseKind::LateralTransfer`.

Key evidence:

- phase span: direct transfer-input accounting;
- implementation: `direct_runtime.rs` direct types, compute, state mutation,
  downstream operands, shadow projection, and counters;
- focused tests: orchestrator R3A/R2A filters and runner R2A opt-in/default
  filters pass;
- no-compatibility proof: direct runtime forbidden-token scan returned no
  matches and `scheduler.rs` has no diff;
- default-disabled H2637 gate: `630.31/640.85/632.08 s`, median `632.08 s`
  against threshold `<= 676.67 s`;
- protected identity: HBP, loss, WAT, and plot checksums stable; PASS parquet
  schema/row equivalence passed by DuckDB;
- closure gates: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`,
  scoped markdown lint, and `git diff --check` passed.

R3A does not claim R4 hydrology-path migration, R6 publication cutover,
endpoint improvement, or default activation readiness.
