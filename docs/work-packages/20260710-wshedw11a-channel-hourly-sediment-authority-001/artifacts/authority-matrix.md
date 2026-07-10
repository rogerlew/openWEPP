# Authority Matrix — Time-Resolved Channel Sediment Routing

Status: `EXECUTED` (authored 2026-07-10 as an operator-directed pre-kickoff
evidence draft by Claude Code; subsequently carried through the package's
dual review/verification cycle — see the Post-review corrections section at
the end for the two Row-3 statements corrected by Review Agent A.)

Evidence mode: mixed, labeled per row.

- `Ran`: reference acquisition and document reads performed 2026-07-10 —
  downloaded and read NSERL Report 10 Chapter 13 in full (identical document
  vendored at `references/50201000/chap13.pdf`); read the newly vendored
  CREAMS Chapter 3 conversion (`references/vendorable/creams/312-ch3.md`);
  grepped the vendored HEC-RAS Hydraulic Reference Manual v6.6 for sediment
  content (none present); ran the web searches that located the external
  authorities cited below.
- `Ran` (second acquisition round, 2026-07-10, operator-supplied files):
  verified the CREAMS Ch. 3 widening-law equations [I-133]-[I-140] and the
  [I-56] quasi-steady statement faithful against the rendered scan
  (`312-ch3.pdf` report pp. 54-55); verified the vendored HEC-RAS 1D
  Sediment Transport capture contains the load-bearing quasi-unsteady
  passages quoted in Row 2; verified KINEROS ARS-77 (`kineros/703.md:974`)
  restates the Bennett 1974 sediment mass balance with citation; identified
  Gilley, Woolhiser & McWhorter 1985 from its title/abstract; searched for
  an open copy of Jeong et al. 2011 (none found; ASABE paywall).
- `Static (delegated)`: pinned-baseline file:line citations
  (`chnrt.for`/`dcap.for`/`detach.for`/`chncon.for`/`wshdrv.for`,
  commit `dac3c950`) are from a delegated source-forensics subagent report,
  cross-consistent with the W11 `baseline-source-map.md` (which ran the
  pinned-commit verification commands) but not independently re-executed
  here.
- Chapter-13 and CREAMS quotes below were read directly (`Ran`), not
  paraphrased from secondary sources.

## Method and grading

Three independent authority directions were swept: (1) the WEPP/CREAMS
lineage documents and the pinned baseline code; (2) USDA-ARS sibling models
with time-resolved sediment routing (KINEROS/KINEROS2); (3) external
engineering-practice and peer-reviewed formulations (USACE HEC-RAS
quasi-unsteady, Bennett 1974, Jeong et al. 2011 SWAT sub-daily).

Per program posture (ADR-0011, ADR-0017), legacy documents/code are graded
as **lineage evidence** (design intent + behavior), never acceptance
authority; external canonical sources supply **process authority** for the
model class. Each row carries a grade:

| Grade | Meaning |
|---|---|
| `LINEAGE-DESIGN` | WEPP/CREAMS documentation states it as design intent |
| `LINEAGE-CODE` | pinned baseline behavior, file:line |
| `EXTERNAL-CANONICAL` | peer-reviewed or agency-standard method |
| `PROGRAM-RECORD` | already ratified in openWEPP ADRs/contracts |
| `MAINTAINER-INTENT` | upstream-maintainer change record (graded per the wepp-forest flag-not-authority posture) |

Row status vocabulary: `RATIFIABLE` (convergent authority sufficient to
amend contracts without an executor science choice), `DECISION-REQUIRED`
(bounded, labeled adjudication with divergent precedent), `GAP` (no
authority; must stay labeled).

## Row 0 — Precondition: supported water branch / time grid

Not re-adjudicated here. Per the W11 `baseline-source-map.md`: the `dtchr`
grid covers 86400 s exactly after normalization (`wshinp.for:463-495`);
routed water series exist for branches `ipeak=3` (kinematic,
`wshchr.for:395-469`) and `ipeak=4/5` (Muskingum-Cunge,
`wshchr.for:473-615`); upstream dependency superposition is grid-shared
(`wshchr.for:231-252`). Scalar branches `ipeak=1/2` have no routed series
authority. `PROGRAM-RECORD`.

