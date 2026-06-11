# SCSTRUCT05 Kickoff — SC-SYSTEM BEI science-review adjudication

Execution mode: **science-steered**, batched (not autonomous).

Autonomy: within a batch, execute the mechanics autonomously — resolve rows to
cited outcomes, relocate historical narrative, run the lint, and dispatch the
closure loop to the `comparator_suite_runner` subagent — but **stop and surface
any row whose binding status needs an authority decision**. Capture early
reduction on the low-risk batch first; do not grind the live-authority cohort
without science input.

## What and why

SCSTRUCT04 added the SC-SYSTEM Binding Exposure Index and routed unresolved rows
to `science-review-follow-on`. This package does the domain adjudication and, as
rows resolve, relocates historical narrative to the sidecar so the contract
shrinks — the SCSTRUCT03 method applied to SC-SYSTEM. Verify the starting state:
`python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`.

Read `package.md` first — authority for the per-row outcomes, the
citation/authority requirement, batching, the closure/test-reconciliation
requirement, and the protected boundaries. **Non-negotiable: no binding obligation
removed or weakened; promotions only via the full review gate; legacy is not
authority; no forced calls; no kernel edits.**

## Required reading
- `package.md` (this WP)
- `../20260610-scstruct04-system-binding-exposure-index-mapping-001/artifacts/science-review-followon-queue.md`
- the `## Binding Exposure Index` in `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contract-spec.md` + `science-contract-provenance-spec.md`
- `../20260608-scstruct03-watbal-bei-science-review-adjudication-001/` (worked precedent + its batch reviews)
- `docs/standards/mechanical-refactor-authoring-guide.md` (§6.2 closure loop)

## Method (per batch, lowest-risk first per package.md)
1. For each row apply the per-row outcome with a cited authority anchor
   (map-to-INV / promote / historical / map-in-core / narrower-HOLD).
2. Relocate only `historical`/`superseded` narrative to
   `provenance/SC-SYSTEM-001-provenance.md` per the provenance spec.
3. `promote` → author the new `INV-SYSTEM-*`/`OBL-SYSTEM-*` with guard map through
   the full dual-review/disposition/verification gate (flagged addition).
4. **Run the closure loop and reconcile any contract-derived test breakage** —
   dispatch the heavy runs (`cargo test --workspace`, comparator/release suites) to
   the `comparator_suite_runner` subagent; consume only its compact metrics + log
   paths. Reconcile path/structure only, never a behavior assertion.
5. Replay `--strict` lint; record deferred-count drop + token delta; checkpoint.

## Deliverables
- Resolved Binding Exposure Index (no bare science-review-follow-on rows).
- Populated `provenance/SC-SYSTEM-001-provenance.md`.
- `artifacts/system-binding-crosswalk.md` (conservation incl. flagged additions).
- `artifacts/system-core-size-delta.md` (per-batch + total token/byte reduction).
- per-batch `--strict` lint evidence + closure-loop gate logs (via subagent) +
  test-reconciliation record.
- Dual review/disposition/verification for promotions + kernel-profile checklist.

## Hard stops
- A row needs external authority not in the repo → narrower science-HOLD; continue
  other batches. Do not force a binding call.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.
  Do not edit kernel code here.
