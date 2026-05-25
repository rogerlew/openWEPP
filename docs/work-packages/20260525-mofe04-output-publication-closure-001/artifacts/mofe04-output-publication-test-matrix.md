# MOFE04 Output Publication Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Contract-derived MOFE04 vectors:
1. `cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area`
- Intent: aligned multi-OFE run publishes canonicalized WB13 row key with explicit policy/provenance and aggregate OFE geometry area.
- Expected post-implementation signals:
  - manifest contains `publication_ofe_policy`, `contributor_ofe_count = 3`, `area_policy`, `publication_area_m2 = 3600.0`.
  - first WB13 data row preserves canonicalized `OFE = 1` and `Area = 3600.0`.

2. `cli03_mofe04_single_ofe_publication_reports_single_contributor_policy`
- Intent: single-OFE run still publishes MOFE04 policy/provenance fields deterministically.
- Expected signal: manifest reports `contributor_ofe_count = 1` and `publication_area_m2 = 1800.0`.

3. `mofe04_addenda_are_present_in_required_contracts`
- Intent: MOFE04 canonical authority is explicit in required `SC-*` contracts.
- Expected signal: addenda and policy keywords present in `SC-WATBAL-001` and `SC-SYSTEM-001`.

## Ran
- Pre-implementation baseline:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe04 -- --nocapture`
  - Result: expected failure before production publication provenance/area implementation.
- Contract closure gate:
  - `cargo test -p openwepp --test mofe04_publication_contract_authority_closure_contract -- --nocapture`
  - Result: passed.
- Post-implementation:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe04 -- --nocapture`
  - Result: passed (2 tests).
