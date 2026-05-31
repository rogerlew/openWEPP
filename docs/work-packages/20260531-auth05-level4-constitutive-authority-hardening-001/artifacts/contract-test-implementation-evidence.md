# AUTH05 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement contract-derived tests that hard-fail when Level-4 authority
  posture regresses to fixture self-consistency or optional assertions.

## Static

1. Added integration target:
   - `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
2. Registered target in `Cargo.toml`:
   - `auth05_level4_constitutive_authority_hardening_contract`
3. Added relax fixture requirement fields to remove optional assertion behavior:
   - `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json`
4. Test assertions now enforce:
   - no legacy-as-authority citations in Level-4 suite docs,
   - runtime FC/WP surface matches independent authority reconstruction on real
     soils (`valid_9002.sol`, `valid_7778.sol`),
   - negative perturbation of `thetfc_0001` must fail with symbol-specific
     mismatch,
   - relax-to-FC branch positivity/non-negativity assertions are explicit and
     required.

## Ran

1. `cargo test --test auth03_level4_constitutive_gate_contract --test auth05_level4_constitutive_authority_hardening_contract`
   - pass (`8 passed` total).
