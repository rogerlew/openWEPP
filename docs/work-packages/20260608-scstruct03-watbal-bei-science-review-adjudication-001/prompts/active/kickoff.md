# SCSTRUCT03 Kickoff — WATBAL Binding Exposure Index science-review adjudication

Execution mode: **science-steered**, batched (not autonomous)

Autonomy: this package is NOT a fire-and-forget autonomous run. Each of the 69
routed rows is a water-balance domain decision. Within a batch, execute the
mechanics autonomously (resolve rows to cited outcomes, relocate historical
narrative, run `--strict`), but **stop and surface any row whose binding status
needs an authority decision** rather than guessing. Capture early reduction on the
low-risk batch first; do not grind through the hard cohort without science input.

## What and why

SCSTRUCT02 routed all 69 unresolved `SC-WATBAL-001` Binding Exposure Index rows to
`science-review-follow-on` and held — it correctly refused to guess which addenda
are live vs superseded. This package does the domain adjudication and, as rows
resolve, relocates historical narrative to the sidecar so the contract finally
shrinks. Verified starting state (`Ran`):
`python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
→ `PASS-DEFERRED … 69 science-review-follow-on row(s) not yet consolidated`, exit 1.

Read `package.md` first — it is the authority for the per-row adjudication
outcomes, the citation/authority requirement, batching, and the protected
boundaries. Non-negotiable: **no binding obligation removed or weakened;
promotions only via the full review gate; legacy is not authority; no forced
calls; no kernel edits.**

## Required reading
- `package.md` (this WP)
- `../20260608-scstruct02-watbal-binding-exposure-index-mapping-001/artifacts/science-review-followon-queue.md`
- the `## Binding Exposure Index` in
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (the 69 routed rows)
- `docs/specifications/science-contract-spec.md` (index schema + lint verdicts)
- `docs/specifications/science-contract-provenance-spec.md` (sidecar format)
- `docs/specifications/science-contract-authoring-procedure.md` (derivation order, gates)
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`

## Method (per batch, lowest-risk first per package.md batching)
1. Take a batch; for each row apply the per-row adjudication outcome with a cited
   authority anchor (WEPP refs > literature > physical > legacy-static-secondary).
2. `map`/`historical` → update the index row; relocate `historical` narrative to
   `provenance/SC-WATBAL-001-provenance.md` per the provenance spec.
3. `promote` → author the new `INV-WATBAL-*`/`OBL-WATBAL-*` with guard map through
   the full dual-review/disposition/verification gate (flagged addition).
4. `narrower-HOLD` → keep routed, but record the specific authority gap + owner +
   next gate.
5. Replay `--strict`; record the deferred-count drop and token delta; checkpoint.

## Deliverables
- Resolved Binding Exposure Index (no bare science-review-follow-on rows).
- Populated `provenance/SC-WATBAL-001-provenance.md`.
- `artifacts/watbal-binding-crosswalk.md` (conservation, incl. flagged additions).
- `artifacts/watbal-core-size-delta.md` (per-batch + total token/byte reduction).
- `--strict` lint evidence advancing toward `PASS`.
- Dual review/disposition/verification for promotions + kernel-profile checklist.

## Hard stops
- A row needs external authority not in the repo → narrower science-HOLD; continue
  other batches. Do not force a binding call.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.
  Do not edit kernel code here.
