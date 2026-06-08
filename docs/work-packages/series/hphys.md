# HPHYS series — execution log

> Per-package log for the HPHYS work-package series (single-hillslope physics
> parity/closure arc). Newest first. Index: [../README.md](../README.md).
> Canonical forward queue: [../../ROADMAP.md](../../ROADMAP.md).

- `20260606-hphys0320-stmtim-start-time-source-line-closure-001/`
  - Purpose: close the HPHYS0319 `stmtim-active-interval-divergence-hold` by
    source-line classifying baseline `winter.for` storm-start normalization
    against OpenWEPP SIMIMPL28 timing projection, then implementing and
    validating the baseline-authoritative timing path if the proof holds.
  - Status: complete; HPHYS0320 source-line classified the `wnttim < 1.0`
    start-time minimum as pinned-baseline `winter.for`/`stmtim.for` authority,
    corrected OpenWEPP SIMIMPL28 timing projection before active-interval
    evaluation, validated H1/H7/H39 trace closure, ran a 39/39 release-binary
    hillslope batch, closed the combined `57` timing-seam carried rows, and
    recorded dual review, verification, and handoff.
- `20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/`
  - Purpose: recover fixed-baseline `stmtim.for` observe values for the 2013
    day 11 hour 11 H1/H7/H39 route, pair them with regenerated OpenWEPP
    `snow.hourly.stmtim.*` traces, and classify source ownership before any
    production precipitation-phase or downstream water-balance edit.
  - Status: executed-hold; HPHYS0319 recovers fixed-baseline H1/H7/H39
    `stmtim` observe values, pairs them with regenerated OpenWEPP
    `snow.hourly.stmtim.*_0011` traces, classifies the divergence as
    `stmtim-active-interval-divergence-hold` caused by baseline adjusted
    `wnttim = 1` versus OpenWEPP `wnttim = 0`, preserves the combined `57`
    carried rows, assigns source-line classification to HPHYS0320, and
    authorizes no production physics or downstream water-balance edit.
- `20260606-hphys0318-stmtim-control-surface-instrumentation-001/`
  - Purpose: add contract-backed OpenWEPP SIMIMPL28 `stmtim` control-surface
    diagnostics for the 2013 day 11 hour 11 positive-`hrsnow` route while
    preserving the HPHYS0317 no-production-edit hold.
  - Status: executed-hold; HPHYS0318 implements OpenWEPP-side
    `snow.hourly.stmtim.*` runtime and HPHYS0245 trace instrumentation,
    preserves the combined `57` carried rows, records fixed-baseline paired
    `stmtim` observe as still unavailable, assigns that recovery to HPHYS0319,
    and authorizes no precipitation-phase, snow-producer, or downstream
    water-balance edit.
- `20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001/`
  - Purpose: join the HPHYS0315 spring-2014 `24` rows and HPHYS0316
    spring-2016 `33` rows under the 2013 day 11 hour 11 positive-`hrsnow`
    hourly snowfall input-surface blocker and classify ownership before any
    production edit.
  - Status: executed-hold; HPHYS0317 preserves all `57` carried rows as
    `paired-input-surface-instrumentation-hold`, assigns paired controlling
    surface instrumentation to HPHYS0318, statically carries forward same-
    runtime H1..H39 metrics because no production code changed, and authorizes
    no producer or downstream water-balance edits.
- `20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/`
  - Purpose: reclassify HPHYS0298-HPHYS0313 H1/H7/H39 snow/`RM` and
    water-balance findings under ADR0017, consolidate the current route ledger,
    record full H1..H39 metrics, and publish owned continuation order before
    any implementation package proceeds.
  - Status: executed-hold; route ledger reclassifies all `57` carried rows,
    statically carries forward same-runtime metrics because no production code
    changed, and authorizes no production physics edits.
