# WS11 Implementation and Test Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Production implementation files updated
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - `crates/openwepp-kernel-contract/src/lib.rs`
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- WS11 routing implementation changes
  - Added explicit `ipeak` guard/read path as typed watershed state symbol.
  - Added explicit WS11 branch selection semantics:
    - `ipeak = 1`: Rational lane
    - `ipeak = 2`: CREAMS lane
    - `ipeak = 3`: kinematic-wave surrogate lane
    - `ipeak >= 4`: Muskingum-Cunge surrogate lane
  - Removed single-path dependence on pre-WS11 gain-only routing behavior by
    introducing branch-dependent `qpo` computation.
  - Preserved typed hard-fail guard-family continuity:
    `WKERNEL-WS10-CHANNEL-E-001..003`.
  - Preserved closure publication semantics:
    `qpo`, `durrof`, `roff` with `roff = qpo * durrof`.
- WS11 test/evidence files updated
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `tests/integration/arch22_typed_state_surface_contract.rs`
  - `Cargo.toml` (`[[test]]` registration for WS11 target)

## Ran
- Formatting
```bash
cargo fmt
```
- Verification tests
```bash
cargo test --test ws11_channel_routing_physics_equivalence_contract
cargo test --test ws10_watershed_kernel_contract
cargo test --test arch22_typed_state_surface_contract
```
- Observed results (post-implementation)
  - `ws11_channel_routing_physics_equivalence_contract`: **pass** (`6 passed`)
  - `ws10_watershed_kernel_contract`: **pass** (`4 passed`)
  - `arch22_typed_state_surface_contract`: **pass** (`6 passed`)
