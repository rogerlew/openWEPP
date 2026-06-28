# Verification

Evidence mode: `Static + Ran`

## Package Gates

- Source threshold verification: `PASS`
  - Evidence: `sturm-thresholds-source-verification.md`.
- Cross-SNOTEL+cancov primary rubric: `FAIL`
  - Activated default: `15` robust fails / `179` robust score.
  - Climate-class candidate: `16` robust fails / `168` robust score.
- Bidirectional densification flip: `FAIL`
  - No robust densification improvements; `harvard_open` regressed on
    `seasonal_densification_trajectory`.
- Persistence guardrail: `FAIL`
  - Candidate worsened `13` robust cells vs activated default.
- Whole-model conservation: `PASS`
  - Candidate trace rows: `159986`.
  - Max snow-state residual: `4.440892098500626e-16 m`.
  - Max partition residual: `5.551115123125783e-17 m`.
  - Tolerance: `1e-9 m`.

Disposition: `HOLD-GATE-FAILURE-NON-PROMOTION`; no default activation.

## Commands

Ran:

- `.venv/bin/python tools/snowfreeze_observed/climate_class_density_specialization.py`
  - Real direct-production WAT/trace run completed; candidate failed promotion
    gates as recorded above.
- `.venv/bin/python tools/snowfreeze_observed/climate_class_density_specialization.py --skip-model-runs`
  - Regenerated artifacts from the completed WAT/trace outputs after adding
    explicit trace conservation closure.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/climate_class_density_specialization.py`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --test snowdensity10_3_22_climate_class_density_specialization`
- `cargo test --test snowdensity02_contract_adr_guard`
- `cargo test --test snowdensity03_physics_bulk_offline_contract`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --workspace`
- `cargo deny check`

Final result: all listed validation commands passed. The full workspace test
suite was rerun after fixing stale contract-version and source-scope guard
assertions, and the final run passed.
