# STAGE2-LATQCC-H2637-MAGNITUDE Kickoff — WB19 Lateral Generation Magnitude

Execution mode: Stage-2 physics-magnitude adjudication — produce a VERDICT, not a fix.

Autonomy: execute end-to-end (operand extraction → equation-correctness → operand plausibility →
like-for-like legacy flag → verdict → handoff). **Do not fix, and do not chase legacy parity.**

## The question

MAGPARITY01 proved the runoff-partition machinery is correct (transfer/area/closure/export at
machine epsilon) and localized H2637's 71% outlet `runvol` to **routed lateral flow** (`latqcc`
~757k m³/OFE, Σ = 13.99M m³ to the outlet). The open question: **is the WB19 lateral-flow
generation magnitude physically correct?** Conservation can't answer it — magnitude is
conservation-independent. Judge it against the lateral-flow **physics**.

## Framing — non-negotiable

- **Magnitude is judged vs physics + external authority, not conservation, not legacy.** A defect
  means a violated `SC-SUBHYD-001` invariant / Eq [6.2.4] / physical implausibility — **not** a
  difference from legacy.
- **Legacy is a flag (ADR-0017).** No "match 55.5%/72.2%." Legacy with_ui is non-conserving,
  disqualified.
- **Do not re-open MAGPARITY01:** transfer (`INV-RUNOFFPART-028`), area scaling, export duality are
  settled; the outlet surface-vs-lateral *partition* is downstream of the generation magnitude.

## Method — verify the equation, then judge the operands

From the closed H2637 run, extract the **per-OFE WB19 operands behind `latqcc`** (not WAT/PASS aliases):

1. **Equation correctness:** does emitted `latqcc` = Eq [6.2.4] (Ch.6) / Dun 2009 Eq 3a-3c on H2637's
   operands (equivalent conductivity, drainable thickness `Ho`, slope, length, `drfc = fc+(1-coca)·dg`,
   drainable water `θd`)? Mismatch → kernel defect (`INV-SUBHYD-003/012`). Match → the magnitude
   question is the operands.
2. **Operand plausibility (the magnitude drivers):**
   - **Forest `ksatadj` equivalent conductivity** — PRIME lead; is the sat-fraction model inflating
     equivalent K vs soil-file `Ksat`/restrictive-layer?
   - **Drainable thickness / saturated-thickness response** (`Ho`, Eq [6.2.5]; `INV-SUBHYD-018`,
     `cas_l4_subhyd_lateral_saturated_thickness_response`) — correct, restrictive-layer-bounded?
   - **`drfc` FC-based threshold** — `fc` correct, so drainable water `θ−drfc` (and `q`) is right?
     (The earlier "FC 2× too low" sub-diagnosis was **withdrawn/wrong** — do not pre-load it.)
3. **Like-for-like legacy flag:** compare WB19 lateral operands + `latqcc` vs legacy, **dimensionally
   identical cut-points** (the comparator-surface-artifact hazard — a round delta is a unit smell).
4. **Verdict** (ADR-0017): `CORRECT` / `OPENWEPP-DEFECTIVE` / `LEGACY-DEFECTIVE` / `UNRESOLVED` /
   `CONTRACT-GAP`, each citing the invariant/equation/authority.

## Verdict + handoff

- `CORRECT` → the **FARPOINT01 runoff flag resolves** (71% is correct forest hydrology); document + close.
- `OPENWEPP-DEFECTIVE` → Defect-Closure ExecPlan item-1 (ADR-0018), citing the invariant + inflated
  driver. No fix here.
- `UNRESOLVED` → name the external authority/data needed.
- `CONTRACT-GAP` → the `cas_l4` suite tests lateral *response* monotonicity, not absolute magnitude;
  if magnitude is equation-consistent + operand-plausible but still unjudgeable, flag the need for an
  absolute-magnitude external-authority suite — do not invent a bound.

## Constraints / truthfulness

- No code/physics fix; no `SC-*` change (gap flagged); no INV-028 reopen; no export change; irrigation deferred.
- Legacy a flag only, with attribution and a dimensional-consistency proof.
- Label evidence Static / Ran. Match a claim's confidence to its grounding — a hypothesis is not a
  verdict; a verdict cites the equation/invariant/authority.

## Required reading

- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/package.md`
- `docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/artifacts/{magparity01-runoff-decomposition,magparity01-external-authority-plausibility,magparity01-handoff}.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (INV-SUBHYD-001/002/003/004/012/018)
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`, `SC-INFILE-GWCOEFF-001.md`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_lateral_saturated_thickness_response_001.md`
- `references/50201000/chap6.pdf` (Eq [6.2.1]-[6.2.5]); `references/annotated_bibliography.md` (R-21 Dun 2009, Srivastava 2013)
- `docs/decisions/0017-...comparator-is-flag-not-target.md`, `0018-defect-closure-execplans-conversion-rule.md`, `docs/defect_closure_execplans.md`
- `docs/specifications/correctness-authority-model.md`, `AGENTS.md`, `docs/work-packages/AGENTS.md`
- The forest `ksatadj` conductivity model + the FC/WP soil-file lineage.
