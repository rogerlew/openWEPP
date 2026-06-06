# Worker Handoff

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Current handoff: WBVAL04 executed and is held on valid-climate openWEPP
invariant failures. The upstream climate producer boundary is no longer the
active blocker for this run.

Ran:

- Climate audit passed with zero CLI radiation-bound exceedances.
- Release WBVAL04 batch ran all `22` single-OFE hillslopes.
- Results: `18` WAT emitters, `4` fail-closed J-95 percolation blockers.

Follow-on 1:

- Defect ID: `WBVAL05-J95-HKERNEL-WB11-PERC-E-003`.
- Observable failure: `p7`, `p11`, `p18`, and `p20` fail closed at
  `sim_day_index=95`, `calendar_year=1990`, `julian_day=95` with
  `HKERNEL-WB11-PERC-E-003`; no WAT is emitted.
- Suspected mechanism: WB11/WB18 percolation/deep seepage state transition
  reaches a domain guard under valid climate.
- In-scope write set for owning follow-on: `SC-PERC-001`,
  percolation/deep seepage kernel/runtime projection tests, and production
  files named by that DC-ExecPlan.
- Correction authority: `SC-PERC-001`, `SC-WATBAL-001`, and pinned baseline
  `/workdir/wepp-forest_260430_baseline` if legacy migration evidence applies.
- Acceptance target: valid-climate runs reach WAT publication or fail closed at
  a newly proven out-of-envelope boundary; no guard loosening.
- Legitimate HOLD conditions: missing or contradictory canonical authority,
  invalid fixture input outside openWEPP, or root cause outside the declared
  percolation/deep seepage envelope.

Follow-on 2:

- Defect ID: `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
- Observable failure: all `18` WAT emitters have annual complete-identity
  residuals above `1.0 mm/year` for years `2..6`; maximum current residual is
  `94.433 mm` (`p4`, year `5`).
- Suspected mechanism: WAT publication and/or vertical water-balance transfer
  accounting omits or mis-signs a storage/flux term after valid-climate runtime
  reaches publication.
- In-scope write set for owning follow-on: `SC-WATBAL-001`, WAT publication,
  water-balance accounting tests, and production files named by that
  DC-ExecPlan.
- Correction authority: `SC-WATBAL-001`, `SC-PERC-001`,
  `SC-SNOWFREEZE-001`, and pinned baseline
  `/workdir/wepp-forest_260430_baseline` if legacy migration evidence applies.
- Acceptance target: years `2..6` complete-identity residuals are within the
  contract tolerance, or the residual is reclassified at a declared authority
  boundary with explicit missing surface evidence.
- Legitimate HOLD conditions: missing initial storage may keep year `1`
  unclassified, but not years `2..6`; contradictory authority or an
  out-of-envelope publication surface may justify HOLD.

Forbidden relay check: satisfied. The handoff names defect closures, not a
single next diagnostic step.
