# HPHYS0218 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Added contract-derived WB19 threshold test target:
  - `tests/integration/hphys0218_wb19_cpm_threshold_contract.rs`
- Registered target in root `Cargo.toml` (`[[test]]` entry).
- Updated WB19-dependent fixtures to include `cpm_####` symbols with valid
  domains for deterministic guard continuity across integration tests.

## New/updated vectors (Static)
1. `hphys0218_wb19_lateral_withdrawal_uses_cpm_adjusted_threshold`
   - Asserts realized WB19 lateral withdrawal is capped by
     `drfc_i = fc_i + (1-cpm_i)*dg_i`.
   - Guards against regression to FC-only withdrawal behavior.
2. `hphys0218_wb19_lateral_rejects_domain_invalid_cpm`
   - Asserts typed domain hard-fail on `cpm > 1`.
3. Existing WB19 lane contract fixture remains passing with `cpm_0001/0002=1.0`
   to preserve baseline-compatible no-adjustment branch.

## Ran evidence
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract --test hphys0218_wb19_cpm_threshold_contract` (pass).
- `cargo test -p openwepp-runner hphys0213_wb19` (pass).
