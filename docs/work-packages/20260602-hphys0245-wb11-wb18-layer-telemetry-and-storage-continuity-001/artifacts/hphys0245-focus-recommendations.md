# HPHYS0245 Focus Recommendations

Status: completed
Evidence mode: Static + Ran

## Recommendation
- Open the next package against WB18 aggregate water accounting first.
- Treat WB19 day-1 lateral transfer as the second focus surface after WB18
  aggregate continuity is corrected or explicitly justified.
- Do not tune or clamp `D`/`Pe` as the first response; the telemetry shows the
  first residual is an aggregate storage handoff problem, not only an excess
  percolation scalar.

## Proposed Next Package
- `20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001`

## Scope for HPHYS0246
- Contract-first: amend canonical SC authority for WB18 aggregate storage
  writeback and the relationship among `wb11_soil_water`, layer theta, dead or
  residual storage, `D`, and `Pe`.
- Add contract-derived tests that preserve the non-theta aggregate storage
  component across WB18 unless the authoritative baseline explicitly removes it.
- Port baseline-authoritative WB18 aggregate writeback behavior; do not invent
  a compensating formula.
- Re-run `H1`, `H7`, and `H39` day-1/day-30 telemetry after the WB18 fix.
- Only then adjudicate whether WB19 lateral day-1 transfer remains excessive.

## Rationale
- WB18 currently publishes `wb11_soil_water = Σtheta_after`.
- HPHYS0245 telemetry shows day-1 seed storage has a `29..40 mm` aggregate
  component outside `Σtheta`.
- The seed-to-WB18 aggregate drop equals WB18 `D` plus that seed gap for all
  three hillslopes.
- WB13 and storage reconciliation are downstream reflections, not the first
  mutation point.

## Hold Posture
- HPHYS0245 closes the diagnostic objective.
- Hillslope water-balance closure remains `HOLD` pending a contract-first WB18
  aggregate storage correction and a follow-up WB19 lateral audit.
