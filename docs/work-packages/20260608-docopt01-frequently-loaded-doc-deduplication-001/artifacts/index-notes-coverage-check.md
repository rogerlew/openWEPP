# Index Notes Coverage Check

Status: complete
Evidence mode: Ran

Ran: parsed `docs/specifications/science-contracts/index.md` registry rows and checked amendment/provenance tokens against each owning canonical contract before replacing optional `notes` cells.

Result: PASS. No row entered HOLD; every stripped row-note token was present in the owning canonical contract.

| contract_id | original note chars | checked tokens | missing tokens | disposition |
|---|---:|---:|---|---|
| `SC-CLIMATE-001` | 1467 | 18 | none | stripped to lifecycle-only note |
| `SC-EVAP-001` | 802 | 12 | none | stripped to lifecycle-only note |
| `SC-HYDRAULICS-001` | 320 | 5 | none | stripped to lifecycle-only note |
| `SC-IMPOUND-001` | 516 | 9 | none | stripped to lifecycle-only note |
| `SC-IRRIG-001` | 295 | 3 | none | stripped to lifecycle-only note |
| `SC-PERC-001` | 1527 | 15 | none | stripped to lifecycle-only note |
| `SC-PLANT-001` | 451 | 7 | none | stripped to lifecycle-only note |
| `SC-RESIDUE-001` | 257 | 5 | none | stripped to lifecycle-only note |
| `SC-ROUTE-001` | 5054 | 42 | none | stripped to lifecycle-only note |
| `SC-RUNOFFPART-001` | 490 | 6 | none | stripped to lifecycle-only note |
| `SC-SED-001` | 4926 | 37 | none | stripped to lifecycle-only note |
| `SC-SNOWFREEZE-001` | 1852 | 27 | none | stripped to lifecycle-only note |
| `SC-SOIL-001` | 2221 | 20 | none | stripped to lifecycle-only note |
| `SC-SUBHYD-001` | 1511 | 14 | none | stripped to lifecycle-only note |
| `SC-SYSTEM-001` | 6270 | 55 | none | stripped to lifecycle-only note |
| `SC-WATBAL-001` | 4516 | 44 | none | stripped to lifecycle-only note |

ADR0017/Entry Order narrative check: PASS. HPHYS/INV/SC pointers from the condensed ADR0017 note and removed Entry Order narratives were present in canonical contracts or governance text.
HOLD rows: none.
