# Covered-consumer contract map

| Live operand | Contract/source authority | Current binding | Disposition |
| --- | --- | --- | --- |
| canopy temperatures and humidity | SC-SNOWENERGY-001 Child 2C; SC-VEGETATION-001@26 | V11 occupancy/canopy-air state | `PASS / persistent one-support proof; matrix pending` |
| leaf/stem/wet-surface conductances | SC-SNOWENERGY-001 Child 2C; SC-VEGETATION-001@26 | derived from current V11 canopy state and admitted geometry | `PASS / persistent one-support proof; matrix pending` |
| shared canopy-air state | SC-SNOWENERGY-001 `INV-SNOWENERGY-036` | one carrier node, not one node per stratum | `PASS / shared-carrier receipt proven; matrix pending` |
| snow temperature, humidity, roughness, emissivity | SC-SNOWENERGY-001 variables and Child 2C carrier | Stage-3 current layer/forcing plus fixed admitted snow geometry; no runner ending state | `PASS / persistent one-support proof; matrix pending` |
| reference atmosphere and wind exposure | SC-SNOWENERGY-001 `INV-SNOWENERGY-037`; SC-SNOWFREEFORCING-001 | sealed provider receipt and exposure identity | `PASS / consumer proof pending` |
| reciprocal longwave | SC-SNOWENERGY-001 `INV-SNOWENERGY-038` | current V11 component temperatures and Stage-3 snow temperature | `PASS / carrier ledger proof; matrix pending` |
| support identity and coupled duration | SC-COUPLEDTIME-001 `INV-COUPLEDTIME-017..020`; SC-SNOWFREEFORCING-001 `INV-SFF-001` | `TimeSupport`, accepted slab, forcing digest | `PASS / consumer proof pending` |
| mass/vapor/energy ledger operands | SC-SNOWENERGY-001 `OBL-SNOWENERGY-C-017`; SC-VEGETATIONTRANSACTION-001 `OBL-VEGTRANSACTION-C-005` | independent reconstruction at shared-carrier/parent boundary | `PASS / persistent one-support proof; terminal/matrix pending` |

`Static:` The contract gate is complete. The remaining matrix and terminal
labels are implementation evidence obligations, not missing science
authority.
