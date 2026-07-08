# Required Reading Map

Status: executed.
Evidence mode: Static.

## Budget

Local required-reading byte total: `421001` bytes.

Disposition: `WARN` because `docs/work-packages/README.md` is required by the
package-preparation standard and is large. The budget is below
`REQUIRES-JUSTIFICATION`; no heavy pre-read justification is required.

The thresholds from `docs/standards/kernel-work-package-preparation.md` are:

- `OK`: `<=400000` bytes.
- `WARN`: `>400000` bytes.
- `REQUIRES-JUSTIFICATION`: `>800000` bytes.

## Core

| Path | Rationale |
| --- | --- |
| `AGENTS.md` | Root repository governance and package authority requirements. |
| `docs/work-packages/AGENTS.md` | Work-package scaffolding, gates, review, verification, and truthfulness rules. |
| `docs/work-packages/README.md` | Package catalog update target. |
| `docs/standards/AGENTS.md` | Standards and prompt wording routing. |
| `docs/standards/prompt-wording-guidance.md` | Required kickoff prompt shape and required-reading budget rule. |
| `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | Target user-facing specification. |
| `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` | Canonical parser-contract authority for `.man` datver, native landuse, and routing extension behavior. |
| `docs/contracts/openwepp-management-lanuse-authority-contract.md` | Interface authority for native `lanuse` operands and no-legacy-field-inference rules. |
| `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/package.md` | Package-local objective, scope, gates, and exit criteria. |

## On Demand

| Path | Trigger |
| --- | --- |
| `crates/openwepp-input-contract/src/parsers/management.rs` | Check exact parser branch behavior or marker names. |
| `tests/integration/infile_management_parser_contract.rs` | Check executable examples and failure modes. |
| `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man` | Check native forest file layout. |
| `tests/fixtures/disturbed_native_route_coefficients/p1.man` | Check native cropland plus route-coefficient extension layout. |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Check Lane D routing-consumer wording if the spec names routing activation rules. |
