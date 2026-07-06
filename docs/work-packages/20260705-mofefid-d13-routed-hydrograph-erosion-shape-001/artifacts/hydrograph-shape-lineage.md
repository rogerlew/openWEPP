# Hydrograph-Shape Lineage

Status: **COMPLETE** (Static, pre-implementation).

Active-routed-water mode needs a water-shape lineage distinct from DC01 source
weights. D13 records the consumer-side lineage; the production routed producer
flip remains follow-on activation work.

| Surface | Units | Basis | Source authority | Consumer | Notes |
|---|---|---|---|---|---|
| DC01 source weights | fraction | lane-local source volume | `SC-RUNOFFPART-001#INV-RUNOFFPART-031`, `SC-OFEROUTE-001#INV-OFEROUTE-012` D12 limb | default/off erosion substrate | Valid while routing does not own water; cannot carry active-routed-water closure. |
| Lane D routed hydrograph weights | fraction | lane/OFE routed outlet water on daily runoff-volume basis | `SC-OFEROUTE-001#INV-OFEROUTE-008`, `SC-SED-001#REF-SED-LANED-ROUTED-HYDROGRAPH` | active-mode erosion substrate | D13 activation-candidate input; missing/malformed/non-closing surfaces fail closed. |
| `V_h` | m3 | HBP EVENT row runoff-volume basis | `SC-SED-001#INV-SED-014` | HBP EVENT / watershed reader | `V_h = runvol * w_h`; `sum_h V_h = runvol`. |
| `S_h` | kg | Wave-1 hourly exported sediment mass | `SC-SED-001#INV-SED-014`, `SC-SED-001#INV-SED-016` | HBP EVENT / inter-OFE erosion handoff | `sum_h S_h = exported sediment mass`; chain EVENT uses exit lane surface with chain `tdet/tdep`. |
