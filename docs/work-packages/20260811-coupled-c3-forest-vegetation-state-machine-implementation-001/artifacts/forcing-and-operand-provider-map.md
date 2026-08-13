# Forcing And Operand Provider Map

Status: `PARTIAL / V5 capped operands authoritative; runtime binding active`

Increment 2B audit: reference wind exists, but distinct `u_leaf` and `u_wet`
providers do not. Stem hydraulic path/gravity and the accepted common-root to
per-layer persistent-state mapping are also absent. No implicit aliases or
derived substitutes were implemented.

| Operand | Unit/time/area basis | Provider/owner | Fail-closed behavior |
|---|---|---|---|
| VIS/NIR direct/diffuse, longwave | W m-2 ground, interval-mean | forcing / energy | typed missing/nonfinite |
| precipitation | kg H2O m-2 ground per interval | forcing / hydrology | typed missing/phase unsupported |
| air/canopy reference T, humidity, CO2, pressure, wind | K, kg kg-1, Pa, m s-1 | forcing / atmosphere | typed domain; calm/nonneutral unsupported |
| soil potential, temperature, conductivity, frozen/accessibility | per layer, snapshot | hydrology | typed missing/profile mismatch |
| layer mineral NH4/NO3 | kg N m-2 ground | BGC | typed missing/profile mismatch |
| leaf/stem area and C/N pools | m2 or kg element m-2 ground | vegetation beginning state | typed identity/closure failure |
| caller `dt_s`, topology/tile fractions | s and ground fraction | orchestrator/config | typed nonpositive/sum/identity failure |
| V2 tile-top rain | kg H2O m-2 tile-ground per interval | exact tile forcing map / hydrology handoff | exact tile-set equality; missing/extra/nonfinite/negative fails before routing |
| V2 conditional LAI/WAI | m2 plant m-2 tile-ground | column engine from shared area and `C_s` | exact `LAI_s/C_s`, `WAI_s/C_s`; stand-area poison differs |
| V2 local layer demand/cap/use | kg H2O m-2 tile-ground per interval | occupancy solver; column boundary | exact occupancy/layer identity; multiply/divide by positive `f_t` once |

No operand is profile-averaged. Increment 2A accepts heterogeneous tile rain
explicitly because the current public `SnowFreeForcing.rain_kg_m2` remains
stand-shaped; public forcing integration is intentionally still fail-closed.
Every rate is converted once at the transaction ledger.

V4 binds displayed leaf C as the only LAI/area-cache provider and displayed
leaf N as the only positive-LAI capacity/leaf-Rd provider. Storage/transfer
leaf pools remain mass operands only. V5 now binds exact stand-ground
authorization amount, tile-ground amount, tile-ground rate, independently
evaluated `q_law`, selected `q_i`, finalized stand-ground amount, equality-
active branch, and configured layer order. The implementation must preserve
each as a distinct typed operand. Until its exact fixture and review gates
pass, no provider alias or finalized-use claim is accepted and the public path
remains fail-closed.
