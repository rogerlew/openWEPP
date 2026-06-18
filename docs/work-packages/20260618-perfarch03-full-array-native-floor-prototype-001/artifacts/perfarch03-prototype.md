# PERFARCH03 Prototype

Evidence class: Static + Ran.

Status: complete 2026-06-18.

## Prototype location

The prototype is an artifact-local Cargo package:

- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/`

It is explicitly outside the production workspace (`[workspace]` in its local
`Cargo.toml`). It depends on the local openWEPP crates only to reuse the public
kernel request, symbols, registry, and boundary-value types for setup and
identity validation.

## What was built

The harness constructs one representative WB11 warm-rain runoff flow with no
snow, no frost, no irrigation, and inactive MOFE. That branch is the PERFARRAY02
anchor and is the package's requested hot-path flow.

The harness runs two paths:

- the current public production WB11 runoff kernel, through
  `Wb11HydrologyKernel::run` and `HillslopeKernelRequest`;
- a copied array-native branch implementation that reads dense inputs, computes
  the same branch formulas, and writes dense outputs by `SymbolId`.

The dense output surface is `Vec<Option<BoundaryValue>>` for state and flux
slots. Output slot lookup is resolved once from the production writeback
symbols, then reused inside the timed array loop.

## Hot-loop boundary

The measured array hot loop is intentionally narrower than the whole harness:

- `array_runoff_physics` performs the branch physics and guards.
- `OutputPlan::write_outputs` writes the same production output symbol set into
  dense state/flux slots by resolved `SymbolId`.
- `time_array_hot_loop` repeats those operations and records the floor.

Logical objects remain in setup, production-baseline execution, validation, and
one-shot materialization only. They are not part of the array floor loop.

## Scope limit

This is a branch-floor prototype, not a production migration. It validates and
measures the warm-rain runoff branch with the same production output write set
for that branch. It does not port the remaining H2637 branches, phases, or
runtime orchestration to array-authoritative production code.

That limitation is deliberate: PERFARCH03 asks whether the true array-native
floor is viable before committing to the broad migration.

## Commands run

```bash
cargo fmt --manifest-path docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/Cargo.toml
cargo check --manifest-path docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/Cargo.toml
cargo clippy --manifest-path docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/Cargo.toml -- -D warnings
cargo build --release --manifest-path docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/Cargo.toml
```

All four commands passed.
