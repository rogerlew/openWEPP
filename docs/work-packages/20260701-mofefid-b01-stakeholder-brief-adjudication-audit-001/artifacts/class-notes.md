# B01 Class Notes — working evidence

Evidence-mode legend per note: **Ran** = command/read executed this package;
**Static** = reasoned from held documents. Legacy source = current
`/workdir/wepp-forest/src/` (post-fix state, HEAD `924ab16d`), read directly
per the adjudication protocol; the openWEPP anchor `wepp_260430_hill`
predates the 2026-05 fixes, so pre-fix semantics are triangulated from
(current source × brief × openWEPP's own expressions).

## Independent legacy source reads (Ran, 2026-07-01)

### B1 — q-cap (`watbal_hourly.for`)
- Current source: `qcap_mm = (runoffin(iplane)*1000. + rmloc_mm + subrin*1000.)`
  (`watbal_hourly.for:1009`), binding check `qofe_mm > qcap_mm` (`:1014`),
  `efflen > slplen` now selecting hard-cap vs soft-limiter behavior
  (`:1017`) rather than gating enforcement.
- **Adjudication-relevant discrepancy:** the brief narrates the q-cap as a
  *hydraulic transport capacity* ("maximum rate at which surface flow can
  move across the strip given its slope, length, and surface roughness").
  The source computes an **availability cap** — runoff out of an OFE bounded
  by water into it (runon + local rain/melt + subsurface runon). The
  *observation* (bottom-OFE runoff exceeding supplied water) is
  conservation-forced; the *"transport capacity" framing* is not what the
  code enforces. Grade the observation and the narrative separately.

### B5 — rain-routing conflation (`winter.for`)
- The aliasing site exists and is now dead code: `cd wmelt(iplane) =
  wmelt(iplane) + hrrain(hour)` (`winter.for:381`, `cd`-commented). The
  Candidate-1 repair is visible in current source; probe records
  `rain_added = 0.0` in the hourly decomposition (`:459` region).
- Confirms the *problem observation* from source history: rain hours were
  added into the melt accumulator pre-fix.

### B7 — QOFE denominators (`watbal.for`)
- Current source: both runoff columns in the WAT write use
  `runoff(iplane)*1000.*efflen(iplane)/totlen(iplane)` — `QOFE ≡ Q` — with
  the 2008 comment in place ("changed runoff value to use cumulative length
  (totlen) because efflen may span OFE's. Matches event output code in
  sedout.for 6-13-2008 dcf,jrf").
- openWEPP (Ran, `01_publication.rs:370-376`): `qofe = q_ofe_m × efflen /
  ofe_length` — the pre-fix `slplen` denominator — and
  `runvol = QOFE × per-OFE area` (the legacy cancellation recipe).
- The physical volume was never wrong on either side; this is column
  *semantics*. The ecosystem contract moved (wepp_260516 + wepppy consumer
  re-anchoring documented in the brief §"Audit and consumer formulas").

### B10 — mixed-melt day-end aggregation (`winter.for:430-466`)
- Current source carries dated corrections ("Fixed 2026-05-16"): net-sign
  test `pstvML + ngtvML <= 0` (ngtvML accumulated as a negative sum) and
  scaling `hrmlt × (1 + ngtvML/pstvML)` so refreeze hours *reduce* positive
  melt. Source intent: mixed thaw/refreeze hours net against each other.
- Brief also reports the defective branch was empirically unreachable in
  their 1,166-hillslope cohort (class3 mixed-melt days = 0 in 21.7M
  winter-active OFE-day rows) — caps practical weight.

### B3 — interception storage (legacy state)
- `pintlv(iplane)` and `resint(iplane)` are real legacy states; the current
  WAT write publishes `(pintlv+resint)*1000.` as the trailing column (the
  WB02-CC-20260504-02 addition). Also observed: the `rm` column remains in
  the write (B2's input-basis column still published; the B2 "fix" was to
  the audit definition, not the outputs — consistent with grading it
  `convention`).

## openWEPP-side evidence — closure basis cluster (B2/B3/B4/B9)

Sources: explorer sweep + my own verification reads of the two load-bearing
sites (**Ran**: `storage.rs:800-845`, `projection.rs:167-204`).

### The enforced identity (basis for B2/B4/B9)
Per-(lane, day) R4B reconciliation (`direct_runtime/storage.rs:804-814`):

```
storage_reconciled = storage_initial
  + precip_input + snow_coupling + runon_input
  + frost_liquid_delta + et_storage_return
  - interception - q_runoff - evapotranspiration
  - deep_seepage - subsurface_loss
```

- **External atmospheric input = precipitation only**; irrigation absent
  from the identity and hardcoded `0.0` at publication
  (`01_publication.rs:301`). Snowmelt enters as the **typed signed flux**
  `snow_coupling_m` (producer `snow_coupling_signed_s_m`); SWE is a
  parallel reservoir reconciled inside the snow producer and published as
  the `Snow-Water` state column. **No term is counted both as input and as
  storage-delta** — the RM double-count is structurally absent.
- **Guard structure (verified):** the R4B `closure_residual_m` re-evaluates
  the same RHS minus `storage_reconciled_m` — algebraically zero; it can
  only catch non-finite arithmetic. The **substantive** conservation
  guards are (a) `validate_nonnegative_direct_m(storage_reconciled_m)`
  (`storage.rs:819-822`) — a zero-input day with outputs exceeding
  available storage fails here; (b) the projection-side **ledger-vs-state**
  check `|Σ layers − storage_reconciled| ≤ tol` plus the frozen-layer
  bound (`projection.rs:179-204`) — a flux ledger that doesn't move the
  layer state fails here. This two-basis shape is exactly the class of
  check that caught the brief's WB-05F defect on their side.
- **Granularity:** enforced per OFE-day by construction (day×lane frames,
  `03_executor.rs:128-136`; failures name lane/day). Dry days run the same
  identity — no special-casing to audit (B9).
- Hardening note (→ A01): the tautological `closure_residual_m` naming
  invites false confidence in contract citations; the real guards are the
  nonnegativity + ledger-vs-state pair. Not a defect.

### B3 — interception storage (openWEPP side)
- **openWEPP has no interception-storage state at all.** Interception is a
  same-day flux (`compute_direct_canopy_interception` from cover/LAI/live
  biomass, `00_builders_and_authority.rs:2054-2065`), subtracted as a loss
  in R4B; throughfall feeds liquid input. The WAT `InterceptionStorage`
  column exists in the schema but the direct producer always emits `None`
  (`01_publication.rs:340-343`).
- Consequence: the legacy defect (real persisted state invisible to
  audits) **cannot occur** here — there is no hidden reservoir. The
  *fidelity* question (legacy `pintlv`/`resint` carry across days and
  evaporate later; openWEPP loses interception same-day) is a known
  deferred backlog item
  (`docs/backlog/20260512-residue-moisture-storage-full-state.md`), not a
  closure defect.

### B2 consumer note
openWEPP's WAT still publishes the legacy `RM` column alongside `P`. Any
downstream reader computing closure with `RM` as external input while also
differencing `Snow-Water` inherits the legacy double-count *in their own
audit*. Worth a one-line consumer caveat in usersum WAT documentation
(follow-up, not a model change).

## openWEPP-side evidence — runoff/event/diagnostics cluster (B1/B8/B9/B11)

- **B1:** openWEPP has **no q-cap of any kind** — and needs none for the
  legacy failure mode: `compute_r4a_runoff_partition` (`runoff.rs:652-688`)
  computes runoff **as the conservation residual** `liquid + runon −
  infiltration − Δdepression − frost_retained (≥0) + saturation_excess`.
  Runoff structurally cannot exceed supplied water; legacy's hourly path
  computed `qofe` from independent hydrograph machinery that could (hence
  its patch). The legacy cap itself (my source read,
  `watbal_hourly.for:1009`) is an *availability* bound, not the hydraulic
  transport capacity the brief narrates. Dormant `QcapSoftLimit` taxonomy
  value exists but is never emitted (`status.rs:113`) → A01 note.
- **B8:** openWEPP has **no event counter at all** — no `sumrun` analog;
  `RM` is an additive rain+melt depth bucket (`01_publication.rs:300`), and
  totals are sums of daily depths, never event-gated accumulations. The
  legacy over-counting mechanism has no surface to exist on.
- **B9:** no dry-day special-casing anywhere; the per-OFE-day identity +
  nonnegativity + ledger-vs-state guards run identically on dry days.
  "No-flux-without-storage-change" as contract text lives in
  `INV-PERC-016/017`; runtime enforcement is via the verified guard pair.
- **B11:** openWEPP currently has **no external WAT/PASS closure-audit
  tool** (owcmp is per-column diff; snow tools audit internal traces), so
  the over-reach cannot occur today — but the lesson (distinguish
  `latqcc`-only days from surface pulses) becomes a **design constraint on
  Lane C2 and any future exported-surface audit**. Related gap → A01-S4:
  per-OFE closure is not currently reconstructable from exports by an
  external reader (`INV-WATBAL-096` warns row aliases are structural only).

## openWEPP-side evidence — winter cluster (B5/B6/B10)

- **B5:** openWEPP **deliberately ratifies the legacy rain-into-melt
  lineage** the brief calls defect #6: rain retained/released by the pack
  is promoted into `routed_melt_m`; only pack-untouched rain survives as
  `post_winter_rain_m` (`runoff_reconciliation.rs:143-158`, finalize
  `infiltration_reconciliation.rs:1671-1672`). This is contract, not
  accident: `INV-SNOWFREEZE-015/021/023`, `INV-WATBAL-055`,
  `REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE`. The conservation hazard that
  broke wepp-forest's kernel is neutralized structurally: the melt-cap
  availability basis **includes retained rain** (`swe + accumulation +
  rain_retained`, `:1643-1644`), overdraw **hard-fails** rather than
  silently clamping (`:1778-1789`), and closure is enforced by the
  ledger-vs-state guard. Channel attribution on rain-on-snow days differs
  from post-Candidate-1 legacy **by design** (our anchor `wepp_260430`
  predates Candidate 1) → comparator-hygiene entry.
- **B6:** the clamp-plus-preserve pattern is structurally absent: outputs
  derive *from* the bounded quantities (SWE-after computed from bounded
  losses), raw uncapped values survive only as diagnostics that "cannot
  create a second SWE debit", and overdraw is a hard-fail (correctness-
  over-completion, the same posture as their Phase-3 gate). No
  baseflow-preservation machinery exists to interact with a cap.
- **B10 — FINDING (contract-vs-implementation tension).** Verified reads:
  `INV-SNOWFREEZE-015` ratifies the **corrected legacy net algebra**
  ("compares net daily melt (pstvML + ngtvML) and, when positive, scales
  positive hourly melt by 1 + ngtvML/pstvML"; original branch rejected as
  bug-compatible archaeology). The implementation
  (`redistribute_daily_signed_snowmelt`,
  `runoff_reconciliation.rs:1133-1162`) instead routes the
  **positive-parts sum** (per-hour `max(0)`, then sum) under the
  SNOWSCI-S1 single-source rationale (routed melt must equal the positive
  water-equivalent loss already applied to the depth/density store;
  negative raw melt is diagnostic-only). On a mixed-sign day these
  disagree: net-algebra routes `pstv+ngtv` (or zero), positive-parts
  routes `pstv`. Empirical reachability is doubtful (the brief's own
  cohort scan found 0 mixed-melt days in 21.7M winter OFE-day rows, and
  months of H2637 byte-identity never tripped), and conservation holds
  either way (single-source store + closure guards). Disposition:
  **contract-consistency follow-up** — reconcile the INV-SNOWFREEZE-015
  text with the SNOWSCI-S1 implementation (amend the invariant to ratify
  positive-parts-under-single-source, or demonstrate equivalence on the
  reachable domain). Not a proven runtime defect.
