# EROD18 Route Topology Ingress Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
| symbol family | ingress source | EROD18 behavior | guard family |
|---|---|---|---|
| `nslpts` | existing runtime slope alias | required scalar, integral, `>=2` | `HKERNEL-EROD18-ROUTE-E-001..003` |
| `xu_0002`, `xl_0002` | runner Wave-2 route-topology seeding | required segment geometry, `xl >= xu` | `HKERNEL-EROD18-ROUTE-E-001..003` |
| `ainf_0002`, `binf_0002`, `cinf_0002` | runner Wave-2 route-topology seeding | required segment coefficient family | `HKERNEL-EROD18-ROUTE-E-001..003` |
| `ainftc_0002`, `binftc_0002`, `cinftc_0002` | runner Wave-2 route-topology seeding | required segment transport-capacity coefficients | `HKERNEL-EROD18-ROUTE-E-001..003` |
| `qostar`, `xdetst`, `lddend` | runner Wave-2 route-topology seeding | required canonical route ingress scalars | `HKERNEL-EROD18-ROUTE-E-001..003` |
| `xdbeg`, `xdend`, `ldlast`, `du`, `dl`, `ndep`, `mshear`, `xc1`, `xc2` | kernel EROD18 route-topology publication | published topology seam for downstream EROD19 branch migration | `HKERNEL-EROD18-ROUTE-E-001..003` |

## Ran
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
