# PL13 Growth Kernel Algorithm

Status: `complete`
Evidence mode: `Static`

## Runtime Algorithm Summary

Production growth-phase dispatch now executes this sequence:

1. Resolve active PL slot/crop selection for runtime `day/year`.
2. Validate growth ordering flags and runtime day domain (`1..=366`).
3. Validate required growth state surface symbols:
   `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`.
4. Enforce growth state domains:
   - `sumgdd`, `vdmt`, `lai`, `rtmass`, `rtd >= 0`
   - `cancov in [0, 0.999]`
   - `hia in [0, 1]`
5. Annual/fallow branch (`imngmt in {1,3}`):
   - validate `jdharv`, `jdplt`, `rw`, `resmgt`
   - deterministic active action selection:
     - `planting_reset` when `day == jdplt`
     - `harvest_reset` when `day == jdharv`
     - `senescence_reset` when outside annual closed growth window
     - `none` otherwise
6. Perennial branch (`imngmt = 2`):
   - validate `jdharv`, `jdplt`, `jdstop`, `rw`, `mgtopt`
   - deterministic active action selection:
     - `planting_reset` when `day == jdplt`
     - `stop_reset` when `jdstop > 0 && day == jdstop`
     - `none` otherwise
7. For reset-class actions, emit explicit post-transition zero-state payload for:
   `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`.
8. Attach typed payload to `HillslopeGrowthKernelContext` as
   `HillslopeGrowthTransitionPayload`.

## Guard/Error Map

- `HS-GROWTH-E-001`: missing required growth symbol
- `HS-GROWTH-E-002`: non-finite growth symbol
- `HS-GROWTH-E-003`: invalid ordering flag value
- `HS-GROWTH-E-004`: unsupported management class
- `HS-GROWTH-E-005`: required integral symbol is non-integral
- `HS-GROWTH-E-006`: growth symbol value outside allowed range
- `HS-GROWTH-E-007`: invalid transition payload state/domain

## Typed Payload Surfaces

Static:

- `openwepp-kernel-contract` adds:
  - `HillslopeGrowthStateSurface`
  - `HillslopeAnnualGrowthControl`
  - `HillslopePerennialGrowthControl`
  - `HillslopeGrowthTransitionControl`
  - `HillslopeGrowthTransitionPayload`
- `HillslopeGrowthKernelContext` now carries optional
  `transition_payload` via `.with_transition_payload(...)`.
