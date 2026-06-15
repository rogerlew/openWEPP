# CQR10 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in an included
runtime-input module file. No public API change is authorized.

Static: public functions retained:

- `build_hillslope_runtime_surface_from_irrigation_fixeddate`
- `seed_hillslope_runtime_surface_from_irrigation_fixeddate`

Static: newly introduced items are private to the included runtime-input module:

- `FixedDateProjectionState`
- `validate_fixeddate_irrigation_header`
- `seed_fixeddate_irrigation_header_symbols`
- `seed_fixeddate_irrigation_events`
- `seed_fixeddate_irrigation_event`
- `seed_fixeddate_irrigation_event_schedule`
- `seed_fixeddate_irrigation_sprinkler_event`
- `seed_fixeddate_irrigation_furrow_event`
- `fixeddate_event_next_record`

Static: no dependency, feature, crate export, CLI, serialization schema, parser
API, symbol alias registry, error enum variant, or public type change was made.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace` both exited `0`.

Conclusion: public API surface parity is preserved.
