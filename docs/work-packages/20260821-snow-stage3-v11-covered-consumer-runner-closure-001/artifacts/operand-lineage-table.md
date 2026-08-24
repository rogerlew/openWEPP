# Surface-liquid child-slab operand lineage

Evidence mode: Static

| Operand | Unit/basis | Authority | Role |
| --- | --- | --- | --- |
| timed parcel mass | `kg H2O m^-2 basis-OFE-ground` | SC-SURFACELIQUID-001 | authoritative |
| surface store | `kg H2O m^-2 tile-ground` | SC-SURFACELIQUID-001 | authoritative |
| WB14 supply/infiltration/excess | `m OFE-ground` | SC-RUNOFFPART-001 / SC-SURFACELIQUID-001 | authoritative |
| parcel enthalpy | `J m^-2 basis-OFE-ground` | SC-SURFACELIQUID-001 | authoritative |
| production soil credit | `kg H2O m^-2 OFE-ground` | existing unified receiver join | authoritative |
| soil-thermal credit | `J m^-2` at named receiving layer | existing unified receiver join | authoritative |

Acceptance will reconstruct child and parent water and enthalpy from produced
receipts and owner deltas. Tile-ground stores, OFE-ground depths, adjacent
diagnostic ledgers, and routed destination bases must have deliberately unequal
fixture values so an alias cannot satisfy the expected result.
