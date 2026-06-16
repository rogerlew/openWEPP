# CQR24 Public API Surface Parity Report

Status: complete.

Static: production edits are private helper extraction in scheduler
seed/runtime paths.

Static: public API surface parity findings:

- `produce_wb16_ealpha_from_runtime_surface` signature unchanged.
- Function remains `pub(super)`.
- No new public type, public symbol, crate dependency, feature, binary target,
  or caller API added.
- Runtime publication symbols unchanged: `ofe{n}_frcteq`, `ofe{n}_alpha`,
  first-OFE `alpha`, and `ealpha`.
- Typed error enum and error variant usage unchanged.

Static: no intentional public API delta.
