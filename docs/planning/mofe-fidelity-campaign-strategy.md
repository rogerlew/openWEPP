# MOFE Fidelity Campaign Strategy

Status: **active strategy** (2026-07-01). Campaign code: **MOFEFID**.
Owner: maintainers (operator-directed campaign; Claude Code authored this plan).

Evidence mode: **Static** synthesis of three grounding documents (the
wepp-forest stakeholder water-balance brief, the forest lateral-flow
authority backlog note, and Papanicolaou et al. 2018) plus **Ran**
inventories of the two asset sets named below and **Ran** greps of the
current publication/contract surface. This file is planning guidance, not
science authority. Canonical authority remains the `SC-*` registry,
ADR-0011/0017/0024, and `docs/ROADMAP.md`.

## 0. TL;DR

The MOFE **closure** rung is complete: MOFE01/FARPOINT01 closed inter-OFE
water routing through 19 OFEs, MAGPARITY01 cleared transfer/area-scaling/
export, and the winter column runs on MOFE lanes in direct production. What
remains — and what this campaign is — is the **fidelity axis**, in four
lanes:

- **Lane A — proactive MOFE defect-review sweep.** Adversarial review of the
  whole MOFE surface (transfer, carry arrays, publication geometry, per-OFE
  closure, winter-on-MOFE) for defects nobody has reported.
- **Lane B — stakeholder-brief defect audit, adjudication-first.** The
  wepp-forest water-balance program reported eleven defect classes in legacy
  MOFE-relevant accounting
  (`/workdir/wepp-forest/docs/20260504-stakeholder-watbalance.md`). **The
  brief is a flag list, not an authority** — each claim is independently
  re-adjudicated (graded `conservation-forced` / `source-intent` /
  `convention` / `unverified`) before openWEPP is audited against the
  adjudicated ground truth, and every row carries a dual verdict (brief
  claim × openWEPP). One row is already known contract-decision-shaped for
  us: our `QOFE` publication follows the pre-`wepp_260516` convention while
  the ecosystem has moved.
- **Lane C — lateral-flow observed-authority rubric.** Promote the landed
  field datasets (`tests/fixtures/forest_lateral_flow_authority/`: HJ Andrews
  WS10, Panola, Maimai M8, Coweeta) into an acceptance envelope and a
  snowfreeze-style observed rubric, then judge openWEPP forest lateral-flow
  magnitude against it — the external authority the FARPOINT01 71% flag has
  been waiting for.
- **Lane D — Papanicolaou 2018 OFE-by-OFE routing implementation.** Implement
  the enhanced-WEPP overland-flow routing (per-OFE kinematic wave,
  space/time-variant resistance, shock capturing) and validate against the
  paper's supplemental datasets
  (`references/copyrighted/Papanicolaou2018-supplemental/`).

Sequencing: A and B first (defect posture before fidelity judgment), C in
parallel once its promotion work is defined (its prerequisite — closed MOFE
water balance — already holds), D contract-first with implementation gated
on A/B disposition. Every lane keeps legacy as flag-not-target (ADR-0017)
and the closure-first ordering (closure → routing → magnitude judged last).

## 1. What changed / what we know

1. **MOFE closure is done.** `20260612-mofe01-inter-ofe-routing-closure-001`,
   FARPOINT01 (19-OFE far-point, three identities closed),
   MAGPARITY01/STAGE2-LATQCC/BASECOND01/POST-BASECOND01 (magnitude arc:
   machinery correct, verdict `CONTRACT-GAP` on absolute lateral magnitude),
   `20260622-r7d4-direct-mofe-dynamic-carry-transfer-001` (direct-runtime
   carry). Contract surface: `INV-RUNOFFPART-028/029/030`.
