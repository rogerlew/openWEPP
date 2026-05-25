# MOFE05 Watershed Contributor Metadata Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Contract-derived MOFE05 vectors:
1. `watershed_cli_mofe05_rejects_multiofe_contributor_without_manifest_metadata`
- Intent: multi-OFE contributor intake fails when metadata source is absent.
- Expected signal: `CLIWAT-E-036`.

2. `watershed_cli_mofe05_rejects_multiofe_contributor_manifest_shape_violation`
- Intent: malformed manifest metadata fails intake.
- Expected signal: `CLIWAT-E-037`.

3. `watershed_cli_mofe05_rejects_multiofe_manifest_count_mismatch`
- Intent: metadata/pass contributor-count mismatch fails intake.
- Expected signal: `CLIWAT-E-037` with contributor count mismatch detail.

4. `watershed_cli_mofe05_accepts_valid_multiofe_metadata_and_reaches_output_guard`
- Intent: valid MOFE metadata passes intake and reaches downstream guard.
- Expected signal: downstream `CLIWAT-E-034` (existing output writer guard),
  with no `CLIWAT-E-036/037`.

5. `mofe05_addenda_are_present_in_required_contracts`
- Intent: canonical MOFE05 authority is explicit in required contract surfaces.
- Expected signal: MOFE05 addendum text in `SC-SYSTEM-001` and runfile
  metadata-surface requirements in `openwepp-watershed-runfile-contract.md`.

## Ran
- Pre-implementation baseline:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_watershed_cli_surface_uses_runfile_pattern_with_legacy_discovery_flag -- --nocapture`
  - `cargo test -p openwepp-runner --test watershed_cli_behavior_contract mofe05 -- --nocapture`
  - Result: expected failures before implementation (`manifest_file` source marker absent and no MOFE05 intake guard behavior).
- Post-implementation:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_watershed_cli_surface_uses_runfile_pattern_with_legacy_discovery_flag -- --nocapture`
  - `cargo test -p openwepp-runner --test watershed_cli_behavior_contract mofe05 -- --nocapture`
  - `cargo test -p openwepp --test mofe05_watershed_contributor_metadata_contract_authority_closure_contract -- --nocapture`
  - Result: all targeted MOFE05 tests passed.
