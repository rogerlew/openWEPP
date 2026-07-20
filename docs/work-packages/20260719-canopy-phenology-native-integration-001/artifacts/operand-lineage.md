# Operand Lineage

Evidence mode: `Static`

Status: `frozen`

| Operand | Units/basis | Source and normalization | Status / consumer |
|---|---|---|---|
| minimum temperature | degC, daily minimum | typed climate `tmin_c`, identity | authoritative GSI forcing |
| VPD | Pa, daily | `max(0, 0.5*(svp(tmax)+svp(tmin))-svp(tdpt))*1000`; materially negative/non-finite fails | authoritative GSI forcing |
| latitude/date | signed degrees / calendar day | typed climate request plus actual year/ordinal | authoritative photoperiod/chronology |
| six GSI thresholds | degC, Pa, hours | required native YAML, exact parser and PL projection | authoritative process parameters |
| `g` | fraction | exact 21-real-sample moving mean from `GsiState` | authoritative continuous activity signal |
| `fe` | fraction | required YAML `evergreen_fraction` | authoritative endpoint parameter |
| `Bf,max` | kg/m2 | required YAML `summer_foliar_biomass_kg_m2` | authoritative summer foliar endpoint |
| live foliar mass | kg/m2 | `Bf,max*(fe+(1-fe)*g)` | authoritative growth/interception state |
| structural biomass | kg/m2 | required YAML; no seasonal transfer | authoritative persistent diagnostic |
| maximum LAI | m2/m2 | existing forest `growth.xmxlai` | authoritative summer LAI endpoint |
| daily LAI | m2/m2 | `xmxlai*(fe+(1-fe)*g)` | authoritative ET/routing state |
| `bb` | m2/kg | existing forest canopy coefficient | authoritative canopy relation operand |
| structural cover | fraction | required YAML, bounded to `0..=0.999` | authoritative persistent canopy floor |
| daily canopy | fraction | `max(structural,1-exp(-bb*Bf))`, cap `0.999` | snow, WB15, ET, erosion/publication |
| leaf-on allocation | kg/m2/day | `max(B_after-B_before,0)` | authoritative plant mass input |
| leaf-off litter | kg/m2/day | `max(B_before-B_after,0)` | authoritative same-day residue input |
| aggregate biomass decline | kg/m2/day | legacy growth-state delta | compatibility diagnostic only; bypassed for native GSI |
| `jdharv` pending litter | kg/m2 | legacy perennial window | compatibility only; rejected on native GSI branch |
