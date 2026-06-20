# R2A Implementation and Test Evidence

Status: complete.
Evidence mode: Static + Ran.

Record:

- focused test commands and results;
- static search/call-graph commands and results;
- runtime counter/audit commands and results;
- release build command and binary SHA;
- default-disabled H2637 regression command and results;
- full Rust closure gates.

## Focused Tests

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`:
  PASS after review fixes, 3 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`:
  PASS after review fixes, 2 tests.

## Static Search / Call-Graph Proof

Ran:

- `rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry|HotSymbolTables|IndexedWritebackSurface|dense|dirty|build_registry_for_run" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
  returned no matches.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  returned no diff.

Static:

- The direct runtime module imports no kernel-contract compatibility types.
- Direct skeleton runner selection is outside the scheduler hot loop.
- The default runner path returns before direct skeleton construction.

## Runtime Counter / Audit Proof

Ran:

- Runner default-disabled fixture test asserted all direct skeleton audit
  counters are zero after compatibility execution.
- Runner explicit opt-in fixture test asserted one skeleton execution.

Review correction: the initial reserved forbidden-compatibility counters were
removed because they were tautological. R2A now records runtime evidence only
for direct skeleton construction and selection behavior.

## Release Build and H2637 Gate

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: PASS.
- Release binary SHA-256:
  `fe91cdce61636de56422ea18fdba7bbc6525ffee9b342236c9cc3a225cbdf45c`.
- Default-disabled H2637 reps: `634.06 s`, `636.01 s`, `640.93 s`.
- Median: `636.01 s`, PASS against `<= 676.67 s`.
- HBP/loss/WAT/plot output checksums stable across reps.
- PASS parquet DuckDB schema and decoded row equivalence passed against the
  established anchor.

## Full Rust Closure Gates

Ran:

- `cargo fmt --check`: PASS after review fixes.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS after
  review fixes.
- `cargo test --workspace`: PASS after review fixes.
- `cargo deny check`: PASS after review fixes.

Final post-review gate results are recorded in `gate-results.md`.
