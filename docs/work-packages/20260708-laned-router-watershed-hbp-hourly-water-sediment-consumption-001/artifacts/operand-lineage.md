# Operand Lineage

Status: `QUEUED`
Evidence mode: `Static scaffold placeholder`

The executing agent must fill this table before production edits.

| Operand | Units | Shape | Source authority | Producer | Consumer | Rejected aliases |
| --- | --- | --- | --- | --- | --- | --- |
| `event.hourly_runoff_volume_m3` | m3/hour slot | 24 slots | `SC-INFILE-HBP-001`, `SC-OFEROUTE-001`, `SC-ROUTE-001` if amended | active Lane D HBP producer | watershed route water limb | daily `runvol_m3`, peak-only triangular fallback, DC01 daily-lump shape |
| `event.hourly_sediment_mass_kg` | kg/hour slot | 24 slots | `SC-INFILE-HBP-001`, `SC-SED-001`, `SC-ROUTE-001` if amended | active Lane D HBP producer | watershed sediment time base | `sediment_concentration * runvol` reconstruction, daily `tdet - tdep` only, zero/synthetic fills |

Acceptance must include independent reconstruction of hourly sums and a
distribution-sensitivity proof. Exact self-consistency of producer arrays is
supporting evidence only.
