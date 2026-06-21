# Owned File Manifest

Status: complete.

## Package Artifacts

| File | Edit class | In-scope rationale | Validation tied to edit |
|---|---|---|---|
| `docs/work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/**` | Package artifacts | Required R6F execution evidence, hold audit, review, verification, and handoff. | `wctl doc-lint --path docs/work-packages` passed. |
| `docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/**` | Follow-on scaffold | Required by R6F hold legitimacy checklist. | Scaffold shape reviewed; `wctl doc-lint --path docs/work-packages` passed. |
| `docs/work-packages/README.md` | Catalog update | Active/held package pointer. | `wctl doc-lint --path docs/work-packages` passed. |

## Execution Edits

| File | Edit class | In-scope rationale | Validation tied to edit |
|---|---|---|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | Direct runtime structure | Add typed publication process input slots, persistent layer carry, and near-zero HBP runoff operands. | Focused orchestrator and runner tests. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | Direct hydrology projection | Add profile depth/porosity-cap projection operands. | `direct_runtime_r4pqz`; R6F typed-input test. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs` | Direct layer conversion | Add state-to-input conversion for carried layers. | R6F typed-input/carry test. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | Tests | Prove typed process inputs and layer carry publish ET/storage/profile operands. | Focused test passed. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs` | Tests | Preserve R4PQZ expectations after new optional operands. | Focused test passed. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | Cutover gate | Emit stable R6F WAT hold marker and reduced field list. | Runner/CLI tests passed; file remains below 3000-line hard threshold. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | Unit correction | Stop converting parsed climate daily precipitation from mm to mm twice. | WAT reduction test passed accepted `P`/`RM`/`Q` fields. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | Tests | Update cutover marker expectations and add HBP/WAT blocker-reduction and exact-marker guard tests. | Focused runner tests passed. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | Direct day input construction and WAT reduction helpers | Preserve climate-only behavior under expanded `DirectPublicationDayInput` and house WAT field-reduction helpers outside the intake file. | Compile, clippy, and runner tests passed. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | CLI test | Update expected marker and no-output fail-closed evidence. | CLI test passed. |

## Out-of-Set Edit Requests

None.
