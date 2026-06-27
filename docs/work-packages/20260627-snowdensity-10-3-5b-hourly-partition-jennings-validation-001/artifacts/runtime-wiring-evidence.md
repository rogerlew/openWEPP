# Runtime Wiring Evidence

Status: complete
Evidence mode: Static/Ran

## Contracted Selector

Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
now defines `SnowPhasePartitionModel` with `LegacyRst` and
`HarderPomeroyHourly`.

Static: `DirectWinterHourlyContext` carries `snow_phase_model`, and
`DirectWinterHourlyForcing` carries selected-model diagnostics:

- `phase_model`
- `rain_fraction`
- `snow_fraction`
- optional `hydrometeor_temperature_c`
- optional `relative_humidity`

Static: the runtime-symbol path accepts only optional numeric symbol
`snow.options.phase_partition_model_harder_pomeroy_hourly`; absent/`0` selects
`LegacyRst`, `1` selects `HarderPomeroyHourly`, and any other value fails closed.

## Direct Snow Consumer

Static: `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
adds package-bound env selector `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` for
diagnostic direct-production runs. Absent/empty/`legacy_rst` selects the default;
`harder_pomeroy_hourly` selects the opt-in path; all other values fail closed.

Static: `DirectProductionSnowFrostAuthority::snow_liquid_partition` passes
`snow_phase_model: self.snow_phase_model` into `direct_winter_hourly_forcing`
before `Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed`.
This is the real direct snow consumer path, not producer-only symbol evidence.

Static: `DirectProductionSnowFrostAuthority::frost_hourly_forcing` is pinned to
`SnowPhasePartitionModel::LegacyRst`; frost hourly forcing is not changed by this
package.

Ran: `snowdensity1035b_direct_snow_consumer_receives_phase_selector` passed in
`cargo test --workspace`.

## Runtime Symbol Diagnostics

Static: the existing `snow.hourly.stmtim.*` projection now also emits:

- `snow.hourly.stmtim.phase_model_####`
- `snow.hourly.stmtim.rain_fraction_####`
- `snow.hourly.stmtim.snow_fraction_####`
- optional `snow.hourly.stmtim.relative_humidity_####`
- optional `snow.hourly.stmtim.hydrometeor_temperature_c_####`

Static: these aliases were registered in the boundary unit catalog and the
symbol-registry audit allowlist so CLI03 multi-OFE registry checks continue to
fail closed on unregistered runtime surfaces.
