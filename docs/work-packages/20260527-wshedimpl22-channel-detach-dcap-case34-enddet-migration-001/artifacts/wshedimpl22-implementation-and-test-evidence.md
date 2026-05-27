# WSHEDIMPL22 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented WS22 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - Added `crfrac` projection intake helper:
    - `ws22_require_crfrac_vector(...)`
  - Added baseline-lineage `dcap` helper surfaces:
    - `ws22_table_column2_to_column1(...)`
    - `ws22_shdist(...)`
    - `ws22_dcap_flagm1(...)`
  - Updated WS20/WS21 route function signature and call chain to carry active
    class-number lineage for symbol-accurate class addressing.
  - Replaced WS21 unconditional unresolved fallback for `excess > 0` with:
    - required `crfrac` gate,
    - `dcap`-driven `du` computation,
    - active `case34` routing branch,
    - active `enddet` iterative closure branch.
  - Preserved explicit unresolved diagnostics for residual `case4 -> detach`
    iterative branch where `nt < cnpart`.
- Implemented WS22 contract-derived vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed
  (`21 passed; 0 failed`).
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed (warnings-only duplicate/unmatched-allowance output
  in `deny.toml`; no deny failures).
