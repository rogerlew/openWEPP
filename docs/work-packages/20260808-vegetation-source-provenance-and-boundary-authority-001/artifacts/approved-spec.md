# Sanitized Vegetation Source Analysis

Status: quarantined source-aware artifact

Request: `VEG-SOURCE-FIREWALL-001`

## 1. Identity and scope

- [CODE-OBSERVED] The inspected source identity is the local read-only RHESSys
  checkout at commit `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- [CODE-OBSERVED] Inspection was bounded to the ten requested vegetation,
  atmosphere, water, carbon, nitrogen, litter, and snow families below.
- [INFERENCE] RHESSys behavior is useful as comparison evidence and as evidence
  that typed ownership boundaries are needed; it is not an openWEPP production
  target.
- [INFERENCE] This artifact defines no numerical method, parameter value,
  parameter bound, default, calibration, validation result, or runtime
  eligibility claim.
- [INFERENCE] Audit coordinates identify where an independent reviewer can
  confirm a semantic observation. They do not authorize translation.

## 2. Sanitization declaration

- [CODE-OBSERVED] The artifact contains no source excerpt, source comment,
  source-local variable name, statement sequence, branch-by-branch account,
  reversible pseudocode, patch, or source-only equation or numerical constant.
- [CODE-OBSERVED] Source-specific function names occur only in fields explicitly
  titled **Audit coordinates**, where they are locators rather than semantic
  vocabulary.
- [INFERENCE] Neutral names below were selected for openWEPP boundary design and
  must not be treated as evidence of RHESSys naming or internal structure.
- [CODE-OBSERVED] No repository-level license file or equivalent grant was found
  in the frozen checkout.
- [INFERENCE] The checkout therefore has no adequate repository-level license
  grant for direct or closely translated incorporation. Public availability or
  community practice does not change that limit. This is a repository-evidence
  disposition, not a broader legal conclusion.

## 3. Process/state inventory

### Family 1: native canopy strata and rooting geometry

**Neutral semantic name:** vertically ordered vegetation stratum set.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/init/construct_canopy_strata.c` — `construct_canopy_strata`;
  `rhessys/util/sort_patch_layers.c` — `sort_patch_layers`;
  `rhessys/cn/update_phenology.c` — `update_phenology`;
  `rhessys/cn/update_rooting_depth.c` — `update_rooting_depth`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Per-stratum identity, vegetation parameter-set
  reference, fractional horizontal cover, within-crown gap fraction, initial
  root depth, carbon and nitrogen pools, stem density, and configuration mode;
  soil effective depth constrains dynamic rooting.
- **Outputs and units:** [CODE-OBSERVED] Stable stratum identity; height and root
  depth in metres; horizontal cover and gap fractions as dimensionless
  fractions; projected and all-sided leaf and plant area as area per ground
  area; above- and below-ground carbon and nitrogen states as areal mass.
- **Cadence and mutation:** [CODE-OBSERVED] Configuration initializes each
  stratum. Daily phenology and growth can change leaf area, woody area, height,
  and root depth. Vegetation is the proposed owner of stratum identity and
  mutable plant geometry; native management owns configuration and initial
  state.
- **Ordering and consumers:** [CODE-OBSERVED] Strata are grouped at equal
  heights and vertical groups are ordered from tallest to shortest before
  radiation and precipitation traversal. Land-surface energy, soil hydrology,
  snow/frost, residue/biogeochemistry, and the hillslope orchestrator consume
  the resulting geometry.
- **Overlap semantics:** [CODE-OBSERVED] Horizontal cover closes independently
  within each equal-height group, while different height groups may overlap
  vertically. Uncovered fraction is retained per height group, so cross-height
  cover is not summed as mutually exclusive land area.
- **Rooting semantics:** [CODE-OBSERVED] The inspected organization exposes a
  stratum root-depth envelope and an aggregate patch root-depth envelope, but
  not an explicit soil-layer root-fraction vector suitable for layer-resolved
  openWEPP withdrawal requests.
- **Literature present:** [CODE-OBSERVED] Chen et al. (1999), *Ecological
  Modelling* 124:99–119 is named for sunlit/shaded leaf-area partition.
  No explicit literature citation was found for equal-height grouping,
  cover closure, height allometry, or root-depth derivation.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Equal-height cover above
  full closure is repaired by perturbing plant state and regrouping rather than
  rejected as invalid input. Zero vegetation, zero leaf area, zero height, and
  zero root depth are admitted. [INFERENCE] State perturbation to repair cover
  is non-promotable; openWEPP should validate cover and ordering without
  altering biomass.
- **Conservation relevance:** [INFERENCE] Cover-weighted extensive state must
  not be confused with intrinsic per-covered-area state, and root geometry
  changes require an explicit soil-water remapping ledger if storage
  partitions depend on that geometry.
- **Licensing disposition:** [INFERENCE] `direct translation prohibited` for
  source organization and algorithms; `eligible for literature-based
  independent derivation` only for the separately cited sunlit/shaded concept;
  otherwise `inspection/comparison only`.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for native
  strata, independent within-layer cover closure, vertical overlap, ordering,
  and configuration/runtime separation; `independently re-derive` leaf/woody
  area, height, and rooting behavior from admitted authority. Owners are native
  management and vegetation; the hillslope orchestrator consumes ordering.

### Family 2: canopy-to-surface radiation ledger

**Neutral semantic name:** vertically cascading spectral and thermal radiation
ledger.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`;
  `rhessys/cycle/canopy_stratum_daily_F.c` — `canopy_stratum_daily_F`;
  `rhessys/rad/compute_direct_radiative_fluxes.c` —
  `compute_direct_radiative_fluxes`;
  `rhessys/rad/compute_diffuse_radiative_fluxes.c` —
  `compute_diffuse_radiative_fluxes`;
  `rhessys/rad/compute_diffuse_radiative_PAR_fluxes.c` —
  `compute_diffuse_radiative_PAR_fluxes`;
  `rhessys/rad/compute_Lstar_canopy.c` — `compute_Lstar_canopy`;
  `rhessys/cycle/surface_daily_F.c` — `surface_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Direct and diffuse shortwave, photosynthetically
  active radiation, downwelling longwave, solar geometry, stratum area and gap
  measures, optical properties, canopy and surface thermal states, snow
  presence, litter cover, ponded-water presence, and soil optical properties.
