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
