# WBVAL series — execution log

> Per-package log for the WBVAL (single-OFE water-balance validation /
> defect-closure) series. Newest first. WBVAL01 and the rung sequence live in
> the [../README.md](../README.md) current-execution narrative.
> Canonical forward queue: [../../ROADMAP.md](../../ROADMAP.md).

- `20260606-wbval02-simimpl28-radbound-defect-closure-001/`
  - Purpose: close the six WBVAL01 `CLIM-RUNTIME-E-017` radiation-bound
    fail-closed single-OFE hillslopes as a Defect-Closure ExecPlan under the
    climate/SIMIMPL28 authority envelope.
  - Status: complete; WBVAL02 reclassified all six as invalid upstream daily
    radiation with typed `radly` evidence, amended `SC-CLIMATE-001`, added
    contract-derived regressions, validated the six wrappers against the
    release CLI, and recorded dual review, verification, disposition, and an
    upstream input-boundary handoff.
- `20260606-wbval03-snowmelt-wb-closure-defect-closure-001/`
  - Purpose: close the four WBVAL01 J-95 `HKERNEL-WB11-PERC-E-003`
    fail-closed hillslopes and the emitted-ledger conservation residual as a
    Defect-Closure ExecPlan under the snowmelt/percolation/water-balance
    authority envelope.
  - Status: executed-hold; current `57eed35` release runs are preempted by
    WBVAL02's typed `radly=486` source guard before WBVAL03's J-95 and WAT
    ledger surfaces are reachable. Static WBVAL01 evidence still anchors the
    J-95 percolation blockers and the complete-identity WAT residual audit, but
    no WBVAL03 production correction is valid until the upstream climate input
    boundary is closed.
- `20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/`
  - Purpose: redo WBVAL01 on `/wc1/runs/in/indispensable-presenter` after the
    WEPPpy observed-Daymet radiation producer is rebuilt into publication-safe
    run artifacts, then produce a fresh single-OFE water-balance closure ledger.
  - Status: executed-hold; current climate audit passed with zero CLI
    radiation-bound exceedances, all `22` single-OFE hillslopes were rerun,
    `18` emitted WAT and all `18` are conservation-break for years `2..6`,
    while `p7`, `p11`, `p18`, and `p20` remain fail-closed at J-95 with
    `HKERNEL-WB11-PERC-E-003`. Follow-ons are
    `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
    `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
- `20260606-wbval05-j95-percolation-defect-closure-001/`
  - Purpose: close defect `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` for `p7`,
    `p11`, `p18`, and `p20`, which still fail closed at J-95 under WBVAL04's
    publication-safe climate.
  - Status: hold-boundary; WB18 percolation now consumes published
    `wb12_infiltration` before optional WB14/WB12 recomputation, clearing the
    `HKERNEL-WB11-PERC-E-003` symptom. Final target validation now fails first
    at upstream `HKERNEL-WB14-RUNOFF-E-003` on negative
    `snow.runtime_swe=-0.006171157610042402`; handoff is a snow/runoff
    boundary closure.
- `20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/`
  - Purpose: close defect `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` for
    the current `18` WBVAL04 WAT emitters whose years `2..6` complete-identity
    annual residuals exceed `1.0 mm/year`.
  - Status: corrected; post-SNOWSCI WAT validation identified omitted daily
    interception flux publication as the in-envelope mechanism. `SC-WATBAL-001`
    v146 and WAT schema/unit contracts now require `hillslope_wat.Interception`;
    22/22 current WAT emitters close with max annual residual
    `0.000001037 mm` when `Interception` is included.