- **Outputs and units:** [CODE-OBSERVED] Absorbed, reflected, transmitted, and
  remaining direct/diffuse radiation as daily energy per ground area; thermal
  exchange as daily energy per ground area; photosynthetically active receipt
  as daily radiant or photon-domain input depending on consumer boundary.
- **Cadence and mutation:** [CODE-OBSERVED] Daily traversal mutates the remaining
  downwelling streams after each height group and accumulates absorbed canopy
  energy. A later surface step distinguishes litter and soil shortwave receipt
  and selects snow, ponded water, or soil for thermal exchange. Land-surface
  energy is the proposed ledger owner; vegetation, snow/frost, and
  residue/biogeochemistry own their constitutive responses.
- **Ordering and consumers:** [CODE-OBSERVED] Taller vegetation receives
  radiation before shorter vegetation. Remaining streams pass to the ground
  snowpack or surface, then litter and soil partitioning. Vegetation consumes
  stratum receipt; snow/frost consumes ground-snow receipt; residue/
  biogeochemistry consumes litter receipt; soil hydrology and land-surface
  energy consume ground receipt and latent-energy debit.
- **Literature present:** [CODE-OBSERVED] Jarvis and Leverenz, drawing on Norman
  (1981), is named for diffuse attenuation; Chen et al. (1997) is named for
  direct attenuation; Brubaker (1996), as referenced through Dingman, is named
  for a snow-surface temperature estimate. No complete citation was found for
  the inspected longwave canopy treatment.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] The diffuse treatment notes
  omitted between-layer interaction; the thermal canopy is treated as a
  homogeneous slab rather than stratum-resolved; litter and soil share an
  approximated thermal term; snow, water, litter, and soil are not represented
  as a single closed typed surface-radiation result.
- **Conservation relevance:** [INFERENCE] Every incoming stream requires
  mutually exclusive absorbed, reflected, and transmitted terms, and every
  latent-energy debit must name the water flux that consumed it. Canopy,
  ground snow, litter, ponded water, and soil terms must remain distinct to
  prevent omission or double counting.
- **Licensing disposition:** [INFERENCE] `eligible for literature-based
  independent derivation` only for the cited radiation concepts;
  source-specific treatment is `inspection/comparison only` and direct
  translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for ordered
  receipts and an explicit radiation ledger; `independently re-derive` all
  constitutive radiation relationships. Proposed owner is land-surface energy;
  consumers are vegetation, snow/frost, residue/biogeochemistry, soil
  hydrology, and the hillslope orchestrator.

### Family 3: liquid precipitation interception

**Neutral semantic name:** stratum liquid-water interception and release
ledger.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/hydro/compute_potential_rain_interception.c` —
  `compute_potential_rain_interception`;
  `rhessys/hydro/compute_rain_stored.c` — `compute_rain_stored`;
  `rhessys/cycle/canopy_stratum_daily_F.c` — `canopy_stratum_daily_F`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Incoming rainfall depth, prior canopy liquid
  storage, projected plant area, within-crown gap fraction, storage-capacity
  parameter, atmospheric demand, radiation, aerodynamic conductance, and rain
  duration.
- **Outputs and units:** [CODE-OBSERVED] End-of-step canopy liquid storage,
  evaporation, and water passed below in metres of water per day or metres of
  stored water; remaining evaporative demand is also updated.
- **Cadence and mutation:** [CODE-OBSERVED] Daily canopy processing owns the
  stratum liquid store and evaporation debit; precipitation passed below is a
  shared transfer to the next stratum or surface. Vegetation is the proposed
  canopy-store owner and land-surface energy supplies the admissible energy
  budget.
- **Ordering and consumers:** [CODE-OBSERVED] Stored water and current input are
  exposed to evaporation, storage is capacity-limited, and non-stored water
  continues downward through the ordered canopy. Soil hydrology, snow/frost,
  and residue/biogeochemistry consume the final ground-reaching water as
  appropriate to phase and surface state.
- **Stemflow and drainage:** [CODE-OBSERVED] No distinct stemflow flux was found
  in the inspected canopy interception path. Overflow and released liquid are
  represented through the common downward-water handoff rather than separately
  typed canopy drainage and stemflow ledgers.
- **Literature present:** [CODE-OBSERVED] Helvey (1964) and Ogee and Brunet
  (2002), *Journal of Hydrology*, are named near interception parameter
  initialization. No explicit citation was found for the inspected storage
  sequencing.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Zero area, zero capacity,
  zero rain, and already-full storage collapse to no new storage; nonnegative
  clipping is used. [INFERENCE] Separate drip, drainage, and stemflow contracts
  remain authority gaps and must not be invented from the combined release.
- **Conservation relevance:** [INFERENCE] For each stratum, prior storage plus
  incident liquid must reconcile to final storage, evaporation, and all named
  downward releases on one area basis.
- **Licensing disposition:** [INFERENCE] `eligible for literature-based
  independent derivation` only where the cited works are independently
  admitted; otherwise `inspection/comparison only`; direct translation is
  prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for a
  vegetation-owned canopy store and typed downstream liquid handoff;
  `independently re-derive` capacity and evaporation physics; `defer` distinct
  stemflow until authority exists. Consumers are land-surface energy, soil
  hydrology, snow/frost, residue/biogeochemistry, and the hillslope
  orchestrator.

### Family 4: conductance, atmospheric demand, and water arbitration

**Neutral semantic name:** potential vegetation water response and
hydrology-returned realization.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/hydro/compute_vascular_stratum_conductance.c` —
  `compute_vascular_stratum_conductance`;
  `rhessys/hydro/compute_xylem_conductance.c` —
  `compute_xylem_conductance`;
  `rhessys/hydro/compute_ra_overstory.c` — `compute_ra_overstory`;
  `rhessys/hydro/compute_ra_understory.c` — `compute_ra_understory`;
  `rhessys/hydro/penman_monteith.c` — `penman_monteith`;
  `rhessys/cycle/canopy_stratum_daily_F.c` — `canopy_stratum_daily_F`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Absorbed light, leaf area, atmospheric carbon
  dioxide, minimum and daytime air temperature, vapour-pressure deficit,
  predawn plant-water status, canopy height, wind, pressure, available energy,
  surface and aerodynamic resistance, root depth, and soil-water state.
