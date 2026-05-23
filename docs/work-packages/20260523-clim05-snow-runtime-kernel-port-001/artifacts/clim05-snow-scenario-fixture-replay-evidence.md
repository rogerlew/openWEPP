# CLIM05 Snow-Scenario Fixture Replay Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Fixture Corpus Used

Location: `tests/fixtures/infile/snow/`

Primary CLIM05 fixture scenarios used in replay:
- `strict_valid.txt`
- missing file path (`does_not_exist.txt`) for default inactive branch
- strict invalid fixtures exercised via parser contract tests:
  - `strict_nonpositive_density_invalid.txt`
  - `strict_nonfinite_invalid.txt`
  - `strict_non_numeric_invalid.txt`
  - `strict_missing_record_invalid.txt`
  - `strict_surplus_records_invalid.txt`
  - `strict_trailing_tokens_invalid.txt`
  - `prefixed_variant_rejected.txt`

## Replay Commands (Ran)

1. `cargo test --test infile_snow_parser_contract`
- result: pass (`12 passed`)
- replay confirms strict/compat parser behavior and typed error/warning posture across fixture corpus.

2. `cargo test --test parser_runtime_seam_integration snow_`
- result: pass (`3 passed`)
- replay confirms parser-to-runtime projection closure and active/inactive control projection behavior.

## Observed CLIM05 Scenario Outcomes

| Scenario | Evidence path | Observed outcome |
|---|---|---|
| Strict valid sidecar present (`strict_valid.txt`) | `snow_parser_to_hillslope_runtime_surface_closure` | `snow.options.rst=0`, `snow.options.newsnw=100`, `snow.options.ssd=250`, `snow.options.snow_file_present=1`, `snow.runtime_swe=0` |
| Missing sidecar file | `snow_runtime_surface_projects_missing_file_flag_as_inactive` | `snow.options.snow_file_present=0` (inactive coupling posture) |
| Domain-invalid density projection (`newsnw > ssd`) | `snow_runtime_surface_rejects_invalid_density_domain_projection` | typed runtime seam failure `HS-RUNTIME-E-053` |

## Replay Interpretation

- Snow-sidecar fixture replay now closes the runtime seam authority path from parsed `snow.txt` controls into CLIM05-consumable hydrology symbols.
- Inactive missing-file behavior remains explicit and typed (no implicit activation).
- Domain-invalid controls are rejected before kernel execution path consumption.
