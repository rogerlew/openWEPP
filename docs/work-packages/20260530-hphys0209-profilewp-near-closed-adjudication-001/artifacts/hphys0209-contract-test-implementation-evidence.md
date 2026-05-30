# HPHYS0209 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test additions
- Static: `tests/integration/hphys0209_profilewp_adjudication_contract.rs`
  - Added package/contract authority presence test.
  - Added WP-lineage perturbation execution test asserting:
    - `ProfileWPStore` responds to authoritative WP perturbation,
    - `ProfileDepth` and `ProfilePorosityCap` remain geometry-stable,
    - profile storage ordering remains valid.
- Static: `crates/openwepp-runner/src/hillslope/mod.rs`
  - Added
    `hphys0209_wb13_wp_storage_guard_rejects_missing_authoritative_symbol`.
  - Verifies typed hard-fail on missing `wb13_profile_wp_store_mm`.
- Static: `Cargo.toml`
  - Added explicit `[[test]]` registration:
    `hphys0209_profilewp_adjudication_contract`.

## Targeted execution evidence
- Ran: `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
  -> pass
  - Log:
    `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_integration.stdout.log`
- Ran: `cargo test -p openwepp-runner hphys0209_` -> pass
  - Log:
    `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_runner.stdout.log`