- **Outputs and units:** [CODE-OBSERVED] Canopy conductance in velocity units;
  aerodynamic resistance in time per length; potential and stressed
  transpiration in water depth per day; demand split between two coarse soil-
  water domains; diagnostic environmental stress multipliers.
- **Cadence and mutation:** [CODE-OBSERVED] Daily vegetation evaluation computes
  conductance, atmospheric demand, and provisional transpiration. Patch
  hydrology aggregates competing withdrawals with surface evaporation,
  mutates soil storage, and returns a realized fraction. Vegetation then
  finalizes transpiration and growth-related state. [INFERENCE] This directly
  supports a Stage A potential response, Stage B hydrologic arbitration, and
  Stage C vegetation finalization boundary, but not the inspected formulas.
- **Root request resolution:** [CODE-OBSERVED] The source partitions demand only
  between coarse saturated and unsaturated domains using root-depth and water-
  table geometry. It does not expose a general soil-layer request vector or
  per-layer reason-coded allocation.
- **Realized-uptake feedback:** [CODE-OBSERVED] Supply limitation scales each
  stratum's transpiration and several coupled carbon, nutrient-demand, and
  conductance quantities before end-of-day growth. No within-day
  vegetation–hydrology iteration was found.
- **Literature present:** [CODE-OBSERVED] The conductance family names the
  Jarvis multiplicative-response concept, Running and Coughlan for temperature
  and plant-water response curves, Mackay et al. (2015), *Water Resources
  Research*, for xylem limitation and recovery, Heddeland and Lettenmaier
  (1995) for aerodynamic resistance, and the Penman–Monteith combination
  framework by name.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] A positive conductance floor
  is imposed even when response multipliers close conductance; demand and
  assimilation are clipped or uniformly rescaled in several limiting states;
  no typed reason distinguishes dry soil, frozen water, rooting exclusion,
  numerical rejection, or competing demand. [INFERENCE] Those floors and
  scaling choices are `NON-PROMOTABLE` absent independent authority.
- **Conservation relevance:** [INFERENCE] The sum of hydrology-returned layer
  withdrawals must exactly equal finalized transpiration water mass, and its
  latent-energy equivalent must be the same transfer recorded by land-surface
  energy. Vegetation must never mutate soil or frozen-water storage.
- **Licensing disposition:** [INFERENCE] `eligible for literature-based
  independent derivation` for independently admitted cited concepts;
  `inspection/comparison only` for observed coupling; direct translation is
  prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for Stage
  A/B/C and shared withdrawals; `independently re-derive` conductance and demand
  science; `reject` vegetation mutation of soil storage. Vegetation owns demand
  and finalization, soil hydrology owns allocations and storage mutation,
  land-surface energy owns latent-energy reconciliation, and the hillslope
  orchestrator owns stage order.

### Family 5: phenology, carbon gain, respiration, allocation, and turnover

**Neutral semantic name:** seasonal plant carbon-state transition system.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/cycle/canopy_stratum_daily_I.c` — `canopy_stratum_daily_I`;
  `rhessys/cn/update_phenology.c` — `update_phenology`;
  `rhessys/cn/compute_farq_psn.c` — `compute_farq_psn`;
  `rhessys/cn/compute_maint_resp.c` — `compute_maint_resp`;
  `rhessys/cycle/canopy_stratum_growth.c` — `canopy_stratum_growth`;
  `rhessys/cn/allocate_daily_growth.c` — `allocate_daily_growth`;
  `rhessys/cn/allocate_annual_growth.c` — `allocate_annual_growth`;
  `rhessys/cn/update_mortality.c` — `update_mortality`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Evergreen/deciduous and static/dynamic phenology
  classification; seasonal timing or climate-response indicators; light,
  temperature, carbon dioxide, conductance, tissue nitrogen, plant-water
  status, prior plant pools, allocation parameters, nutrient availability,
  and disturbance or mortality drivers.
- **Outputs and units:** [CODE-OBSERVED] Leaf-on/leaf-off progression; gross and
  net carbon gain; maintenance and growth respiration; current, stored, and
  transfer carbon and nitrogen pools for foliage, fine roots, live and dead
  wood, and coarse roots; litter and coarse-woody transfers; height, leaf area,
  woody area, and root-depth updates. Carbon and nitrogen use areal mass units;
  respiration and allocation are areal mass per day.
- **Cadence and mutation:** [CODE-OBSERVED] Daily initialization performs
  mortality, turnover, litterfall, phenology, and geometry updates; daily
  process evaluation computes respiration, assimilation, and potential nutrient
  demand; end-of-day finalization performs allocation, growth respiration, pool
  updates, and root-depth updates. Some allocation and turnover decisions occur
  at seasonal or annual transition points.
- **Ordering and consumers:** [CODE-OBSERVED] Phenology and initial plant state
  precede canopy radiation and gas exchange; hydrologic realization precedes
  final carbon allocation; litter and coarse-woody transfers cross to the
  ground dead-material owner. Vegetation owns live plant and standing-dead plant
  state until explicit transfer; residue/biogeochemistry consumes transferred
  dead material.
- **Literature present:** [CODE-OBSERVED] The inspected material names Thornton
  (1997, 1998) and BIOME-BGC lineage; Farquhar and von Caemmerer (1982), de Pury
  and Farquhar (1997), Wullschleger (1993), Woodrow and Berry (1980), and Kuehn
  and McFadden (1969) for photosynthesis; Ryan (1991) and Tjoelker et al. (2001)
  for respiration; Jolly et al. (2005) for a growing-season indicator; Chen et
  al. (1999) for sunlit/shaded partition; Nambiar et al. (1991) for live-wood
  nitrogen retranslocation; Dickenson et al. (1998), Landsberg and Waring
  (1997), and Reyes et al. (2017) for allocation-related concepts.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Multiple selectable
  allocation and phenology modes coexist; one photosynthetic pathway is forced
  in the inspected routine; negative or very small pools may be clipped; some
  fatal process errors terminate execution; empirical defaults and source-
  lineage constants are interwoven with literature-named concepts.
- **Conservation relevance:** [INFERENCE] Each carbon and nitrogen pool transfer
  requires source decrement, destination increment, atmospheric sink or source,
  and an area basis. Hydrologic downscaling of carbon gain must occur before
  allocation so unrealized transpiration cannot support realized assimilation.
- **Licensing disposition:** [INFERENCE] `eligible for literature-based
  independent derivation` only from independently consulted cited literature;
  source lineage and source-specific integration are `inspection/comparison
  only`; direct translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for the
  vegetation-owned state machine and typed dead-material transfers;
  `independently re-derive` all physiology; `defer` detailed allocation,
  mortality, and turnover laws until authority and calibration plans exist.
  Proposed owner is vegetation; consumers are residue/biogeochemistry,
  land-surface energy, soil hydrology, and the hillslope orchestrator.

### Family 6: carbon and nitrogen custody boundary

**Neutral semantic name:** live-plant elemental ledger and dead-material
handoff.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/cn/update_C_stratum_daily.c` — `update_C_stratum_daily`;
  `rhessys/cn/update_N_stratum_daily.c` — `update_N_stratum_daily`;
  `rhessys/cn/update_mortality.c` — `update_mortality`;
  `rhessys/cn/compute_leaf_litfall.c` — `compute_leaf_litfall`;
  `rhessys/cn/compute_froot_litfall.c` — `compute_froot_litfall`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Live-plant carbon and nitrogen pools, photosynthetic
  source, respiratory sinks, nutrient uptake allocation, retranslocated
  nitrogen, mortality and turnover fractions, tissue composition, and soil
  mineral-nitrogen availability.
