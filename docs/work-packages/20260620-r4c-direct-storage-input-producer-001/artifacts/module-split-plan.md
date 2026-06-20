# R4C Module Split Plan

Status: complete.
Evidence mode: Static.

R4B left `direct_runtime.rs` at 2101 lines. R4C will avoid adding another large
span to that file by moving storage-related direct-runtime code into:

```text
crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
```

Move set:

- R4B storage-reconciliation execution methods;
- R4B storage reconciliation input/state/downstream/shadow/report types;
- R4C storage-input execution methods;
- R4C storage input state/downstream/shadow/report types.

Non-goals:

- no behavior change from the split itself;
- no scheduler, compatibility runtime, or publication path movement;
- no broad direct-runtime decomposition beyond the storage slice.

Validation:

Rust focused tests, full Rust gates, no-compatibility source scan, and line-count
governance must prove the split preserved behavior and reduced the WARN risk.
