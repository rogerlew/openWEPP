# Disposition

Status: completed/HOLD
Evidence mode: mixed

Static: HPHYS0278 completed output-unit metadata registry alignment. Focused
implementation gates pass, review findings are dispositioned, and dual
verification is complete. Package remains HOLD only because full workspace
testing is blocked by the pre-existing SIMIMPL18/PL14S
`HKERNEL-WB11-ET-E-003` failure reproduced on clean `HEAD`.

## Closure Summary

- Added output-unit registry authority in `openwepp-sim-contract`.
- Validated hillslope WAT schema unit metadata against registry authority.
- Validated watershed output schema unit metadata against registry authority.
- Modeled dynamic row-level watershed loss outputs with `unit_source = "units"`
  and registry rows for the numeric `value` fields.
- Preserved output values and publication column names.

## Review Disposition

| ID | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| A-1 | Medium | accepted/resolved | `WatershedWriterError` is typed and preserves `UnitMetadata` failures. |
| A-2 | Medium | accepted/resolved | `validate_output_schema_unit(...)` centralizes output schema unit checks. |
| B-1 | Medium | accepted/resolved | Dynamic loss `value` columns use row-level `units` source metadata and output-registry rows. |
| B-D1 | Follow-up | accepted/resolved | Resolved by A-1 typed watershed writer errors. |
| B-D2 | Follow-up | accepted/resolved | Negative coverage rejects missing publication-only contract and invariant authority. |

No review finding remains undispositioned. No deferred follow-up is required for
HPHYS0278 scope.

## Gate Disposition

Ran:

- Focused HPHYS0278 registry tests: pass.
- Hillslope output crate tests: pass.
- Watershed output crate tests: pass.
- Unit-registry release gate: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing duplicate-crate and unmatched-license
  warnings.
- `cargo test --workspace`: HOLD on the two known SIMIMPL18/PL14S
  `HKERNEL-WB11-ET-E-003` failures only.

Final posture: completed/HOLD.