- **Outputs and units:** [CODE-OBSERVED] Plant pool updates, atmospheric carbon
  sinks, plant nutrient uptake, above- and below-ground litter transfers,
  coarse-woody transfers, and aggregate carbon/nitrogen balance diagnostics in
  areal mass or areal mass per day.
- **Cadence and mutation:** [CODE-OBSERVED] Vegetation mutates live, standing-
  dead, storage, transfer, and retranslocation pools daily. The inspected source
  also directly mutates litter and soil-nitrogen pools from vegetation
  routines. [INFERENCE] openWEPP should replace those cross-owner mutations
  with immutable transfer proposals and owner-applied receipts.
- **Ordering and consumers:** [CODE-OBSERVED] Potential plant nitrogen demand is
  aggregated with decomposition demand, a shared availability decision occurs,
  and realized nutrient acquisition constrains growth allocation. Litter and
  coarse woody debris subsequently enter decomposition. [INFERENCE] Full soil
  biogeochemistry and nutrient routing remain outside vegetation ownership.
- **Literature present:** [CODE-OBSERVED] Thornton (1997, 1998) and BIOME-BGC
  lineage are named for several pool-transfer routines; Nambiar et al. (1991)
  is named for a nitrogen retranslocation concept. No single complete external
  authority for the integrated custody scheme was found.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Negative mobile pools may be
  clipped, and a mortality path explicitly acknowledges possible elemental
  balance loss when invalid negative pools occur. Some diagnostics report
  imbalance without typed failure. [INFERENCE] Silent clipping and unbalanced
  cross-owner mutation are non-promotable.
- **Conservation relevance:** [INFERENCE] Vegetation must publish elemental
  transfer records containing donor, receiver, material class, above- or
  below-ground placement, carbon mass, nitrogen mass, cadence, and event cause.
  Both vegetation and residue/biogeochemistry must reconstruct identical
  transfers.
- **Licensing disposition:** [INFERENCE] `inspection/comparison only`; cited
  literature may support an `eligible for literature-based independent
  derivation` decision after independent review; direct translation is
  prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for distinct
  live-plant and dead-material custody; `reject` direct vegetation mutation of
  soil and litter stores; `defer` soil biogeochemistry and nutrient routing.
  Owners are vegetation and residue/biogeochemistry, with soil hydrology as the
  eventual dissolved-nutrient transport owner and the hillslope orchestrator
  as transfer coordinator.

### Family 7: canopy snow and ground snow coupling

