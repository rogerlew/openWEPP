# Forcing And Operand Provider Map

Status: `FROZEN`

| Operand | Unit/time/area basis | Provider/owner | Missing behavior |
|---|---|---|---|
| VIS/NIR direct/diffuse, longwave | W m-2 ground, interval-mean | forcing / energy | typed missing/nonfinite |
| precipitation | kg H2O m-2 ground per interval | forcing / hydrology | typed missing/phase unsupported |
| air/canopy reference T, humidity, CO2, pressure, wind | K, kg kg-1, Pa, m s-1 | forcing / atmosphere | typed domain; calm/nonneutral unsupported |
| soil potential, temperature, conductivity, frozen/accessibility | per layer, snapshot | hydrology | typed missing/profile mismatch |
| layer mineral NH4/NO3 | kg N m-2 ground | BGC | typed missing/profile mismatch |
| leaf/stem area and C/N pools | m2 or kg element m-2 ground | vegetation beginning state | typed identity/closure failure |
| caller `dt_s`, topology/tile fractions | s and ground fraction | orchestrator/config | typed nonpositive/sum/identity failure |

No operand is profile-averaged. Every rate is converted once at the transaction ledger.
