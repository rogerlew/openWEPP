# Contract-Test Implementation Evidence

Status: executed
Evidence mode: Static + Ran

Static:
- Added `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`.
- Registered the test in `Cargo.toml`.
- The test verifies:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-027`.
  - `SC-RUNOFFPART-001#INV-RUNOFFPART-024`.
  - `SC-WATBAL-001#INV-WATBAL-071`.
  - Review-disposition guard text requiring per-window defective-model verdict,
    reconstruction controlled experiment, independent correctness adjudication,
    and documented-legacy-defective re-tiering rather than deletion.
  - Existing trace surfaces needed for snow/`RM` acceptance.
  - Corrected negative-melt routed/state-loss authority remains in source.

Ran:
- Baseline-path correction rerun:
  - `cargo fmt --check`
  - `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`
  - `wctl doc-lint --path docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001`
  - Result: passed after updating the static assertion to require
    `/workdir/wepp-forest_260430_baseline` for baseline authority.
- Post-review disposition rerun:
  - `cargo fmt --check`
  - `cargo fmt`
  - `cargo fmt --check`
  - `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`
  - `wctl doc-lint --path docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001`
  - Result: passed after formatting the updated static assertion.
- `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`
- Result: passed, `3 passed; 0 failed`.
