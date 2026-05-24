# EROD12 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Contract-Derived Test Implementation

Implemented:
- `tests/integration/erod12_cross_domain_contract_closure_contract.rs`
  - Validates required EROD12 addendum presence in companion contracts.
  - Validates EROD10-AH-002 Wave-0 cross-domain blocker rows are now
    dispositioned to `closed`.
  - Validates explicit retention of non-Wave-0 `non-promotable` holds.
- `tests/integration/erod11_alias_boundary_ownership_contract.rs`
  - Updated row-scoped assertions for Wave-0 cross-domain blocker rows that are
    now canonically `closed` after EROD12.

Wiring update:
- Added `[[test]]` entry in `Cargo.toml`:
  - `name = "erod12_cross_domain_contract_closure_contract"`
  - `path = "tests/integration/erod12_cross_domain_contract_closure_contract.rs"`

Ran:
- `cargo test --test erod12_cross_domain_contract_closure_contract`
  - Result: `3 passed; 0 failed`
- `cargo test --test erod11_alias_boundary_ownership_contract`
  - Result: `4 passed; 0 failed`
  - Purpose: confirm prior EROD11 assertions were updated to remain consistent
    with post-EROD12 canonical gap posture.
