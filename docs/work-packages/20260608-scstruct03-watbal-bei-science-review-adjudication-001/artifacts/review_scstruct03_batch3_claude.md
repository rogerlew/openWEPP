# SCSTRUCT03 Batch 3 Review — Claude Code (reviewer)

Reviewer: Claude Code
Subject: Batch 3 (WB13/WB16/WB12) — narrower-HOLD, no relocation
Evidence mode: **Ran + Static** — read the `SC-WATBAL-001` diff and batch-3
adjudication; **ran** `--strict` lint.

## Verdict: correct restraint — accept

Batch 3 relocated nothing, and that is the right call. It distinguished
**live closure/publication authority** (WB12/WB16/WB13) from the historical
narrative of batches 1–2.

### Conservation (Ran/Static)
- Diff: **5 ins / 5 del** — BEI note/gate text only. No section removed; invariant
  table untouched.
- `--strict` lint: `PASS-DEFERRED … 56 rows` — deferred unchanged (zero rows
  resolved, expected for a no-relocation batch). Core size unchanged.
- WB12/WB16 rows → narrower science-HOLD with owners
  `SCSTRUCT03-WB12-BEI-PROMOTION` / `SCSTRUCT03-WB16-BEI-PROMOTION`.

### Why this is the right inverse of batch 2
WB13 *has* a preliminary mapping (INV-026/029/064) but was **kept core-resident**
because the section carries active schema/order/guard/test-vector and
lineage-register obligations beyond those IDs. A row that "looks mapped" is not
relocation-eligible unless the mapping is *complete*. WB12/WB16 are genuinely
unpromoted live authority (required surfaces, equations, guard codes, test
vectors) → routed to promotion, not relocated. No obligation was put at risk.

## Finding

### B3-F1 — INFO — yield outlook refined (mixed, not uniformly optimistic)
- Batch 2 suggested many deferred rows might be already-promoted and relocate
  cleanly. Batch 3 shows the counter-class: **genuinely-unpromoted live authority**
  (WB12/WB16/WB13) that must be *promoted into invariants/obligations first* —
  real contract-authoring work that does not net-reduce tokens (content moves from
  prose to invariant rows) and only enables relocation afterward.
- Implication for the end-state: WATBAL will shed genuinely-historical narrative
  (snow arc, ProfileFC, and similar retired material) but **retain** the
  live-authority cohort (WB12/13/14/15/16/19 coupling), possibly reorganized into
  invariants. Total token reduction will plateau short of the full ~2,240-line
  addendum block because much of it is load-bearing. This is the honest economics,
  not a regression.

## Recommendation

**Accept batch 3.** Conservation is trivially intact (nothing moved) and the
live-vs-historical discrimination is exactly the discipline the package requires.
The two promotion follow-ons (WB12/WB16) and the WB13 mapping-completion are real,
appropriately-scoped contract-authoring work — not quick relocations. Batches 4
(WB14/WB15/CLIM/IRRIG coupling) and 5 (WB19) will likely be a mix of both classes.

Findings surface issues + evidence; disposition and domain calls remain with Codex
and the operator.