2. **The stakeholder brief is a defect census we have not systematically
   consumed.** It documents seven repaired legacy/new-kernel defects, two
   *unfixed-in-legacy-production* defects (R01 cascade-tail rain-event
   over-counting; dry-day per-OFE residual), one latent-fixed math defect
   (winter day-end mixed-melt aggregation), and the post-`wepp_260516`
   `QOFE = Q` ecosystem contract with canonical-volume recipes.
3. **Ran:** openWEPP publishes the **pre-fix** `QOFE` convention.
   `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:370-376`
   computes `QOFE = q_ofe × efflen / ofe_length` (the legacy inflated
   identity) and `runvol = QOFE × per-OFE area` (the legacy cancellation
   recipe). This matches our byte anchor `wepp_260430_hill`, which predates
   the `wepp_260516` fix. `SC-RUNOFFPART-001` currently defines `QOFE = Q`
   only for single-OFE WB13 rows.
4. **The lateral-flow authority data has landed** (commit `08d6994e`,
   2026-07-01): four provenance-bearing observed datasets with checksums and
   a README that stipulates five use-limit conditions before any authority
   promotion. Coweeta is context-only by its own README — it must not carry
   a direct `latqcc` verdict.
5. **The Papanicolaou supplemental is in-repo and usable** (**Ran** probe):
   `Figure_4.xlsx` carries `Enhanced_WEPP` / `Original_WEPP` series for the
   four validation cases; `3.1_Validation_Input.docx` carries complete case
   inputs (plot dimensions, slope, rainfall, soil texture, `k_o`);
   Figures 6–9 carry the storm-magnitude/gradient/curvature/stream-power
   experiment series. Published Ef targets: 0.91 (bare), 0.75 (roughness
   elements), 0.87 (vegetation patchiness), 0.88 (curvature/shock).
6. **Perf is a settled constraint, not an open question.** H2637 direct is
   32.77 s = 3.52× legacy (2026-07-01). Fidelity work must not push the
   default path back over the ≤5× budget; compute-heavy routing goes in as
   opt-in/policy-gated first.

## 2. Campaign shape

```text
Lane A (defect sweep)  ──┐
                         ├─→ dispositions ──→ Lane D implementation gates
Lane B (brief audit)   ──┘         │
                                   └─→ defect-closure ExecPlans as found
Lane C (authority rubric) ── independent; needs only closed MOFE WB (done)
Lane D (routing model)    ── contracts/fixtures first; implementation after A/B
```

Lanes A and B are review-shaped and cheap; they establish the defect posture
of the surface the other two lanes will judge and extend. Lane C is the
FARPOINT01 continuation and does not depend on A/B outcomes (it judges
magnitude against observations, not against internal structure). Lane D is
the large build; its contract/fixture stages can start immediately, its
runtime stages wait for A/B dispositions so new routing is not built on an
undiagnosed defect.

## 3. Lane A — proactive MOFE defect-review sweep

**Shape:** adversarial review (find → verify) of the MOFE-specific surface:

- inter-OFE `TransferInput`/`TransferOutput` lineage and area-scaling
  provenance (`INV-RUNOFFPART-028/029`);
- 24-slot hourly carry arrays (surface + lateral) and their day-boundary
  semantics;
- publication geometry (`Q`/`QOFE`/`runvol`/peak operands, effective
  lengths) per OFE;
- per-OFE closure at the exported-surface basis (can an external reader
  reconstruct per-OFE conservation from our WAT/PASS the way the wepp-forest
  audit does from `H.wat`/`H.pass`?);
- winter column on MOFE lanes (frost/snow carry vs. transfer interactions —
  the FARPOINT01 `watbtm` double-count lived exactly here);
- single-OFE specialization invariants (zero-upstream lanes must be
  bit-stable against the MOFE machinery).

**Acceptance:** every finding verified (confirmed/refuted) with file:line
evidence and an explicit disposition; confirmed defects convert to
Defect-Closure ExecPlans per ADR-0018. A clean sweep is a valid outcome and
is recorded as one.

## 4. Lane B — stakeholder-brief defect audit (adjudication-first)