## Row 1 — Temporal sediment quantum: the `dtchr` interval (the water grid)

**Adjudication: hour vs `dtchr` resolves to `dtchr`.** The hour is a
*serialization* quantum (the HBP `V_h`/`S_h` surfaces), not a *solve*
quantum. Every time-resolved sediment authority found couples the sediment
computation to the flow-routing step; none supports a sediment grid coarser
than the water grid:

- `EXTERNAL-CANONICAL` — KINEROS2 (USDA-ARS): the sediment equations reuse
  "the same time and space steps employed by the numerical solution of the
  surface water flow equations"
  (https://tucson.ars.ag.gov/KINEROS/Docs/html/Erosion.html; vendored
  chapter `references/copyrighted/Smith_1990_Kineros.pdf`).
- `EXTERNAL-CANONICAL` — HEC-RAS 1D sediment (USACE): the computational
  increment is "the hydraulic **and sediment transport** time step"; bed
  geometry updates after each increment (vendored capture
  `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf`,
  AR-14 below; load-bearing passages verified present 2026-07-10).
- `EXTERNAL-CANONICAL` — Jeong et al. 2011 (Trans. ASABE 54(5)): sub-daily
  channel sediment retrofit to a daily agricultural watershed model,
  computed at the flow-routing time step.
- `PROGRAM-RECORD` — ADR-0036 D2 defines `V_h` as hour-integrated volume
  with derived hour-mean `q_h`; the W11A water lane already projects hourly
  bins onto the `dtchr` grid by exact interval overlap.
  `SC-ROUTE-001#INV-ROUTE-005(e)` already mandates carrying the per-hour
  inlet sediment array on routed-inlet state for this consumption.

Hourly ingress surfaces are boundary conditions projected onto the grid
(hour-uniform within each hour — the ADR-0036 D2 reconstruction semantics).

**Status: `RATIFIABLE`.**

## Row 2 — Solve form per quantum: per-interval quasi-steady sequence

**Adjudication: run the existing Chapter-13/CREAMS steady spatially-varied
sediment solve once per `dtchr` interval at that interval's routed
discharge, with channel state carried between intervals (Row 3). Record the
full unsteady advection form as the named falsification fallback, not the
production form.**

The lineage itself names the reduction and its reason:

- `LINEAGE-DESIGN` — CREAMS Ch. 3 (Foster, Lane, Nowlin, Laflen & Young
  1980; vendored `references/vendorable/creams/312-ch3.md`), verbatim: "A
  number of fundamentally based models (1, 20) compute detachment and
  transport at various times during the runoff event. While these models
  are powerful, their excessive use of computer time practically prohibits
  simulating 20 to 30 years of record. The model described herein uses
  characteristic rainfall and runoff factors for a storm... this amounts to
  a single time step for models which simulate over the entire runoff
  event." And at Eq. [I-56]: "The assumption of quasisteady state allows
  deletion of time terms." The closing assessment concedes the stronger
  form: "Comprehensive models that simulate erosion over space and over
  time through a runoff event are potentially more powerful than this
  model" (cost-justified against 1980 computing). **The event-scalar
  collapse is a documented compute-cost decision, not a physics claim.**
- `LINEAGE-DESIGN` — WEPP Ch. 13 §13.5.5 repeats the same statement
  ("deletion of time terms from Eq. [13.5.17]"), anchored today as
  `REF-ROUTE-CH13-CONT`/`INV-ROUTE-009`.
- `LINEAGE-DESIGN` — WEPP Ch. 14 (WEPPSIE impoundments, vendored
  `references/50201000/chap14.pdf`) is the internal precedent that
  time-resolved sediment routing is not foreign to the lineage: adaptive
  time-step continuity integration, "sediment deposited and the outflow
  concentration for each time step." The lineage time-resolves sediment
  where it mattered (impoundments); channels simply never received the same
  treatment.
- `EXTERNAL-CANONICAL` — HEC-RAS quasi-unsteady is the canonical authority
  for exactly this model class: "a series of steady flow profiles"
  with sediment continuity solved and bed updated per computational
  increment, justified when "bed geometry does not change enough between
  computational increments to alter hydrodynamics appreciably." That
  justification condition transfers directly (per-interval geometry deltas
  in field-scale channels are small).
