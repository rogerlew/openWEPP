# CQR25 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in runner intake
and lane setup paths. No public API change is authorized.

Static: observed public API parity:

- `execute_hillslope_run` remains `pub` with the same signature and return
  type.
- No manifest schema identifier changed.
- No public request or report type changed.
- No output writer public API changed.
- No typed error enum variant was added, removed, or renamed.

Static: one crate-private helper type was re-exported within
`openwepp-runner`:

```rust
pub(crate) use lane_setup_helpers::StaticOfeLaneSlice;
```

Static: that re-export is not public API. It lets the parent hillslope runner
module name the existing internal slice type while preserving the helper module
ownership boundary.
