# Parser Implementation Prioritization Rubric

Date: 2026-05-21
Evidence mode: `Static`

## Purpose

Define a repeatable, correctness-first scoring model to prioritize parser
implementation order across all `active` `SC-INFILE-*` surfaces.

Evidence anchors:
- `[DIRECT]` `docs/specifications/wepp-input-files/input-surface-registry.md`
- `[DIRECT]` `docs/specifications/science-contracts/contracts/SC-INFILE-*.md`
- `[DIRECT]` `docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md`
- `[DIRECT]` `docs/planning/wepp-input-file-parser-survey.md`

## Scoring Dimensions

Each surface receives 0..5 scores on six dimensions.

1. `critical_path_value` (weight `0.30`)
- How necessary the surface is for first runnable high-confidence scope
  (single OFE + daily water-balance).

2. `dependency_centrality` (weight `0.25`)
- How many downstream parser surfaces and runtime entry paths are blocked if
  this surface is missing.

3. `tier_a_leverage` (weight `0.20`)
- Degree to which the parser directly improves Tier-A comparator readiness per
  ARCH-01 comparator policy.

4. `observability_leverage` (weight `0.10`)
- Degree to which parser correctness here improves developer stimulation,
  traces, and targeted diagnostics.

5. `compatibility_burden` (penalty `-0.10`)
- Legacy variant burden (datver ambiguity, silent rewrites, branch complexity,
  sidecar uncertainty). Higher means harder.

6. `hold_gap_risk` (penalty `-0.05`)
- Governance uncertainty from contract `*-GAP-*` HOLD registers and unresolved
  authority notes. Higher means more risk.

## Formula

`weighted_score`
`= 0.30*critical_path_value`
`+ 0.25*dependency_centrality`
`+ 0.20*tier_a_leverage`
`+ 0.10*observability_leverage`
`- 0.10*compatibility_burden`
`- 0.05*hold_gap_risk`

## Wave Assignment Rules

Wave assignment is score-informed but dependency-constrained.

1. Wave 1 (Tier-A MVP)
- Must include the minimum parser set for single OFE + daily water-balance
  startup signal.

2. Wave 2 (Tier-A Extension)
- Hillslope-adjacent sidecars and policy sentinels that extend coverage without
  requiring watershed orchestration.

3. Wave 3 (Watershed Core)
- Watershed structural/topology surfaces required before watershed sidecars.

4. Wave 4 (Watershed Sidecar Extension)
- Channel and watershed sidecars dependent on core watershed parsers.

Wave precedence is authoritative for execution:
- complete Wave `N` before starting Wave `N+1`.
- ranking is used for `intra-wave` ordering only.

## Tie-Breakers

When scores are similar, prioritize by:

1. Higher `critical_path_value`
2. Lower `compatibility_burden`
3. Lower `hold_gap_risk`
4. Earlier dependency unlock (more blocked surfaces released)

## Traceability Requirements

The scoring matrix must include:

1. `execution_rank` (global order, constrained by wave precedence)
2. `intra_wave_rank`
3. `gap_ids` (explicit `*-GAP-*` IDs used to justify `hold_gap_risk`)

`hold_gap_risk` scores without explicit `gap_ids` are non-auditable and
non-compliant with this rubric.

## Correctness Gates Before Promotion Between Waves

1. Parser contract invariants and guards mapped for each implemented surface.
2. Explicit typed error behavior for strict vs compatibility mode divergence.
3. Tier-A comparator readiness retained as primary promotion signal.
4. Any unresolved high-severity parser correctness issue keeps wave status
   `HOLD`.

## Notes

- This rubric prioritizes execution order, not final product importance.
- Low-ranked surfaces are not optional; they are sequenced later because they
  are more uncertain, less central to Tier-A startup, or both.
