# Authority Intake

Status: complete / pre-implementation

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

## Exact source findings

PySnobal 0.2.3 `pysnobal/point/snobal.py::mass_bal` orders time compaction,
precipitation, melt, evaporation/condensation, H2O compaction, and runoff.
`h2o_compact` uses `(snow_state.melt + input1.m_rain) / snow_state.m_s` when
precipitation is present. Its `snowcover` predicate is fixed from layer presence
at the start of the timestep, so mixed precipitation that first creates a pack
does not enter H2O compaction until a later active-pack interval. This matches
openWEPP's current pack-contact rain classification; it does not authorize all
daily rain.

Exact pinned-baseline blobs were read because the local baseline worktree HEAD
is not the pinned commit. At `dac3c950...`:

- `src/stmtim.for:32-37,43-95` creates nonnegative hourly rain/snow and tracks
  rain without a pack separately;
- `src/winter.for:296-300,366-374` partitions precipitation before `snowd` and
  receives post-snowd melt afterward;
- `src/melt.for:238-262,272-301` uses rain heat, emits signed melt, and bounds
  only nonnegative generated melt;
- `src/snowd.for:180-283,303-316` converts only positive melt to a snow-depth
  effect, retains positive melt/rain below the density gate, and commits pack
  state; and
- `src/winter.for:420-464` performs signed daily redistribution and routes
  residual positive rain only after those pack-contact stages.

Legacy WEPP therefore supplies chronology, sign, and anti-alias authority, not
the later Anderson/SNOBAL wet-compaction formula. Rain heat that generates melt
and rain mass remain distinct operands.

## Current exact mapping

- generated melt: `SnowHourlyState::melt_raw_m`, the capped/applied CoE melt
  result before retention/routing; sum its positive hourly parts, not the
  signed daily sum;
- snow-contact rain: `ActiveSnowDailyTotals::rain_retained_m +
  rain_released_m`, which is limited to hours with a pack at interval start;
- selected driver: `sum(max(hourly melt_raw_m, 0)) + rain_retained +
  rain_released`; and
- transport: one private daily `SnowCouplingOutcome` scalar into the existing
  bulk/multilayer `SnowDensityRuntimeInputs::liquid_for_compaction_m` boundary.

Static investigator A found two duplicate consumers: the production density
handoff in `runoff_reconciliation.rs` and the offline CoE-bound replay in
`snowbench_coe_density.rs`. Static investigator B independently confirmed the
pinned chronology and the absence of an Anderson/SNOBAL H2O-compaction operator
in legacy WEPP. Neither investigator modified files.
