# CQR21 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction inside the
shared climate runtime adapter. No public API change is authorized.

Static: completed production edits only changed private formatting internals in
`lib.rs` and added a private `fmt_message` helper.

Static: no public Rust item, crate export, enum variant, error code mapping,
parser type, runtime symbol, alias, unit, serialization surface, formula, or
float expression order was added, removed, or renamed.

Static: focused characterization validates stable error IDs and display strings
for every `SharedClimateRuntimeInputError` variant.
