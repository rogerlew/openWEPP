# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static: `docs/specifications/unit-governance.md` now requires output schemas
that attach `units` metadata to resolve `(schema_id, column_name)` through the
output-unit registry. Boundary-backed rows must cross-check the
boundary-symbol registry; publication-only rows require explicit rationale.
Dynamic row-level unit outputs must publish `unit_source = "units"` metadata
and resolve the numeric value column through the output-unit registry.

Implemented authority surface:

- `OutputUnitAuthority`
- `OutputUnitEntry`
- `OutputUnitRegistry`
- `canonical_output_unit_entries()`

Ran: not applicable for this artifact; contract-derived tests are recorded in
`contract-test-implementation-evidence.md`.
