# SCSTRUCT07 Kickoff — SC-SUBHYD BEI science-review adjudication

Execution mode: **science-steered**, batched (not autonomous).

Autonomy: within a batch, execute the mechanics autonomously — resolve rows to
cited outcomes, relocate historical narrative, run the lint, dispatch the closure
loop to the `comparator_suite_runner` subagent — but **stop and surface any row
whose binding status needs an authority decision**. Capture early reduction on the
low-risk batch first; do not grind the live-authority cohort without science input.

## What and why

SCSTRUCT06 added the SC-SUBHYD Binding Exposure Index and routed unresolved rows to
`science-review-follow-on`. This package adjudicates them and relocates historical
narrative — the SCSTRUCT03/05 method applied to SC-SUBHYD (the WB19 subsurface
family). Expect more map-in-core/promote than relocation: many rows are
live constitutive authority (HPHYS0224–0227 carry Level-4 suite linkages), so the
token yield is smaller than SC-SYSTEM — the win is auditability + the gate.

Verify the start state:
`python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`.

Read `package.md` first. **Non-negotiable: no binding obligation removed or
weakened; promotions only via the full review gate (keep Level-4 suite linkages);
legacy is not authority; no forced calls; no kernel edits.**

## Required reading
- `package.md` (this WP)
- `../20260610-scstruct06-subhyd-binding-exposure-index-mapping-001/artifacts/science-review-followon-queue.md`
- the `## Binding Exposure Index` in `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contract-spec.md` + `science-contract-provenance-spec.md`
- `../20260610-scstruct05-system-bei-science-review-adjudication-001/` (most recent worked precedent: batches, test-reconciliation, pre-existing-red handling)
- `docs/specifications/external-authority/suite-schema.md` (for HPHYS0224–0227 suite linkage on promotion)
- `docs/standards/mechanical-refactor-authoring-guide.md` (§6.2 closure loop)

## Method (per batch, lowest-risk first per package.md)
1. For each row apply the per-row outcome with a cited authority anchor
   (map-to-INV / promote / historical / map-in-core / narrower-HOLD).
2. Relocate only `historical`/`superseded` narrative to
   `provenance/SC-SUBHYD-001-provenance.md` per the provenance spec.
3. `promote` → author the new `INV-SUBHYD-*`/`OBL-SUBHYD-*` with guard map (and
   Level-4 suite linkage where the addendum carries one) through the full gate.
4. **Run the closure loop and reconcile any contract-derived test breakage** —
   dispatch heavy runs to `comparator_suite_runner`; consume only compact metrics.
   Reconcile path/structure only; assertions should *strengthen* (verify BEI +
   mapped INV + sidecar), never weaken. Confirm any failing gate is pre-existing
   and unrelated (diff scope + cross-reference) before treating it as a blocker.
5. Replay `--strict` lint; record deferred drop + token delta; checkpoint.

## Deliverables
- Resolved Binding Exposure Index (no bare science-review-follow-on rows).
- Populated `provenance/SC-SUBHYD-001-provenance.md`.
- `artifacts/subhyd-binding-crosswalk.md` (conservation incl. flagged additions + suite linkages).
- `artifacts/subhyd-core-size-delta.md` (per-batch + total token/byte reduction).
- per-batch `--strict` lint evidence + closure-loop gate logs (via subagent) +
  test-reconciliation record.
- Dual review/disposition/verification for promotions + kernel-profile checklist.

## Hard stops
- A row needs external authority not in the repo (e.g. open `GAP-SUBHYD-*`) →
  narrower science-HOLD; continue other batches. Do not force a binding call.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.