- `20260606-hphys0315-hourly-snowfall-input-lineage-closure-001/`
  - Purpose: diagnose and, only if source-line evidence authorizes it, correct
    the branch-gated hourly snowfall input lineage where fixed baseline records
    positive `hrsnow` but openWEPP records zero homologous snowfall for the
    H1/H7/H39 spring-2014 settling-route rows.
  - Status: executed-hold; HPHYS0315 preserved all `24` carried spring-2014
    rows as `UNRESOLVED`/`forcing-input-surface-parity-hold`, assigned
    follow-on closure to HPHYS0317, statically carried forward same-runtime
    H1..H39 metrics because no production code changed, and authorized no
    producer or downstream water-balance edits.
- `20260606-hphys0316-2013-terminal-carry-recursion-closure-001/`
  - Purpose: recurse H1/H7/H39 spring-2016 year-start inherited snowpack rows
    into the 2013 terminal carry chain feeding 2014 day 1 hour 1 and classify
    the first material paired divergence before any production edit.
  - Status: executed-hold; HPHYS0316 routed all `33` carried spring-2016 rows
    through the 2013 terminal carry state to the same 2013 day 11 hour 11
    hourly snowfall input-surface blocker owned by HPHYS0317, statically
    carried forward same-runtime H1..H39 metrics because no production code
    changed, and authorized no producer or downstream water-balance edits.
- `20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/`
  - Purpose: execute the HPHYS0312 required continuation by reconstructing the
    full-precision 2013 settling/depth route and recursively scanning the 2014
    terminal carry-state chain feeding 2015 day 1 hour 1.
  - Status: executed-hold; diagnostics represented all `57` HPHYS0309 rows
    carried by the six HPHYS0312 inherited terminal groups, routing `3` to
    `hourly-snowfall-input-lineage-hold`, `3` to
    `recursive-year-start-inherited-state-hold`, and authorizing no production
    edits.
- `20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/`
  - Purpose: execute the HPHYS0311 required continuation by scanning the prior
    calendar year for each inherited terminal snowpack delta and classifying
    the first material fixed-comparator/openWEPP snowpack divergence before any
    producer or downstream water-balance edit.
  - Status: executed-hold; diagnostics represented all `57` HPHYS0309 rows
    carried by the six inherited HPHYS0311 groups, routing `3` to
    `settling-depth-update-hold`, `3` to
    `year-start-inherited-state-hold`, and authorizing no production edits.
- `20260605-hphys0311-snow-carry-source-line-parity-closure-001/`
  - Purpose: execute the HPHYS0310 required continuation by comparing
    fixed-comparator `snowd.for`/`winter.for` carry-state source lines against
    openWEPP snow runtime projection and hourly snow-state update code for the
    seven HPHYS0310 first-divergence groups.
  - Status: executed-hold; diagnostics represented all `58` HPHYS0309 rows as
    `7` HPHYS0310 groups, routing `6` to
    `prior-year-terminal-state-hold`, `1` to
    `fixed-observe-precision-hold`, and authorizing no production edits.
- `20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/`
  - Purpose: execute the HPHYS0309 required continuation by reconstructing the
    first prior-day/day-start snowpack carry-state divergence for affected
    H1/H7/H39 groups before any snow-producer, branch-predicate, or downstream
    water-balance edit.
  - Status: executed-hold; diagnostics represented all `58` HPHYS0309 rows as
    `7` hillslope/window/year groups, routing `6` to
    `initial-carry-state-projection-hold`, `1` to
    `density-settling-carry-state-hold`, and authorizing no production edits.
- `20260605-hphys0309-snow-carry-depletion-lineage-closure-001/`
  - Purpose: execute the HPHYS0308 required continuation by comparing
    fixed-comparator prior-day/hour snow carry state against openWEPP
    day-start runtime and hourly before/after depletion timing for the `58`
    baseline-extra snow-state carry holds before any branch-predicate,
    melt-term, or downstream water-balance edit.
  - Status: executed-hold; diagnostics classified all `58` HPHYS0308
    snow-state carry holds as prior carry-state lineage (`45`
    `pre-day-carry-deficit-hold`, `13` `prior-day-openwepp-meltout-hold`) and
    authorized no production edits.
