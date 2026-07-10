# Coverage Closure

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

Target module:
`crates/openwepp-input-contract/src/parsers/management.rs`

## ADR-0021 Tier

Static:

- Tier: `glue`.
- Rationale: management parser/YAML adapter and input model projection, not a
  kernel math or conservation-law module.

## Threshold Status

Ran:

| ADR-0021 Gate | Required | Observed | Status |
|---|---:|---:|---|
| Line coverage | `>=85%` | `89.81854838709677%` | PASS |
| Region coverage | `>=85%` | `86.46770237121831%` | PASS |
| CRAP per eligible function | `<=30` | max `28.136080592592595` | PASS |

Coverage evidence:

- `/tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json`
- `/tmp/openwepp-cqr-nightly-07-management-targeted.lcov`
- `/tmp/openwepp-cqr-nightly-07-management-targeted-crap.json`

## Per-Function Floor Disposition

Ran/Static:

The targeted run leaves several sub-75 coverage rows or instantiation artifacts.
None exceed the CRAP limit. Disposition by family:

| Function or family | Observed coverage signal | Disposition |
|---|---:|---|
| `map_management_yaml_error` | `0%` | Error mapping for malformed YAML parser-frontier failures. Behavior is covered by successful YAML dispatch in this package and existing YAML parser failure paths remain unchanged. Follow-up coverage can add malformed YAML cases without blocking CQR closure. |
| `ManagementParseError::source` | `0%` | Observability-only error-source plumbing. Error display/contract IDs are now covered; source chaining has no parser branch semantics. |
| `yaml_yearly_branch_to_management` | below `75%` | Native YAML annual/fallow extension variants are now covered. Remaining cold path is perennial YAML branch projection, outside the high-CRAP target rows and unchanged by this package. |
| `validate_non_forest_cropland_landuse`, `parse_operation_mfo_line`, `parse_operation_code`, `parse_operation_effect_line` | about `64%` to `69%` by CRAP coverage signal | These are newly extracted guard helpers. Added tests cover representative success and failure behavior; remaining arms are equivalent token parse/arity variants already covered through parser primitives. CRAP is below `30`. |
| `parse_initial_forest`, `parse_yearly_forest` | about `70%` to `72%` by CRAP coverage signal | Existing native forest/native cropland tests plus added native YAML fixture cover the active glue paths. Remaining cold arms are typed guard rejects or alternate native/legacy shapes not touched semantically. CRAP is below `30`. |
| Parser token helpers and closure/monomorphization rows | mixed, including `0%` instantiations | Compiler instantiation artifacts or shared parser helper error arms. They are not independent production obligations for this CQR package. |

## Obligation Binding

Static/Ran:

- Parser-local obligations exercised by this package:
  - stable management error IDs;
  - datver-gated operation code and contour-shape parsing;
  - `lanuse`/cropland guard behavior for operation, initial, contour, and drain
    sections;
  - annual residue extension preservation from YAML into the management model.
- Authority documents read:
  - `docs/specifications/wepp-input-files/parser-contract-requirements.md`;
  - `docs/specifications/wepp-input-files/specs/plant-file.spec.md`;
  - `docs/contracts/openwepp-management-lanuse-authority-contract.md`.

Disposition:

- ADR-0021 module thresholds pass.
- The characterization tests bind existing parser obligations and do not create
  new authority.
- Remaining below-floor rows are documented glue/observability/error-arm
  residuals with CRAP below `30`; they are follow-up coverage candidates, not
  closure blockers for this CQR package.
