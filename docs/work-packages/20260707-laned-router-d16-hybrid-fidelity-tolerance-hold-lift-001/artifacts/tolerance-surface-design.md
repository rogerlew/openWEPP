# Tolerance Surface Design

Status: EXECUTED-NON-BINDING. Evidence mode: Static.

This package will not ratify numeric thresholds from H2637 alone. It records
the surfaces that must be compared before any future threshold decision.

## Non-Binding Surface Set

Any future active-path hybrid default promotion must compare at least:

- Active closure residuals: supply reconstruction, router cascade, seam
  cross-ledger, and day identity maxima.
- Routing magnitude surfaces: `total_source_m3`, `total_routed_outlet_m3`,
  `total_end_window_storage_m3`, `total_clamp_m3`, `total_tail_fold_m3`, and
  `total_latqcc_outlet_m3`.
- Publication files: HBP, pass parquet, loss JSON, and WAT parquet where the
  runfile publishes WAT.
- Pass sediment surfaces: `tdet`, `sedcon_1..5`, changed-row counts,
  aggregate sums, and material row-level sign/zero transitions.
- Hydrograph/timing surfaces where available: routed outlet shape,
  hour-24 tail fold, peak/timing, and per-day/aggregate outlet movement.
- Runtime surfaces: endpoint time, `solver_steps`, `solver_steps_implicit`,
  `implicit_equilibrium_map_evaluations`, `implicit_branch_evaluations`, and
  active closure counters.

## Required Cohort Shape

The minimum useful cohort remains:

- H2637-class wet steep multi-OFE forest.
- Dry or mostly dry hillslope.
- Low-runoff hillslope.
- High-runoff hillslope.
- Steep routed hillslope.
- Multi-event hillslope.
- At least one non-forest/agricultural case if default-promotion language would
  cover it.

Each member must run both active plain and active explicit hybrid with the same
source-authorized active Lane-D inputs. Missing route operands are a hard
cohort-construction failure, not a reason to insert surrogate coefficients.

## Threshold Posture

No numeric tolerance threshold is ratified here. The observed D16 H2637 deltas
(`-0.4396 %` routed outlet and `-6.474 %` pass sediment sums) remain
diagnostic until a cohort-backed contract amendment defines acceptance.
