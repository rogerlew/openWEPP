# SCSTRUCT02 Kickoff — WATBAL Binding Exposure Index mapping + consolidation

Execution mode: package-end-to-end (batched; replay the lint between batches)

Autonomy: execute end-to-end — adjudicate every blocking Binding Exposure Index
row, promote genuinely-unpromoted obligations through the review gate, relocate
historical narrative to a sidecar, prove conservation, and run the lint green —
without asking for direction on intermediate steps. Ask only if hard-blocked or
at a declared stop-boundary (science-review-follow-on or a contract↔kernel
mismatch).

## What and why

SCSTRUCT01 added a Binding Exposure Index to `SC-WATBAL-001` and stopped at HOLD
because ~61 addenda carry binding language with no precise canonical
`INV-WATBAL-*`/`OBL-WATBAL-*` mapping. This package pays that debt so the contract
can be safely slimmed. Verified starting state (`Ran`):
`python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
→ `FAIL … 133 issue(s)`.

Read `package.md` first — it is the authority for the per-row adjudication rule,
batching, the authority envelope, and the protected boundaries. The
non-negotiable boundary: **no binding obligation removed or weakened; additions
only via the flagged review gate; the before/after `INV-*`/`OBL-*` crosswalk must
prove conservation; no kernel code touched.**

## Required reading
- `package.md` (this WP)
- `../20260608-scstruct01-science-contract-doc-split-and-provenance-framework-001/artifacts/phase2-binding-exposure-index-hold.md`
- the current `## Binding Exposure Index` in
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contract-spec.md` (index schema + lint contract)
- `docs/specifications/science-contract-provenance-spec.md` (sidecar format)
- `docs/specifications/science-contract-authoring-procedure.md` (Binding Exposure Workflow + gates)
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`

## Method (per batch)
1. Take one sub-domain batch (see `package.md` batching list).
2. For each row, apply the per-row adjudication rule: map-to-existing-`INV`,
   promote (flagged addition via full gate), historical (sidecar-eligible), or
   route science-review-follow-on. Cite the specific obligation — never a token
   scrape.
3. Relocate only `historical`/`superseded` narrative to
   `.../contracts/provenance/SC-WATBAL-001-provenance.md` per the provenance spec.
4. Re-run the lint; record the issue-count delta.
5. Checkpoint, then take the next batch.

## Deliverables
- Resolved Binding Exposure Index (no unresolved `unpromoted-binding`/`undecidable`).
- `SC-WATBAL-001-provenance.md` sidecar.
- `artifacts/watbal-binding-crosswalk.md` — before/after `INV-*`/`OBL-*`
  conservation proof + flagged additions.
- `artifacts/watbal-core-size-delta.md` — token/byte before/after.
- Lint green evidence.
- Dual review/disposition/verification + kernel-profile compliance checklist.

## Hard stops
- A row needs a science decision → mark `science-review-follow-on`, keep its
  narrative in core, continue the rest. Do not force a binding call.
- A mapping exposes a contract↔kernel mismatch → HOLD, route a separate kernel
  package. Do not edit kernel code here.
