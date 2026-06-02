# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static + ran

Static:

- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs` adds
  `hphys0252_wb19_lateral_uses_frozen_adjusted_capacity_and_withdrawal_floor`.
- The vector constructs a one-layer lateral state with `drfc=0.7`,
  `frzw=0.4`, and expected `fzdrfc=0.3`. It requires realized `q=0.5`,
  post-withdrawal `theta=0.3`, and `wb11_soil_water=0.5`.

Ran:

- Pre-implementation:
  `cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0252 -- --nocapture`
  failed as expected with `q=0.10000000000000009`.
- Post-implementation focused vector passed `1/1`.
- Final WB19 contract integration suite passed `12/12`.