**Neutral semantic name:** vegetation-owned intercepted-snow store with typed
ground-snow release.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/hydro/compute_potential_snow_interception.c` —
  `compute_potential_snow_interception`;
  `rhessys/hydro/compute_snow_stored.c` — `compute_snow_stored`;
  `rhessys/hydro/compute_snow_sublimation.c` —
  `compute_snow_sublimation`;
  `rhessys/cycle/canopy_stratum_daily_F.c` — `canopy_stratum_daily_F`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`;
  `rhessys/hydro/snowpack_daily_F.c` — `snowpack_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Incoming snowfall water equivalent, prior
  intercepted-snow storage, plant area, gap fraction, capacity parameter, air
  temperature, vapour pressure, aerodynamic conductance, pressure, and canopy
  radiation.
- **Outputs and units:** [CODE-OBSERVED] End-of-step intercepted-snow water
  equivalent, sublimation, phase-changed liquid release, and solid release to
  the next stratum or ground, in metres of water equivalent per day or stored
  metres of water equivalent; used latent energy is recorded in the canopy
  energy accounting.
- **Cadence and mutation:** [CODE-OBSERVED] Daily stratum processing mutates
  intercepted snow before the separate ground-snowpack process mutates ground
  snow state. Vegetation is the proposed intercepted-snow owner; snow/frost is
  the proposed ground-snow owner; land-surface energy owns the shared energy
  debit.
- **Ordering and consumers:** [CODE-OBSERVED] Snow passes through vertically
  ordered strata. Canopy sublimation or phase change and capacity limitation
  determine releases. The final solid and liquid releases are then inputs to
  ground snow and surface hydrology. Canopy height relative to snowpack height
  affects whether a stratum is processed above or below the ground snowpack.
- **Literature present:** [CODE-OBSERVED] Storck (2002) and Andreadis (2009) are
  named for interception concepts; Bras (1990) for phase-change energy;
  Lundberg (1994) for snow evaporation/conductance concepts; Mahat (2011) and
  Hedstrom and Pomeroy (1998) for unloading; Price and Dunne (1976) and Murray
  (1967) for sublimation components. Some literature-mentioned alternatives
  are disabled in the inspected source and therefore do not establish observed
  runtime behavior.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] A literature-described
  unloading term is disabled; very small stores are removed by a source-only
  threshold; phase change is selected from daily air temperature; several
  energy and conductance approximations coexist. [INFERENCE] Disabled paths,
  thresholds, and formula choices are `NON-PROMOTABLE`.
- **Conservation relevance:** [INFERENCE] Prior intercepted storage plus solid
  input must reconcile to final storage, solid release, liquid release, and
  sublimation. Sublimation must reconcile to one latent-energy debit. Ground
  snow may consume releases but must never share mutable canopy storage.
- **Licensing disposition:** [INFERENCE] `eligible for literature-based
  independent derivation` only after independent consultation of named
  literature; source behavior is `inspection/comparison only`; direct
  translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept`. The source
  organization supports a single-owner boundary: vegetation owns intercepted
  canopy snow through storage, sublimation, and release calculation; snow/frost
  owns ground snow after typed solid/liquid receipt; land-surface energy owns
  energy closure; the hillslope orchestrator orders the exchange.

### Family 8: daily stage order and feedback

**Neutral semantic name:** three-stage vegetation–hydrology transaction.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/cycle/zone_daily_I.c` — `zone_daily_I`;
  `rhessys/cycle/patch_daily_I.c` — `patch_daily_I`;
  `rhessys/cycle/canopy_stratum_daily_I.c` —
  `canopy_stratum_daily_I`;
  `rhessys/cycle/zone_daily_F.c` — `zone_daily_F`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`;
  `rhessys/cycle/canopy_stratum_daily_F.c` —
  `canopy_stratum_daily_F`;
  `rhessys/cycle/canopy_stratum_growth.c` —
  `canopy_stratum_growth`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Beginning-of-day vegetation, soil, snow, litter,
  and meteorological state; management/disturbance events; radiation and
  precipitation forcing; hydrologic availability; nutrient availability.
- **Outputs and units:** [CODE-OBSERVED] Updated daily vegetation geometry and
  pools, potential water and nutrient demands, hydrology-realized withdrawals,
  finalized transpiration and carbon gain, interception releases, dead-material
  transfers, and owner-specific end states using the units stated in the
  corresponding family entries.
- **Cadence and mutation:** [CODE-OBSERVED] The source has a beginning-of-day
  update phase, a potential canopy response phase, a patch-scale water-supply
  arbitration phase, and an end-of-day plant growth phase. [INFERENCE] For
  openWEPP these map cleanly to Stage A potential ecosystem response, Stage B
  hydrologic arbitration, and Stage C vegetation finalization, with
  beginning-of-step state preparation treated as orchestrator input assembly.
- **Prerequisite order:** [INFERENCE] Native management and prior state must be
  projected first; phenology/geometry and environmental forcing must precede
  Stage A; all competing soil-water requests and frozen-water exclusions must
  reach Stage B together; Stage B receipts must precede Stage C; Stage C
  transfers must be applied by receiving owners before closure checks.
- **Feedback and iteration:** [CODE-OBSERVED] The inspected daily path returns a
  common realized-water fraction and adjusts vegetation carbon-related state
  once; no general vegetation–hydrology fixed-point iteration was found.
  [INFERENCE] Iteration is not required by this evidence, but Stage B must
  support typed rejection or non-convergence if a future canonical solver adds
  iteration.
- **Literature present:** [CODE-OBSERVED] none found for the integrated call
  order.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Surface evaporation and
  transpiration compete within a coarse shared water calculation, while the
  returned reduction lacks per-layer and per-reason detail. [INFERENCE] A
  single scalar realization is insufficient for openWEPP layer-resolved
  authority.
- **Conservation relevance:** [INFERENCE] The transaction identifier, forcing
  interval, request ledger, allocation ledger, finalized flux ledger, and all
  owner receipts must refer to the same timestep and area basis.
- **Licensing disposition:** [INFERENCE] `inspection/comparison only`; direct
  translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for Stage
  A/B/C, but `reject` source-specific scalar feedback as the openWEPP contract.
  The hillslope orchestrator owns ordering; vegetation and soil hydrology own
  their stage outputs; land-surface energy, snow/frost, and residue/
  biogeochemistry consume typed ledgers.

### Family 9: degenerate states, failures, and closure

