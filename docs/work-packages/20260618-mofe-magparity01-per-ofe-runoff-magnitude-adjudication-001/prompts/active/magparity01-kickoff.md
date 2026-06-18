# MOFE-MAGPARITY01 Kickoff — Per-OFE Runoff Magnitude Adjudication

Execution mode: science adjudication — produce a per-term VERDICT, not a fix.

Autonomy: execute end-to-end (per-OFE decomposition → INV-RUNOFFPART-028 closure + area-scaling
check → external-authority plausibility → per-term verdict → handoff). The verdict is the
deliverable. **Do not fix anything, and do not chase legacy parity.**

## The question

On H2637 (19-OFE far-point), openWEPP exports **71.0 %** of precip as outlet runoff vs legacy
without_ui **55.5 %** (legacy with_ui is the broken 127.7 % q-cap case). Both **bounded** —
conservation is already closed. FARPOINT01 deferred this here. **Adjudicate, per term, whether
openWEPP's extra ~28 % is expected Stage-2 magnitude divergence or a defect.**

## Framing — non-negotiable

- **Closure, not magnitude, not comparator-match.** The routed balance is already closed. This
  is a *magnitude* question against a closed structure. A term is a **defect** only if it
  violates a governing `SC-*` invariant or a physical bound — **not** because it differs from
  legacy.
- **Legacy is a flag, not the target (ADR-0017).** Do **not** push openWEPP toward 55.5 %.
  Legacy's runoff is known non-conserving (with_ui 127.7 %); its bounded variant is a weak flag.
- **The real question:** is openWEPP's 71 % consistent with `SC-RUNOFFPART-001` and physically
  plausible for H2637's soils/cover/climate?

## Method — decompose, judge each term against its contract

From the existing **closed** H2637 run (per-OFE HBP/interchange parquet — no physics re-run):

1. **Per-OFE runoff generation** (`Q_partition`): is openWEPP's per-OFE runoff coefficient
   consistent with `SC-RUNOFFPART-001` and plausible? Where does it differ most from legacy's
   per-OFE columns (flag only)?
2. **Inter-OFE run-on re-infiltration** — **the prime suspect**, governed by
   **`INV-RUNOFFPART-028`**: `runon_input = UpStrmQ + SubRIn` with
   `UpStrmQ = Σui_SUrunf·Aupstream/Acurrent`, event closure
   `local_liquid + runon_input = infiltration + Q_partition + Δdepression + ε`. On a 19-OFE
   cascade, does openWEPP **re-absorb** run-on per contract, or **under-absorb** (inflating the
   outlet without breaking conservation)? Confirm INV-028 closure per OFE and attribute the
   extra runoff to a specific OFE band. **Verify the `Aupstream/Acurrent` scaling does not
   over-scale `runon_input`** (the QOFE/Q area-duality hazard — it has over-scaled runoff volume
   before).
3. **Export / duality** (`SC-ROUTE-001`): does openWEPP's `QOFE_outlet × A_outlet ≡ Q_outlet ×
   A_hillslope` hold (legacy's is 1.0000)? Any export-scaling gap is its own term.

## Verdict + handoff

Per term, render: **expected Stage-2 magnitude** or **defect** (cite the violated invariant /
bound). Then:

- expected Stage-2 → document the magnitude + the external-authority check that should judge it;
  defer to the Stage-2 physics-magnitude item. No fix.
- defect → author a **Defect-Closure ExecPlan** item-1 (ADR-0018, `docs/defect_closure_execplans.md`):
  the invariant violated, the localizing evidence, the conversion-rule handoff. No fix here.
- contract gap (no invariant constrains the magnitude) → flag an `SC-RUNOFFPART-001` amendment
  proposal; do not invent a bound.

## Constraints / truthfulness

- No code/physics fix; no `SC-*` change; no conservation re-litigation; irrigation deferred.
- Legacy used **only as a flag**, with attribution ("legacy's column shows X"); the verdict
  rests on contract + bound + external-authority plausibility, not the legacy delta.
- Label evidence Static / Ran. Match the claim's confidence to the grounding — a hypothesis is
  not a verdict; a verdict needs a cited invariant/bound or a plausibility basis.

## Required reading

- `docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/package.md`
- `docs/work-packages/20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/artifacts/{fc-legacy-closure-contrast,disposition,fixture-and-baseline-evidence}.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` (INV-RUNOFFPART-028 esp.; INV-013/014)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`, `SC-ROUTE-001.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`,
  `0018-defect-closure-execplans-conversion-rule.md`, `docs/defect_closure_execplans.md`
- `docs/specifications/correctness-authority-model.md`, `docs/specifications/external-authority/`
- `docs/ROADMAP.md` (closure-not-magnitude ordering; Stage-2 physics-magnitude item)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`