- `LINEAGE-DESIGN` — applicability domain supports the class's implicit
  no-advection assumption: Ch. 13 §13.6 bounds the model to small
  watersheds (< 260 ha, no perennial streams), where channel travel time is
  short relative to any plausible `dtchr` interval; CREAMS Ch. 3 states the
  same ("time of concentration is usually small").
- `PROGRAM-RECORD` — ADR-0036 D1 already ratified the identical
  decomposition (per-quantum quasi-steady solves of a steady-state
  continuity equation, daily totals as quantum sums, labeled refinement
  beyond legacy source-intent with a recorded fallback) for hillslope
  erosion. One solve philosophy across hillslope and channel avoids a
  two-physics inconsistency.

**The rejected-for-now alternative** (record in the contract as the named
fallback): full unsteady advection continuity
`∂(A·C)/∂t + ∂(Q·C)/∂x = e + q_s` (Bennett 1974 — the deleted "time terms";
KINEROS2's production form, with kinetic exchange `e = c_g(C_mx − C_s)A`).
Two concrete costs, no compensating acceptance driver at this tier:
(1) it requires hydraulic state `A(x,t)` along the channel, which only the
kinematic branch (`ipeak=3`) natively supplies — the Muskingum-Cunge
branches (4/5) are coefficient routings without cross-section state;
(2) it replaces the contract-anchored CREAMS four-case analytic march
(`REF-ROUTE-CH13-DETDEP`, migrated through WSHEDIMPL18-31) with a
finite-difference advection solver. Note the deposition kinetics are
already shared: KINEROS2's transfer coefficient and Ch. 13's
`α_er = v_f/q_w` (Eq. [13.5.22]) are the same settling-velocity
construction, so the quasi-steady sequence loses no deposition physics
relative to the unsteady form's source term.

**Status: `RATIFIABLE` with recorded fallback (ADR-0036 D1 pattern).**

## Row 3 — Geometry / profile / bed state: carries monotonically, never resets

**Adjudication: channel geometry state (incision depth toward the
nonerodible layer; eroded width) advances monotonically through the
interval sequence in time order. No reset at interval, event, day, or
month boundaries. The only reseeds are run start and the existing
primary-tillage rule.** Convergent from all three directions:

- `LINEAGE-DESIGN` — Ch. 13 §13.5.1, verbatim: "The ephemeral gully
  cross-sectional geometry is updated after each precipitation event that
  causes detachment in order to calculate channel hydraulics for
  subsequent events." CREAMS Ch. 3: "CREAMS considers the decay in erosion
  with time due to previous erosion; most models do not."
- `LINEAGE-CODE` (delegated) — the geometry state
  `depa/depb/wida/widb/wera/werb` lives in COMMON `/gully/`
  (`cgully.inc:7-9`), is initialized once (`chncon.for:132-143` from
  `wshini.for:319`), is mutated **in place during** the event solve
  (`dcap.for:188, 254-262`; `detach.for:290-297`;
  `chnrt.for:594-595`), and has no per-event reset — the only reseed is
  primary tillage on naturally-eroded channels (`wshdrv.for:1179-1189`).
- `EXTERNAL-CANONICAL` — HEC-RAS updates bed geometry after each
  computational increment and uses it for the next; state carry between
  quanta is the defining property of the quasi-unsteady class.

**The widening clock (previously the open sub-item — now resolved by the
vendored primary source).** CREAMS Ch. 3 Eqs. [I-133]-[I-140] define
widening after nonerodible-layer contact as an explicit time evolution:
`ω = 1 − exp(−t*)` with `ω = (W − W_i)/(W_f − W_i)`,
`t* = (t − t_i)(dW/dt)_i/(W_f − W_i)`, initial rate
`(dW/dt)_i = 2K_ch(τ_b − τ_cr)^1.05/ρ_soil` [I-136], and flow-dependent
final width `W_f(Q)` [I-139]-[I-140]; incision rate
`d_ch = e_m/ρ_soil` [I-131]. The law is state-based and memoryless given
`(W_i, t_i, W_f(Q), (dW/dt)_i)` — so per-interval evaluation is
**native**, not an invention: each interval advances the clock by its own
above-critical-shear duration at its own `Q_i` (which sets `W_f(Q_i)` and
the shear operands), with the carried `W_i` as the initial condition.
Monotonic non-narrowing is implied by the physics (removed soil does not
return) and by the code (`wera` grows monotonically). When an interval's
`W_f(Q_i) ≤ W_i` (flow smaller than what cut the existing width), no
widening occurs — the erosion-rate-zero branch already present in the
lineage ("erosion rate decreases with time until the flow is too shallow
to cause detachment," Ch. 13 §13.5.1).

**Status: `RATIFIABLE`.**

## Row 4 — Sediment mass state between quanta: closes per quantum, no carried suspended pool

**Adjudication: per-class sediment mass closes within each interval solve
(ingress = egress + deposition + detachment contribution). No suspended
mass pool carries interval-to-interval, event-to-event, or across
midnight.**

- Model-class property: in a quasi-steady sequence each interval solve is
  a complete steady spatial solution — flux in, flux out, sources/sinks
  along the reach. A carried suspended pool has no slot in the
  formulation (that state belongs to the unsteady class, Row 2 fallback).
- `MAINTAINER-INTENT` — `wepp-forest/docs/jimf-wepp-2023-diff-audit.md`
  records Frankenberger's 2023 "channel sediment initialization fix to
  prevent carryover to following storms" (`wshdrv.for`, r1305): the
  upstream maintainer treated cross-storm sediment-mass carry as a
  **defect** while geometry carry remains design. Graded as maintainer
  intent, not physics authority — but it disentangles the two "carry"
  notions exactly as this row does.
- `LINEAGE-CODE` (delegated) — daily load accumulators are zeroed per
  event (`wshdrv.for:1103-1105`); the between-event carry is geometric
  only.

**Sub-row — no re-erodible deposited-bed store.** The lineage channel
model has no bed-storage pool: deposited mass leaves the active accounting
(it does not later re-entrain), and detachment draws only on the channel
boundary soil down to the nonerodible layer. KINEROS2/HEC-RAS do carry
exchangeable bed layers, but importing one would be new physics with no
WEPP-lineage support and no acceptance driver at this tier. Keep the
lineage behavior and record the absence as a labeled scope note in the
contract amendment. **Status of sub-row: `GAP` (recorded, intentionally
retained).**

**Status: `RATIFIABLE` (with the recorded sub-row GAP).**

## Row 5 — Per-quantum ingress / coupling operands

**Adjudication: the interval solve consumes the routed water surface and
projected sediment surfaces on the shared grid, replacing the event-scalar
operands one-for-one:**

| Legacy event operand (Ch. 13 / baseline) | Per-interval operand |
|---|---|
| `qe = peakot` characteristic discharge (`chnrt.for:166`) | routed interval discharge `q1(it)` at the reach (grid-shared with upstream superposition, `wshchr.for:231-252`) |
| inlet flux `gpart/rundur` (Eq. [13.5.18], `chnrt.for:319-323`) | projected inlet class mass for the interval / interval length (from carried per-hour inlet array, `INV-ROUTE-005(e)`; upstream-channel ingress = upstream interval egress) |
| lateral flux from event volumes (`chnrt.for:325-348`) | interval-projected lateral class mass / interval length (hour-uniform within each hour per ADR-0036 D2) |
| `dur_rof` rate↔mass integrator | interval length (exact) |

The spatially-varied-flow, effective-length, and shear-partition machinery
(`REF-ROUTE-CH13-SVF`/`EFFLEN`/`SHEAR`, Eqs. [13.5.1]-[13.5.16]) runs
unchanged per interval at the interval discharge — this is what the
HEC-RAS class means by computing a new steady profile per increment.
`LINEAGE-DESIGN` + `EXTERNAL-CANONICAL` + `PROGRAM-RECORD`.

**Status: `RATIFIABLE`.**

## Row 6 — Per-quantum closure; retirement of the triangular shear-time surrogate

**Adjudication: the Chapter-13 four-case detachment/deposition machinery
(Eqs. [13.5.19]-[13.5.29], `REF-ROUTE-CH13-DETDEP`, migrated via
WSHEDIMPL18-31) closes each interval solve per particle class; day totals
are interval sums. One internal surrogate is retired rather than
replicated:** the triangular-shear-in-time construction — Ch. 13 §13.5.4 /
CREAMS Ch. 3 verbatim: "Shear stress is assumed to be triangularly
distributed in time during the runoff event in order to estimate the time
that shear stress is greater than the critical shear stress" — realized in
the baseline as `tb = 2·rundur` (`chnrt.for:205`) and
`timsh = tb(1 − τ_cr/τ)` (`dcap.for:154`). The surrogate exists solely to
recover time-above-critical-shear from event scalars; the routed `dtchr`
series resolves that quantity directly (each interval computes its own
shear; its above-critical duration is the interval length). This is the
identical demotion argument ADR-0036 applied to the triangular hydrograph:
the surrogate's referent is now modeled. `LINEAGE-DESIGN` (the surrogate is
itself labeled an estimate) + `PROGRAM-RECORD` (demotion pattern).

Closure identities for the contract amendment (forms, not tolerances):

- per class, per interval: ingress + detachment = egress + deposition;
- per class, per day: Σ over intervals of each term; egress sum equals the
  published daily class mass;
- geometry-mass consistency: incision/widening volume × ρ_soil equals the
  boundary-detached mass (CREAMS [I-131] `d_ch = e_m/ρ_soil` ties them);
- projection exactness: interval-projected inlet/lateral class masses sum
  exactly to the source hourly masses (`Σ` over intervals = `Σ S_h`
  contribution), the same exact-sum posture as ADR-0036 D4.

Inherited internal tolerance: the Yalin transport-capacity iteration
closes at 1% or 20 iterations (Ch. 13 §13.5.6) — keep as-is per solve.
Numeric day-closure tolerances are `contract-disposition.md` work under
ADR-0003 (semantic, not bit).

**Fidelity note for the contract author:** Ch. 13 presents detachment as
linear excess-shear `D = K_ch(τ̄ − τ_cr)` [13.5.19] and the baseline
implements the linear form (`dcap.for:166`); the CREAMS original is
`K_ch(1.35τ̄ − τ_cr)^1.05` [I-128]. WEPP's linear adaptation is the
governing lineage form; cite CREAMS for provenance, not as an override.

**Status: `RATIFIABLE`.**

## Row 7 — Degenerate states

| State | Rule | Authority | Status |
|---|---|---|---|
| Zero-flow interval | No solve, no detachment; any incoming class mass deposits in the reach | `EXTERNAL-CANONICAL` KINEROS2 boundary rule (zero inlet transport capacity: "any lateral input of sediment will be subject there to deposition"); `LINEAGE-CODE` event-scale analog `runvol ≤ 0.001 m³` gate (`wshdrv.for:1097-1117`) | `RATIFIABLE` |
| Sub-threshold trickle intervals | Numeric floor below which an interval is treated as zero-flow | No direct authority for the constant; legacy's `0.001 m³` event gate is the only lineage precedent | `DECISION-REQUIRED` (bounded constant choice; propose deriving from the event gate and labeling) |
| Dry spell between events / cross-day carry | Geometry carries (Row 3); no sediment mass carries (Row 4) | Rows 3-4 authorities; `MAINTAINER-INTENT` jimf-2023 fix | `RATIFIABLE` |
| Sediment in end-of-grid routed channel storage | **The one genuinely divergent-precedent item.** If the water grid ends the day with nonzero channel storage (`wshchr.for:618-720` daily volume includes initial-minus-final storage), the suspended mass in that stored water has no lineage disposition rule. HEC-RAS (unsteady state) carries it; the quasi-steady class and the lineage (event-closed solves; jimf-2023 defect posture) close it. **Recommended default: deposit-at-grid-end, GAP-labeled** (conservative; keeps every day mass-closed; consistent with Row 4) | Divergent — decision must be labeled in the contract | `DECISION-REQUIRED` |
| Cross-midnight flow state | The `dtchr` grid covers 86400 s exactly; no cross-midnight flow or sediment state exists in any lineage authority; carry is geometry-only (+ the Row-7 storage rule once adjudicated) | `LINEAGE-CODE` grid normalization (`wshinp.for:463-495`); Rows 3-4 | `RATIFIABLE` |

## Row 8 — Per-class continuity and enrichment limits

npart-resolved class handling runs through every stage (Ch. 13 §13.5.5:
all sediment-load calculations per particle class, default five;
`chnrt.for:300-348, 846-885` per-class ingress/egress/fractions; KINEROS2
likewise routes up to five classes independently). The per-class time
resolution of ingress is governed by `SC-SED-001#GAP-SED-008` in its
**E.4-narrowed** state: per-hour class compositions exist in the hillslope
solver, but the serialized interchange `S_h` stays total-mass (no
per-class-hourly channel), so the channel consumer sees the day-level
class-fraction blend applied uniformly to the projected interval masses
and **must not treat the uniform split as enriched timing** (GAP-SED-008's
consumer clause). The per-class-hourly channel remains a future additive
interchange extension outside this package's scope. `PROGRAM-RECORD`.

**Status: `RATIFIABLE` (with the retained, already-labeled GAP).**

## Reference tracking

Matrix-local IDs use the `AR-##` prefix to avoid colliding with the
corpus-wide `R-##` namespace in `references/annotated_bibliography.md`;
bibliography cross-references are given where entries exist. Rights classes
follow `references/rights_classification_first_pass_2026-05-11.md` (2026-07-10
addendum records the acquisitions below). Acquisition round completed
2026-07-10: everything gating is now held; the remaining not-held items are
covered by held secondary sources per the operator's secondary-reference
posture, or are optional.

| # | Reference | Matrix role | Local status | Rights |
|---|---|---|---|---|
| AR-01 | Flanagan & Nearing (eds.) 1995, NSERL Report No. 10, **Ch. 13** (Ascough, Baffaut, Nearing, Flanagan) | Primary lineage authority: Rows 2-8; all `REF-ROUTE-CH13-*` anchors | Vendored `references/50201000/chap13.pdf` | vendorable (USDA) |
| AR-02 | NSERL 10 **Ch. 14** (WEPPSIE) | Internal time-stepped sediment precedent (Row 2) | Vendored `references/50201000/chap14.pdf` | vendorable (USDA) |
| AR-03 | NSERL 10 **Ch. 11** (hillslope erosion) | Steady-state-continuity parallel; `INV-SED-011` label lineage | Vendored `references/50201000/chap11.pdf` | vendorable (USDA) |
| AR-04 | Knisel (ed.) 1980, CREAMS, USDA Conservation Research Report No. 26 (full report) | Parent model | **Vendored** `references/vendorable/creams/312.pdf` (2026-07-10; bibliography R-104) | vendorable (USDA) |
| AR-05 | Foster, Lane, Nowlin, Laflen & Young 1980, CREAMS **Ch. 3** | Primary source of `chnrt` physics; widening law [I-133]-[I-140] **verified against the rendered scan**; quasi-steady rationale [I-56] (Rows 2, 3, 6) | **Vendored** `references/vendorable/creams/312-ch3.pdf` + `.md` conversion (2026-07-10; bibliography R-105 carries the conversion caveat) | vendorable (USDA) |
| AR-06 | Foster & Meyer 1972, closed-form erosion equation (Shen sedimentation symposium, Ch. 12) | Steady-state continuity parent (cited by Ch. 13 §13.5.1) | Not held — **secondary-covered**: working form abstracted in held AR-05 ("abstracted from Foster and Meyer (10)") and Ch. 13 (bibliography R-111) | copyrighted |
| AR-07 | Yalin 1963, J. Hydraulics Div. ASCE 89(HY3) | Transport-capacity equation (Row 6) | Not held — **secondary-covered**: full working equations incl. the multi-class modification the baseline implements are in held AR-05 [I-93]ff (bibliography R-112) | copyrighted |
| AR-08 | Graf 1971, *Hydraulics of Sediment Transport* | Shear-partition basis (Ch. 13 §13.5.4) | Not held — low priority (book; partition already contract-anchored; shear-partition equations [I-141]-[I-143] held in AR-05) | copyrighted |
| AR-09 | Chow 1959, *Open-Channel Hydraulics* | Spatially-varied flow parent | Not held (annotated in bibliography R-03) — low priority | copyrighted |
| AR-10 | McCool, Gwinn, Ree & Garton 1966, Trans. ASAE 9(3) | β = 1.56 energy coefficient (Eq. [13.5.4]) | Not held — **secondary-covered**: provenance restated in held AR-05 ("1.56 used from McCool and others (23)") (bibliography R-113) | copyrighted |
| AR-11 | Haan, Johnson & Brakensiek (eds.) 1982, ASAE Monograph 5 | Triangular synthetic hydrograph lineage (`REF-ROUTE-CH13-PEAKIN`, the demoted fallback) | Not held — low priority | copyrighted |
| AR-12 | Smith 1990, "KINEROS — a kinematic runoff and erosion model" (chapter) | External unsteady class + same-grid rule (Rows 1-2); kinetic deposition term | Vendored `references/copyrighted/Smith_1990_Kineros.pdf` + `.md` (bibliography R-11) | copyrighted (local cache) |
| AR-13 | Woolhiser, Smith & Goodrich 1990, *KINEROS: A Kinematic Runoff and Erosion Model — Documentation and User Manual*, USDA-ARS ARS-77 | Fuller equation set behind AR-12; formal citable USDA documentation; **held secondary for AR-15** (restates the Bennett 1974 mass balance with citation, `703.md:974`) | **Vendored** `references/vendorable/kineros/703.pdf` + `.md` conversion (2026-07-10; bibliography R-106) | vendorable (USDA) |
| AR-14 | USACE, *HEC-RAS 1D Sediment Transport* manual (quasi-unsteady flow chapters) | Canonical quasi-steady-sequence class authority (Rows 1-3) | **Vendored** `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf` — load-bearing quasi-unsteady passages verified present 2026-07-10; the online 1D Sediment Transport Technical Reference Manual remains the formal citable source (bibliography R-107) | vendorable (USACE) |
| AR-15 | Bennett 1974, "Concepts of mathematical modeling of sediment yield," WRR 10(3):485-492 | The unsteady parent equation (the deleted "time terms"); fallback-form authority (Row 2) | Not held — **secondary-covered** by held AR-13 (`703.md:974` + full citation in its references) (bibliography R-109) | copyrighted |
| AR-16 | Jeong, Kannan, Arnold, Glick, Gosselink, Srinivasan & Harmel 2011, Trans. ASABE 54(5):1685-1691 | Procedural precedent: sub-daily channel sediment retrofit to a daily ag model (Row 1) | Not held — no open copy located (searched 2026-07-10; ASABE paywall); citation-tracked, non-gating (bibliography R-110) | copyrighted |
| AR-17 | Ascough, Baffaut, Nearing & Liu 1997, "The WEPP watershed model: I. Hydrology and erosion," Trans. ASAE 40(4):921-933 | Peer-reviewed companion of Ch. 13 (citable confirmation of steady-state channel continuity) | Not held — optional | copyrighted |
| AR-18 | `wepp-forest/docs/jimf-wepp-2023-diff-audit.md` (r1305 channel-sediment initialization fix; `dailykr`/`ch_run` daily-erodibility change) | Maintainer-intent evidence: mass carry = defect, geometry carry = design (Row 4) | Local (wepp-forest repo) | program record |
| AR-19 | W11 `baseline-source-map.md` + pinned baseline `dac3c950` (`chnrt`/`chnero`/`dcap`/`detach`/`trncap`/`case12`/`case34`/`cgully.inc`) | `LINEAGE-CODE` rows throughout | Local (W11 package + pinned worktree) | program record |
| AR-20 | ADR-0036; `SC-ROUTE-001` (v50, `INV-ROUTE-005(e)`, `INV-ROUTE-008/009/010`, `REF-ROUTE-CH13-*`, GAP-ROUTE-009); `SC-SED-001` (`INV-SED-011`); `SC-INFILE-HBP-001`; `SC-INFILE-WATERSHED-CHANNEL-001` (`chnedm`/`chneds` immutable) | `PROGRAM-RECORD` rows throughout | Local (openWEPP repo) | program record |
| AR-21 | Gilley, Woolhiser & McWhorter 1985, "Interrill soil erosion — Part I: Development of model equations," Trans. ASAE 28(1):147-153 | KINEROS-lineage source-term development supporting AR-12/AR-13 upland terms; secondary context only, not channel physics | Local cache `references/copyrighted/Gilley,Woolhiser,McWhorter_1985.pdf` + `.md` (2026-07-10; gitignored per policy, metadata tracked; bibliography R-108) | copyrighted (local cache) |

Optional follow-up: the KINEROS2 web documentation pages
(`.../KINEROS/Docs/html/Erosion.html`, `Channel.html`) are living ARS pages;
AR-13 now holds the formal equations, so snapshots are convenience-only.

## Summary against package exit criteria

| Exit-criterion element | Matrix result |
|---|---|
| Supported water branch/time grid | Row 0 — inherited from W11, `PROGRAM-RECORD` |
| Temporal sediment quantum + state-carry order | Rows 1, 3, 4 — `RATIFIABLE` (quantum = `dtchr`; geometry carries monotonically; mass closes per quantum) |
| Per-class ingress/egress and detachment/deposition/storage closure | Rows 5, 6, 8 — `RATIFIABLE`; triangular shear-time surrogate retired; no-bed-store GAP recorded |
| Typed failure behavior and degenerate states | Row 7 — mostly `RATIFIABLE`; **two labeled decisions**: sub-threshold trickle constant, end-of-grid storage disposition (recommended default: deposit-at-grid-end, GAP-labeled) |
| Tolerances | Closure identities specified (Row 6); numeric bounds deferred to `contract-disposition.md` under ADR-0003; Yalin 1%/20-iteration inherited |
| Test vectors / W11 resume instructions | Downstream artifacts (`contract-disposition.md`, `w11-handoff.md`) — not this artifact's scope |

**Net: no `EXECUTED-HOLD-AUTHORITY` trigger found.** The former blocking
sub-item (widening-law time semantics) is resolved by the AR-05
acquisition: the CREAMS widening law is an explicit, state-based time
evolution that per-interval sequencing evaluates natively — verified
against the rendered scan, not only the conversion. The two
`DECISION-REQUIRED` items are bounded, labeled adjudications with
recommended defaults — not missing physics — and are eligible for
resolution inside the contract disposition.

**Acquisition round closed 2026-07-10.** All gating authorities are held
locally: AR-04/AR-05 (CREAMS), AR-13 (KINEROS ARS-77), AR-14 (HEC-RAS 1D
sediment capture), AR-21 (Gilley 1985, local cache). The remaining
not-held items are covered by held secondary sources per the operator's
secondary-reference posture — AR-06/AR-07/AR-10 via AR-05's restatements,
AR-15 via AR-13's restatement — or are optional/non-gating (AR-08, AR-09,
AR-11, AR-16, AR-17). Bibliography entries R-104 through R-113 index the
round; the 2026-07-10 rights addendum records the classifications.

