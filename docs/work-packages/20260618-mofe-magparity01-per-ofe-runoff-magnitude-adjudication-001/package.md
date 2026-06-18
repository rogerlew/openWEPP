# MOFE-MAGPARITY01 - Per-OFE Runoff Magnitude Adjudication

Status: scaffolded 2026-06-18 (Stage-2 magnitude triage; adjudicates the runoff-magnitude
divergence FARPOINT01 surfaced and deferred here)

Package type: **Science adjudication — produces a per-term VERDICT, not a fix.** The output is
a disposition: for each component of the runoff-to-outlet magnitude, *expected Stage-2
divergence* (defer to the broader Stage-2 physics-magnitude judgment) or *defect-shaped
follow-on* (hand off a Defect-Closure ExecPlan, ADR-0018). No code or contract change lands
here unless the adjudication finds a contract gap to flag.

## The finding (FARPOINT01, assumption-light)

On H2637 (19-OFE far-point, 34-year), exported hillslope surface runoff as a fraction of total
precip (`19,837,945 m³`), via the WSHED01 duality `QOFE_outlet × A_outlet ≡ Q_outlet ×
A_hillslope` (legacy duality holds at ratio `1.0000`):

| engine / variant | outlet runoff | % of precip | runoff ≤ precip? |
|---|---|---|---|
| **openWEPP** (wepp_ui-invariant) | 14,085,670 m³ | **71.0 %** | ✅ |
| legacy `wepp_260606` without_ui | 11,011,150 m³ | 55.5 % | ✅ |
| legacy `wepp_260606` with_ui | 25,331,296 m³ | 127.7 % | ❌ (q-cap non-conservation) |

openWEPP exports **~28 % more** runoff than legacy without_ui — both **bounded** (conservation
already closed). FARPOINT01 classed this a *magnitude* difference and deferred it here. The
adjudication's job is to confirm or refine that, **per term**.

## The framing (non-negotiable)

- **Closure, not magnitude, not comparator-match.** The routed water balance is **already
  closed** (conservation + bounds). So this is a *magnitude* question judged against an
  already-closed structure — magnitude error must not be aliased with structural error. A term
  is a **defect** only if it violates a governing `SC-*` invariant or a physical bound, **not**
  because it differs from legacy.
- **Legacy is a flag, not the target (ADR-0017).** The 55.5 % is an *investigation flag*, not
  authority. **Do not** "fix" openWEPP toward 55.5 %. Legacy's runoff is in fact known-buggy
  (with_ui = 127.7 %, the WB-05A OFE19 q-cap non-conservation) — its bounded variant is a weak
  reference at best.
- **The question:** is openWEPP's 71 % consistent with `SC-RUNOFFPART-001` and physically
  plausible for H2637's substrate/climate, and is the divergence from legacy **expected
  Stage-2 magnitude** or a **defect**?

## Governing authority

- **`SC-RUNOFFPART-001`** (Surface Runoff Partition Process Contract) — the runoff
  generation/partition authority. In particular **`INV-RUNOFFPART-028`** (MOFE inter-OFE
  runoff/lateral transfer): `UpStrmQ = Σui_SUrunf(ii)·Aupstream/Acurrent`,
  `SubRIn = Σui_LfUrf(ii)·Aupstream/Acurrent`, `runon_input = UpStrmQ + SubRIn`, with event
  closure `local_liquid + runon_input = infiltration + Q_partition + Δdepression + ε`.
- `SC-WATBAL-001` (daily root-zone closure — the conservation that is already closed).
- `SC-ROUTE-001` (watershed routing/channel) for the export/duality.

## Method — decompose, then judge each term against its contract

Adjudicate the outlet-runoff magnitude as a **sum of terms**, using the existing H2637 outputs
(per-OFE HBP/interchange parquet) — no re-run of physics required beyond reading the closed run:

1. **Per-OFE runoff generation** (infiltration-excess `Q_partition` on each OFE): is openWEPP's
   per-OFE runoff coefficient consistent with `SC-RUNOFFPART-001` and plausible for the
   substrate? Where does generation differ most from legacy's per-OFE columns (flag only)?
2. **Inter-OFE run-on re-infiltration** (`INV-RUNOFFPART-028`) — **the prime suspect.** On a
   19-OFE cascade, run-on from upslope re-infiltrates on downslope OFEs. Does openWEPP re-absorb
   run-on per the contract's event closure, or **under-absorb** (more `Q_partition`, inflating
   the outlet)? Verify the `Aupstream/Acurrent` area scaling does **not** inflate `runon_input`
   (the QOFE/Q area-duality hazard — mixing per-OFE-area vs hillslope-area has over-scaled
   runoff volume before). Confirm INV-028 event closure holds per OFE and use it to attribute
   the extra runoff to a specific OFE band / term.
