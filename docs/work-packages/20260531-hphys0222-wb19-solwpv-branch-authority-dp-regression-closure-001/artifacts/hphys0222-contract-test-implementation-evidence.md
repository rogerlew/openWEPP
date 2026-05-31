# HPHYS0222 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Objective
Add contract-derived tests that fail on the pre-fix branch mismatch and pass
only when WB19 mutation scope matches `solwpv < 2006`.

## Added/updated tests and fixtures
- Added:
  - `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`
  - `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/solwpv_fcdep_branch_cases.json`
  - `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/fixtures.sha256`
  - `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/fixtures.provenance.yaml`
- Updated:
  - `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
    - added `solwpv >= 2006` no-mutation vector.
  - `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
    - includes new active Level-4 suite fixture root + suite doc checks.
  - `tests/integration/auth07_fc_authority_cohort_contract.rs`
    - clippy-clean divisor conversion.
  - `Cargo.toml`
    - new test target `auth08_wb19_solwpv_fcdep_branch_constitutive_contract`.

## External-authority governance wiring
- Added suite spec:
  `docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- Added registry entry (`required`, `hard-fail`) in:
  `docs/specifications/external-authority/registry.yaml`

## Ran evidence
Pre-fix expected failure (before production edit):
- `cargo test --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract`
  - failure vector: `solwpv_9002_does_not_update_fcdep` expected `fcdep=1`,
    observed `fcdep=0`.

Post-fix pass:
- `cargo test --test auth06_fixture_provenance_hash_enforcement_contract --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract`
- `cargo test --test hphys0219_wb19_coca_threshold_contract --test wb19_lateral_drainage_physics_kernel_contract`

## Result
- `MEASURE-HP222-002`: pass.
- `MEASURE-HP222-004`: pass.