## Post-review corrections (2026-07-10, dual-review cycle)

Two Row-3 statements were corrected by Review Agent A (see
`review_agent_a.md` A-1/A-4) and are superseded by the SC-ROUTE-001 v51
text as fixed:

- **Widening-law realization**: the WEPP lineage *adapted* the CREAMS
  widening equations (linear rate with the `^1.05` exponent dropped, a
  `1.0176`-modified exponential, fitted `f(x_b)` shear distribution,
  verified in both `dcap.for` and the migrated Rust lane). This matrix's
  Row-6 fidelity note applied that discipline to detachment but Row 3 did
  not carry it into the widening law; the contract adjudicates the
  lineage realization as governing with CREAMS as structural provenance.
- **"Memoryless / native, not an invention"** overstated: the CREAMS law is
  anchored at layer contact; per-interval re-anchoring is the
  interval-ization of the lineage's per-event re-anchoring — an inference-
  graded labeled refinement, not a direct evaluation.

Third correction (Codex post-hoc review M3, 2026-07-10): Row 1's HEC-RAS
bullet says "bed geometry updates after each increment" — the manual's
precise claim is that bed-change **state** advances/carries each increment
while the cross-section *refresh* is gated behind a minimum-bed-change
threshold (vendored manual PDF p. 178). The contract anchor
(`REF-ROUTE-HECRAS-QUS`) is narrowed accordingly in v52; geometry-update
authority for the lane is the Chapter-13/lineage carry, not HEC
cross-section refresh policy.
