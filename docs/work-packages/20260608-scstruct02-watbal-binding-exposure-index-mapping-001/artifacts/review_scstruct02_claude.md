# SCSTRUCT02 Execution Review — Claude Code (reviewer)

Reviewer: Claude Code
Subject: SCSTRUCT02 executed to the science-review HOLD boundary
Evidence mode: **Ran + Static** — read the `SC-WATBAL-001` diff, the lint diff,
the provenance sidecar, and the package status; **ran** the lint
(`python3 tools/check_sc_binding_exposure.py SC-WATBAL-001.md`).

## Conservation verification (passed)

- `git diff --stat` on `SC-WATBAL-001.md`: **69 insertions / 69 deletions** — only
  the `Review gate` column of 69 Binding Exposure Index rows changed to
  `science-review-follow-on`. A filter for changed lines outside the index
  returned empty: **no `INV-*`/`OBL-*` row changed, no narrative relocated.**
- File still 2590 lines (unchanged since SCSTRUCT01). Provenance sidecar is a
  16-line stub; no narrative moved into it.
- No production kernel/runtime files modified.

Binding semantics preserved. The HOLD is legitimate and appropriately bounded:
routing undecidable rows to science review rather than forcing a call is exactly
the protected-boundary behavior the package mandates.

## Findings

### S02-F1 — MED/HIGH (governance) — lint semantics changed without amending the spec
- Artifacts: `tools/check_sc_binding_exposure.py:10,68,73-78` (new `ROUTED_GATES`
  exemption); `docs/specifications/science-contract-spec.md` (Lint Contract,
  **unchanged**).
- Issue: the lint no longer fails an `active`/`unpromoted-binding` entry with no
  canonical IDs, nor an `undecidable` entry, when its gate is
  `science-review-follow-on`. But the spec's normative Lint Contract still states
  the lint **must** fail in those cases, with no exemption. Code now contradicts
  its own normative spec.
- Why it matters: the framework's value is that the spec defines "green." A silent
  code-only change to what green means is exactly the drift the framework exists
  to prevent — applied to the framework itself. The exemption is also a
  framework-policy decision, made inside a consolidation package whose own scope
  note said "lint code fixes belong to the framework, not this package."
- Proposed disposition: **amend** — ratify the exemption into
  `science-contract-spec.md`'s Lint Contract (or revert the code). The exemption
  is *reasonable* in principle; it just must be normative, not implicit. Operator/
  framework decision, not a unilateral edit.

### S02-F2 — MED — routed-PASS is indistinguishable from clean-PASS at the gate
- Artifact: `tools/check_sc_binding_exposure.py:84-91`.
- Issue: a contract with 69/75 rows merely routed (not resolved) exits **0
  (PASS)**, same exit code as a fully-consolidated contract. Ran evidence:
  `PASS … 75 binding exposure row(s), 69 science-review routed row(s)`, exit 0.
  The message discloses the routed count, but exit-code-based gates cannot tell
  "consolidated" from "deferred."
- Why it matters: "green" loses its forcing function; a contract can sit
  indefinitely green with most obligations parked in a science-review limbo.
- Proposed disposition: **amend** — define a distinct routed-PASS state (separate
  exit code, or a `PASS-DEFERRED` verdict, or a bounded/tracked routed-row budget)
  in the spec so plain green continues to mean *actually consolidated*.

### S02-F3 — INFO (scope truth) — no consolidation or token reduction was achieved
- Issue: SCSTRUCT02 as executed produced **zero** narrative relocation and **zero**
  token reduction. All 69 non-mapped rows (including ones Phase 0 and the package
  flagged as *likely historical* — the `(Historical)`-titled ProfileFC family and
  the ADR-0017-retired snow-comparator arc) were routed to science review rather
  than adjudicated. The package's objective (realize the WATBAL win) is fully
  deferred to follow-on `SCSTRUCT02-WATBAL-BEI-SCIENCE-REVIEW`.
- Assessment: this is **defensible, not a fault.** Confidently calling even the
  "likely historical" rows requires domain rigor; doing so on title/heading
  signal would reintroduce the token-scrape overconfidence the framework guards
  against. Whether any subset is safely adjudicable without full science review is
  itself a science-review question. Recorded as scope truth, not a defect: the
  effort is now gated on inherently-manual domain adjudication an agent cannot
  shortcut.

### Note — dual review/verification not run
- The package's review/disposition/verification slots are not-run placeholders.
  Consistent with a HOLD (the package cannot close), so acceptance criterion #6 is
  legitimately unmet, not evaded.

## Recommendation

- **Accept the HOLD** — conservation held, the boundary was respected, and the
  routing is the honest call.
- **Fix S02-F1 and S02-F2 at the framework level** before SCSTRUCT02 is treated as
  closed: ratify (or revert) the lint exemption in `science-contract-spec.md`, and
  make routed-PASS a distinct, tracked state. These keep the framework's own
  "green" trustworthy.
- **Decide the strategic fork on the WATBAL win** (operator): invest in the
  science-review adjudication of the 69 rows (the only path to the token
  reduction), or bank the framework win and accept that WATBAL stays large while
  the framework prevents *future* bloat. The infrastructure value is already
  realized; the contract slimming is genuine, manual, domain-science work.

Findings surface issues + evidence; disposition and architecture remain Codex's.
