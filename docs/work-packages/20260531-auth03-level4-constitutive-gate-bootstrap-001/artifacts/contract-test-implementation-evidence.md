# AUTH03 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement Level-4 contract-derived test coverage for:
  - FC at `-33 kPa`,
  - WP at `-1500 kPa`,
  - relax-to-FC / near-FC `Dp` cutoff behavior,
  - typed fail-closed constitutive symbol guard posture.

## Implemented tests
- Added `tests/integration/auth03_level4_constitutive_gate_contract.rs` and
  test target registration in `Cargo.toml`.
- Added fixture families:
  - `tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/`
  - `tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/`
  - `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/`

## Assertions covered
1. Contract/suite registry linkage exists and points to canonical invariants.
2. FC/WP fixture vectors satisfy constitutive ordering/bounds.
3. WB18 near-FC kernel branch emits zero percolation and above-FC emits
   positive branch outputs.
4. Missing/non-finite constitutive symbols fail closed with typed error/status
   IDs.
