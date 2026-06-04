# Implementation-Test Evidence

Status: completed
Evidence mode: ran

Ran:
- `cargo fmt --check`: pass.
- `cargo test -p openwepp-runner hphys0281 -- --nocapture`: pass, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture`: pass, 1 test.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18 -- --nocapture`: pass, 2 tests.
- `tools/release/check_unit_registry.sh`: pass, 13 registry tests plus focused clippy.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with existing duplicate/unmatched-license warnings only.
- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md --path docs/specifications/units/boundary-symbol-unit-registry.md --path docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001 --path docs/work-packages/README.md`: pass, 24 files.
- `git diff --check`: pass.

Ran after Review A/B dispositions:
- `cargo fmt`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner hphys0281 -- --nocapture`: pass, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture`: pass, 1 test.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18 -- --nocapture`: pass, 2 tests.
- Final combined rerun: `cargo test --workspace`, `cargo deny check`,
  scoped `markdown-doc lint`, and `git diff --check` all returned status 0.

Ran after dual verification dispositions:
- Fixed the HPHYS0281 producer fixture to use nonzero
  `wb17_residue_interception = 0.000_2`.
- `cargo fmt`: pass.
- `cargo test -p openwepp-runner hphys0281 -- --nocapture`: pass, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture`:
  pass, 1 test.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md --path docs/specifications/units/boundary-symbol-unit-registry.md --path docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001 --path docs/work-packages/README.md`: pass, 24 files.
- `git diff --check`: pass.
- `cargo test --workspace`: pass.

Ran / HOLD:
- `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md`: fail with 11 pre-existing HPHYS0279 SC-EVAP unit-compliance findings for `Ep`/`Es`/`Er`; no finding names the new `pmet.es_storage_return_m` symbol.
