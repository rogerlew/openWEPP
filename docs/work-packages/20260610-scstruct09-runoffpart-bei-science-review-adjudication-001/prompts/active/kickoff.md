# SCSTRUCT09 Kickoff — SC-RUNOFFPART BEI science-review adjudication

Execution mode: **science-steered**, batched (not autonomous).

Subagent requirement (REQUIRED, not optional): spawn the `comparator_suite_runner`
subagent (gpt-5.3-codex-spark) for the closure loop (`cargo test --workspace`,
clippy, deny) and any comparator/population runs. **Do NOT run heavy batch/closure
work on the parent model** unless the subagent is unavailable, in which case record
command-level evidence as justification. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`; it returns compact metrics + log
paths only (no source/contract edits). See `docs/standards/prompt-wording-guidance.md` §4a.

Autonomy: within a batch, execute the mechanics autonomously — resolve rows to
cited outcomes, relocate historical narrative, run the lint, **spawn
`comparator_suite_runner` for the closure loop** — but **stop and surface any row
whose binding status needs an authority decision**. Capture early reduction on the
low-risk batch first; do not grind the live-authority cohort without science input.

## What and why

SCSTRUCT08 added the SC-RUNOFFPART Binding Exposure Index and routed unresolved rows
to `science-review-follow-on`. This package adjudicates them and relocates historical
narrative — the SCSTRUCT03/05/07 method applied to SC-RUNOFFPART. Mixed character:
expect *some* relocation token-yield from the ADR-0017-retired snow/`RM` arc rows
(HPHYS0296–0298) plus map-in-core for live WB12/14/16 runoff authority.

Verify the start state:
`python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`.

Read `package.md` first. **Non-negotiable: no binding obligation removed or
weakened; promotions only via the full review gate; legacy is not authority; no
comparator re-tiering; no forced calls; no kernel edits.**

## Required reading
- `package.md` (this WP)
- `../20260610-scstruct08-runoffpart-binding-exposure-index-mapping-001/artifacts/science-review-followon-queue.md`
- the `## Binding Exposure Index` in `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contract-spec.md` + `science-contract-provenance-spec.md`
- `../20260610-scstruct07-subhyd-bei-science-review-adjudication-001/` and
  `../20260608-scstruct03-watbal-bei-science-review-adjudication-001/` (worked precedents: map-in-core + snow-arc relocation)
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` (snow/`RM` rows)
- `docs/standards/mechanical-refactor-authoring-guide.md` (§6.2 closure loop)
- `docs/standards/prompt-wording-guidance.md` §4a (subagent requirement)

## Method (per batch, lowest-risk first per package.md)
1. For each row apply the per-row outcome with a cited authority anchor
   (map-to-INV / promote / historical / map-in-core / narrower-HOLD).
2. Relocate only `historical`/`superseded` narrative to
   `provenance/SC-RUNOFFPART-001-provenance.md` per the provenance spec.
3. `promote` → author the new `INV-RUNOFFPART-*`/`OBL-RUNOFFPART-*` with guard map
   through the full gate.
4. **Spawn `comparator_suite_runner` to run the closure loop** and reconcile any
   contract-derived test breakage; consume only its compact metrics + log paths.
   Reconcile path/structure only; assertions should *strengthen* (verify BEI +
   mapped INV + sidecar), never weaken. Confirm any failing gate is pre-existing
   and unrelated (diff scope + cross-reference) before treating it as a blocker.
5. Replay `--strict` lint; record deferred drop + token delta; checkpoint.

## Deliverables
- Resolved Binding Exposure Index (no bare science-review-follow-on rows).
- Populated `provenance/SC-RUNOFFPART-001-provenance.md`.
- `artifacts/runoffpart-binding-crosswalk.md` (conservation incl. flagged additions).
- `artifacts/runoffpart-core-size-delta.md` (per-batch + total token/byte reduction).
- per-batch `--strict` lint evidence + closure-loop gate logs (from
  `comparator_suite_runner`) + test-reconciliation record.
- Dual review/disposition/verification for promotions + kernel-profile checklist.

## Hard stops
- A row needs external authority not in the repo (e.g. open `GAP-RUNOFFPART-*`) →
  narrower science-HOLD; continue other batches. Do not force a binding call.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.
- `comparator_suite_runner` unavailable → record command-level evidence, then run
  the loop locally as the documented fallback.
