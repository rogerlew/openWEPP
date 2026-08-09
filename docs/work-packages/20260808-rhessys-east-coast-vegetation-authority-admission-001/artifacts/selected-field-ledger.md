# Selected Pine/Oak Field Ledger

Status: `COMPLETE / 71 parameter rows x 2 selected profiles`

Evidence mode: `Ran + Static + primary-source inspection`

The source CSV contains one profile-name row followed by the 71 parameter rows
below. The profile names are `chestnut.oak.bgc` and `eastern.white.pine`; their
candidate IDs are `805` and `807`. “Consume” means required by the proposed
first runtime boundary, not admitted for use. “Raw” means preserve for
diagnostics but exclude from that boundary. No `HOLD` value is promoted.

Source-route codes:

- `W00`: White et al. (2000), Appendix A and ORNL DAAC 652 parameter families.
- `H09-T2/T3`: Hwang et al. (2009), Tables 2 or 3, Coweeta WS18.
- `R99-T2`: Reich et al. (1999), Table 2 leaf-trait observations.
- `F10-TI`: Ford et al. (2010), Table I, dated Coweeta stand observations.
- `PHY`: physical fraction/closure authority only; no empirical cell value.
- `SRC`: pinned source behavior or CSV identity only; not science authority.
- `NONE`: the inspected header routes do not identify the cell.

`H09-T3` values are catchment-scale/model parameters, not chestnut-oak
measurements. Hwang et al. explicitly says the modeled values were
composition-weighted and that some allocation and phenology inputs were not
measured at species level. A numerical match therefore establishes likely
lineage but can also disprove the profile label's species-level meaning.

