# PERFARCH02 Floor Prototype

Evidence class: Ran locally on 2026-06-18.

## Location

The prototype is an artifact-local standalone Cargo package:

`docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/`

It declares its own `[workspace]`, so it is not part of the production Rust
workspace. It depends on the local `openwepp-kernel-contract` and
`openwepp-sim-contract` crates to reuse real contract types and writeback
status semantics.

## What It Measures

The harness compares two implementations of the same bounded writeback/guard
surface:

- `logical_current`: real `evaluate_kernel_writeback` plus
  `apply_kernel_writeback` over `BTreeMap<BoundarySymbol, BoundaryValue>`.
- `array_authoritative`: pre-resolved `SymbolId` fields, sorted once by id,
  finite/range checks over dense array slots, then direct array update.

The synthetic surface is deliberately representative of the current seam rather
than a full WB11 port:

- 3,072 state symbols;
- 1,024 flux symbols;
- 96 bounded state updates per iteration;
- 64 bounded flux updates per iteration;
- 200,000 timed iterations;
- 5 repeats.

The H2637 projection uses 235,961 OFE-days and the PERFIDX06 same-machine
legacy no-UI median of 9.12 seconds.

## Correctness Checks

The prototype performs two correctness checks before timing:

- success-path identity: apply the same payload through the current logical
  path and through the array path, export the array to maps, and require exact
  equality of state and flux maps;
- failure-path parity: submit invalid non-finite/domain payloads and require
  the array evaluator to reject, preserve the current message id class, and
  resolve lazy diagnostic subjects.

These checks make the measured array path a correct replacement for the
prototyped writeback/guard flow, not a stripped benchmark.

## Limitations

This is not a full hydrology-kernel migration. It does not execute WB11 runoff,
frost, plant, lateral drainage, erosion, storage, or publication code. It
measures the blocked representation class that PERFIDX03 and PERFIDX05 exposed:
string-keyed writeback application and guard evaluation.

The right use of this result is a CONDITIONAL GO to an integrated WB11 pilot,
not a direct production migration.

## Commands Run

```text
cargo fmt --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml
cargo check --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml
cargo run --release --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml
cargo clippy --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml -- -D warnings
```

The release timing output is recorded in `perfarch02-floor-prototype.tsv`.
