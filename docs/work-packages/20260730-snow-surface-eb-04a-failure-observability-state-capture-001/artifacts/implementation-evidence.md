# Implementation Evidence

Ran:

- `Wb11HydrologyKernelGuardError::SnowStage3Conductivity` preserves the exact
  `MeteorologyError`, rejected layer, complete thermal control-volume layer
  vector, projected temperature, and atmospheric pressure.
- `Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch` preserves the
  aggregate value/expectation, prior SWE/depth, and complete prior-layer
  vector.
- The direct research snow trace now publishes daily shortwave energy, signed
  vapor mass, and the separately accumulated mass-times-latent-heat energy,
  plus 24 hourly shortwave, longwave, signed-mass, effective-latent-heat, and
  latent-flux operands.
- Review exposed a units error in diagnostic aggregation: an epsilon expressed
  for water depth suppressed hourly latent heat for tiny but nonzero
  `kg m^-2` vapor exchanges. The condition now tests exact nonzero signed mass.
  It changes diagnostic observability only; the exchange, energy, and snow
  state were already computed and remain unchanged.
- The existing impossible-cold-content regression now calls the captured
  conductivity payload's typed `replay()` and proves it returns the identical
  `MeteorologyError`. The layer-aggregate unit test applies the same
  mass-threshold filter as production and reconstructs the captured value,
  expected prior scalar, and prior SWE.
- `tools/run_diagnostics.py` targets only the 24 frozen EB-04 failures, records
  source/executable/fixture/selector/day identity, independently audits the
  published ledgers, and generates deterministic plots and sidecars.

No process equation, constant, threshold, tolerance, selector, default,
forcing, fixture, observation, parser, or user schema changed.
