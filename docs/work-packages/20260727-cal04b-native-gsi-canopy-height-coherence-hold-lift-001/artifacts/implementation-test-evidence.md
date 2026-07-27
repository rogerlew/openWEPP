# Implementation And Test Evidence

Status: `PASS / TERMINAL GATES PENDING`

Evidence class: `Static + Ran`

## Production correction

- The native projection validates selected-crop `bbb/hmax` before advancing
  phenology, advances a cloned candidate, computes checked
  `Bt=Bs+Bf`/`Hc`, and commits the candidate only after height succeeds.
- The same `DirectGrowthStateSurface` publishes live foliar/interception
  biomass, LAI, cover, and height.
- ET and active/shadow Lane D consume that post-growth surface.
- Active erosion now reads `growth.canopy_height_m`; the prior optional-PMET
  `map_or(0.0, ...)` stale path is removed.
- Frost receives post-growth height as an explicit typed thermal override
  rather than retaining management-seed height.
- Exact consumed-height fields prove the active erosion and frost reads.
- The legacy/fallow helper preserves its exact `+0.0` short circuits before
  exponent arithmetic, including `hmax=-0.0`.

`Bf` remains the foliar/interception handoff. No fallback, clamp, empirical
height fit, calibration-domain edit, population run, or Harvard access was
introduced.

## Contract-derived tests

Ran:

- orchestrator native/legacy height focus: `5 passed`;
- runner source-order and real-production consumer focus: `2 passed`;
- real native production traversal independently reconstructs height and
  proves exact growth, ET, erosion, and frost consumption while retaining snow,
  WB15, residue/litter, and frost-residue proofs;
- full orchestrator package suite at production checkpoint: `409 passed`,
  including three slow tests;
- checker regressions: `3 passed`;
- SC-PLANT binding, SC-ROUTE non-regression, contract/unit governance, Rustfmt,
  focused warnings-denied Clippy, and runner `cargo check`: `PASS`.

The height tests cover `TV-PLANT-GSI-HC-001..005`: deciduous zero-to-positive,
deciduous and structural leaf-off, evergreen floor, exact zero/monotonic/
saturation boundaries, legacy zero parity, typed parameter-domain failures,
checked-sum/product overflow taxonomy, and positive-biomass underflow.

`TV-PLANT-GSI-HC-006` is carried by the frozen 12-case native replay plus the
real-consumer traversal and active/shadow Lane D negative source/runtime
guards. Its terminal replay receipt is recorded in `gate-results.md`.
