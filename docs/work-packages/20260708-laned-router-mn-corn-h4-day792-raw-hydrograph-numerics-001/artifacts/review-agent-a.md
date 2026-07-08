# Review Agent A

Evidence mode: Static + Ran (`git diff --check`, `cargo fmt --check`).

Reviewer: `rust_code_reviewer`.

## Findings

### A-M1 Boundary/bin attribution not independently ruled out

The package kept boundary/outlet bin attribution correction in scope, but the
first hold evidence only ruled out sign/limiter defects. The analyzer's
top-bin window summaries used full overlapping steps, not clipped step mass
overlaps, so they could not prove the published 900-second outlet bins were
not the mechanism.

Severity: Medium.

Disposition: Accepted.

Fix: `analyze_raw_hydrograph_numerics.py` now reconstructs each published
outlet bin by clipping step `outflow_m3` over `[t_start_s, t_end_s]` to the
bin span. The hold gate now rejects CFL classification if reconstruction Linf
exceeds `1e-12 m3`. Rerun evidence:

- `dx1p25` clipped reconstruction Linf:
  `3.4694469519536142e-18 m3`.
- `dx0p625` clipped reconstruction Linf:
  `1.5265566588595902e-16 m3`.

### A-M2 Mesh/spatial CFL evidence missing

The trace showed a temporal transition but did not record actual mesh cell
count, `dx`, or the max-Courant controlling cell/x-position.

Severity: Medium.

Disposition: Accepted.

Fix: the row-scoped trace now serializes:

- `mesh_cell_count`
- `mesh_dx_m`
- `max_courant_cell_index`
- `max_courant_cell_center_x_m`

Rerun evidence:

- `dx1p25`: 65 cells, `dx=1.2492307692307694 m`,
  max-Courant cell `64` at `80.575384615384621 m`.
- `dx0p625`: 130 cells, `dx=0.62461538461538468 m`,
  max-Courant cell `8` at `5.3092307692307701 m`.

## Residual Risk

The reviewer found no Rust default/off-path blocker: step trace is gated through
runner/env validation and orchestrator config; default `route_single_ofe` and
`run_with_options` keep trace disabled. Solver per-unit-width to lane-volume
conversion looked consistent.

## Verdict

Post-disposition verdict: Accepted for executed hold, subject to final gates.
