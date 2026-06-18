# PERFARRAY01 Contract Shell

Evidence class: Static + Ran.

## Scope Landed

Stage A landed an inert array-authoritative contract shell in
`openwepp-kernel-contract`:

- `crates/openwepp-kernel-contract/src/lib_mod/array_hot_state.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/mod.rs`

The shell adds:

- `ArrayHotState`: dense state/flux slots keyed by `SymbolId`;
- `ArrayWritebackField` and `ArrayWritebackPayload`: id-backed writeback
  fields with value and finite/range bounds;
- `evaluate_array_writeback`: id-backed finite/domain evaluator preserving the
  current writeback accept/reject message-id classes;
- `apply_array_writeback`: id-backed apply into dense slots with id-only
  success-path apply result;
- logical materialization via `ArrayHotState::export_btreemap_surfaces`.

The code is default-unwired. No scheduler, runner, CLI, HBP/parquet writer, or
kernel phase dispatch path calls it yet.

## Stage A Test Evidence

Ran:

```text
cargo fmt --check -p openwepp-kernel-contract
cargo check -p openwepp-kernel-contract
cargo test -p openwepp-kernel-contract
cargo clippy -p openwepp-kernel-contract --all-targets -- -D warnings
```

Focused array tests included in `array_hot_state.rs`:

- dense state round-trips logical state/flux surfaces;
- id-backed accept decision matches `evaluate_kernel_writeback`;
- id-backed rejection preserves logical message-id class and violation subject;
- id-backed apply exports the same logical maps as current logical apply.

The accept/reject tests wrap `evaluate_array_writeback` with the existing
symbol-registry audit and assert zero new `BoundarySymbol` construction during
evaluation. That verifies the id-backed evaluator does not rebuild logical
symbols on the success path and resolves logical names only for diagnostics.

## Default Path

The default path is unchanged by construction: Stage A added exported types and
tests only. There is no production call site from the scheduler or runner into
the new module. Therefore flag-off/default execution remains byte-identical by
static write-set: no default execution branch changed.

## Stage A Limits

This shell is not a WB11 integrated pilot. It does not change
`HillslopeKernelRequest`, scheduler authority, output publication, or any kernel
phase. It only supplies the contract surface required for a later honest pilot.