**Posture (operator directive, 2026-07-01): the brief is a flag list, not
an authority.** The wepp-forest program's conclusions are not fully trusted
— its own record shows why: the U1–U7F patch ladder traded defect families
at every promotion boundary; `wepp_260501` was released and failed on a
second project two days later; R01 passed tri-hillslope validation and then
regressed 30/40 at cohort scale; acceptance gates were governance-widened
mid-campaign. Several "defects" are on inspection *convention choices*
(audit input basis, column denominator semantics) rather than physical
errors. Lane B therefore applies to the brief the same discipline ADR-0017
applies to the legacy binary: each claim earns its authority; none inherits
it from the narrative.

**Adjudication protocol — run per class, before any openWEPP verdict:**

1. **Restate the brief's claim and separate its parts.** The *problem
   observation* and the *repair conclusion* are distinct claims with
   distinct evidence (R01 is the type case: the observation — reported
   runoff exceeding precipitation — is conservation-forced and solid; the
   repair failed its own cohort gate).
2. **Grade the claim's authority:**
   - `conservation-forced` — violating it breaks a mass/closure identity
     that holds independent of anyone's narrative (runoff > precipitation;
     flux with no compensating Δstorage). Strongest grade.
   - `source-intent` — backed by legacy source intent (ADR-0024 A0
     anchor). Valid only after **we re-read the cited source ourselves**;
     the brief's source reading is not accepted secondhand.
   - `convention` — a definitional/basis choice (what counts as external
     input; which denominator a column uses). Here "defect" can only mean
     "inconsistent with a declared contract," and the right contract for
     openWEPP is adjudicated from ADR-0019 consumer closure semantics —
     not from what wepp-forest chose.
   - `unverified` — asserted in the brief but not reproducible from
     evidence we hold. Recorded as such; generates no openWEPP obligation.
3. **Derive the openWEPP-native correct behavior** from our own authority
   chain (`SC-*` invariants, conservation identities, ADR-0024 source
   intent, ADR-0019 schema ownership). "Because wepp-forest concluded X"
   is never a justification.
4. **Then** audit openWEPP against the adjudicated ground truth, with
   evidence (**Ran** where a test/run can decide, **Static** otherwise).

**Dual verdict per class:** a *brief-claim disposition* (`upheld` /
`partially-upheld` / `convention-not-defect` / `unsubstantiated`) **and**
an *openWEPP disposition* (`correct-by-construction` / `defect` /
`not-applicable` / `contract-decision`). A class where the brief is wrong
and openWEPP is fine is a legitimate, valuable outcome — record it.

