# WB19 Hourly Cap Withdrawal Publication Diagnosis

Status: completed

Evidence mode: static

## Baseline Lineage

- Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:629-745`
  computes hourly lateral potential from `subq = fcdep * anisrt * latk *
  fslope`, converts to `latqcc = subq / slplen`, caps `latqcc` by active-layer
  `tdvv`, and floors negative values to zero.
- Static: capacity-active hourly layers use `fzdrfc = max(drfc-frzw,0)` and
  `meblfc`; conductivity-active layers still use unfrozen `drfc` with `fffx`.
- Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:774-832`
  publishes `sbrunf` and `ui_lfcrf(ii)` from the capped amount, then withdraws
  layer storage top-down and subtracts any unwithdrawn residual from both
  `sbrunf` and `ui_lfcrf(ii)`.
- Static: baseline publication authority is therefore realized lateral
  withdrawal, not uncapped potential and not an unwithdrawn target.

## openWEPP Diagnosis

- Static: openWEPP already computed `q_lateral_potential`,
  `q_lateral_target = min(q_lateral_potential, available_pool, tdvv)`,
  withdrew top-down, accumulated realized `q_lateral`, and published `Qd =
  Qdd + q_lateral` when drainage was present.
- Static: the missing defect was observability: continuation evidence could not
  separate potential, target, active `tdvv`, and realized withdrawal without
  opening internal code paths.
- Static: HPHYS0258 therefore added contract-backed diagnostics rather than
  changing the baseline-authoritative lateral-flow equations.
