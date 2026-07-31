# Failure Diagnosis

Status: `bounded / exact sub-cause unresolved`

Evidence mode: `Static + Ran`

## Ran

- B completed 12/12, L 10/12, S 2/12, and LS 0/12.
- Candidate operands were nonzero before rejection.
- Retained pre-failure rows passed daily mass, producer-carried Stage 3
  cold-content closure, producer-carried latent/mass residual, and
  physical-temperature checks. Full independent surface-component and
  latent/mass reconstructions are not available because the needed operands
  were not retained.
- Failures span early and late chronology and occur with material SWE,
  wrapper-reported densities from about `124` to `522 kg m^-3`, and both cold
  and zero-degree prior-day layer states.
- Twenty-two failures are
  `snow.stage3_effective_snow_conductivity_w_m_k` wrapper errors. The other two
  are `prior_layers.thickness_m` reconciliation failures:
  `harvard_open/S` on day 2643 and `marcell_open/LS` on day 3371.

## Static

`stage3_control_volume_state` calls
`snow_effective_thermal_conductivity_snobal(density, temperature, pressure)`.
Its `map_err` converts every `MeteorologyError` from density, temperature,
pressure, saturation-pressure, or computed-conductivity validation into the
same hydrology error and reports `layer.density_kg_m3` as the observed value.
The resulting message therefore does not prove that a `522 W m^-1 K^-1`
conductivity was computed; `522` is the layer density passed to the helper.

## Disposition

The exact lower-level cause of the 22 wrapped conductivity-path failures is
not recoverable from retained EB-04 output. The two thickness reconciliation
failures remain separately classified and are not attributed to that wrapper.
Changing the wrapper, numeric policy, or process state and rerunning would be a
post-result implementation change, not execution of the frozen round. EB-04
therefore retains all typed rejections, invokes the stop-loss, and remains
HOLD/nonpromotion. A future defect package may preserve the underlying typed
error and reproduce a minimal failing state, but it does not authorize another
calibration/factorial round.
