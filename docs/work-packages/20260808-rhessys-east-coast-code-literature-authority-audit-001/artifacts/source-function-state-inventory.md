# Source Function And State Inventory

Status: `audit-complete`

Evidence mode: `Ran + Static`

All paths below are relative to pinned RHESSysEastCoast commit
`375c75b1cd2202217651dff43aa113d80b9c1118` unless identified as GIS2RHESSys.
Coordinates are symbols because line numbers are source-identity-local.

## Transitive Population

| ID | Entry/symbol population | Caller/cadence | Scientific state and mutation | Owner/disposition |
| --- | --- | --- | --- | --- |
| `SRC-001` | GIS2RHESSys `vegCollection.csv`; `libraries/g2w_cf_RHESSysEC.R` and `_soil_fullextraction.R` vegetation read/write loops | project generation | Selects a profile by row-1 ID and writes all 71 `value, key` pairs without schema/domain validation; command-line `default` can fetch an unpinned raw GitHub `master` file | format provenance only; unpinned fallback `REJECT`; strict local input `ADAPT` |
| `SRC-002` | `init/construct_stratum_defaults.c::construct_stratum_defaults` plus `parse_veg_type`, `parse_phenology_type`, `parse_dyn_flag`, `parse_alloc_flag` | initialization | Reads 119 named keys, silently defaults absent keys, overwrites `livewood_cn`, derives litter fractions, and sometimes clamps invalid ratios | strict loader owner; source behavior `REJECT` |
| `SRC-003` | `init/construct_canopy_strata.c::construct_canopy_strata` | initialization | Creates stratum pools, LAI/PAI, height, root depth, phenology windows, and hidden initialized accumulators; zeroes pools by lifeform | vegetation topology/state; `RE_DERIVE` |
| `SRC-004` | `util/sort_patch_layers.c`; `cycle/patch_daily_F.c` layer loop and `canopy_stratum_daily_F` calls | daily, top-down by layer | Mutates patch radiation after each stratum and cover-weights returned fluxes; explicit null-cover path | vegetation/orchestrator boundary; `ADAPT` |
| `SRC-005` | `cycle/canopy_stratum_daily_I.c` | daily initialization | Computes predawn LWP, phenology/LAI initialization inputs, and resets daily state | vegetation/hydrology observation boundary; `RE_DERIVE` |
| `SRC-006` | `cycle/canopy_stratum_daily_F.c::canopy_stratum_daily_F` | daily final pass | Central mutator for radiation, stores, resistances, ET, conductance, photosynthesis, respiration, N demand, and patch handoff | must be decomposed into typed stages; `REJECT` as a direct port |
| `SRC-007` | `cycle/canopy_stratum_hourly.c::canopy_stratum_hourly` and `hydro/compute_hourly_rain_stored.c` | hourly precipitation | Accumulates intercepted liquid/throughfall and deposition stores | liquid interception candidate; `RE_DERIVE` |
| `SRC-008` | `cycle/canopy_stratum_growth.c`, `cn/allocate_daily_growth.c`, `cn/allocate_annual_growth.c` | daily/annual growth | Moves C/N among pools from photosynthesis, respiration, storage, and allocation flags | minimum selected C state transition `BLOCK_SUCCESSOR`; generic N/alternate allocation branches may be deferred only at an immutable-N boundary |
| `SRC-009` | `rad/compute_direct_radiative_fluxes.c`; `compute_diffuse_radiative_fluxes.c`; `compute_diffuse_radiative_PAR_fluxes.c`; `compute_radiative_fluxes.c` | per stratum/day | Mutates incident direct/diffuse broadband and PAR; runtime consumes reflectance but not parsed absorptance/transmittance; diffuse routines accept but ignore `extinction_coef` | vegetation/LSE component boundary; source behavior `REJECT`; `BLOCK_SUCCESSOR` |
| `SRC-010` | `rad/compute_Lstar_canopy.c::compute_Lstar_canopy` | per stratum/day | Treats the canopy as one homogeneous air-temperature slab, branches among snow/pond/soil lower emitters, mutates patch downwelling longwave, and clips negative canopy net longwave to zero on warm day/night periods | LSE owner; source behavior `REJECT`; `BLOCK_SUCCESSOR` |
| `SRC-011` | `hydro/compute_potential_rain_interception.c`; `compute_rain_stored.c`; hourly variants | hourly/daily | Limits rain storage by all-sided PAI capacity, produces throughfall, and depletes storage with evaporation | vegetation water ledger; `RE_DERIVE` |
| `SRC-012` | `hydro/compute_snow_stored.c`; semantic callees `compute_potential_snow_interception.c` and `compute_snow_sublimation.c` | daily | Mutates canopy snow store/sublimation and snow-adjusted radiation state; the callees supply capacity/efficiency and sublimation/aerodynamic relations | canopy-snow ownership and complete source chain acknowledged but constitutive law excluded; citations grouped in `CIT-031`; `DEFER` |
| `SRC-013` | `hydro/compute_ra_overstory.c`, `compute_ra_understory.c`, `compute_ra_surface.c`, `compute_vapour_conductance.c` | per stratum/day | Derives aerodynamic resistance/conductance from canopy height, wind, attenuation, roughness, and stability-related inputs | LSE/vegetation aerodynamic boundary; `BLOCK_SUCCESSOR` |
| `SRC-014` | `hydro/compute_nonvascular_stratum_conductance.c` | per stratum/day | Converts relative rain storage after assumed 1 mm drying to surface conductance; its reciprocal feeds wet-surface PM | selected wet-surface chain `BLOCK_SUCCESSOR`; defer only excluded nonvascular lifeforms |
| `SRC-015` | `hydro/compute_vascular_stratum_conductance.c`; six `cn/leaf_conductance_*_curve.c` callees | sunlit and shade, daily | Multiplies APAR/LWP/CO2/Tmin/VPD factors, computes but omits Tavg, adds cuticular conductance, scales by LAI, and floors result | vegetation conductance; `BLOCK_SUCCESSOR` |
| `SRC-016` | `hydro/penman_monteith.c::penman_monteith` | multiple wet/dry, day/night, sun/shade calls | Converts net radiation, VPD, surface resistance, and aerodynamic resistance to energy, mass, or depth rate, but computes psychrometric constant without the defined `EPS=0.6219` divisor | ET/LSE boundary; executed algebra `REJECT`; chain `BLOCK_SUCCESSOR` |
| `SRC-017` | `cn/compute_farq_psn.c::compute_farq_psn` | sunlit/shade daily | Uses conductance to solve C3 Rubisco- and electron-transport-limited assimilation; hardcodes C3 and iterates growth respiration ten times | vegetation carbon; `BLOCK_SUCCESSOR` |
| `SRC-018` | `cn/update_phenology.c`; `compute_annual_litfall.c`; `compute_leaf_litfall.c`; `compute_froot_litfall.c`; `compute_annual_turnover.c`; `compute_deadleaf_turnover.c` | daily/annual | Moves leaf/root pools and recomputes LAI/PAI across static/dynamic windows | vegetation/residue boundary; `BLOCK_SUCCESSOR` for selected profiles |
| `SRC-019` | `cn/compute_maint_resp.c`; `compute_growth_resp.c` | daily | Temperature/Q10-scaled maintenance and fixed-fraction growth respiration by pool | vegetation carbon; `BLOCK_SUCCESSOR` |
| `SRC-020` | `cn/update_rooting_depth.c`; `hydro/compute_lwp_predawn.c`; `compute_soil_water_potential.c` | initialization/daily | Derives one root depth and predawn water potential from patch soil state; no layer-resolved demand vector | hydrology observation/vegetation request boundary; `BLOCK_SUCCESSOR` |
| `SRC-021` | `cn/compute_potential_N_uptake*.c`, including Dickenson/Waring/combined allocation branches; `resolve_sminn_competition.c`; `update_N_stratum_daily.c` | daily | Competes for mineral N and updates vegetation N pools; alternate branches name Dickinson et al. (1998) and Landsberg and Waring (1997) family leads | nutrient extension; citations grouped in `CIT-032`; `DEFER` with severability from initial water/radiation slice |
| `SRC-022` | `cn/update_mortality.c`, `update_branch_mortality.c` | event/daily | Transfers live/dead pools using mortality and branch turnover parameters | later disturbance/material-transfer owner; `DEFER` |
| `SRC-023` | `cycle/patch_daily_F.c` root-zone demand/arbitration blocks | daily after strata | Aggregates stratum sat/unsat transpiration requests and mutates patch hydrologic stores | hydrology is authoritative owner; direct source coupling `REJECT` |
| `SRC-024` | canopy-stratum output functions | daily/monthly/yearly | Publishes source diagnostics and state but does not prove scientific authority | comparator/diagnostic only; `DEFER` |
| `SRC-025` | `rad/compute_surface_heat_flux.c::compute_surface_heat_flux`; callers in `patch_daily_F` and `canopy_stratum_daily_F` | patch and zero-height-stratum daily energy preparation | Estimates profile-mean temperature, moisture-interpolated volumetric heat capacity, and a daily heat term; the wet-capacity branch divides by `deltaz` while the dry branch multiplies by it | LSE owner; dimensionally inconsistent source behavior `REJECT`; `BLOCK_SUCCESSOR` |
| `SRC-026` | `canopy_stratum_daily_F` available-energy, day/night evaporation, and sun/shade transpiration blocks | per stratum/day after longwave, heat, and conductance | Forms PM energy operands, clamps negative terms, and contains a branch that clears night energy when day energy is negative; later allocates used energy back to shortwave, longwave, and heat-flux stores | vegetation/LSE/ET custody boundary; `RE_DERIVE`; `BLOCK_SUCCESSOR` |
| `SRC-027` | GIS2RHESSys `libraries/g2w_cf_RHESSysEC.R` and `g2w_cf_RHESSysEC_soil_fullextraction.R` worldfile initialization blocks | project generation/initial state | Derives leaf, live/dead stem/root, fine-root C and tissue N from LAI plus fixed CSV row positions; canonicalizes nonfinite dead fractions, hardcodes deadwood C:N `333.33`, leaf-allocation C at 5%, root depths, and zero state | initialization provenance only; formulas/values `REJECT`; `BLOCK_SUCCESSOR` |
| `SRC-028` | `cn/compute_growingseason_index.c`; dynamic branches in `update_phenology.c` | daily plus future-window lookahead | Combines temperature, VPD, daylength, and soil-potential indices and applies hidden thresholds/windows to leaf-on/off decisions | selected phenology `BLOCK_SUCCESSOR`; source defaults `REJECT` |
| `SRC-029` | `update_phenology.c` sunlit/shade LAI predictor/corrector loop | daily after pool turnover | Iterates projected LAI and sunlit fraction until a relative threshold with no iteration cap/nonfinite guard | vegetation state/numerics; `REJECT`; `BLOCK_SUCCESSOR` |
| `SRC-030` | `cn/update_rooting_depth.c::update_rooting_depth` | daily when growth/root state changes | Applies an Arora-Boer-cited biomass/root-distribution expression with direction and max-depth controls | root-profile authority `BLOCK_SUCCESSOR` |
| `SRC-031` | full branch bodies of `compute_ra_overstory`, `compute_ra_understory`, `compute_ra_surface`; `compute_nonvascular_stratum_conductance` | daily by layer/lifeform/store state | Selects distinct aerodynamic regimes and an empirical storage-to-conductance curve with forward 1 mm drying, zero-capacity branch, and clamps | aerodynamic/nonvascular laws unadmitted; `BLOCK_SUCCESSOR` when selected, otherwise named deferral |

