# Authority Route Attempts

Status: `executed / frozen-boundary blocker narrowed`

Evidence mode: `Ran + Static + primary-source inspection`

| Route | Result | Frozen-boundary disposition |
| --- | --- | --- |
| pinned CSV/source provenance | usable for raw identity only | code/data provenance is not scientific authority |
| exact 71-by-2 selected-field ledger | complete | every raw value, first-boundary role, unit/basis, alias, source route, domain issue, and terminal disposition is recorded in `selected-field-ledger.md` |
| CSV first-row bibliography bundle | inspected as discovery routes | the repeated column-level string is not itself a mapping, but its exact table/figure leads were followed below |
| White et al. (2000) and ORNL DAAC 652 | family definitions acquired | Appendix A and the public ORNL dataset describe parameter families, units, biome groupings, and literature rows; they provide no dated stand initial state and do not identify which rows/conversions produced every selected GIS cell |
| Reich et al. (1999), Table 2 | partial pine SLA lineage | `Pinus strobus` SLA is `92 cm2 gDM^-1`; reaching the CSV's `18.8 m2 kgC^-1` requires an unrecorded dry-mass carbon fraction of about `0.489` plus projected-area identity, so the exact cell remains held |
| Hwang et al. (2009), Tables 2-3 | strong oak/Coweeta lineage and counterevidence | Table 2 reports `Quercus prinus` observations; Table 3 reports catchment parameters. Several oak CSV values match catchment values, while others differ or appear under the wrong key. The authors state that parameters were composition-weighted and some allocation/phenology data were not measured at species level |
| Martin/Mitchell/Vose-Bolstad/Sullivan leads through Hwang Table 2 | partially incorporated by the primary table | Hwang identifies which Table-2 columns derive from these sources and gives sample counts, but the selected GIS column neither records the transformation nor consistently reproduces the Table-2 row |
| Day and Monk (1977) and Coweeta biomass/NPP routes | stand-data lead, not complete state | these constrain Coweeta biomass/productivity and allocation context, but do not provide one dated, topology-compatible leaf/root/wood C/N state for both selected profiles |
| Ford et al. (2010), Tables I-V and Figure 4 | dated species/stand evidence acquired | WS18 `Quercus prinus` and adjacent WS17 `Pinus strobus` have 2005-2006 LAI, basal area, density, DBH, height, sapwood/leaf area, transpiration, and conductance observations; they are different watersheds and do not observe every required C/N/root pool |
| tracked Harvard/Hubbard observations | checked for replacement-state coverage | useful phenology/litter observations, but not a complete selected C/N/root/geometry state and not a shared selected stand/date for these profiles |
| Jarvis/Gash/Kelliher/Shuttleworth-Wallace/Farquhar/de Pury/Wullschleger sources | process-family leads | none supplies the complete pine/oak 71-cell map or dated stand state |
| physical conservation/dimensional invariants | binding for closure and rejection | cannot determine empirical physiological values or ecosystem initial pools |
| GIS initializer implementation | rejected | hard-coded ratios, fixed row positions, nonfinite canonicalization, and SLA divergence |
| replace with assumed executable values | prohibited | would be surrogate parameter authority and violate the frozen selected boundary |

The search disproved the earlier claim that the header sources could add no
information. They recover partial lineage and useful observations, but they
also show that the selected oak column mixes species, catchment, transformed,
and apparently misassigned values. The remaining blocker is narrower:

1. no complete, internally consistent selected-parameter replacement has every
   consumed value, equation, conversion, area/leaf basis, and domain admitted;
2. the package has no selected stand/plot/date/topology on which an initial
   state could be defined; and
3. the best inspected Coweeta observations cover adjacent pine and hardwood
   watersheds and omit required root and elemental pools.

No inaccessible paper is claimed as the sole blocker. Lifting the hold requires
a user-selected simulation site/date/topology plus a complete observation or
independently authoritative synthesis route, not broader unspecific searching.
