# Implementation

Status: EXECUTED. Evidence mode: Static + Ran.

## Code Changes

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
  - Added `CellParameters::is_bare_skin_only()`.
  - Added exact `bare_skin_equilibrium_discharge_direct()` for the Shen-Li
    laminar and Hirsch turbulent bare-skin branch fixed points.
  - Split the old fixed-point map into
    `equilibrium_discharge_converged_iterated()` and dispatch through the
    direct evaluator only for validated bare skin-only cells.
  - Added branch-equivalence, addend-guard, and invalid inactive-operand tests.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
  - Made the implicit stepper validate `CellParameters` before each cell solve,
    aligning implicit fail-closed behavior with the explicit route wrapper.
  - Updated the warm-seed cost regression to use a non-bare fixture so it still
    exercises the generic fixed-point map.
  - Added composed solve tests for rain-term, zero-`k_o`, and near-crossover
    bare-skin cases, plus a fail-closed regression for invalid inactive raw
    operands.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
  - Updated the source-memory hybrid regression to assert branch work remains
    counted while bare-skin map work is zero.

## Protected Non-Changes

- No default/off selector behavior changed.
- No mesh-resolution policy changed.
- No publication schema, publication ownership, routing-surface ownership, or
  compatibility fallback changed.
- No tolerance weakening, surrogate physics, or empirical retuning was added.

## Focused Test Evidence

Ran:

- `cargo test -p openwepp-hillslope-orchestrator bare_skin_direct_equilibrium -- --nocapture`
  - PASS, 5 tests.
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::implicit_recession::tests::bare_skin_direct_equilibrium_composed_edge_cases_close_cell_residuals -- --exact --nocapture`
  - PASS, 1 test.
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing -- --nocapture`
  - PASS, 95 passed, 1 ignored.

Note: one attempted exact filter omitted the module path and matched zero tests;
the broader `bare_skin_direct_equilibrium` focused run included the intended
invalid inactive-operand regression.

## Line-Count Governance

Touched Rust files:

- `kinematic_wave.rs`: `2125` lines, above the 2000-line WARN threshold but
  below the 3000-line mandatory split threshold. This package adds a local
  direct evaluator and retained tests only; a mechanical split would be
  unrelated risk. Follow-on intent: split branch-equilibrium/evaluator tests
  into a local submodule when the next nontrivial routing refactor touches this
  file.
- `cascade.rs`: `1408` lines, below WARN.
- `implicit_recession.rs`: `969` lines, below WARN.