## Dependency Direction

The audited source direction is atmospheric/soil observation -> Jarvis
conductance -> Penman-Monteith transpiration and Farquhar assimilation.
Assimilation does not solve stomatal conductance. Carbon and phenology feed
future LAI, N, stores, and root state, so coupling exists across state and time,
not as a same-step bidirectional stomatal solve. Hydrologic withdrawal occurs
outside the stratum after demand aggregation and directly mutates patch stores;
openWEPP must replace that ownership with the contract's Stage A/B/C request,
arbitration, and finalization protocol.

## Boundary And Failure Audit

Observed source failure behavior includes process exit on selected parser and
photosynthesis errors, warning-and-clamp behavior, negative-store reset,
`9999.0` LWP bypass, `-999.9` aerodynamic sentinel, hidden parser defaults,
fixed conductance floors, and commented/disabled branches. None is adopted.
The successor must use typed failures and explicit, contract-authorized
degenerate branches.

Worldfile initialization adds a second compatibility break: it computes initial
leaf carbon from the profile SLA cell before serialization, while the pinned C
parser ignores that profile SLA because its key does not match. Consequently
initial pools and the runtime LAI conversion can use different SLA values even
for the same generated profile.

## Deferred-Surface Custody

- Canopy snow (`SRC-012`) is owned by a future joint vegetation/snow/LSE
  contract package and cannot execute in a snow-present case before that gate.
- Nonvascular conductance (`SRC-014`) is owned by a future nonvascular-surface
  package; the first tree-profile selector rejects nonvascular lifeforms.
- N uptake/competition (`SRC-021`) is owned by a future
  `VEGETATION-CARBON-NUTRIENT-MATERIAL` package. It is severable only if the
  first slice contracts immutable admitted leaf N and excludes N-limited
  allocation and every N/material mutation.
- Mortality/material transfer (`SRC-022`) is owned by that same later package;
  any background turnover needed to advance selected persistent C/LAI state is
  instead part of `AUTH-RHEC-011` and remains successor-blocking.
- Diagnostics (`SRC-024`) have no feedback consumer and stay comparator-only.

Crossing any exclusion, accepting another profile/lifeform, or advancing a
state that depends on a deferred process is the trigger for prospective
reclassification and contract review. Package size is not the severability
argument.
