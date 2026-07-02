# MOFEFID-A01 — Proactive MOFE Defect-Review Sweep

Status: **EXECUTED — REVIEW-READY** (2026-07-01). Six surfaces swept; six findings (headline: F-A2 runon re-infiltration source-intent divergence → Lane D contract stage); no conservation defect. See `artifacts/findings.md`.
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane A.
Owner: Claude Code (operator-directed role break; review is Claude's native
lane). Reviewer at close: Codex (operator dispatches).

## Objective

Adversarial find→verify review of the MOFE-specific surface for defects
nobody has reported. This is a review package: **no production code or test
edits**; confirmed defects convert to Defect-Closure ExecPlans (ADR-0018)
executed under their own packages. A verified clean sweep is a valid
outcome and is recorded as one.

## Scope (campaign §3)

| # | Surface | Anchors |
|---|---|---|
| S1 | Inter-OFE `TransferInput`/`TransferOutput` lineage, area-scaling provenance | `INV-RUNOFFPART-028/029` |
| S2 | 24-slot hourly carry arrays (surface + lateral): day-boundary and reset semantics | `INV-RUNOFFPART-028` |
| S3 | Publication geometry per OFE: `Q`/`QOFE`/`runvol`/peak operands, effective lengths | `01_publication.rs` runoff operands |
| S4 | Per-OFE closure reconstructability from exported WAT/PASS by an external reader | wepp-forest audit formula as the shape reference (flag, not authority) |
| S5 | Winter column on MOFE lanes: frost/snow carry vs transfer interactions | FARPOINT01 `watbtm` precedent |
| S6 | Single-OFE specialization: zero-upstream lanes bit-stable against MOFE machinery | `INV-RUNOFFPART-029` single-OFE clause |

Out of scope: sediment coupling (`INV-RUNOFFPART-030` hold), Lane D routing
physics, magnitude judgment (Lane C).

## Method

Find: per-surface source read with adversarial prompts (what state leaks
across the day boundary? which operand pairs use mismatched area/length
duals? what does commit_day drop?). Verify: each candidate finding is
confirmed or refuted with file:line evidence and, where a cheap runtime
probe decides it, a **Ran** check (H2637 19-OFE fixture; env-gated traces;
targeted existing tests). Every claim carries its evidence class.

## Acceptance gates

1. All six surfaces swept; per-surface notes in `artifacts/sweep-notes.md`.
2. Findings table in `artifacts/findings.md`: every finding
   confirmed/refuted with evidence class and disposition
   (`defect → ExecPlan` / `refuted` / `contract-decision → Lane B or ADR` /
   `hardening-candidate`).
3. No production source modified by this package.
4. Codex review + disposition artifacts at close.

## Artifacts

- `artifacts/sweep-notes.md` — per-surface working notes (S1..S6)
- `artifacts/findings.md` — the verdict table
- `artifacts/review-codex.md`, `artifacts/review-disposition.md` — at close
