# Conservation / Output Lineage

Status: **COMPLETE**.

Static:

- Daily volume authority remains the existing lane-local `runvol/area` source
  basis; D12 changes only the time distribution used by shared source-shape
  consumers.
- Producer identity: `sum_h snow.hourly_routed_melt_m[h] ==
  snow.routed_melt_m` within `1.0e-12 m`.
- Consumer identity: `dc01_surface_hourly_weights[h]` normalizes
  `wb14_hourly_excess_m[h] + hourly_saturation_carry_m[h] +
  hourly_routed_melt_m[h]`.
- Dry runoff surface: no-runoff days return all-zero weights after validating
  finite/non-negative limbs.
- Invalid surfaces: non-finite or negative source limbs fail closed; downstream
  R4G nonclosure fails closed.

Ran:

- H2637 native shadow: protected HBP and pass parquet bytes match shadow-off.
- H2637 source reconstruction: `max_supply_reconstruction_rel =
  5.434281268840262e-16`.