| # | Brief defect class | openWEPP audit question |
|---|---|---|
| B1 | Hourly transport-capacity (q-cap) bypass at bottom OFE (`efflen <= slplen` mis-gate) | Is the q-cap/case-classifier enforced on every OFE in our hourly partition, including the bottom lane? |
| B2 | Snowmelt double-count in the closure basis (`RM` as input + `Snow-Water` as storage) | Do all openWEPP closure gates/audits use `P + Irr` external-input basis with snow in Δstorage? |
| B3 | Missing interception-storage export (`pintlv + resint` absent from `H.wat`) | Do our WAT/HBP schemas expose interception storage, or does an external per-OFE audit see that water vanish? (ADR-0019: we own the schema — this is a contract decision, not parity.) |
| B4 | Zero-input day emitting flux without storage change (their new-kernel defect) | Is "no flux without compensating Δstorage" a per-OFE hard invariant in the direct runtime? |
| B5 | Rain-routing conflation (rain aliased into the melt channel on rain-on-snow days; their Candidate-1 contract) | Do our typed rain/melt channels keep rain in the rain stream under rain-on-snow with mid-day pack exhaustion? |
| B6 | Clamp-plus-preserve interaction (snowmelt cap + baseflow preservation leaving unbalanced output) | Does any openWEPP surface clamp an input while preserving a dependent output un-scaled? (Pattern audit — their Phase-3 fail-fast is the reference discipline.) |
| B7 | `QOFE = n × Q` denominator defect; ecosystem now `QOFE = Q` (`wepp_260516`) | **Known defect-shaped for us (Ran, §1.3).** Adjudicate openWEPP's native convention: adopt post-fix `QOFE = Q`, hold `runvol` canonical-invariant, coordinate wepppy consumers (`totalwatsed3`, `hillslope_watbal`). Needs an ADR + schema change control; breaks byte parity with our pre-fix anchor by design. |
| B8 | R01 cascade-tail rain-event over-counting — **unfixed in legacy production** | Is our event counting producer-classified (not bucket-inferred)? Record comparator hygiene: MOFE event-count deltas vs legacy are *expected*; legacy carries the defect. |
| B9 | Dry-day per-OFE mass-balance residual — **open in legacy** | Does our per-OFE closure gate cover dry days at per-OFE granularity? Same comparator-hygiene note. |
| B10 | Winter day-end mixed-melt aggregation math defect (legacy fix `03fee455`; branch empirically unreachable in their cohort) | Does our melt-day aggregation carry the corrected math, and is the mixed-sign branch reachable/tested? |
| B11 | Audit-tooling over-reach: surface-pulse check firing on `latqcc`-only days | Do our diagnostics distinguish surface-runoff-absent lateral-cascade days? |

Known skepticism hooks going in (to be tested, not assumed):

- **B2 (RM double-count):** the "defect" was in the *audit's* input-basis
  definition, not the model — production outputs were unchanged. Likely
  grade `convention`; openWEPP's obligation is that *our* closure gates use
  a self-consistent basis, which we verify directly.
- **B6 (Shape A clamp+preserve):** scaling baseflow with the shortfall is
  *one* resolution consistent with closure, not the uniquely forced one;
  the invariant worth keeping is "clamp+preserve without compensation is
  non-conserving," not their specific partition choice.
- **B7 (QOFE):** the physical volume was never wrong — the question is
  column *semantics*. The ecosystem's move to `QOFE = Q` matters to us as a
  consumer-contract fact (wepppy re-anchoring), and the adjudication weighs
  that against our own schema ownership; the brief's "18-year defect"
  framing is not itself the authority. Grade `convention`, resolved by
  contract decision.
- **B8 (R01):** observation `conservation-forced`; repair `unsubstantiated`
  (failed its own cohort gate). openWEPP is audited against the
  observation only.
- **B10 (winter mixed-melt):** verify the sign/magnitude math against the
  Dun-dissertation source intent ourselves; the brief reports the branch
  empirically unreachable in their cohort, which caps its practical weight.

**Acceptance:** one artifact with an eleven-row verdict table carrying the
**dual verdict** (brief-claim disposition + openWEPP disposition), the
authority grade, and the evidence class per row. B7 is expected to spawn
the campaign's first contract package (`QOFE` ecosystem-contract
adjudication) — as a contract decision, not an inherited defect;
B8/B9 produce comparator-hygiene entries even if openWEPP is clean.

## 5. Lane C — lateral-flow observed-authority rubric and fidelity evaluation

The FARPOINT01 71% `runvol` flag closed internally as `CONTRACT-GAP`: the
machinery, equation, and operands are correct; absolute magnitude had no
authority to be judged against. The authority data now exists in-repo. This
lane converts it into a measuring bar, mirroring the snow/frost pattern
(`tools/snowfreeze_observed/` harness + `INV-SNOWFREEZE-047/048/050`-style
rubric).

Stages, honoring the fixture README's five use-limits verbatim:

