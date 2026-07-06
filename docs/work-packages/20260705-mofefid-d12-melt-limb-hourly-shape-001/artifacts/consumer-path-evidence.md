# Consumer-Path Evidence

Status: **COMPLETE**.

Static:

1. Producer: `Wb11HydrologyKernel::finalize_active_snow_coupling` builds
   `SnowCouplingOutcome.hourly_routed_melt`.
2. Public partition:
   `DirectSnowLiquidPartition.hourly_routed_melt_m`.
3. Day input:
   `DirectSnowCouplingInputs.hourly_routed_melt_m` in the runner day builder.
4. Runtime validation/downstream:
   `DirectSnowCouplingInputs.hourly_routed_melt_m` is validated at R4G and
   handed to `DirectSnowCouplingDownstreamOperands.hourly_routed_melt_m`.
5. Shared source-shape consumer:
   `dc01_surface_runoff_hourly_weights(q, wb14, saturation, routed_melt)`.
6. Publication/shadow:
   `DirectPublicationDayRow.dc01_surface_hourly_weights` feeds
   `LanedShadowCollector` through `LanedShadowLaneDayOperands`.
7. Manifest:
   `LanedShadowSummary` emits `days_uniform_shape_with_routed_melt` and
   `days_uniform_shape_without_routed_melt`.

Ran:

- H2637 native shadow proves the real downstream consumer reads the new path:
  `days_uniform_shape_with_routed_melt=0`.
- Existing protected-output identity is preserved by comparing shadow-off and
  shadow-on HBP/parquet bytes in the ignored H2637 evidence test.
