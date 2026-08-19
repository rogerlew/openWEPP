# Live-owner HOLD intake

Evidence class: `Static`.

Intake commit and `origin/main` were both
`09bbd5d844456c3c11b3bd9c88909dbe0d5f8ae3`; the tree was clean and
`git diff --check` passed before this investigation.

## Accepted defect

The scheduler now supplies an opaque climate/GSI capability, but the V10
consumer still accepts a caller-built `DirectV10ShadowDayInput` (an alias for
`DirectV9ShadowDayInput`). Its interval template remains the physics owner for
root-zone matric potential, soil-to-root path length, layer gravitational head,
root accessibility, ground optics, upward longwave, runon, and WB14 values.

## Exact authority contradiction

Repository and contract tracing found no admitted owner for two inputs required
by `SoilLayerForcing`:

- `matric_potential_mm`: `DirectSubsurfaceLayerState` contains water,
  conductivity, depth, porosity, capacity, and frozen state, but no matric
  potential or admitted per-layer retention operator. WB14
  `matric_potential_m` is the nonnegative Green-Ampt infiltration parameter and
  is not the negative root-zone water potential consumed by vegetation.
- `root_path_length_mm` and `gravity_root_mm`: `RootLayer` owns layer identity,
  root fractions, and `lateral_root_length_m`. `StratumConfiguration` owns plant
  height. Neither owns the contract-required explicit soil-to-root path length
  or per-layer gravitational head. The V3 contract says these operands remain
  explicit; it does not define them as layer depth, lateral root length, height,
  or any combination of those values.

Typed unsupported cannot preserve the declared positive rooted snow-free
domain: every nontrivial rooted layer requires these operands. Consequently no
closure-eligible live-owner interval can be constructed without inventing
physics or retaining caller custody.

## Search record

Searched definitions and consumers:

- `openwepp-vegetation::{RootLayer, StratumConfiguration, SoilLayerForcing}`;
- `openwepp-hillslope-orchestrator::{DirectSubsurfaceLayerState,
  DirectRunFrame, RealHydrologyLaneLayerMap}`;
- LSE bare-mineral `top_layer_saturated_matric_potential_mm` and
  `top_layer_clapp_hornberger_b` plus the admitted surface operator;
- WB14 `DirectOfeWb14Parameters::matric_potential_m`;
- `SC-VEGETATION-001`, `SC-SUBHYD-001`, `SC-SURFACELIQUID-001`,
  `SC-SOIL-001`, `SC-EVAP-001`, and repository-wide symbol search;
- the pinned baseline was not used to promote an unreviewed equation because
  the current contract explicitly requires these as supplied layer operands.

## HOLD legitimacy

This is the prompt's named legitimate future HOLD: exact required fields lack
an authority-preserving source after repository and contract tracing. The first
narrow lift action is a contract amendment that admits a per-layer root-zone
hydraulic owner containing (or canonically deriving) root-zone matric
potential, soil-to-root path length, and gravitational head, with units,
configuration digest, OFE/layer mapping, domain, and test vectors. It must
explicitly distinguish root water potential from WB14 wetting-front suction.

No production implementation, restart wire, selector, default, output, or
activation was changed.
