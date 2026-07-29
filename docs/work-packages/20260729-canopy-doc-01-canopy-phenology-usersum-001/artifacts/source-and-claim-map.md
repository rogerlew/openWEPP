# Source and Claim Map

Status: `complete`

Evidence mode: `Static`

| Claim family | Public evidence | Canonical repository authority | Evidence class | Narrative home and boundary |
| --- | --- | --- | --- | --- |
| Cropland-encoded forest is a compatibility representation and cannot directly express continuous native seasonal structure | Flanagan & Nearing (1995); no claim that the historical input was invalid | ADR-0034; roadmap rationale; management YAML contract | Published model documentation + Static decision | “Why Forest Needs Its Own Seasonal State”; distinguishes compatibility from native capability |
| Temperature, VPD, and photoperiod form multiplicative GSI indicators with a 21-day mean | Jolly et al. (2005); Allen et al. (1998) for daylight | `SC-PLANT-001` CP-GSI01 | Published primary source + Static contract | “From Weather to Leaf-On and Leaf-Off”; no claim that GSI is a complete ecosystem model |
| Evergreen fraction and GSI determine foliage and LAI; persistent cover/biomass remain separate | Flanagan & Nearing (1995) for inherited WEPP canopy/height relations | `SC-PLANT-001` CP-GSI02; management schema | Published model documentation + Static contract | “Turning Foliar Activity into a Canopy” |
| Daily foliage change closes allocation/litter mass and same-day litter reaches residue before depth/frost | WEPP Chapters 8–9 for plant/residue coupling | `SC-PLANT-001` INV-PLANT-035/036; `SC-RESIDUE-001` INV-RESIDUE-019/021 | Static contract backed by prior Ran direct-consumer evidence | Canopy and consumer sections; no quantitative assurance result copied |
| One post-phenology canopy state reaches interception, ET, snow, routing, erosion, residue, and frost | WEPP Chapters 5, 8, 9, and 11 | `SC-PLANT-001` INV-PLANT-036/038; `SC-RESIDUE-001` | Static contract backed by prior Ran integration evidence | “How the Seasonal State Reaches Water and Sediment”; responses remain causal possibilities, not application verdicts |
| Exact field names, units, domains, and lack of hidden GSI defaults | None needed beyond software authority | management YAML contract; schema `PlantForest*` types; runtime plant/residue contracts | Static implementation/contract | Coefficient guide; capability is explicitly configured |
| Thirty-seven retained timing members are partially identifiable and support-boundary limited | Reader-facing quantitative table deferred to assurance | CAL-04B accepted ensemble and identifiability artifact | Ran retained evidence | Coefficient guide and calibration sequence; scoped to Hubbard Brook and never called typical |
| Mature LAI 3.5–8.0 is a source observation interval, not a physiological bound | Hubbard Brook admitted observation lineage | CAL-03 observation ledger; CAL-04B later-stage results | Retained observation + Ran operator | `growth.xmxlai` row |
| A terminal forest-floor stock cannot identify litter source and decay separately | General first-order stock/flow reasoning | CAL-05 exact five-pair equifinality ridge | Ran retained evidence | Calibration sequence; no preferred rate or source claimed |
| Surface/root pools use first-order environmentally modified decay; residue mass drives exponential cover and a separately derived depth boundary | Flanagan & Nearing (1995) | `SC-RESIDUE-001` INV-RESIDUE-017/019/020; native initial-seed and direct-production projection | Published model documentation + Static implementation/contract | Litter/decomposition primer and coefficient guide |
| Configured-zero `oratea` with recurring native litter invokes a narrow 0.5 yr^-1 fallback | Olson (1963) and Qualls (2016) support the first-order/turnover and forest-floor rate class, not the exact constant as a universal value | `SC-RESIDUE-001` `FOREST_LITTER_FALLBACK_DECAY_RATE`; frost residue-cover implementation review correction | Published primary sources + Static contract + prior Ran implementation | Litter/decomposition section; explicitly not calibrated, typical, or recommended |
| Needle/fine-wood observations require tissue, dry-mass, temporal, and spatial custody | Keane (2008a, 2008b) | litter-source authority admission matrix; plant/residue external-boundary addenda | Published primary source + Static contract | Litter/decomposition section |
| Branch turnover is not identical to ground deposition and needs branch/crown/stand state | Lim et al. (2024) | litter-source authority law/operand lineage | Published primary source + Static contract | Litter/decomposition section; predictive law remains unavailable |
| Digital camera greenness is not identical to leaf mass/LAI and depends on observation semantics | Donnelly et al. (2022); Keenan et al. (2014) | CAL-07E source register and synthesis | Published primary sources | Calibration sequence |
| Tropical dry-forest phenology can involve water and species mechanisms outside GSI | Chapotin et al. (2006); Méndez-Alonzo et al. (2013); Rivera et al. (2002) | CAL-07E claim-evidence matrix | Published primary sources | GSI limitation paragraph; analogs do not set Bezà parameters |
| Harvard transfer is unsupported and the assessed tropical dry-forest contradiction triggered stop-loss | Quantitative results deferred to assurance | CAL-04B holdout; CAL-07F final disposition | Ran retained evidence + decision | Calibration sequence and interpretation; no claim that northern calibration is invalid |

## Reference audit

All reader-facing citations resolve to published sources. Work packages,
contracts, schema files, and ADRs appear only in this internal map and are not
linked from `usersum`. DOI/title identities were checked against the admitted
source ledgers. The public narrative contains no detailed evaluation table,
reproduction command, internal verdict vocabulary, or repository-only link.