- `20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/`
  - Purpose: execute the HPHYS0307 required continuation by extracting exact
    branch-extra key state surfaces and predicate outcomes from fixed-baseline
    `winter.for`/`snowd.for` observe data and openWEPP
    `snow_hourly_melt_branch_active` traces before any melt-term or downstream
    water-balance edit.
  - Status: executed-hold; key-level diagnostics classified `58`
    baseline-extra keys as `snow-state-carry-depletion-hold`, one H7
    first-2013 openWEPP-extra key as `baseline-branch-instrumentation-hold`,
    and authorized no production edits.
- `20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/`
  - Purpose: execute the HPHYS0306 required continuation by comparing fixed
    baseline `winter.for`/`snowd.for` melt-call branch activation against
    openWEPP `snow_hourly_melt_branch_active` publication semantics before any
    numeric melt-term correction or downstream compensation.
  - Status: executed-hold; HPHYS0307 classified seven rows as
    `baseline-extra-melt-call-hold`, one H7 first-2013 row as
    `openwepp-extra-melt-call-hold`, and H39 first-2013 as
    `same-hour-multi-source-hold`, with no production edit authorized.
- `20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/`
  - Purpose: close the HPHYS0305 missing-`amelt` blocker by making
    branch-active/inactive fixed-baseline melt-term observe semantics explicit,
    reclassifying H1/H7/H39 paired melt-term/forcing/snow-state surfaces on the
    correct comparison domain, and preserving the no-production-edit gate until
    a source-owned divergence is identified.
  - Status: executed-hold; branch-active reclassification eliminated the
    HPHYS0305 inactive-hour zero-imputation ambiguity, routed eight windows to
    melt-call mask divergence, routed H39 first-2013 to same-hour
    `cmelt`/`snodpt` divergence, and authorized no production physics edits.
- `20260605-hphys0305-paired-melt-term-state-instrumentation-001/`
  - Purpose: implement the second ADR-0016 Required Continuation Order step by
    instrumenting paired fixed-baseline/openWEPP melt-term and snow-state
    surfaces (`amelt`, `bmelt`, `cmelt`, `dmelt`, forcing lanes, `snodpt`,
    `densgt`) for the H1/H7/H39 snow/`RM` target windows before any producer
    or downstream hydrology edit is authorized.
  - Status: executed-hold; paired instrumentation ran and proved fixed observe
    identity, but all nine target windows remained blocked by incomplete
    baseline `amelt` paired surfaces, routing continuation to HPHYS0306
    branch-active observe semantics closure.
- `20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/`
  - Purpose: execute ADR-0016 Required Continuation Order step 1 by rerunning
    the H1..H39 semantic suite against the fixed `wepp_260430` comparator
    baseline and reclassifying H1/H7/H39 snow/`RM` target windows under
    ADR-0011 confidence tiers.
  - Status: executed-hold; fixed-baseline semantic rerun produced `0/39`
    semantic-pass hillslopes with no structural row/key failures and no
    material movement from HPHYS0302 metrics, reclassified all nine target
    windows as `fixed-baseline-unchanged-term-state-hold`, preserved the
    HPHYS0302 production-edit `HOLD`, and scaffolded HPHYS0305 for paired
    melt-term/state instrumentation.
- `20260605-hphys0303-adr0016-fixed-comparator-ratification-001/`
  - Purpose: execute the local ADR-0016 ratification work for the fixed
    `wepp_260430` comparator anchor: preserve immutable archaeology tags,
    create a fixed negative-melt comparator branch/tag, rebuild and hash fixed
    binaries, regenerate baseline comparator artifacts where feasible, amend
    ADR-0012/ADR-0016 and negative-melt provenance citations, and carry the
    HPHYS0302 production-edit HOLD forward.
  - Status: executed-accepted-ready; execution was local only, did not push
    remote refs, created fixed comparator commit
    `47ac4c32faeea81bb99081f955a14c38b815ef4d`, regenerated H1..H39 fixed
    baseline comparator parquets with year/key validation, proved H1/H7/H39
    observe identity, passed SC unit/provenance lint after contract-table
    amendments, and amended ADR-0016/ADR-0012 plus negative-melt provenance
    citations. HPHYS0302 production-edit `HOLD` remains active; H1..H39
    openWEPP-vs-fixed-baseline semantic rerun remains required continuation.
