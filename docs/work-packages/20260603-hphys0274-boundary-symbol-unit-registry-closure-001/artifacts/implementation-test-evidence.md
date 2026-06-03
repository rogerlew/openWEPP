# Implementation Test Evidence

Status: completed
Evidence mode: static-and-run

Static: Implemented `BoundaryUnitRegistry`, `BoundaryUnitEntry`, dimension/domain
classes, typed-boundary posture, validation errors, canonical entries, exact
alias lookup, template alias lookup, and required-alias gate in
`crates/openwepp-sim-contract/src/units.rs`.

Ran: `cargo test --test sim_contract_boundary_unit_registry` passed before final
artifact disposition. Final command output is recorded in `gate-results.md`.

## Implementation Notes

- Exact aliases use deterministic `BTreeMap` reverse lookup.
- Template aliases support `{idx4}` for four-digit suffixes and `{ofe}` for
  OFE-scoped aliases.
- Validation rejects empty fields, duplicate canonical rows, duplicate alias
  rows, ambiguous aliases, unsupported template tokens, missing dimensional
  units, and scalar exceptions without reasons.
- `prcp` and WAT `P` are separate rows because the runtime climate seam uses
  meters while the WAT publication column uses millimeters.
- `stmdur` and `timem_####` are seconds at the runtime seam; `stmstr` remains
  an hour-of-day marker.
- WB13 runtime profile symbols are registered alongside WAT profile publication
  columns.
- Cross-unit runtime rows no longer own WAT publication aliases; publication
  aliases are unique and validator-checked.
- Dimensional symbols that still use `BoundaryValue::scalar` are registered as
  `FollowUpRequired`; scalar exceptions are limited to fractions, counters, and
  control-like unitless surfaces with reasons.
