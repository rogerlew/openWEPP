# Source-Shape Lineage

Status: **COMPLETE**.

Static:

| Surface | Units | Lane/OFE basis | Timing basis | Source authority | Consumer | Notes |
|---|---|---|---|---|---|---|
| `runvol/area` | m | lane-local | daily total | DC01 publication / `DirectPublicationDayRow` | Lane D shadow | Daily source volume remains unchanged by D12. |
| `wb14_hourly_excess_m[h]` | m/hour slot | lane-local | 24 hourly bins | `SC-RUNOFFPART-001` WB14 profile | `dc01_surface_runoff_hourly_weights` | Existing rainfall-excess limb. |
| `ui_SCrunf` lineage / `hourly_saturation_carry_m[h]` | m/hour slot | lane-local | 24 hourly bins | `SC-SUBHYD-001#INV-SUBHYD-023` via DC01 | `dc01_surface_runoff_hourly_weights` | Existing saturation/exfiltration carry limb. |
| `snow.hourly_routed_melt_m[h]` | m/hour slot | lane-local | 24 hourly bins | `SC-RUNOFFPART-001#INV-RUNOFFPART-022` | `dc01_surface_runoff_hourly_weights`, Lane D shadow | D12 limb; sum closes to `snow.routed_melt_m` within `1.0e-12 m`. |

Ran:

- H2637 native-shadow evidence reconstructs source supply with
  `max_supply_reconstruction_rel=5.434281268840262e-16`.
- `days_uniform_shape_with_routed_melt=0`; the remaining uniform class has no
  source-shape limb, including no routed melt.