- `20260605-hphys0302-comparator-surface-audit-closure-001/`
  - Purpose: audit comparator surfaces for `RM`, `Snow-Water`, and melt-term
    lineage across H1/H7/H39 target windows before any new producer-defect
    conclusion or production patch.
  - Status: executed-hold; comparator-surface audit emitted 45 surface rows:
    `RM` passes as daily WB13/WAT like-for-like output, `Snow-Water` passes as
    a daily output surface, raw `hrmlt` and post-raw `wmelt` pass only as
    aggregate cut-point surfaces, and all nine term-level melt rows remain
    blocked on missing paired baseline `amelt`/`bmelt`/`cmelt`/`dmelt`
    term-state surfaces; no production edit is authorized.
- `20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/`
  - Purpose: resolve the H39 first-2013 HPHYS0300 forcing lane by reconciling
    baseline residual rain-on-snow evidence against openWEPP raw, retained,
    released, post-winter rain, raw melt, and routed melt traces before any
    production forcing or snow-producer edit.
  - Status: executed-hold; contract-first forcing-function package reclassified
    H39 first-2013 from raw forcing correction to residual-rain/release
    lineage after the apparent `-16.476986 mm` raw-rain delta collapsed to
    `-0.237193 mm` against openWEPP released plus post-winter rain; no
    production forcing, snow-producer, or downstream WB17/WB18/WB19/WB13 edit
    is authorized, and Claude Code review now requires the next package to
    audit comparator surfaces for `RM`, `Snow-Water`, and melt-term lineage
    before any producer-defect conclusion or paired `melt.for` / `snowd.for`
    implementation work.
- `20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/`
  - Purpose: localize and, only if contract-authorized, correct the raw
    hourly `hrmlt` and post-raw `wmelt`/routed-melt lineage for the H1/H7/H39
    target windows after HPHYS0299 corrected the `hrsnow` unit/provenance seam.
  - Status: executed-hold; contract-first diagnostics and same-HEAD full
    H1..H39 metrics ran, all raw/post-raw rows remain aggregate-only
    term/state holds, no production edits are authorized, and continuation must
    use a forcing-function package: fix the independently localized H39
    first-2013 forcing seam when source-line proof is confirmed, instrument
    paired `melt.for`/`snowd.for` term/state lineage for raw-melt windows, and
    implement the producer correction once a named source is isolated instead
    of opening another diagnostic-only package.
- `20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/`
  - Purpose: correct and validate the HPHYS0298 hourly snow/rain partition
    lineage by comparing pinned-baseline `hrsnow` snow-depth to openWEPP
    `snow_hourly_snowfall_depth_sum_m`, not derived snowfall water equivalent,
    before authorizing any producer physics migration or downstream hydrology
    focus change.
  - Status: executed-hold; corrected depth-vs-depth diagnostics supersede
    HPHYS0298's all-window hourly-forcing verdict, routing seven windows to
    raw hourly melt, one H7 first-2013 row to post-raw routed-melt/negative-melt
    follow-on without legacy-defective acceptance, and only H39 first-2013 to a
    remaining corrected-depth hourly-forcing defect; no downstream compensation
    is authorized.
- `20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/`
  - Purpose: implement one large paired baseline/openWEPP snow-`RM` lineage
    observation strategy for all nine H1/H7/H39 target windows, proving
    baseline observe identity and assigning first-divergent cut-point verdicts
    before any downstream hydrology compensation.
  - Status: executed-hold; paired diagnostics ran and were reviewed, but
    HPHYS0299 now audits a discovered diagnostic unit/provenance seam before
    treating the all-window hourly-forcing verdict as production migration
    authority.
