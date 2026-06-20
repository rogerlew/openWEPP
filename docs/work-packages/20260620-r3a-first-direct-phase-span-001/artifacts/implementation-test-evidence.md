# R3A Implementation and Test Evidence

Status: complete.
Evidence mode: Static + Ran.

## Focused Tests

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`:
  PASS, 3 tests.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`:
  PASS, 3 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`: PASS, 2 tests.

## Static Search / Call-Graph Proof

Ran:

- Forbidden direct runtime source scan returned no matches:
  `rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry|HotSymbolTables|IndexedWritebackSurface|dense|dirty|build_registry_for_run" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  returned no diff.

## Runtime Counter / Audit Proof

Ran:

- Orchestrator R3A unit test asserted exact span report counters:
  phase entries `2`, direct compute `1`, state mutation `1`, downstream
  operand `1`, shadow projection `1`, compatibility edge invocations `0`.
- Runner explicit opt-in fixture increments the production
  compatibility-edge handoff counter to `1`, proving the field is not an
  always-zero placeholder while the direct span itself records zero
  compatibility edges.
- Runner default-disabled fixture asserted all direct runtime counters are
  zero.
- Runner explicit opt-in fixture asserted direct span counters are positive and
  exactly one compatibility-edge handoff is recorded at the opt-in return to
  compatibility publication.

## Release Build and H2637 Gate

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: PASS,
  `56.38 s`.
- Release binary SHA-256:
  `d55aa166376ccaeec51db3ef507a0fa15ffbb0b7f73a74c651e203a04d48b60e`.
- Final release sidecar SHA-256:
  `bb177b4f8000a79dbabd41a12fa66fde867e02887ba9bf86bf94c5e48806bb32`.
- Default-disabled H2637 reps: `630.31 s`, `640.85 s`, `632.08 s`.
- Median: `632.08 s`, PASS against `<= 676.67 s`.
- HBP/loss/WAT/plot output checksums stable across reps.
- PASS parquet DuckDB schema and decoded row equivalence passed against the
  established anchor.

## Full Rust Closure Gates

Ran:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS after
  numeric-literal separator fix.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
