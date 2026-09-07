# Gate results
Ran/Static: commands below cwd /workdir/openWEPP; logs retain actual output. Directory
PASS is structural, not scientific/activation approval. Required gate failures remain.

| Gate | Result | Evidence / exact invocation |
|---|---|---|
| Original source and whole reading | PASS Static | source-manifest.json,clause-preservation-map.md,review A full original |
| Python actual runner | PASS83 exit0 | .venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py tests/python/test_sc_contract_directory.py; logs/pytest-final.log |
| Candidate binding/default/strict conformance | PASS | Above CLI negatives plus .venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md; logs/strict-candidate.log |
| Candidate unit | PASS exit0 | .venv/bin/python tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md; logs/unit-candidate.log |
| Schema/profile/provenance/reference structure | PASS bounded | Shared full32-key/required metadata/marked definitions/dependency/provenance grammar; semantic coverage independently reviewed A |
| A0 admission | PASS after consumer correction | PATH="$PWD/.venv/bin:$PATH" bash tools/release/check_science_contract_admission.sh --base-ref b932db101cce07d0860b45b5ecaa8ddb7f455b58 --worktree; logs/schema-admission-corrected.log; initial FAIL retained |
| Selected Rust/governance | FAIL exit100 | logs/rust-candidate-reviewed.log129run111pass18fail; baseline same18 reasons; no gate waiver |
| Rust formatting | PASS exit0 | nix develop -c rustfmt --check --edition2021 over8 owned candidate Rust files; logs/rustfmt-candidate.log |
| Python syntax | PASS exit0 | .venv/bin/python -m py_compile tools/sc_contract_directory.py tools/check_sc_binding_exposure.py tools/release/check_sc_unit_compliance.py tests/python/test_sc_contract_directory.py |
| Applicable Clippy | FAIL exit101 | nix develop -c cargo clippy same8 targets -- -D warnings; logs/clippy-candidate.log; unchanged production dependencies1136 errors |
| Science preservation | PASS Static/Ran | Review A complete source audit100IDs/36fences/474table rows/275paragraphs/qualifiers; strict checker separate |
| Fresh selective exercises | PENDING full assessment | exercise01–04 and context-usability.md;03 sufficient/reduced independently PASS, no runtime total |
| Exact restoration/archive | PENDING terminal check | candidate-recovery.json,restore_candidate.py; parent isolated strict/unit PASS |
| Independent terminal assurance | NOT RUN yet | two new verifiers after corrected substantive freeze |

## Rust command and interpretation
`nix develop -c cargo nextest run --no-fail-fast --test land_surface_energy_balance_authority_contract --test solver_architecture_authority_contract --test surface_liquid_hydrology_custody_authority_contract --test snow_stage3_terminal_receiver_authority_contract --test snow_stage3_shared_carrier_authority_contract --test vegetation_boundary_authority_contract --test stage3_native_vegetation_laned_throughput_recovery --test hphys0279_sc_unit_compliance_lint_contract`
Full runs initial/second/third/final/corrected retained. Corrected final authoritative
candidate run is rust-candidate-reviewed.log; older failures not hidden. Baseline
comparison temporarily restored only owned files and restored candidate byte-for-byte;
logs/rust-baseline-comparison.json documents exact controlled restoration. Precomparison
evidence bundle verified at /tmp/openwepp-lse-candidate-before-baseline-20260907.
Heavy runner unavailable: actual comparator_suite_runner model quota failure recorded
logs/runner-unavailable.md; supported parent nix command fallback used openly.
No scientific campaign solely for document relocation. No external-authority suites,
cohort fixtures/required bindings or production/kernel physics changed, so no newly
triggered full correctness/anti-evasion campaign solely from this diff. No TESTGATE.
Inherited absent v31 production seam and frozen external runtime identity cannot be
fixed in authorized document/consumer scope. Every raw FAIL remains FAIL. All-pass
adoption gate remains unmet; no retrospective same-failure-set acceptance substitution.
