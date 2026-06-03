# Contract-Test Implementation Evidence

Status: completed

Evidence mode: mixed

- Static: added
  `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
  `hphys0258_hourly_lateral_publishes_realized_cap_diagnostics`.
- Static: the vector forces `wb19_q_lateral_potential > q`, binds
  `q = wb19_q_lateral_target = wb19_lateral_capacity_tdv = wb19_tdvv`, proves
  per-layer withdrawal sums to `q`, proves `ui_LfCrf_0001` uses realized `q`,
  and proves `Qd = Qdd + q`.
- Ran: pre-implementation run failed on missing
  `wb19_q_lateral_potential`.
- Ran: post-implementation targeted and full WB19 integration test runs passed.