- `20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/`
  - Purpose: convert HPHYS0296 snow/`RM` candidates into an auditable defect
    ledger by reconstructing pinned-baseline negative-melt behavior against
    openWEPP traces, assigning per-window verdicts, and preserving spring-2016
    producer holds without downstream compensation.
  - Status: executed-hold; contracts/tests and full H1..H39 diagnostics ran,
    all nine H1/H7/H39 target windows remain `UNRESOLVED`, the pinned-baseline
    negative-melt branch alone does not reconstruct baseline `RM` to the named
    `2.000 mm` tolerance, and no downstream WB17/WB18/WB19/WB13 compensation or
    residual re-tiering is authorized.
- `20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/`
  - Purpose: classify H1/H7/H39 snow/`RM` producer residuals into an auditable
    defect-ledger posture before any downstream hydrology compensation.
  - Status: executed-hold; contracts/tests and full H1..H39 diagnostics ran,
    Claude Code review tightened the acceptance gate, six target windows remain
    unresolved corrected-negative-melt candidates pending reconstruction and
    independent correctness verdicts, all spring-2016 target windows remain
    snow/winter producer magnitude/timing holds, and no downstream
    WB17/WB18/WB19/WB13 compensation patch is authorized.
- `20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/`
  - Purpose: diagnose cumulative row-to-row storage-budget ownership across
    WB17 `Ep`/`Es`, WB18 `D`, WB19 `latqcc`, and HPHYS0293 excluded snow/`RM`
    masks before any production physics edits.
  - Status: executed-hold; contracts/tests and full H1..H39 diagnostics ran,
    H1/H7/H39 cumulative windows are dominated by snow/`RM` residuals with
    small residual budget gaps, and no WB17/WB18/WB19/WB13 production patch is
    justified without a follow-on snow/`RM` producer acceptance or authority
    alignment package.
- `20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001/`
  - Purpose: diagnose post-ingress `Total-Soil`/`SoilWaterTotal` residual
    ownership after HPHYS0293 snow-producer residual exclusion and HPHYS0292
    `Q` closure, separating WB18 aggregate/percolation identity from WB19
    lateral-retention magnitude before production edits.
  - Status: executed-hold; contracts/tests and full H1..H39 diagnostics ran,
    WB18 aggregate identity and `D=Pe` close on H1/H7/H39 target rows, WB19
    target/unrealized lineage is internally closed, and no production WB18/WB19
    patch is justified without a follow-on cumulative storage-budget owner.
- `20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/`
  - Purpose: diagnose and correct baseline-authoritative winter melt
    magnitude/timing and spring snowpack depletion after HPHYS0292 closed WB14
    routed-melt infiltration capacity and `Q` parity.
  - Status: executed-hold; contracts/tests now classify HPHYS0293 snow
    producer depletion separately from WB14, full H1..H39 still has semantic
    parity `0/39` with `Q` parity `39/39`, and target rows show internally
    closed snow-state accounting but persistent corrected-negative-melt snow
    producer residuals versus the pinned comparator.
- `20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/`
  - Purpose: diagnose and correct baseline-authoritative spring snowmelt
    producer partitioning upstream of WB13, separating melt magnitude/timing
    from WB12 infiltration-capacity and `Q` ownership for H1/H7/H39.
  - Status: executed-hold; WB14 now conserves routed snowmelt mass while using
    producer hourly melt timing, target H1/H7/H39 spring rows infiltrate routed
    melt before residual `Q`, and full H1..H39 runtime passes with `Q` parity
    `39/39`, but semantic parity remains `0/39` and dual independent
    review/verification was not dispatched under current tool policy.
