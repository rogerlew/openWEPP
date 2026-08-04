# Authority Intake

Status: scaffolded / pre-implementation

Evidence mode: Static

- Current caller:
  `runoff_reconciliation.rs::resolve_typed_snow_density_outcome` supplies
  `snowpack_state_loss + routed_melt_m`.
- Current compact ledger:
  `liquid_handoff = snowpack_swe_loss + rain_released`.
- Retained reconstruction: the resulting current input equals
  `2 * snowpack_swe_loss + rain_released` within `2.78e-17 m`; duplicated
  state-loss total is `73.123 m` over the prior primary windows.
- Primary implementation source: PySnobal 0.2.3, PyPI archive SHA-256
  `78f97faf0452816038494b9fde332a2c2a14d92ec2e5960378abd7606d82fda2`.
- Primary chronology: time compaction, precipitation, melt, evaporation,
  H2O compaction, then runoff.
- Primary operand meaning: liquid water added by generated melt and rain,
  normalized by current snow mass.
- Pinned WEPP baseline: commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; required for CoE melt/rain
  lineage but not sufficient authority for the later SNOBAL wet-compaction
  operator.

The authority milestone must map this physical operand onto exact current
hourly fields before the contract or production path is changed.