**Neutral semantic name:** typed vegetation-domain and conservation result.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/util/sort_patch_layers.c` — `sort_patch_layers`;
  `rhessys/cn/update_phenology.c` — `update_phenology`;
  `rhessys/cn/compute_farq_psn.c` — `compute_farq_psn`;
  `rhessys/hydro/penman_monteith.c` — `penman_monteith`;
  `rhessys/cn/update_mortality.c` — `update_mortality`;
  `rhessys/cycle/canopy_stratum_growth.c` —
  `canopy_stratum_growth`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Potentially empty vegetation, zero leaf or root
  area, zero storage capacity, excessive same-height cover, negative or tiny
  elemental pools, invalid solver domains, water scarcity, and iterative
  sunlit/shaded geometry.
- **Outputs and units:** [CODE-OBSERVED] The source variously returns zero flux,
  clips state, perturbs state, emits diagnostics, returns an error indicator,
  or terminates execution. Water, carbon, and nitrogen residual diagnostics
  are maintained on their respective mass bases.
- **Cadence and mutation:** [CODE-OBSERVED] Guards occur during initialization,
  daily potential response, finalization, and end-of-day closure. Proposed
  mutation remains with each state owner; the hillslope orchestrator owns
  failure propagation and transaction rollback or rejection.
- **Observable degenerate states:** [CODE-OBSERVED] No vegetation, no leaves,
  no roots, no incoming radiation or precipitation, full or empty canopy
  stores, unavailable soil water, snow-covered vegetation, and absent growth
  mode all have observable special behavior.
- **Non-convergence:** [CODE-OBSERVED] Sunlit/shaded leaf-area reconciliation
  iterates to a relative tolerance but no explicit iteration bound or typed
  non-convergence return was found. [INFERENCE] The algorithm and tolerance are
  `NON-PROMOTABLE`; a future contract must bound iteration and return typed
  non-convergence without partial publication.
- **Domain failures:** [CODE-OBSERVED] Photosynthesis can report an invalid
  mathematical domain; some upstream callers terminate. Invalid output-mode
  selection in the atmospheric-demand routine terminates. Negative plant pools
  can be clipped, sometimes with acknowledged balance risk.
- **Literature present:** [CODE-OBSERVED] none found for the integrated guard,
  clipping, failure, or closure policy.
- **Ambiguity and limiting cases:** [INFERENCE] Source-side clipping does not
  distinguish harmless roundoff from physical invalidity, and diagnostic
  residuals do not consistently control publication. These behaviors cannot
  define openWEPP acceptance thresholds.
- **Conservation relevance:** [CODE-OBSERVED] Daily water, carbon, and nitrogen
  residuals are observable. [INFERENCE] openWEPP needs independent operand
  reconstruction and fail-closed publication for water, latent energy, carbon,
  nitrogen, and dead-material transfers; any normalization requires separate
  canonical authority.
- **Licensing disposition:** [INFERENCE] `inspection/comparison only`; direct
  translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for explicit
  degenerates, typed failures, bounded iteration, and closure receipts;
  `reject` silent clipping, state perturbation, and process termination as
  contract semantics. All domain owners validate their inputs; the hillslope
  orchestrator controls publication.

### Family 10: aggregate compatibility views

**Neutral semantic name:** read-only aggregate vegetation compatibility
adapter.

- **Audit coordinates:** [CODE-OBSERVED]
  `rhessys/cycle/patch_daily_I.c` — `patch_daily_I`;
  `rhessys/cycle/patch_daily_F.c` — `patch_daily_F`;
  `rhessys/output/output_patch.c` — `output_patch`;
  `rhessys/output/output_growth_patch.c` — `output_growth_patch`, all at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- **Inputs:** [CODE-OBSERVED] Finalized stratum cover, height, leaf area,
  root-depth envelope, transpiration, evaporation, interception storage,
  carbon/nitrogen totals, photosynthesis, and litter state.
- **Outputs and units:** [CODE-OBSERVED] Cover-weighted or otherwise aggregated
  canopy height, leaf area, maximum rooting depth, transpiration and evaporation
  water depths, canopy water and snow storage, elemental totals, and net carbon
  gain. Units remain those of the source quantities after declared area
  weighting.
- **Cadence and mutation:** [CODE-OBSERVED] Aggregates are assembled daily after
  stratum processing and do not constitute an independent vegetation state
  machine. [INFERENCE] An openWEPP compatibility adapter must be read-only and
  derived exclusively from finalized native state.
- **Ordering and consumers:** [INFERENCE] Stage C must finish before adapter
  reduction. Existing aggregate canopy, ET, and litter consumers may read only
  fields they semantically understand; the hillslope orchestrator owns adapter
  invocation and versioning.
- **Reduction semantics:** [CODE-OBSERVED] Different aggregate quantities use
  different reductions, including cover-weighted sums, a maximum root-depth
  envelope, and at least one unweighted canopy-area average. [INFERENCE] Every
  adapter field therefore requires an explicit reduction operator, area basis,
  missing-stratum rule, and dimensional check; there is no universal canopy
  averaging rule.
- **Literature present:** [CODE-OBSERVED] none found for the compatibility
  reductions.
- **Ambiguity and limiting cases:** [CODE-OBSERVED] Empty-stratum behavior and
  overlap-aware effective cover are not uniformly represented by aggregate
  outputs. [INFERENCE] Aggregation cannot reconstruct vertical overlap,
  layer-resolved withdrawal, sunlit/shaded state, or transfer provenance and
  must never feed back into native vegetation state.
- **Conservation relevance:** [INFERENCE] Extensive adapter values must be
  reducible from the same finalized ledger as native outputs; adapter rounding
  or omission cannot become an owner-side balance operand.
- **Licensing disposition:** [INFERENCE] `inspection/comparison only`; direct
  translation is prohibited.
- **Outcome and ownership:** [INFERENCE] `adopt boundary concept` for a named,
  read-only compatibility adapter; `reject` a second aggregate vegetation
  model and any cutover claim without real downstream consumption. Vegetation
  owns the adapter projection; the hillslope orchestrator and legacy aggregate
  consumers consume it.

## 4. Ordering and shared-transfer ledger

### Required transaction order

- [INFERENCE] **Input assembly:** the hillslope orchestrator supplies one
  timestep identity, meteorology, radiation boundary, precipitation phase,
  prior owner states, native-management projection, soil-layer availability
  descriptors, and snow/frost exclusions.
- [INFERENCE] **Stage A — potential ecosystem response:** vegetation may update
  timestep-entry phenology, then returns stratum radiation/interception
  responses, conductance diagnostics, potential transpiration, layer-resolved
  water requests, provisional carbon gain, potential nutrient demand, and
  proposed litter transfers. Stage A does not mutate soil or frozen-water
  storage and does not publish realized transpiration or realized assimilation.
- [INFERENCE] **Stage B — hydrologic arbitration:** soil hydrology receives all
  layer requests and competing withdrawals together, applies liquid and frozen
  availability plus canonical priority rules, mutates only hydrologic storage,
  and returns per-stratum, per-layer allocations with typed reason codes and a
  withdrawal ledger.
- [INFERENCE] **Stage C — vegetation finalization:** vegetation consumes the
  exact Stage B allocation, finalizes transpiration and coupled carbon response,
  updates only vegetation state, and publishes elemental and dead-material
  transfers. Receiving owners apply their own receipts.
- [INFERENCE] **Closure and publication:** land-surface energy reconciles latent
  energy to water flux; snow/frost reconciles canopy releases to ground-snow
  receipts; residue/biogeochemistry reconciles litter and coarse-woody receipts;
  the hillslope orchestrator publishes only after all owner receipts agree.

### Shared transfer records

- [INFERENCE] **Layer water withdrawal:** key by timestep, source soil layer,
  destination stratum, requested water depth, allocated water depth, liquid/
  frozen eligibility, and reason code. Producer: soil hydrology. Consumers:
  vegetation, land-surface energy, hillslope orchestrator.
- [INFERENCE] **Actual transpiration:** key by timestep and stratum, with the
  exact sum of allocated layer withdrawals and the corresponding latent-energy
  debit. Producers: vegetation and land-surface energy each reconstruct their
  side. Consumer: hillslope orchestrator.
- [INFERENCE] **Canopy liquid release:** key by timestep, emitting stratum,
  receiving lower stratum or ground surface, phase, and water depth. Producer:
  vegetation. Consumers: vegetation for a lower stratum, then soil hydrology,
  snow/frost, or residue/biogeochemistry at ground.
- [INFERENCE] **Canopy snow release:** key by timestep and emitting stratum,
  distinguishing solid release, liquid phase-change release, and sublimation.
  Producer: vegetation. Consumers: snow/frost, soil hydrology, land-surface
  energy, and hillslope orchestrator.
- [INFERENCE] **Radiation receipt:** key by timestep, band/thermal stream,
  direction, receiving stratum or ground component, absorbed energy, reflected
  energy, and transmitted energy. Producer: land-surface energy. Consumers:
  vegetation, snow/frost, residue/biogeochemistry, soil hydrology.
- [INFERENCE] **Dead-material transfer:** key by timestep, source stratum,
  event cause, material class, above-/below-ground destination, carbon mass,
  and nitrogen mass. Producer: vegetation. Consumer: residue/biogeochemistry.
- [INFERENCE] **Nutrient request/allocation:** potential plant demand and any
  future realized mineral-nutrient receipt must be separate typed records.
  Vegetation owns demand; residue/biogeochemistry owns availability and pool
  mutation; full routing is deferred.

## 5. Canopy-snow boundary evidence

- [CODE-OBSERVED] Intercepted canopy snow is stored on vegetation strata and is
  modified by interception, sublimation, phase change, capacity overflow, and
  passage through successive strata.
- [CODE-OBSERVED] Ground snow is a separate state processed after canopy
  passage, with canopy releases serving as boundary inputs.
- [CODE-OBSERVED] Canopy snow processes consume atmospheric and radiation
  information also relevant to ground snow, but the two stores need not share
  mutable custody.
- [INFERENCE] The evidence supports a promotable single-owner split at the
  semantic level: vegetation owns intercepted snow; snow/frost owns the ground
  snowpack; land-surface energy owns shared energy closure; the hillslope
  orchestrator owns ordering and receipt reconciliation.
- [INFERENCE] The split is promotable only as a boundary concept. Interception,
  unloading, sublimation, phase-change, and capacity formulas remain
  non-promotable until independently derived from admitted authority.
- [INFERENCE] Required handoffs are typed solid release, typed liquid release,
  sublimated water mass, latent-energy debit, and end-of-step intercepted
  storage. A shared mutable snow store or duplicate sublimation owner is
  rejected.

## 6. Literature and authority anchors

- [LITERATURE] The inspected material explicitly names Jarvis and Leverenz and
  Norman (1981) for diffuse canopy radiation and Chen et al. (1997) for direct
  canopy radiation.
- [LITERATURE] The inspected material explicitly names Chen et al. (1999) for
  sunlit/shaded canopy partition.
- [LITERATURE] The inspected material explicitly names the Penman–Monteith
  combination framework and the Jarvis multiplicative conductance framework;
  Heddeland and Lettenmaier (1995), Running and Coughlan, and Mackay et al.
  (2015) are named for aerodynamic or physiological response concepts.
- [LITERATURE] The inspected material explicitly names Farquhar and von
  Caemmerer (1982), de Pury and Farquhar (1997), Wullschleger (1993), Woodrow
  and Berry (1980), Kuehn and McFadden (1969), Ryan (1991), and Tjoelker et al.
  (2001) for photosynthesis or respiration concepts.
- [LITERATURE] The inspected material explicitly names Jolly et al. (2005),
  Nambiar et al. (1991), Dickenson et al. (1998), Landsberg and Waring (1997),
  Reyes et al. (2017), and Thornton (1997, 1998) or BIOME-BGC lineage for
  phenology, allocation, turnover, or pool-transfer concepts.
- [LITERATURE] The inspected material explicitly names Helvey (1964), Ogee and
  Brunet (2002), Storck (2002), Andreadis (2009), Bras (1990), Lundberg (1994),
  Mahat (2011), Hedstrom and Pomeroy (1998), Price and Dunne (1976), Murray
  (1967), and Brubaker (1996) for interception, snow, sublimation, or surface
  energy concepts.
- [INFERENCE] These are discovery anchors only. The inspected source is not a
  substitute for consulting the publications, resolving bibliographic details,
  checking applicable domains, or admitting them through openWEPP science-
  contract review.
- [INFERENCE] No equation or numerical constant from the source is promoted by
  this artifact. All such source-only material is `NON-PROMOTABLE`.

## 7. Licensing/provenance dispositions

- [CODE-OBSERVED] No adequate repository-level license grant for direct or
  closely translated incorporation was found in the frozen checkout.
- [INFERENCE] **Direct translation prohibited:** all source expression,
  organization, local naming, control flow, equations lacking independently
  admitted authority, source-only constants, empirical defaults, thresholds,
  and repair logic.
- [INFERENCE] **Inspection/comparison only:** observed stage ordering, aggregate
  feedback, storage sequencing, combined flux paths, diagnostics, and all
  behavior not traceable here to independently consulted external authority.
- [INFERENCE] **Eligible for literature-based independent derivation:** only
  scientific concepts for which the named external literature is separately
  obtained, reviewed, and admitted. Eligibility does not promote the source's
  formula, parameterization, unit conversion, branching, or defaults.
- [INFERENCE] **Adopt boundary concept:** native strata; distinct configuration
  and runtime state; ordered radiation and precipitation handoffs; separate
  vegetation, hydrology, energy, snow, and dead-material custody; Stage A/B/C;
  typed shared-transfer ledgers; and a read-only compatibility adapter.
- [INFERENCE] **Compare only:** RHESSys outputs or behavior may flag a difference
  after independent implementation but may not define acceptance.

## 8. Rejected and deferred material

### Rejected

- [INFERENCE] Direct, close, mechanical, or statement-by-statement translation
  of any inspected routine.
- [INFERENCE] Source-derived names, comments, control-flow structure, code-only
  formulas/constants, implicit thresholds, or hidden defaults in production or
  implementation-facing artifacts.
- [INFERENCE] Vegetation mutation of soil-layer water, groundwater, frozen
  water, ground snow, litter, or soil nutrient stores.
- [INFERENCE] A shared mutable canopy/ground snow store or duplicate owner for
  sublimation and latent energy.
- [INFERENCE] Silent clipping, state perturbation to repair cover, unbounded
  iteration, process termination, or diagnostic-only imbalance as openWEPP
  failure semantics.
- [INFERENCE] A scalar water-stress reduction as a replacement for per-layer,
  reason-coded hydrologic allocation.
- [INFERENCE] An aggregate adapter that becomes a second vegetation model,
  mutates native state, or supports a cutover claim without a proven consumer.

### Deferred

- [INFERENCE] Full photosynthesis, respiration, allocation, phenology,
  mortality, turnover, xylem recovery, and rooting constitutive laws.
- [INFERENCE] Full soil biogeochemistry, nutrient competition, fixation,
  mineralization, immobilization, decomposition, and dissolved nutrient
  routing.
- [INFERENCE] Distinct stemflow physics and any drainage partition beyond a
  typed generic canopy release.
- [INFERENCE] Empirical parameter values, bounds, defaults, vegetation
  parameter sets, calibration, and validation.
- [INFERENCE] Canopy-snow constitutive formulas and all source-described but
  disabled alternatives.
- [INFERENCE] Runtime selection, native-management schema activation, public
  output, compatibility cutover, and default eligibility.

## 9. Open questions and non-promotable gaps

- [INFERENCE] **Native cover contract:** specify whether cover is intrinsic or
  ground-area weighted, its allowed domain, within-height closure tolerance,
  cross-height overlap semantics, tie ordering, and empty-stratum behavior.
- [INFERENCE] **Parameter/initial-state split:** enumerate immutable parameter
  sets separately from per-run initial state and evolving state; no inspected
  default may fill an authority gap.
- [INFERENCE] **Root profile:** select independent authority for a normalized
  soil-layer root-fraction profile, dynamic profile change, inaccessible and
  frozen layers, and conservation-preserving remapping.
- [INFERENCE] **Radiation closure:** define bands, directions, area bases,
  stratum overlap, reflected/transmitted coupling, longwave stratum resolution,
  and mutually exclusive ground recipients.
- [INFERENCE] **Interception detail:** decide whether drainage, drip, and
  stemflow are distinct transfers and supply independent authority for each.
- [INFERENCE] **Stage B policy:** define competition with soil evaporation and
  other demands, priority/fairness, per-layer constraints, reason codes,
  tolerances, and failure publication.
- [INFERENCE] **Carbon–water coupling:** define how partial water allocation
  changes conductance, assimilation, respiration, nutrient demand, and
  allocation without adopting a source scalar or introducing proxy
  physiology.
- [INFERENCE] **Canopy snow:** independently establish interception capacity,
  unloading, phase change, sublimation, temperature state, aerodynamic
  coupling, and energy closure.
- [INFERENCE] **Elemental custody:** define the exact moment live/standing-dead
  material becomes residue, transfer classes, carbon/nitrogen closure, and how
  receiving owners reject a transfer.
- [INFERENCE] **Non-convergence:** define bounded iteration, typed failure, and
  no-partial-publication behavior wherever constitutive feedback requires a
  solve.
- [INFERENCE] **Compatibility:** define each field's reduction operator and
  dimensional basis, identify actual legacy consumers, and prohibit feedback
  from the adapter into native state.
- [INFERENCE] Every formula, numerical constant, tolerance, threshold,
  parameter bound, and default not tied to separately admitted literature or
  named openWEPP canonical authority remains `NON-PROMOTABLE`.

## 10. Independent-authorship sufficiency statement

- [INFERENCE] **Yes, with explicit limits.** An author who sees only this
  artifact can design a differently structured typed contract for native
  strata, configuration/runtime separation, radiation and interception
  receipts, Stage A/B/C water arbitration, canopy/ground snow ownership,
  elemental transfers, typed closure, and an aggregate compatibility adapter
  without recovering source expression.
- [INFERENCE] The artifact is intentionally insufficient to reproduce RHESSys
  algorithms, formulas, constants, defaults, statement order, or control flow.
  That insufficiency is required by the source-analysis firewall.
- [INFERENCE] The author must leave constitutive physiology and empirical
  values as authority gaps, or independently derive them from separately
  reviewed external literature or existing canonical openWEPP authority.
- [INFERENCE] A contract based on this artifact must be organized around typed
  owners and reconstructible transfer ledgers, not around the inspected source
  modules or call graph.
