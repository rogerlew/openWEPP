# Disposition

Status: complete

Evidence mode: Ran

Ran:

Final disposition: `complete`

production_timing_edit_authorized: `true`

source_line_classification: `OPENWEPP-DEFECTIVE`

carried_rows_closed_for_timing_seam: `57`

HPHYS0320 closes the HPHYS0319 `stmtim-active-interval-divergence-hold`.
Pinned-baseline source lines show that breakpoint `stmstr` is assigned to
`wnttim` and then normalized to a minimum start hour of `1.0` before
`stmtim.for` active-interval evaluation. OpenWEPP SIMIMPL28 omitted that
normalization, leaving `wnttim = 0` and suppressing the active hourly snow
branch for the HPHYS0319 2013 day 11 hour 11 key.

Implemented closure:

- OpenWEPP SIMIMPL28 now applies the baseline-authoritative `wnttim < 1.0` to
  `1.0` normalization before active-interval evaluation.
- Non-finite `wnttim` fails closed as `ClimateRuntimeInputError::NonFiniteField`.
- H1/H7/H39 regenerated traces record `wntdur = 11`, `wnttim = 1`, active
  interval `1`, snow branch `1`, and `hrsnow ~= 0.00074545 m`.
- H1..H39 release-binary runtime passed `39/39`.

No follow-on package is required for this timing seam. Remaining comparator
investigation, if any, must be based on a newly proven source lane rather than
the HPHYS0319 `wnttim = 0` active-interval divergence.
