# Snow Code Science Review (Two-Stage)

## Status

- `state`: **promoted (staged)** — Stage 1 active; Stage 2 backlog
- `date`: 2026-06-05 (created); 2026-06-06 (promoted and split into two stages)
- `relates`: [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
  [ADR-0018](../decisions/0018-defect-closure-execplans-conversion-rule.md)
- Stage 1 work package:
  `docs/work-packages/20260606-snowsci-stage1-snow-mass-conservation-closure-001/`
- Stage 2: remains backlog, `default_path: not eligible`, behind the snow
  physics-magnitude protected boundary.

## Why this is now two stages

This note began as a single deferred dossier. Static analysis of the J-95
negative-SWE site (2026-06-06) showed the snow problem actually separates into
two questions with different authority, urgency, and risk:

1. **Snow mass conservation (Stage 1, a hard gate, now).** The snow store is
   multi-represented (`snow.runtime_swe`, `snow.runtime_depth_m`,
   `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count` are carried as
   separate, separately-guarded quantities and reconciled across the
   projection → WB14 redistribution → WB13 publication → next-day carry chain).
   The mass balance does not close by construction — the runner already
   instruments a `snow_runtime_swe_closure_error_m`, and negative SWE
   (`-0.006171 m` at J-95) is defended by four accreted `>= 0` guards at the
   consumers rather than prevented at the source. This is an
   **accounting/architecture** defect: it drives `SWE < 0` and is the leading
   candidate cause of the WBVAL06 conservation residual (`R > 0`, water
   vanishing). Conservation/bounds are hard gates (ADR-0011 + correctness
   re-anchoring), and this one sits **on** rung-1's water-balance closure gate,
   so it cannot be deferred past the gate it blocks.

2. **Snow physics magnitude (Stage 2, deferred).** Whether the implemented
   `snowd.for` melt/settling/density/partition *equations* are physically
   correct against external authority (CRM Chapter 3.7, WEPP User Doc) — the
   `XXX` markers, blank `driftf`/`driftg` definitions, the
   daily-temperature-in-an-hourly-loop threshold, and the Eq. 3.7.5
   code-vs-documentation divergence. This is a **magnitude/correctness** question,
   not a closure question. It stays behind the protected boundary and trails
   Stage 1.

The dividing line is the Stage-1 protected boundary: **Stage 1 makes the snow
store conserve and single-source by construction; it does not change any snow
physics-magnitude equation.** If conservation cannot be achieved without changing
a physics-magnitude equation, Stage 1 stops at that boundary and escalates the
specific equation to Stage 2 — that escalation is a success (it proves
separability is false for that term), not a failure.

This split is what keeps "do the review now" from meaning "halt rung-1 until the
entire 19-years-deferred physics dossier is done." Stage 1 is bounded, has a
crisp conservation acceptance, and unblocks rung-1; Stage 2 remains the deferred
science.

## Stage 1 — Snow Mass Conservation and Single-Sourcing (promoted)

Owner package:
`docs/work-packages/20260606-snowsci-stage1-snow-mass-conservation-closure-001/`
(Defect-Closure ExecPlan).

Scope (summary; full envelope in the package):

- Single-source the snow store so `SWE >= 0` holds by construction and
  `in = out + ΔStorage` closes within a named tolerance; depth/density are
  derived from or reconciled to the authoritative store rather than carried as
  independent quantities that can drift negative.
- Make the existing `snow_runtime_swe_closure_error_m` an enforced gate that
  reads ~0 on valid runs; the four `>= 0` snow-state guards become assertions
  that never fire rather than failure points.
- Subsumes the WBVAL05 negative-SWE follow-on
  (`HKERNEL-WB14-RUNOFF-E-003` on `snow.runtime_swe = -0.006171` for
  `p7`/`p11`/`p18`/`p20`) and `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`,
  pending the package's own common-cause confirmation.

Authority: conservation/physical invariants + `SC-SNOWFREEZE-001` /
`SC-WATBAL-001` contract text — **not** legacy-baseline replication (the legacy
snow physics is exactly what Stage 2 adjudicates).

Protected boundary: no snow physics-magnitude change, and no silent clamp of
negative SWE to zero (a clamp converts the visible fail-closed into the invisible
WBVAL06 leak — forbidden by ADR-0018).

Provenance (2026-06-06 upstream check): the negative SWE is **openWEPP-introduced,
not inherited**. The pinned baseline, the newer wepp-forest tip (its only snow diff
is `watbal_process_probe` instrumentation), `jimf-wepp-2023` (mainline),
`wepp-forest-wb61`, and `wepp-forest-revegetation` all carry the identical
`snowd.for` negative-pack guard (`if (snodep .le. 0.0) then
wmelt = snodpt*densg*0.001; snodep = 0.0; densgt = 0.0`). Legacy is
depth-authoritative and conserving by construction — it never produces negative
pack — so there was nothing to patch upstream. openWEPP's SWE-centric multi-carry
is the regression. This **lowers Stage-1 separability risk**: legacy proves a
conserving snow accounting exists without resolving any Stage-2 physics question,
and its depth-authoritative clamp is a conservation reference pattern (corroborating
evidence, not a magnitude oracle, per ADR-0017) for Milestone 1's localization.

## Stage 2 — Snow Physics-Magnitude Science Review (deferred)

`state`: backlog, `default_path: not eligible`. Remains behind the snow
physics-magnitude protected boundary until Stage 1 closes and a promotion
decision is made. Produces a contract-grade dossier that reconciles the legacy
`snowd.for` implementation against external authority or records owned scientific
questions before any production snow-physics-magnitude correction is authorized.

### Legacy Evidence (Stage 2 material)

All source citations refer to pinned baseline
`/workdir/wepp-forest_260430_baseline/src/snowd.for` unless stated otherwise.

| Evidence | Location | Why it matters |
|---|---:|---|
| Implemented equation diverges from documentation | `:295` — `XXX -- Note: This equation differs from the on[e] in the User Doc.` for Eq. 3.7.5 | The source records an unreconciled code-versus-documentation discrepancy. openWEPP needs a documented authority decision before treating either side as target behavior. |
| Density-mixing input is questioned in two branches | `:169`, `:183` — `XXX -- Shouldn't "snodpt" be replaced by the snow depth *yesterday* ?` | This directly affects `densgt` and carried snow depth in cold-snowing and melt-density branches. |
| `driftf` / `driftg` definitions are blank | `:18-19` | HPHYS0313 showed these terms can be misattributed. Their physical meaning, units, and active/dead status need explicit evidence. |
| Daily quantity controls an hourly branch | `:112-116` — Dun 2007 changed `hrtemp .lt. -4.0` to `(tmax+tmin)/2 .lt. 0`, with comment `a daily modle in a hourly way` | A daily mean temperature and shifted threshold control an hourly snow/melt regime. The change is documented in source but not reconciled to a physics specification here. |
| Density threshold/cap was edited in place | `:128-129` — prior `if(densgy.gt.250) setf = 1` commented and replaced with `if(densgy.gt.ssd) setf = 1` | The source preserves provenance of a threshold change, but not a contract-grade rationale or authority citation. |
| External snow-equation authority is cited | `:124`, `:137`, `:168`, `:182`, `:294` — CRM Eq. 3.7.1 / 3.7.2 / 3.7.3 / 3.7.5 | The source points to reviewable external authority. The missing work is reconciliation, not discovery. |

These are not style findings. They are unresolved process questions in the legacy
code path that controls snowpack state, density mixing, melt routing, and
water-balance publication surfaces.

### Stage 2 Proposed Review Scope

A future Stage-2 work package should produce a snow science-review dossier
covering: (1) an **evidence ledger** enumerating every snow `XXX`, dated edit,
blank definition, and documentation-divergence marker with file/line/symbol/
branch/affected surface; (2) a **dimensional audit** declaring units for
`snodpt`, `densg/densgt/densgy`, `hrsnow`, `hrmlt`, `driftf`, `driftg`, `wmelt`,
and density-settling terms, separating daily/hourly/state/publication surfaces;
(3) a **regime audit** of branch predicates and time-base assumptions, especially
the daily-mean-temperature condition inside hourly processing; (4)
**external-authority reconciliation** against CRM Chapter 3.7 and the WEPP User
Documentation, classifying each discrepancy as documented correction, legacy
defect, ambiguous scientific question, or intentionally replicated legacy wobble;
and (5) **contract-grade disposition** producing proposed `SC-SNOWFREEZE-001`
amendments or explicit `HOLD` artifacts.

### Stage 2 Open Questions

- Is the Eq. 3.7.5 / User Documentation divergence at `snowd.for:295` a legacy
  implementation defect, an undocumented correction, or a documentation error?
- Does density mixing in the flagged branches use current-hour snow depth or
  prior-day snow depth, and what does CRM Eq. 3.7.3 require?
- What are `driftf` and `driftg` physically and dimensionally? Are they active in
  the forest configuration or dead inputs?
- Is the daily-mean-temperature condition inside hourly processing defensible as a
  regime proxy, or does it require re-derivation?
- Which reviewed behavior, if any, should be preserved as documented legacy wobble
  rather than corrected physics?
- **Negative-melt pack/routing semantics (routed in from SNOWSCI-S1).** SNOWSCI-S1
  superseded the prior `INV-SNOWFREEZE-019` wepp-forest-negmelt-fix interpretation
  with a conservation-first treatment in which negative raw melt is diagnostic-only
  (no SWE debit, no routed-melt or pack effect). Stage 2 must physically ratify
  whether negative melt truly has no independent pack/routing role, or whether the
  wepp-forest negmelt-fix intended a real refreeze/sublimation pack term — and if
  the latter, reconcile it with the Stage-1 conservation rule.

### Stage 2 Promotion Criteria

Stage 2 becomes eligible for a work package only when it can name its authority
set and validation gates: a pinned `snowd.for`/`winter.for` source ledger; exact
CRM Chapter 3.7 and WEPP User Documentation references; specific
`SC-SNOWFREEZE-001` invariants/obligations to amend or create; review/specification
before implementation; and each unresolved scientific question carrying an owner,
next evidence gate, and follow-on trigger.

## Governance (both stages)

- Correctness authority remains ADR-0011 + canonical `SC-*` contracts +
  conservation and external physics authority. Legacy source is evidence of
  implementation and intent, not a correctness oracle.
- Any resulting change must be contract-first: canonical `SC-*` amendment →
  contract-derived tests → pre-implementation gate evidence → production
  implementation.
- This is an institutional maintenance gap, not a contributor-blame finding. The
  dated S. Dun 2007 edits are valuable because they preserve authorship, intent,
  prior behavior, and uncertainty in source comments; the deferred work is the
  custodian-side obligation to convert those markers into documented decisions.
- Non-goals: no silent adoption of legacy behavior as correct without external
  authority; no empirical compensation in WB13/WB17/WB18/WB19/WB12 for unresolved
  snow defects; no personal blame for historical comments or edits.
