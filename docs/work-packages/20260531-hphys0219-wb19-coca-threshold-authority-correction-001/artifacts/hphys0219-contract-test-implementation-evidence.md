# HPHYS0219 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Added contract-derived WB19 threshold test target:
  - `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- Registered target in root `Cargo.toml` (`[[test]]` entry).
- Removed superseded cpm-threshold test target:
  - `tests/integration/hphys0218_wb19_cpm_threshold_contract.rs`
- Updated WB19-touching fixture suites to include valid `coca_####` symbols so
  guard continuity is explicit.

## New/updated vectors (Static)
1. `hphys0219_wb19_lateral_withdrawal_uses_coca_adjusted_threshold`
   - Asserts realized WB19 lateral withdrawal is capped by
     `drfc_i = fc_i + (1-coca_i)*dg_i`.
2. `hphys0219_wb19_lateral_rejects_domain_invalid_coca`
   - Asserts typed domain hard-fail on invalid `coca`.
3. `hphys0219_wb19_lateral_is_independent_of_cpm_when_coca_fixed`
   - Guards against reintroducing `cpm` as WB19 threshold authority.

## Ran evidence
- `cargo test --test hphys0219_wb19_coca_threshold_contract --test wb19_lateral_drainage_physics_kernel_contract` (pass)
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::` (pass)
