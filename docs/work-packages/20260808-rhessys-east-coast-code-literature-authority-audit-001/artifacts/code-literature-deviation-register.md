# Code/Literature Deviation Register

Status: `audit-complete`

Evidence mode: `Static + Ran`

| ID | Deviation | Evidence | Severity | Terminal disposition |
| --- | --- | --- | --- | --- |
| `DEV-001` | Five GIS keys are not the parser keys; profile SLA, all-sided LAI ratio, both VPD thresholds, and mortality are silently ignored. | exact CSV/parser string comparison | critical | `BLOCK_SUCCESSOR` for selected profiles; correct in strict loader |
| `DEV-002` | Fifty-three parser keys are absent from every GIS profile and receive hidden defaults. | 119 unique parser keys minus 71 CSV rows, accounting for five mismatches | critical | `BLOCK_SUCCESSOR`; make selected dependencies explicit |
| `DEV-003` | `livewood_cn` is read, then overwritten with `froot_cn`. | `construct_stratum_defaults` | high | `REJECT` overwrite |
| `DEV-004` | Conductance computes Tavg response and profile parameters but omits it from the executed product. | `compute_vascular_stratum_conductance` | high | `REJECT`; select literature-backed branch |
| `DEV-005` | CO2 conductance law has placeholder citation text. | `leaf_conductance_CO2_curve` | high | `REJECT` |
| `DEV-006` | Tmin thresholds are described as arbitrary and lack exact cited authority. | `leaf_conductance_tmin_curve` | high | `REJECT` |
| `DEV-007` | LWP `9999.0` disables water stress; aerodynamic `-999.9` is a sentinel. | conductance/default parser | high | typed rejection, never port sentinel semantics |
| `DEV-008` | Canopy conductance is floored at `1e-7 m s^-1`, preventing exact zero. | vascular conductance | high | `REJECT` unless later contract authority supplies a threshold |
| `DEV-009` | Negative stores and fluxes are frequently clamped/reset after computation. | canopy daily/interception paths | high | typed failure or authorized exact-zero branch required |
| `DEV-010` | Farquhar path hardcodes C3, including grass/crop profiles. | `compute_farq_psn` | critical | `BLOCK_SUCCESSOR`; selected lifeform route must be explicit |
| `DEV-011` | Fixed ten-pass growth-respiration feedback has no convergence criterion or failure. | `compute_farq_psn` | high | `RE_DERIVE` with contract iteration or remove with authority |
| `DEV-012` | `Jmax=2.1 Vmax` replaces a Wullschleger relationship but code attribution does not support the executed constant. | Farquhar comments/primary locators | high | `RE_DERIVE` |
| `DEV-013` | Wet-canopy Penman-Monteith has no sparse/fully ventilated regime guard. | code vs Pereira et al. | high | `BLOCK_SUCCESSOR` |
| `DEV-014` | Root demand is only saturated/unsaturated and directly mutates hydrology; no layer-resolved request/authorization ledger exists. | canopy/patch daily path | critical | `RE_DERIVE` under Stage A/B/C |
| `DEV-015` | Static profile phenology dates and species values have no cell-level primary locators. | CSV header/profile matrix | high | `BLOCK_SUCCESSOR` for selected profiles |
| `DEV-016` | The broad CSV citation bundle repeats without field-to-source mapping; several columns use personal-name-only notes. | CSV metadata rows | high | every cell remains `PARAMETER_DATA` |
| `DEV-017` | Source exits the process or warns-and-canonicalizes instead of returning typed scientific failures. | parser, photosynthesis, litter-fraction guards | high | direct port `REJECT` |
| `DEV-018` | Canopy snow is intertwined with radiation/conductance while canonical canopy-snow constitutive authority is absent. | source inventory and `SC-VEGETATION-001` | medium | `DEFER` behind joint snow/LSE authority |
| `DEV-019` | `compute_surface_heat_flux` divides by soil depth in its positive rain-capacity branch but multiplies by depth in the alternate branch, producing incompatible dimensions. | exact function reconstruction and declared `J m^-3 K^-1`, m, K inputs | critical | `REJECT`; successor must use an independently authorized areal-energy law |
| `DEV-020` | Canopy longwave collapses all strata to one air-temperature slab and deletes negative net longwave during warm periods. | `compute_Lstar_canopy` homogeneous-slab comment and three sign clamps | critical | `REJECT`; `BLOCK_SUCCESSOR` for available energy |
| `DEV-021` | The negative daytime available-energy guard mistakenly zeros the nighttime term. | `canopy_stratum_daily_F` day/night `rnet_evap` guards | critical | `REJECT`; independent branch vectors required |
| `DEV-022` | Worldfile generators derive initial C/N pools from fixed row indices, hardcode 5% leaf allocation and root depths, and disagree on deadwood C:N. | `g2w.R` and `g2w_cf_RHESSysEC.R` initialization blocks | critical | `BLOCK_SUCCESSOR`; define a typed, dated, authority-backed initializer |
| `DEV-023` | GIS initialization uses the profile SLA cell to derive leaf carbon, but the generated key mismatch makes the C parser use a different hidden SLA default at runtime. | generator row 48 plus parser exact-key comparison | critical | `BLOCK_SUCCESSOR`; one exact admitted SLA identity must govern initialization and runtime |
| `DEV-024` | Penman-Monteith omits `EPS=0.6219` from the psychrometric constant although the same repository's snow sublimation uses `/0.622`. | exact `penman_monteith.c` and `compute_snow_sublimation.c` comparison | critical | `REJECT`; independently re-derive and vector-test PM |
| `DEV-025` | GIS generator `default` paths fetch mutable raw GitHub `master` parameter files rather than pinned content. | both accepted g2w entry scripts | critical | `REJECT`; require local digest/commit-bound source identity |
| `DEV-026` | Canopy runtime ignores parsed absorptance and transmittance, and diffuse routines ignore their extinction argument. | parser/use-site and function-body trace | critical | `REJECT`; contract exact component operator and dead-field policy |
| `DEV-027` | Nine profile optical triples fail unit closure; selected chestnut oak broadband values sum to 1.03. | exact 32-profile CSV reconstruction | high | `BLOCK_SUCCESSOR`; raw preservation plus validation/error, never normalization without authority |
| `DEV-028` | Dynamic GSI is controlled entirely by parser-only defaults, including contradictory soil-potential units. | GSI source/parser/53-default appendix | high | `REJECT`; selected phenology remains blocked |
| `DEV-029` | Sunlit/shaded LAI predictor/corrector has no iteration bound or nonfinite/convergence failure. | `update_phenology` exact loop | critical | `REJECT`; contract a bounded solver or closed form |
| `DEV-030` | Root-depth logic cites Arora-Boer but exposes only a single dynamic depth and hidden parameters. | `update_rooting_depth` and parser | high | `BLOCK_SUCCESSOR`; layer-resolved authority still required |

Every deviation is either a blocker, an explicit rejection, or a severable
later boundary. None is converted into production authority.
