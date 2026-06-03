# Contract Implementation Evidence

Status: completed

Evidence mode: static

## Contract Amendments

- Static: `SC-SUBHYD-001` contract version `30` adds
  `INV-SUBHYD-029`, requiring opt-in traces to carry WB19 potential/target/
  `tdvv`/realized-withdrawal plus `q`/`Qdd`/`Qd` lineage for residual
  localization.
- Static: `SC-WATBAL-001` contract version `85` adds `INV-WATBAL-045`,
  requiring trace-grade WB19 identity evidence before residual ownership is
  assigned back to WB19 cap/publication logic.
- Static: both contracts add HPHYS0259 addenda and alias rows for trace fields:
  `wb19_q_lateral_potential_m`, `wb19_q_lateral_target_m`,
  `wb19_lateral_capacity_tdv_m`, `wb19_tdvv_m`,
  `wb19_q_lateral_unrealized_m`, `wb19_lateral_withdrawal_layers_m`,
  `q_m`, `qdd_m`, and `qd_m`.
