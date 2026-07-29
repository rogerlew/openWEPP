# Terminal Scientific Re-review — Agent B

Evidence class: `Ran: independent read-only exact-head checks + Static:
science, status, incident, and claim-boundary review`

Disposition: `PASS`

## Finding closure

| Finding | State | Evidence |
| --- | --- | --- |
| TB-01 | `CORRECTED` | Both the 116,800-row primary grid and 36,500-row ridge now carry surface, interrill, and rill state exactly across all days and years. The expanded producer trace retains seeds, forcing, temperature/water/environment/decay factors, surface/ground/root/depth states, downstream factors, and partition states. All 16 independent reconstructions pass with no divergent operand; the maximum difference is `8.881784197001252e-16`, inside `1e-12`. |
| TB-02 | `CORRECTED` | All 28 Harvard output rows now retain `litter_row_count`, `stock_replicate_count`, and `stock_use_not`. Independent rebinding to the admitted plot-matching source confirms exact key coverage, custody fields, source values, periods, and stock year. The diagnostic remains descriptive, noncontemporaneous, and pooled. |
| TB-03 | `CORRECTED` | Eight central finite-difference sensitivities are retained from already frozen grid members: source sensitivity is centered at `S020`, and rate sensitivity is centered at `K050`. Independent arithmetic reproduces every value and unit. The five-pair ridge covariance is `0.16261898422070004 kg m^-2 yr^-2`; correlation is `0.9993772023530584`. Saturation evidence correctly distinguishes the interior limiting temperature factor from saturated, nonlimiting water factors. The readiness matrix now contains local-sensitivity, covariance/correlation, and saturation rows. |

## Final evidence

- The current terminal validator passes: 116,800 primary rows, 16
  reconstructions, one recovered daily-series truth, five terminal-ridge
  members, 16 typed boundary/failure cases, and 28 Harvard rows.
- Primary and ridge ground-state carry are exact. Surface, flat-partition, and
  total-partition publication differences are zero under the frozen zero
  partition offsets.
- `S020-K050` remains the unique zero-SSE daily truth. All five analytic
  ridge members remain within the frozen terminal tolerance.
- Incident 001 remains a fail-closed pre-publication event with its
  independently approved serialization amendment. Incident 002 changes only
  the derived sensitivity method and covariance units using already frozen
  members; it adds no axis, fit, preferred vector, or scientific boundary.
- All three ADR-0042 status columns use allowed enums. The direct-runtime
  surface-decay readiness result does not propagate to native leaf-source or
  source-composition stages.
- Harvard carbon remains separate from modeled dry mass and organic-horizon
  stock remains separate from modeled surface residue. No decomposition value
  is fitted.
- The missing-physics recommendation remains appropriately contract-first:
  it determines configuration need and authority before implementation and
  assigns no needle/fine-woody magnitude, allocation, recurrence, carbon
  conversion, or decay adjustment.

## Conclusion

TB-01 through TB-03 are corrected. CAL-05 terminal scientific review B is
`PASS` within the declared synthetic-readiness and descriptive-observation
boundaries. This pass accepts
`direct_runtime_surface_decay=CALIBRATION_READY_DATA_LIMITED` and the retained
source/rate partial-identifiability result; it does not lift the empirical
source-composition or decomposition-fit authority blocks.
