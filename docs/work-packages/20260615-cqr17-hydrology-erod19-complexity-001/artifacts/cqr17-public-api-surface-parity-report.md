# CQR17 Public API Surface Parity Report

Status: closed.

Static: planned production edits are private helper extraction in hydrology
phase code. No public API change is authorized.

Static: actual production edits added private file-local structs and private
associated helper functions only. No `pub`, `pub(crate)`, exported type,
dependency, trait implementation, binary interface, parser entrypoint, or
runtime publication symbol was added, removed, or renamed.

Static: preserved crate-visible method:

```rust
Wb11HydrologyKernel::erod19_xcrit_classification(
    a: f64,
    b: f64,
    c: f64,
    tauc: f64,
    xb: f64,
    xe: f64,
) -> (f64, f64, f64)
```

Static: no change to `run_erod19_route_segment_migration` call sites or
`WritebackField` output symbols.
