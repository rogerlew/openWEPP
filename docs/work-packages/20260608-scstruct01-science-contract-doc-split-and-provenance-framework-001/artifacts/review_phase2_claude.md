# SCSTRUCT01 Phase 2 Checkpoint Review — Claude Code (reviewer)

Reviewer: Claude Code
Subject: Phase 2 Binding Exposure Index + lint, stopped at HOLD
Evidence mode: **Ran + Static** — read the added Binding Exposure Index diff, the
HOLD checkpoint, the Phase 1 disposition, and the lint source; **ran** the lint
(`python3 tools/check_sc_binding_exposure.py SC-WATBAL-001.md`).

## Verdict: legitimate HOLD, independently verified

The HOLD is correct and well-evidenced. Phase 2 took the safest possible first
step (build the index, run the lint) and failed closed before any irreversible
action.

### Boundary verification (passed)
- `git diff --stat` on `SC-WATBAL-001.md`: **+86 lines, 0 deletions** — only a
  `## Binding Exposure Index` section added. No `INV-*`/`OBL-*` row changed; no
  addendum narrative relocated.
- No production kernel/runtime files modified.
- Procedure-doc path preserved.

### Lint verified executionally
- **Ran:** `FAIL docs/.../SC-WATBAL-001.md: 133 issue(s)`, exit 1. The red state
  is real, not a captured claim. The lint enforces the spec's Binding Exposure
  Lint Contract (active entry needs canonical IDs; unpromoted-binding needs a
  mapping; undecidable blocks; referenced IDs must exist in core).

### F4 internalized
The index marks the HPHYS0308 / EROD12 "all-~94-invariants" rows `undecidable`
("broad scrape is not a precise binding map") and routes WB16 / HPHYS0202
historical rows to `science-review-follow-on` rather than relocating. This is the
exact correction F4 asked for — the token scrape is not trusted as authority.

## What the HOLD reveals (strategic)

The framework worked, but the honest outcome is that **this pass did not reduce
WATBAL — it grew it by 86 lines — and the slimming is now entirely future work.**
Index counts: `maps-to-existing-INV` 6, `undecidable` 8, **`unpromoted-binding`
61**.

The 61 is the real finding: most WATBAL addenda carry binding language that was
never promoted to a numbered invariant, so the bloat is load-bearing and cannot
be deleted. The framework converted "too big" into an enumerated, lint-gated
worklist and made the hidden contract-authoring debt visible and conserved. The
token win requires adjudicating those 61 addenda one by one — genuine science /
contract-authoring effort, routed to follow-on package
`scstruct02` (`SCSTRUCT01-WATBAL-BEI-MAPPING`).

## Findings

### P2-F1 — LOW — lint mis-flags rows whose Source title contains inline backticks
- Artifact: `tools/check_sc_binding_exposure.py:8` (`ROW_RE`).
- Issue: the Source-cell pattern allows at most two backtick groups
  (`` `lines X-Y` `` + title). Titles containing inline backticks — `HPHYS0218
  WB19 `drfc`-…`, `HPHYS0222 WB19 `solwpv` …` — produce a third group, so the
  regex fails and the row is reported `malformed index row` instead of being
  classified.
- Impact: cosmetic for the HOLD (both rows are `unpromoted-binding` regardless),
  but a real robustness gap — a malformed-row escape could mask a genuinely
  missing mapping. Proposed disposition: **amend** — parse the Source cell
  tolerant of arbitrary inline backticks (split on unescaped `|`), delegated to
  Codex.

### P2-F2 — INFO — Phase 1 sign-off recording
- The Phase 2 disposition required recorded operator acceptance before Phase 2.
  Acceptance did occur (operator instructed Codex to proceed) but was not
  captured at the time; now recorded in
  `artifacts/phase1-framework-operator-signoff.md`. Only additive/reversible work
  (index + lint) preceded the recording, so no irreversible action bypassed the
  gate. Resolved.

### P2-F3 — confirmed — Phase 1 findings F1–F5 accepted and folded
- `artifacts/disposition_phase1_review.md` accepts all five and links concrete
  actions (Phase 0 recast, front-matter migration scoping, `amended` retained,
  ADR-0003 restored). Verified consistent with the doc diffs.

## Recommendation

- **Ratify the framework** — it is reusable infrastructure that gates all future
  contract work against re-accretion, independent of when WATBAL is slimmed.
- **WATBAL consolidation → `scstruct02`** (61-addendum BEI mapping), scheduled or
  parked as the operator chooses. The token reduction is real but is a sizable
  contract-authoring effort, not a mechanical pass.
- **Fix P2-F1** (lint parser) via Codex.

Findings surface issues + evidence; disposition and architecture remain Codex's.