1. **C1 — authority promotion.** Define, per site: the judged openWEPP
   output and matching observed metric (candidates: event lateral-flow
   depth/fraction, trenchflow per event, storm runoff ratio); temporal
   aggregation and unit conversions; uncertainty and acceptance envelope;
   site-to-H2637 applicability mapping (slope, wetness, soil depth,
   conductivity regime). Roles per README: HJ Andrews WS10 = primary
   magnitude candidate; Panola = contrasting event-scale/threshold
   behavior; Maimai M8 = wet-hillslope bracket; **Coweeta = context only,
   no `latqcc` verdict.** Output: `SC-SUBHYD-001` (or successor) authority
   suite + envelope, ratified before any judgment.
2. **C2 — rubric harness.** `tools/` observed harness with compare +
   classify stages and a verdict taxonomy (defect-eligible vs.
   inconclusive-routed vs. bounded), gated by the C1 invariants — the same
   discipline that made the frost rubric a usable acceptance bar.
3. **C3 — evaluation and disposition.** Run openWEPP over the mapped
   site/condition set and judge. Honest outcomes: **in-envelope** (close the
   FARPOINT01 flag; downgrade the backlog note), **out-of-envelope**
   (Defect-Closure ExecPlan against the conductivity/lateral model — the
   provisional forest `ksatadj` sat-fraction model is the leading suspect
   surface), or **not-applicable** (site conditions don't map; record the
   boundary). "Fidelity tuning" here means parameter/physics work executed
   against the ratified envelope — never against legacy output and never by
   relaxing the envelope.

**Boundary:** input-parameter authority (soil/management files) is
wepppy-side; physics/contract authority is ours. If C3 attributes the gap to
inputs, the disposition is a documented handoff, not an openWEPP code edit.

## 6. Lane D — Papanicolaou 2018 OFE-by-OFE routing implementation

**What it is.** The enhanced-WEPP overland-flow framework: route the event
hydrograph OFE-by-OFE (each OFE's outlet hydrograph is the next OFE's
upstream boundary condition) instead of the equivalent-plane/equilibrium-
storage aggregation; compute resistance per OFE per timestep from the
additive friction-factor menu (skin: Shen & Li / Hirsch by `Re` regime;
form: Abrahams; wave: Hu & Abrahams by `Fr`; vegetation: Katul/Thompson);
solve the 1-D kinematic wave with the TVD-MacCormack shock-capturing scheme
under CFL. Authority: Papanicolaou et al. (2018), WRR 54 — WEPP-lineage
(Flanagan and Frankenberger are co-authors), validated against four
published experiments.

**Why openWEPP is the right host.** The paper's obstacle in legacy — the
equivalent-plane representation — does not exist here: lanes *are* OFEs and
`TransferOutput → TransferInput` is already the inter-OFE seam
(`INV-RUNOFFPART-029`). The enhancement replaces the *content* of the
transfer (hydrograph instead of daily aggregate) without inventing a new
topology.

**Stages (staged-increment port template, FDHP01 D3 shape):**

1. **D1 — contracts and ADR.** `SC-*` authority for the KWE/TVD scheme,
   friction-factor menu, CFL policy, and per-OFE hydrograph handoff
   (extend `SC-RUNOFFPART-001` or author `SC-OFEROUTE-001`); an ADR fixing
   the representation decision and the activation policy (opt-in
   policy-gated first; default activation is a separate later gate with its
   own no-regression evidence, frost-Policy-B analog). Includes the
   dimensional/unit ledger for eqs. 2–6.
2. **D2 — validation fixtures from the supplemental.** Extract the four
   validation-case inputs (3.1 docx) and the Figure 4 series into typed test
   fixtures; likewise Figures 6–9 for the storm/gradient/curvature/
   stream-power experiments. Fixture-governance note in §8.
