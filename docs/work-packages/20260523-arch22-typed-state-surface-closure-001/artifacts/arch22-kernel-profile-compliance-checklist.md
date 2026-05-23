# ARCH22 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

Checklist source:
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`

## Compliance Items
- [x] Canonical `SC-*` authority files updated in
  `docs/specifications/science-contracts/contracts/`.
- [x] ARCH22 typed production-surface authority implemented for covered
  hillslope and watershed lanes.
- [x] Contract-derived migration proof tests implemented from amended contract
  authority (`tests/integration/arch22_typed_state_surface_contract.rs`).
- [x] Pre-implementation contract gate recorded with failing test compile stage
  before production migration code edits.
- [x] Production migration implemented after contract + test + pre-implementation
  gate sequence.
- [x] Covered production guard/accessor signatures no longer consume raw string
  symbol identifiers where typed ARCH22 symbols exist.
- [x] Required verification gates executed and passing:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
