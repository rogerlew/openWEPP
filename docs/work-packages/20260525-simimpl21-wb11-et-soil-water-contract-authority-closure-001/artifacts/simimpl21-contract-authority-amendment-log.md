# SIMIMPL21 Contract Authority Amendment Log

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL21 completed canonical contract-authority amendments for WB11 ET and
  soil-water closure surfaces identified by SIMIMPL20.
- Amendment summary:

| Contract | Version change | SIMIMPL21 authority closure added |
|---|---|---|
| `SC-EVAP-001` | `7 -> 8` | Stage-memory and uptake-lineage authority (`s1`, `s2`, `tu`, `tv`, `UPi`, `Ui`) with `INV-EVAP-013`. |
| `SC-WATBAL-001` | `34 -> 35` | Baseline WB11 sequencing + aggregate lineage closure with `INV-WATBAL-028/029`. |
| `SC-SOIL-001` | `3 -> 4` | ET/soil-water alias-lineage closure with `INV-SOIL-013`. |
| `SC-PLANT-001` | `12 -> 13` | WB11 stress/uptake lineage coupling closure with `INV-PLANT-023`. |
| `SC-SYSTEM-001` | `23 -> 24` | WB13 ET/soil-water publication-lineage closure with `INV-SYSTEM-027`. |
| `science-contracts/index.md` | registry update | Last-reviewed/notes updated for SIMIMPL21-amended contracts. |

## Ran
- `rg -n "INV-EVAP-013|REF-EVAP-LEGACY-STAGE|REF-EVAP-LEGACY-SOILX|REF-EVAP-LEGACY-SWU|OBL-EVAP-P-005|OBL-EVAP-C-005|GAP-EVAP-005|SIMIMPL21 WB11 ET Stage-Memory" docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `rg -n "INV-WATBAL-028|INV-WATBAL-029|REF-WATBAL-LEGACY-ORDER|REF-WATBAL-LEGACY-WATCON|REF-WATBAL-LEGACY-WB13|OBL-WATBAL-P-015|OBL-WATBAL-C-005|SIMIMPL21 WB11 ET/Soil-Water" docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `rg -n "INV-SOIL-013|REF-SOIL-LEGACY-WB11|OBL-SOIL-P-005|OBL-SOIL-C-005|SIMIMPL21 WB11 Soil-Water" docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `rg -n "INV-PLANT-023|REF-PLANT-LEGACY-WATBAL|REF-PLANT-LEGACY-SWU|OBL-PLANT-P-008|OBL-PLANT-C-006|SIMIMPL21 WB11 ET Stress-Lineage" docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `rg -n "INV-SYSTEM-027|REF-SYSTEM-LEGACY-WATBAL|REF-SYSTEM-LEGACY-OUTFIL|OBL-SYSTEM-P-015|OBL-SYSTEM-C-005|SIMIMPL21 WB13 ET/Soil-Water" docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `rg -n "SC-EVAP-001|SC-WATBAL-001|SC-SOIL-001|SC-PLANT-001|SC-SYSTEM-001" docs/specifications/science-contracts/index.md`