3. **D3..Dn — staged implementation.** Shadow-first: friction-factor
   kernels as pure functions with unit tests against tabulated values; then
   the single-OFE KWE/TVD solver validated on Case 1/2/4 (Case 4 = Iwagaki
   shock capture — the scheme-correctness case); then the OFE-by-OFE cascade
   handoff (Case 3 vegetation patchiness — the MOFE case); per-increment
   conservation hard stops throughout (the TVD scheme is documented
   continuity-preserving; assert it, don't assume it).
4. **D-val — acceptance.** Ef against observed series meeting-or-explaining
   the published values (0.91/0.75/0.87/0.88); Zone 1/Zone 2 stream-power
   taxonomy reproduced qualitatively (thresholds within stated tolerance of
   the paper's k/l fits); H2637 default path byte-flat while the feature is
   opt-in.
5. **D-scope (explicitly out, this campaign):** sediment coupling of the
   routed hydrograph (the `INV-RUNOFFPART-030` erosion-`qin` hold) — Lane D
   produces the hydraulics that a later sediment package consumes; curvature
   *input* surfaces beyond what slope files already carry; watershed-tier
   consumption of hillslope hydrographs.

**Perf guardrail:** hourly-per-OFE PDE solving is compute-bearing. The
opt-in path carries its own budget (target: within the current legacy-anchor
envelope on event days, measured at D-val); the default path must remain
unchanged (byte-flat) until an activation decision with fresh endpoint
timings.

## 7. Sequencing and gates

| Order | Package (proposed) | Lane | Gate to proceed |
|---|---|---|---|
| 1 | `MOFEFID-A01` defect-review sweep | A | — (start now) |
| 1 | `MOFEFID-B01` brief-audit (11 classes) | B | — (start now, parallel with A01) |
| 2 | `MOFEFID-B02` QOFE ecosystem-contract adjudication | B | B01 confirms B7 shape (already Ran-verified; B02 can scaffold immediately after B01 lands) |
| 2 | `MOFEFID-C01` authority promotion (metric/envelope/applicability) | C | — (independent) |
| 3 | `MOFEFID-C02` rubric harness | C | C01 envelope ratified |
| 3 | `MOFEFID-D01` routing contracts + ADR | D | — (authoring can start now) |
| 3 | `MOFEFID-D02` supplemental fixtures | D | D01 drafted |
| 4 | `MOFEFID-C03` evaluation + disposition | C | C02 green |
| 4 | `MOFEFID-D03..` staged implementation | D | A01 + B01 findings dispositioned; D01 ratified |
| 5 | `MOFEFID-D-val` validation acceptance | D | D3..Dn conservation stops green |

Stop conditions (campaign-wide): a confirmed Lane A/B defect in a surface a
later stage depends on halts that stage until the Defect-Closure ExecPlan
lands; Lane C never proceeds past an unratified envelope; Lane D never
activates by default inside this campaign.

## 8. Guardrails

- **Legacy is a flag, not a target** (ADR-0017) — doubly binding here: the
  brief proves legacy MOFE production *currently carries known unfixed
  defects* (B8, B9). Comparator deltas on those surfaces are expected and
  must be recorded as such, not chased.
- **The wepp-forest program's conclusions are also a flag, not an
  authority** (operator directive, 2026-07-01). Its problem observations
  are leads; its repair conclusions and convention choices carry no
  authority in openWEPP until independently adjudicated under the Lane B
  protocol (§4). This extends the ADR-0017 posture from the legacy binary
  to the legacy-side *program record*.
- **Closure first, magnitude last** (ROADMAP ordering principle). Lane C
  judges magnitude only because the closure prerequisites already hold.
- **Like-for-like cut-points** before any divergence is called a defect
  (the comparator-surface-artifact lessons: units, depth-vs-water-equiv,
  raw-vs-released).
- **Fixture use-limits are binding.** The lateral-flow README's five
  conditions and the Coweeta no-verdict restriction are part of this plan.
- **Copyright governance.** `references/copyrighted/` content (paper +
  supplemental) stays in-repo for internal validation only; derived test
  fixtures carry citation and the minimum numeric content needed; nothing
  from that directory is republished in public-facing docs.
- **Perf endpoint protection.** Default-path H2637 stays byte-flat through
  Lane D; any activation decision re-runs the endpoint timing gates.
- **Truthfulness discipline.** Every audit row and rubric verdict carries
  its evidence class (`Ran`/`Static`); review lanes label assessments
  accordingly.

## 9. Assets

| Asset | Location | Role |
|---|---|---|
| Stakeholder brief | `/workdir/wepp-forest/docs/20260504-stakeholder-watbalance.md` | Lane B defect census; QOFE canonical definitions; audit-formula reference |
| Lateral-flow observed data | `tests/fixtures/forest_lateral_flow_authority/` (HJ Andrews WS10, Panola 2002, Maimai M8, Coweeta) | Lane C authority candidates (README use-limits binding) |
| Papanicolaou 2018 paper | `references/copyrighted/Papanicolaou2018.md` (bibliography R-63) | Lane D physics authority |
| Brooks, Boll & McDaniel 2004 | `references/copyrighted/brooks2004.pdf` (bibliography R-62) | Lane C parameter-scale authority: hillslope-scale lateral `Ks` 3.2–13.7× core-scale; double-exponential `Ks(depth)`; macropore control — bounds what `ksatadj`/anisotropy may encode |
| Papanicolaou supplemental | `references/copyrighted/Papanicolaou2018-supplemental/` (case inputs docx, Figure 4–9 xlsx) | Lane D validation inputs + series |
| MOFE contract surface | `SC-RUNOFFPART-001` INV-028/029/030, `SC-WATBAL-001` INV-096/097/099 | Lanes A/B/D contract anchors |
| Snowfreeze observed harness | `tools/snowfreeze_observed/` | Lane C rubric template |
| Backlog note | `docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md` | Lane C promotion criteria (now satisfiable) |
| H2637 fixture + closure gates | staged WB05A replay inputs; direct-runtime closure guards | Cross-lane regression net |

## 10. Open decisions (operator)

1. **B7/QOFE posture:** switch `QOFE` to the post-fix `QOFE = Q` convention
   outright, or publish both (legacy-shaped column + canonical) during a
   wepppy consumer-migration window? Recommendation: switch outright with an
   ADR + change-control entry; the ecosystem brief already documents the
   post-fix recipes and wepppy consumers are being re-anchored.
2. **Lane D activation ambition:** is the target of this campaign
   opt-in-validated only (recommended), or default activation?
3. **Lane C tuning ownership:** if C3 lands out-of-envelope and attributes
   to input parameters (`ksatadj` coefficients, anisotropy), does openWEPP
   host the parameter-authority work or hand off to wepppy?
4. **Supplemental-derived fixture governance:** approve the minimal-numeric-
   content-with-citation approach in §8, or keep all derived series inside
   `references/copyrighted/`?

## 11. References / authority

- Papanicolaou, A. N., et al. (2018). Flow resistance interactions on
  hillslopes with heterogeneous attributes. *WRR*, 54, 359–380.
  doi:10.1002/2017WR021109 (+ supporting information, in-repo;
  bibliography R-63).
- Brooks, E. S., J. Boll, and P. A. McDaniel (2004). A hillslope-scale
  experiment to measure lateral saturated hydraulic conductivity. *WRR*,
  40, W04208. doi:10.1029/2003WR002858 (in-repo; bibliography R-62).
- wepp-forest stakeholder brief (2026-05-04, updated 2026-05-16) — legacy
  water-balance program record, QOFE canonical definitions, defect census.
- `docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`
  — Lane C promotion criteria.
- `docs/decisions/0011…0024` — architecture-first, comparator-as-flag,
  reference-implementation-intent authority.
- FARPOINT01 / MAGPARITY01 / STAGE2-LATQCC / BASECOND01 packages — the
  magnitude arc Lane C completes.

### 11.1 Acquired literature (2026-07-01 sweep; bibliography R-62..R-76)

Deep-research sweep (three parallel research passes, DOIs Crossref-verified,
PDFs title-verified at intake) acquired into `references/copyrighted/`:
Brooks 2004 (R-62), Papanicolaou 2018 + supplemental (R-63), Tromp-van
Meerveld & McDonnell 2006 parts 1+2 (R-64/65), Freer 2002 (R-66), McGuire &
McDonnell 2010 WS10 (R-67, the prime Lane C envelope anchor: quick-flow
ratio ≈0.58 above ~20 mm antecedent on the exact held hillslope), Weiler et
al. 2005 synthesis (R-68), Blume & van Meerveld 2015 (R-69), Srivastava et
al. 2017 (R-70), Pirastru et al. 2017 (R-71, CC-BY), Hu & Abrahams 2006
(R-72, formulation-tier for eq. 5), Wu/Yevjevich/Woolhiser 1978 CSU HP96
(R-73, the equivalent-plane legacy baseline), Iwagaki 1955 (R-74, Case 4
shock data), Abban 2017 (R-75, CC-BY), Helmers 2012 (R-76). Already held:
Dun 2009 (R-21), Srivastava 2013 dissertation (R-22), NSERL Report 10
chapter set (`references/50201000/`). **Operator same-day acquisitions
(R-77..R-98):** Lawrence 1997 and Katul 2011 (closing two of the five
formulation-tier gaps), Jomaa 2012, Thompson 2011, both TVD-MacCormack
numerics papers, the full Lane C observational foundation (Whipkey 1965,
Hewlett & Hibbert 1963, Dunne & Black 1970, Harr 1977, Mosley 1979,
McGlynn 2002, **Woods & Rowe 1996 M8 trench paper + transcription**,
Bachmair & Weiler 2011), the Palouse/anisotropy set (Brooks 2007,
Wigmosta 1994, Beven & Germann 1982/2013, McDaniel 2001/2008), and two
SMR-lineage extras (Hasan/Troch/Boll 2006; O'Keeffe et al. 2023).
Transcription companions exist for Harr 1977, Dunne & Black 1970,
Mosley 1979, and Woods & Rowe 1996.

### 11.2 Library freeze (2026-07-01, operator decision)

The reference library is **frozen at the current holdings**; the seven
remaining items are **cited as secondary** through in-hand carriers rather
than acquired:

| Un-acquired primary | Cite via (in hand) | Consequence |
|---|---|---|
| Shen & Li (1973) | Papanicolaou 2018 (R-63) eqs. (2)–(3) statement | eq. 2–3 constants/regime bounds taken from R-63; unit convention confirmed empirically when the D2 fixtures reproduce the Figure-4/Ef targets |
| Abrahams (1998) discussion + Lawrence reply | Lawrence 1997 (R-77, primary in hand) + R-63's simplified eq. (4) | form-resistance applicability limits carry a documented-uncertainty note in D1 instead of the published critique |
| Woolhiser (1975) k₀ tables | KINEROS documentation (Smith 1990, in repo) + R-63 | laminar coefficients from the KINEROS reproduction |
| Hewlett & Hibbert (1967) | McGlynn 2002 (R-88), Weiler 2005 (R-68), Bachmair & Weiler 2011 (R-90) | conceptual VSA framing only; fully covered by held syntheses |
| Frankenberger et al. (1999) | Brooks 2007 (R-91), Dun 2009 (R-21), O'Keeffe 2023 (R-98) | SMR lineage adequately documented by descendants |
| Zaslavsky & Sinai (1981) | Brooks 2004 (R-62), Pirastru 2017 (R-71), Wigmosta 1994 (R-92) | anisotropy anchored empirically rather than theoretically |
| Neibling & Alberts (1979) | supplemental `Figure_4.xlsx` series + R-63 §3.1.3 | Case-3 fixture built from the supplemental data with R-63 as provenance |

Rule: any `SC-*` invariant that leans on one of these must cite the in-hand
carrier (e.g. `R-63`), not the un-held primary, and mark the anchor
`secondary` — the standing bibliography convention for print-source
companions (cf. R-02/R-03).