| # | Exact key | Oak raw | Pine raw | First boundary | Units/basis | Alias | Source route | Terminal disposition |
| ---: | --- | ---: | ---: | --- | --- | --- | --- | --- |
| 1 | `stratum_default_ID` | `805` | `807` | identity | integer | exact | `SRC` | schema identity admitted; uniqueness/runtime guard missing |
| 2 | `K_absorptance` | `0.5` | `0.8` | consume | fraction | exact | `PHY/NONE` | `HOLD`; oak optical triple fails closure |
| 3 | `K_reflectance` | `0.31` | `0.1` | consume | fraction | exact | `PHY/NONE` | `HOLD`; no cell/domain locator |
| 4 | `K_transmittance` | `0.22` | `0.1` | consume | fraction | exact | `PHY/NONE` | `HOLD`; oak optical triple fails closure |
| 5 | `PAR_absorptance` | `0.8` | `1` | consume | fraction | exact | `PHY/NONE` | `HOLD`; no cell/domain locator |
| 6 | `PAR_reflectance` | `0` | `0` | consume | fraction | exact | `PHY/NONE` | `HOLD`; no cell/domain locator |
| 7 | `PAR_transmittance` | `0.2` | `0` | consume | fraction | exact | `PHY/NONE` | `HOLD`; no cell/domain locator |
| 8 | `epc.alloc_crootc_stemc` | `0.22` | `0.29` | minimum C state | ratio | exact | `H09-T3/W00` | `HOLD`; oak matches catchment value, not species authority |
| 9 | `epc.alloc_frootc_leafc` | `1.21` | `0.994` | minimum C state | ratio | exact | `H09-T3/W00` | `HOLD`; oak matches catchment value, not species authority |
| 10 | `epc.alloc_livewoodc_woodc` | `0.16` | `0.076` | minimum C state | fraction | exact | `H09-T3/W00` | `HOLD`; oak matches catchment value, not species authority |
| 11 | `epc.alloc_prop_day_growth` | `0.3` | `0.5` | minimum C state | unresolved fraction/cadence | exact | `SRC` | `HOLD`; semantic authority absent |
| 12 | `epc.alloc_stemc_leafc` | `2.073333` | `2.73` | minimum C state | ratio | exact | `H09-T3/W00` | `HOLD`; Hwang catchment value is `1.0`, so oak does not match |
| 13 | `epc.allocation_flag` | `dickenson` | `waring` | consume/branch | enum | exact | `SRC` | `REJECT`; selected branches and hidden coefficients unadmitted |
| 14 | `epc.daily_fire_turnover` | `0` | `0` | raw | day^-1 | exact | `W00` family | `EXCLUDE`; fire/disturbance out of scope |
| 15 | `epc.day_leafoff` | `295` | `295` | raw | day of year | exact | `H09-T3/NONE` | `EXCLUDE`; existing openWEPP GSI owns phenology |
| 16 | `epc.day_leafon` | `100` | `60` | raw | day of year | exact | `H09-T3/NONE` | `EXCLUDE`; Hwang catchment value is `105`, not species data |
| 17 | `epc.deadwood_fcel` | `0.77` | `0.725` | raw | fraction of dry matter | exact | `W00` | `EXCLUDE`; dead-material owner out of scope |
| 18 | `epc.deadwood_flig` | `0.23` | `0.275` | raw | fraction of dry matter | exact | `W00` | `EXCLUDE`; dead-material owner out of scope |
| 19 | `epc.ext_coef` | `0.5` | `0.51` | consume | dimensionless | exact | `H09-T3/W00` | `HOLD`; Hwang catchment value is `0.54`, not `0.5` |
| 20 | `epc.flnr` | `0.07` | `0.033` | consume | leaf-N fraction | exact | `W00` | `HOLD`; biome family only, no selected-cell locator |
| 21 | `epc.froot_cn` | `63.5` | `53.6` | immutable-N state | kg C kg N^-1 | exact | `H09-T3/W00` | `HOLD`; Hwang catchment value is `51.1`, not oak cell |
| 22 | `epc.froot_turnover` | `0.5` | `0.332` | minimum C state | yr^-1 | exact | `W00` | `HOLD`; selected-cell/domain mapping absent |
| 23 | `epc.frootlitr_fcel` | `0.44` | `0.495` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 24 | `epc.frootlitr_flab` | `0.34` | `0.252` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 25 | `epc.frootlitr_flig` | `0.22` | `0.253` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 26 | `epc.gl_c` | `2.33E-05` | `2.40E-05` | consume | m s^-1 | exact | `H09-T2/W00` | `HOLD`; no exact conversion/scale locator |
| 27 | `epc.gl_smax` | `0.00265299` | `0.0024` | consume | m s^-1 | exact | `H09-T2/W00` | `HOLD`; Hwang oak maximum is `0.0234 m s^-1`, a tenfold mismatch |
| 28 | `epc.gr_perc` | `0.5018194` | `0.2` | minimum C state | fraction | exact | `SRC` | `HOLD`; executed growth-feedback meaning lacks authority |
| 29 | `epc.height_to_stem_coef` | `12.5` | `11.39` | consume | allometry coefficient; area basis unresolved | exact | Martin et al. lead | `HOLD`; equation/species/domain mapping not established |
| 30 | `epc.height_to_stem_exp` | `0.35` | `0.57` | consume | dimensionless exponent | exact | Martin et al. lead | `HOLD`; equation/species/domain mapping not established |
| 31 | `epc.kfrag_base` | `0.00002` | `0.01` | raw | day^-1 | exact | `W00/SRC` | `EXCLUDE`; material fragmentation owner out of scope |
| 32 | `epc.lai_ratio (allside_oneside)` | `2` | `2.54` | consume | all-sided/projected ratio | candidate only | `H09-T2/W00` | `HOLD`; parser key differs and unit/basis authority precedes alias |
| 33 | `epc.leaf_cn` | `19.9226` | `26.1` | immutable-N state | kg C kg N^-1 | exact | `H09-T2/W00` | `HOLD`; Hwang oak value is `25.9`, not the CSV cell |
| 34 | `epc.leaf_turnover` | `1` | `0.332` | minimum C state | yr^-1 | exact | `W00` | `HOLD`; selected-cell/domain mapping absent |
| 35 | `epc.leaflitr_cn` | `41.31` | `93` | raw | kg C kg N^-1 | exact | `H09-T3/W00` | `EXCLUDE`; litter/material owner out of scope |
| 36 | `epc.leaflitr_fcel` | `0.44` | `0.447` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 37 | `epc.leaflitr_flab` | `0.38` | `0.328` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 38 | `epc.leaflitr_flig` | `0.18` | `0.225` | raw | fraction | exact | `W00` | `EXCLUDE`; litter/material owner out of scope |
| 39 | `epc.livewood_cn` | `75.6` | `50` | immutable-N state | kg C kg N^-1 | exact | `H09-T3/W00` | `HOLD`; oak is catchment value and source parser overwrites it |
| 40 | `epc.livewood_turnover` | `0.7` | `0.769` | minimum C state | yr^-1 | exact | `W00` | `HOLD`; selected-cell/domain mapping absent |
| 41 | `epc.max_lai` | `6` | `20` | consume | m^2 m^-2 ground | exact | `F10-TI/W00` | `HOLD`; Ford observed stand peaks `6.2` and `7.2`, not these cells |
| 42 | `epc.maxlgf` | `0.05` | `0.05` | minimum C state | fraction | exact | `SRC` | `HOLD`; meaning/bound authority absent |
| 43 | `epc.ndays_expand` | `20` | `20` | raw | days | exact | `H09-T3/NONE` | `EXCLUDE`; existing GSI owns phenology; Hwang reports 35 days |
| 44 | `epc.ndays_litfall` | `20` | `50` | raw | days | exact | `H09-T3/NONE` | `EXCLUDE`; existing GSI owns phenology; Hwang reports 50 days |
| 45 | `epc.phenology.type` | `DECID` | `EVERGREEN` | identity | enum | exact | `SRC` | lifeform identity retained; no source phenology execution |
| 46 | `epc.phenology_flag` | `static` | `static` | raw | enum | exact | `SRC` | `EXCLUDE`; no second phenology owner |
| 47 | `epc.ppfd_coef` | `0.0234` | `0.03` | consume | PPFD-response units unresolved | exact | `H09-T2/SRC` | `HOLD`; oak equals Hwang max conductance numerically, suggesting misassignment |
| 48 | `epc.proj_sla m2_kgC` | `24.30784314` | `18.8` | consume | m^2 projected leaf kg C^-1 | candidate only | `H09-T2/R99-T2` | `HOLD`; parser key differs; pine match requires unrecorded carbon fraction |
| 49 | `epc.proj_swa` | `1.4` | `1.4` | consume | projected sapwood-area basis unresolved | exact | `NONE` | `HOLD`; definition/domain absent |
| 50 | `epc.psi_close` | `-2.2` | `-2.3` | consume | MPa predawn leaf water potential | exact | `W00` | `HOLD`; family is documented, selected observation/domain is not |
| 51 | `epc.psi_open` | `-0.924` | `-0.63` | consume | MPa predawn leaf water potential | exact | `W00` | `HOLD`; family is documented, selected observation/domain is not |
| 52 | `epc.storage_transfer_prop` | `1` | `1` | minimum C state | fraction/cadence unresolved | exact | `SRC` | `HOLD`; transition meaning/authority absent |
| 53 | `epc.tcoef` | `0.6725` | `0.2` | consume | temperature-response coefficient unresolved | exact | `SRC` | `HOLD`; equation/domain absent |
| 54 | `epc.tmax` | `42.5` | `40` | consume | deg C | exact | `SRC` | `HOLD`; response family/domain absent |
| 55 | `epc.topt` | `21.8` | `15` | consume | deg C | exact | `SRC` | `HOLD`; response family/domain absent |
| 56 | `epc.veg.type` | `TREE` | `TREE` | identity | enum | exact | `SRC` | vascular-tree identity retained; no parameter value admitted |
| 57 | `epc.vpd_close (x1000)` | `19127` | `3100` | consume | apparent Pa after decorated scaling | candidate only | `W00` | `HOLD`; parser key differs and scale semantics are unproved |
| 58 | `epc.vpd_open (x1000)` | `658` | `600` | consume | apparent Pa after decorated scaling | candidate only | `W00` | `HOLD`; parser key differs and scale semantics are unproved |
| 59 | `gsurf_intercept` | `100000000` | `1000000` | consume wet canopy | source conductance units inconsistent | exact | `SRC` | `REJECT` source law; `HOLD` replacement wet-surface conductance authority |
| 60 | `gsurf_slope` | `0` | `0` | consume wet canopy | conductance/storage slope unresolved | exact | `SRC` | `REJECT` source law; `HOLD` replacement wet-surface conductance authority |
| 61 | `lai_stomatal_fraction` | `1.45` | `1` | consume | LAI scaling fraction/ratio | exact | `NONE` | `HOLD`; value exceeds fraction interpretation and basis is absent |
| 62 | `max_heat_capacity` | `0` | `0` | raw | source storage-heat dimensions invalid | exact | `SRC` | `REJECT`; land-surface energy owns available-energy operands |
| 63 | `min_heat_capacity` | `0` | `0` | raw | source storage-heat dimensions invalid | exact | `SRC` | `REJECT`; land-surface energy owns available-energy operands |
| 64 | `mortality` | `0.005` | `0.002` | raw | fraction yr^-1 apparent | rejected alias | `H09-T3/W00` | `EXCLUDE`; cadence differs from parser key and mortality is out of scope |
| 65 | `mrc.per_N` | `0.3452163` | `0.3506968` | consume | respiration per N; units unresolved | exact | `SRC` | `HOLD`; exact equation/unit locator absent |
| 66 | `mrc.q10` | `2.326667` | `2` | consume | dimensionless Q10 | exact | `H09-T2/W00` | `HOLD`; oak likely aggregates Hwang species Q10, mapping is undocumented |
| 67 | `specific_rain_capacity` | `0.0005` | `0.00024` | consume | m water per all-sided PAI | exact | `W00/NONE` | `HOLD`; selected-cell/domain mapping absent |
| 68 | `specific_snow_capacity` | `0.001` | `0.00024` | raw | m water per all-sided PAI | exact | `W00/NONE` | `EXCLUDE`; canopy snow explicitly deferred |
| 69 | `ustar_overu` | `-999.9` | `-999.9` | consume | dimensionless | exact | `SRC` | `REJECT`; unsupported sentinel, no aerodynamic regime authority |
| 70 | `wind_attenuation_coef` | `0.4` | `0.4` | consume | dimensional interpretation unresolved | exact | `NONE` | `HOLD`; equation/unit/domain absent |
| 71 | `epc.branch_turnover` | `0.03` | `0.03` | raw | yr^-1 input | exact | `W00` | `EXCLUDE`; material/mortality owner out of scope |

## Ledger Result

The ledger closes population and disposition, not empirical admission. Exact
source matches are partial and often expose aggregation, transformation, or
misassignment. No selected row is a complete executable parameter set. The
strict raw/acquisition/schema form is canonically admitted; every consumed
empirical value remains held or rejected and every raw-only field is severed
from the first implementation boundary.
