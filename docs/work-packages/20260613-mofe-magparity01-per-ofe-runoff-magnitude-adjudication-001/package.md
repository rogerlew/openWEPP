# MOFE-MAGPARITY01 — Per-OFE Runoff Magnitude Adjudication

Status: queued (follow-on from MOFE01 M-F-REDO2 / M-H; operator-directed 2026-06-13)

Package type: characterization/adjudication work package

## Objective

Adjudicate the **per-OFE runoff magnitude divergence** between openWEPP and the
legacy-clean 1–5-OFE reference observed during MOFE01: routing **closure**
(conservation) holds, but per-OFE runoff *magnitudes* differ from legacy by
~10–25%, **scattered both directions** with no systematic bias (Claude M-F-REDO2
review: OFE1 pure-generation means H1 +12%, H6 −13%, H9 −25%, H11 +19%). Decide
whether this is **expected Stage-2 magnitude divergence** (contract-first engine
vs the comparator flag, ADR-0017) or a **characterizable openWEPP defect**.

## Included scope

- Characterize the magnitude divergence per OFE count and per term (Q local
  generation vs routed accumulation; ET; storage), means and distributions,
  across the 36-run ladder — not just maxes.
- Isolate generation vs routing: OFE1 (no upstream) divergence is pure
  generation; downstream divergence mixes generation + routing.
- Cross-reference the FDHP01 magnitude posture and the snow/Stage-2 backlog —
  is the runoff-magnitude difference inherited from upstream magnitude items
  (snow, ET, infiltration) already deferred to Stage-2?
- Verdict: expected-divergence (record posture, no production change) or
  defect-shaped follow-on (contract-first).

## Excluded scope / protected boundaries

- No comparator-match tuning (ADR-0017). The output is an adjudication, not a
  fit-to-legacy.
- No routing-conservation changes — MOFE01 closure stands.
- Snow/ET/infiltration magnitude (Stage-2) remains behind its protected
  boundary unless this characterization promotes a specific slice with
  evidence.

## Acceptance / exit criteria

- A measured per-term, per-OFE-count magnitude-divergence characterization.
- An explicit verdict (expected-divergence vs defect) with evidence, and — if
  defect — a defect-shaped follow-on naming the responsible term.

## Dependencies

- MOFE01 (`20260612-mofe01-inter-ofe-routing-closure-001/`).
- `docs/backlog/20260605-snow-code-deferred-science-review.md` (Stage-2
  magnitude), FDHP01 magnitude posture, ADR-0017.
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/` + legacy outputs.