- `20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/`
  - Purpose: guard the same-day snow publication flux lifecycle from runoff
    reconciliation through WB13, then localize remaining H1/H7/H39
    snowpack/liquid partition residuals without changing WB13 publication math.
  - Status: executed-hold; WB13 now consumes fail-closed flux-only
    `snow.post_winter_rain_m + snow.routed_melt_m + Irr`, runoff
    reconciliation publishes both same-day snow fluxes, and trace localization
    is flux-only, but full H1..H39 semantic parity remains `0/39`.
- `20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/`
  - Purpose: publish and consume an explicit post-winter `rain(iplane)`
    equivalent for WB13 `RM`, replacing the HPHYS0289 inference branch.
  - Status: executed-hold; WB13 now consumes fail-closed explicit
    `snow.post_winter_rain_m + snow.routed_melt_m + Irr`, and H39 2014-146 is
    proven to be baseline warm-rain/no-snow restoration rather than WB13
    inference, but full H1..H39 semantic parity remains `0/39`.
- `20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/`
  - Purpose: diagnose and correct WB13 `RM`/`Snow-Water` publication lineage
    so daily rows consume baseline-authoritative post-winter
    `rain + wmelt + irrigation` and runtime snowpack storage surfaces.
  - Status: executed-hold; WB13 `RM` now consumes routed `wmelt` and
    `Snow-Water` consumes runtime SWE, but full H1..H39 semantic parity
    remains `0/39`; continue with explicit post-winter rain publication
    seam.
- `20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/`
  - Purpose: diagnose and correct baseline-authoritative residual rain-on-snow
    release into the winter `hrmlt`/`wmelt` runoff/infiltration forcing seam,
    preserving HPHYS0287 fail-closed snow-state guards.
  - Status: executed-hold; residual rain-on-snow release now routes into
    snowmelt forcing and improves `Ep`/storage/lateral metrics, but full
    H1..H39 semantic parity remains `0/39` and `Q`/`RM`/`Snow-Water` are
    effectively unchanged for continuation.
- `20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/`
  - Purpose: diagnose and correct snow liquid retention, runoff, and
    infiltration partition residuals left after HPHYS0286.
  - Status: executed-hold; restored fail-closed runtime snow-state guarding
    before inactive snow fallback and liquid partition; this was guard
    hardening, not valid-run snow-magnitude parity progress. Full H1..H39
    metrics are unchanged from HPHYS0286, leaving semantic parity open at
    `0/39`.
- `20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/`
  - Purpose: diagnose and correct post-ingress layer capacity/retention and
    WB18/WB17 coupling after HPHYS0285 same-pass liquid ingress.
  - Status: executed-hold; post-ET lower-layer upper-limit redistribution
    fixed and metrics improved, but `Q`, `RM`, and `Snow-Water` remained
    unchanged for HPHYS0287 continuation.
- `20260604-hphys0285-spring-soil-storage-retention-closure-001/`
  - Purpose: diagnose and correct the post-HPHYS0284 spring liquid/profile
    storage retention residual across infiltration capacity, WB18 percolation,
    and aggregate `Total-Soil` lineage.
  - Status: executed-hold; same-pass liquid ingress fixed and metrics improved,
    but H1..H39 semantic parity remains `0/39` for continuation.
- `20260604-hphys0284-spring-snowpack-retention-timing-closure-001/`
  - Purpose: diagnose and correct the post-HPHYS0283 spring snowpack
    timing/retention residual in H1/H7/H39 before returning to downstream
    `Ep` or aggregate-storage residuals.
  - Status: complete; corrected negative-melt snowpack carry-state lineage,
    improved `Snow-Water`/`RM`/`Q` semantics, and left downstream
    storage/runoff residuals for continuation.
- `20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/`
  - Purpose: localize and correct the spring 2014 snowmelt runoff/infiltration
    partition causing H1..H39 `Total-Soil` collapse after the post-0281
    rebaseline showed no movement in Ep/storage metrics.
  - Status: complete; contract-first package scoped to baseline-authoritative
    `wmelt` infiltration/runoff coupling, targeted H1/H7/H39 traces, full
    H1..H39 semantic metrics, and dual review/verification disposition.
