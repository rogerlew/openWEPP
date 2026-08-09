# Citation Discovery Ledger

Status: `audit-complete`

Evidence mode: `Static + Ran`

| ID | Source lead | Exact affected surface | Primary-source status | Authority consequence |
| --- | --- | --- | --- | --- |
| `CIT-001` | “biome bgc” | APAR response; default parser families | White et al. (2000) read at Table 1 and Appendix A family sections; it supports biome parameter families, not every GIS cell | family lead only; profile cells remain `PARAMETER_DATA` |
| `CIT-002` | “Jarvis model” | vascular multiplicative conductance | Jarvis (1976), Eqs. 4-9, read from restricted local cache | functional family supported; RHESSys scaling/parameters not thereby authorized |
| `CIT-003` | Running and Coughlan | Tmin and LWP multiplier comments | exact cited edition/equation not supplied in source and not acquired as a supporting locator | `CITATION_MISMATCH`; blocker for those exact curves |
| `CIT-004` | “insert paper” | CO2 conductance multiplier | no citation exists | `CODE_ONLY`; reject as authority |
| `CIT-005` | “current literature does not support” | computed Tavg multiplier omitted from final product | no named source or locator | code comment cannot authorize omission; `CODE_ONLY` |
| `CIT-006` | Penman-Monteith / BIOME-BGC | `penman_monteith` | Shuttleworth-Wallace and JULES primary/official locators read; algebraic family supported | resistance/area/cadence chain still blocked |
| `CIT-007` | Farquhar, von Caemmerer, Berry (1980) | Rubisco/electron-transport limited leaf assimilation | primary article read at model-development equations | C3 leaf equations supported, RHESSys constants/iterations partly deviate |
| `CIT-008` | de Pury and Farquhar (1997) | electron transport and sunlit/shade scaling | primary article read at Table 1 and sun/shade derivation | supports separate sun/shade scaling; not the source's simplified full implementation |
| `CIT-009` | Wullschleger (1993) | `Jmax` relationship | primary article located; code's fixed `Jmax=2.1 Vmax` is attributed to a later BGC change, not Wullschleger's regression | `CITATION_MISMATCH` for the executed expression |
| `CIT-010` | Kuehn and McFadden (1969); Woodrow and Berry review | Rubisco N/activity constants | citation leads only; exact constant chain not fully acquired/adjudicated | block constant admission |
| `CIT-011` | Peter Thornton BIOME-BGC versions | photosynthesis, respiration, phenology, allocation, turnover | software/person attribution without exact immutable source locator | `CODE_OBSERVED`, not primary authority |
| `CIT-012` | Ryan (1991) | maintenance respiration | citation supplied in code; exact regression/domain not acquired in this audit | block selected carbon path |
| `CIT-013` | Jolly et al. (2005) | dynamic GSI defaults absent from CSV | source comment identifies a global uniform parameter lead; no profile-specific locator | deferred dynamic-GSI authority; current source defaults are not admitted |
| `CIT-014` | White et al. (2000) | forced `livewood_cn=froot_cn` and many CSV families | paper read; no support found for silently overwriting supplied livewood C:N with fine-root C:N | `SILENT_DEVIATION`; reject overwrite |
| `CIT-015` | Lundberg (1994) | snow conductance reduction | source comment only; canopy snow excluded | `DEFER` |
| `CIT-016` | Gash (1979); Best et al. (2011) | rain interception/store/throughfall | primary/official sources read in predecessor review | candidate finite-store/event relations; source implementation requires re-derivation |
| `CIT-017` | Kelliher et al. (1995) | leaf/canopy/surface conductance scaling | primary article read from restricted cache | rejects silent scale aliasing and universal parameter transfer |
| `CIT-018` | Heddeland and Lettenmaier (1995); “Xuewen Wang code” | aerodynamic resistance | incomplete citation/software attribution; no exact admitted equation/domain | `BLOCK_SUCCESSOR` |
| `CIT-019` | CSV top-row bundle: Reich, Schulze, Martin, Vose/Bolstad, Day/Monk, Hwang, Ford, Pataki/Oren, Sullivan, Mitchell, Farquhar | multiple East-Coast profile columns | bundle repeats across columns but supplies no field-to-paper/table mapping; spelling and year details are incomplete | discovery leads only; cannot authorize any cell |
| `CIT-020` | profile annotations “mostly Tony”, “Tony”, “Taehee”, “White” | final profile columns | personal-name or surname-only provenance | `PARAMETER_DATA`; no scientific locator |
| `CIT-021` | Jarvis and Leverenz Eq. 8.18; Norman (1981) | diffuse broadband/PAR penetration | source comment is incomplete (`??`) and the primary chapter was not acquired | discovery lead only; mixed/sparse diffuse operator remains `AUTH-RHEC-003` |
| `CIT-022` | Chen et al. “BEPS” (1997) | direct radiation | source gives no complete article/title/equation locator; Chen et al. (1999) is separately named for sunlit LAI | citation identity is ambiguous; source expression remains unadmitted |
| `CIT-023` | Brubaker (1996), via Dingman | snow-surface temperature used by canopy longwave | indirect citation only; exact edition/equation was not acquired and source ultimately uses dew point clipped at zero | `CITATION_MISMATCH`; longwave route blocked |
| `CIT-024` | Tjoelker et al. (2001), Global Change Biology 7:223-230 | temperature acclimation of respiration Q10 | code supplies a bibliographic lead but no coefficient/table mapping for profile defaults | family lead only; selected respiration remains blocked |
| `CIT-025` | Nambiar et al. (1991), Tree Physiology | live-stem/coarse-root turnover | incomplete article/equation locator; no selected profile parameter mapping | later turnover lead; no production authority |
| `CIT-026` | Chen et al. (1999), Ecological Modelling 124:99-119 | iterative sunlit/shaded LAI fraction in phenology update | article lead identified, but executed tolerance, zero branch, SLA coupling, and daily state transition were not adjudicated | selected future-LAI state remains blocked |
| `CIT-027` | Helvey (1964); Ogee and Brunet (2002), Journal of Hydrology | litter moisture capacity default | source comment supplies forest-context leads but no exact equation/table or transfer map | litter process is deferred; default not admitted |
| `CIT-028` | Arora and Boer, Eq. 11 | dynamic rooting depth | source supplies no complete bibliographic identity; primary equation was not acquired | discovery lead only; `AUTH-RHEC-010` remains blocked |
| `CIT-029` | Williams and Flanagan (1996), Oecologia | nonvascular water-content/conductance relation | source describes moss measurement domain plus a later quadratic fit, but exact equation/data table was not acquired | domain-mismatched lead; no tree/wet-canopy authority |
| `CIT-030` | Kelliher, Black, and Price (1986), Water Resources Research 22(13):1891-1899 | `compute_vapour_conductance` forest-floor resistance curve | exact article/equation was not acquired; the source itself warns that the relation came from a dense Douglas-fir plot with sandy-loam soil and that transfer is at the user's risk | discovery lead only; domain transfer and the curve remain unadmitted under `AUTH-RHEC-005` |
| `CIT-031` | Deardorff (1978); Kuusinen (2012); Bras (1990), Eq. 3.19; Mahat (2011); Hedstrom and Pomeroy (1998); Storck (2002); Andreadis (2009); Price and Dunne (1976); Murray (1967); Lundberg (1994) | `compute_snow_stored` plus semantic callees `compute_potential_snow_interception` and `compute_snow_sublimation` | transitive source comments were inventoried, but the exact primary equations, observation domains, and coefficient provenance were not acquired or adjudicated | grouped `DEFER` discovery row only; no canopy-snow constitutive authority is claimed and `AUTH-RHEC-012` remains the future owner |
| `CIT-032` | Dickinson et al. (1998), Journal of Climate; Landsberg and Waring (1997) | alternate potential-N-uptake/allocation branches in `compute_potential_N_uptake_Dickenson`, `compute_potential_N_uptake_Waring`, and `compute_potential_N_uptake_combined` | source comments name only family-level leads and no exact equation/table/domain was acquired in this audit | grouped `DEFER` discovery row; nutrient/allocation branches remain severed under `SRC-021`, while any selected persistent carbon-state transition remains blocked by `AUTH-RHEC-011` |

No citation lead remains `NOT_YET_VERIFIED`: each is either tied to a read
primary locator or explicitly classified as an authority gap. The incomplete
leads above are terminally classified as gaps/deferred discovery rather than
silently used. No broad source request is necessary to finish this precursor
because exact required authorities are carried as successor blockers.