3. **Export / duality** (`SC-ROUTE-001`): does openWEPP's `QOFE_outlet × A_outlet ≡ Q_outlet ×
   A_hillslope` duality hold (as legacy's does at 1.0000)? Any export-scaling discrepancy is a
   distinct term.

For each term, render a verdict: **expected Stage-2 magnitude** or **defect** (cite the
violated invariant / bound).

## Leads to test (surface, do not pre-decide)

- The **inter-OFE run-on re-infiltration** is the most likely locus of a 28 % outlet inflation
  on a 19-OFE cascade — under-absorbed run-on cascades to the outlet without breaking
  conservation. INV-RUNOFFPART-028 is the test.
- The **area scaling** (`Aupstream/Acurrent`) — confirm it is applied consistently and does not
  over-scale `runon_input` (see the QOFE/Q area-duality record).
- **Legacy's 55.5 % is not ground truth** — its runoff path is known non-conserving in with_ui;
  the bounded variant is a flag only. An **external-authority** plausibility check (expected
  runoff coefficient for H2637's soils/cover/climate) is worth more than the legacy delta.

## Verdict + handoff

- **Expected Stage-2 magnitude** (per term): document the magnitude + the external-authority
  check that should judge it, and defer to ROADMAP item *Stage-2 physics-magnitude*. No fix.
- **Defect** (per term): author a **Defect-Closure ExecPlan** (ADR-0018,
  `docs/defect_closure_execplans.md`) item-1 — the specific invariant violated, the localizing
  evidence, and the conversion-rule-shaped handoff. No fix in this package.
- If the adjudication exposes a **contract gap** (the magnitude is unjudgeable because no
  invariant constrains it), flag it for an `SC-RUNOFFPART-001` amendment proposal — do not
  invent a bound.

## Scope

In scope: the H2637 far-point fixture (and the MOFE fixtures if they sharpen the per-OFE
attribution); the per-term decomposition + contract check + verdict; the conditional
defect-closure ExecPlan or Stage-2 deferral.

Out of scope:

- **No code/physics fix** — this package adjudicates; fixes are defect-closure follow-ons.
- **No "match legacy"** — legacy is a flag (ADR-0017).
- No `SC-*` change (a contract gap is *flagged*, not patched).
- No conservation re-litigation (the routed balance is closed; this is magnitude).
- Irrigation deferred/inert.

## Acceptance Criteria

- **Per-OFE decomposition** of outlet runoff into generation / inter-OFE re-infiltration /
  export terms, from the existing closed H2637 run, with INV-RUNOFFPART-028 event closure
  confirmed per OFE.
- **Per-term verdict** (expected Stage-2 vs defect), each citing the governing invariant/bound
  or the external-authority plausibility basis — not the legacy delta alone.
- **Area-scaling check** (`Aupstream/Acurrent`) recorded as sound or flagged.
- **Handoff:** a Defect-Closure ExecPlan item-1 for any defect term, or a documented Stage-2
  deferral with the external-authority check named, or a contract-gap flag.
- Evidence labeled Static / Ran; legacy used only as a flag, with attribution.
- Markdown lint clean. (No Rust gates unless code is touched — none expected.)

## Deliverables

- `artifacts/magparity01-runoff-decomposition.md` (per-OFE generation/re-infiltration/export terms)
- `artifacts/magparity01-inv028-closure-check.md` (INV-RUNOFFPART-028 event closure + area scaling)
- `artifacts/magparity01-external-authority-plausibility.md` (runoff-coefficient plausibility for H2637)
- `artifacts/magparity01-per-term-verdict.md` (expected-Stage-2 vs defect, per term, with citations)
- `artifacts/magparity01-handoff.md` (defect-closure ExecPlan item-1 / Stage-2 deferral / contract-gap flag)
- `artifacts/magparity01_disposition.md`

## Dependencies

- `docs/work-packages/20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/artifacts/{fc-legacy-closure-contrast,disposition,fixture-and-baseline-evidence}.md` (the finding + the H2637 fixture/baseline)
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` (esp. INV-RUNOFFPART-028; INV-013/014)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`; `SC-ROUTE-001.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` (legacy = flag)
- `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`; `docs/defect_closure_execplans.md`
- `docs/specifications/correctness-authority-model.md`; `docs/specifications/external-authority/`
- `docs/ROADMAP.md` (the closure-not-magnitude ordering; Stage-2 physics-magnitude item)
- `AGENTS.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the per-OFE decomposition across the 19
OFEs (reading the interchange parquet per OFE) is parallelizable. Record command evidence.

## Autonomy

Execute end-to-end through the per-OFE decomposition, the INV-RUNOFFPART-028 closure + area-scaling
check, the external-authority plausibility check, the per-term verdict, and the handoff
(defect-closure ExecPlan / Stage-2 deferral / contract-gap flag). The verdict is the
deliverable — render it from contract + bound + plausibility evidence, with legacy as a flag
only. A term that is genuinely expected Stage-2 magnitude is a valid, complete outcome; so is a
defect with a clean handoff. Do not fix and do not chase legacy parity.
