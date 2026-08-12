# Parameter And Configuration Manifest

Status: `canonical manifest selected`

Evidence mode: `Static`

Every row is required per stratum unless marked stand/solver. Aliases are
explicitly versioned; unknown keys are retained as raw evidence but never
consumed. Absence, duplicate aliases, sentinel values, or incompatible units
fail. No value below has an openWEPP default.

| Family | Required canonical fields (units) | Class/domain |
|---|---|---|
| identity/topology | model version, stratum ID, evergreen/seasonal-deciduous woody enum, rank, tile membership/fraction, height/crown/base (`m`), projected leaf/stem orientation | caller configuration; unique IDs; fractions `(0,1]` |
| optics | leaf/stem reflectance and transmittance for VIS/NIR; `leaf_angle_chi`; clumping index | caller parameter; each optical pair `>=0`, sum `<1`; CLM approximation requires `chi in [-0.4,0.6]`; clumping `(0,1]` |
| area | SLA (`m2 leaf kgC-1`), stem-area relation, all/projected-sided convention | caller parameter; positive and explicit area basis |
| stomata | `g0` (`umol H2O m-2 leaf s-1`), `g1` (`sqrt(kPa)`), Medlyn diffusivity ratio when not model constant | caller parameter; `g0,g1>=0` |
| FvCB/N | Rubisco-N coefficient, electron-transport:N coefficient, `Tp25:Vcmax25`, `Rd25:leafN`, `Kc25`, `Ko25`, `Gamma25`, activation/deactivation enthalpies and entropy (`J mol-1`, `J mol-1 K-1`) | caller/calibratable except curvatures/yields frozen by version; positive finite |
| aerodynamic/energy | leaf, dry-stem, and common wet-surface characteristic dimensions (`m`), displacement and momentum/heat/vapor roughness, leaf/stem/wet-surface emissivity | caller parameter; positive; emissivities `(0,1]` |
| interception | `alpha_liq`, `p_liq` (`kg m-2 plant area`), `stemflow_fraction` | caller parameter; `alpha_liq,stemflow_fraction` in `[0,1]`, `p_liq>=0` |
| hydraulics | `root_fraction[layer]`, `k1a_max`, `k1b_max`, `k2_max`, `k3_max`, `p50_leaf`, `p50_stem`, `p50_root` (`mm H2O`), `ck`, `root_to_leaf_area`, `lateral_root_length`, path heights | caller parameter; root fractions sum 1; conductances/lengths positive, `p50<0`, `ck>0`; no capacitance in v1 |
| respiration | tissue-specific maintenance coefficient at 25 C (`kgC kgN-1 s-1`), temperature response, growth respiration fraction | caller/calibratable; nonnegative; growth fraction `<1` |
| allocation | fine-root:leaf, stem:leaf, coarse-root:stem, livewood fraction, current-growth fraction | caller/calibratable; nonnegative; fractions `[0,1]` |
| stoichiometry | C:N for leaf, leaf litter, fine root, live/dead wood and coarse root; dry-matter C fraction; mineral-N root fraction by layer; NH4 request fraction | caller parameter; C:N positive; C fraction `(0,1]`; root fractions sum one; NH4 fraction `[0,1]` |
| phenology/turnover | phenology enum, canonical GSI edge signal, deterministic on/off thresholds and hysteresis, onset/offset duration (`s`), leaf/root/wood lifetimes (`s`), background mortality rates (`s-1`) | caller/calibratable; on threshold above off threshold; durations positive; rates nonnegative; no alternative phenology formulation menu |
| initialization | exact full state object described below, timestamp, area/topology/profile/digest identities | caller initial state; complete and finite |

Fixed `OPENWEPP_C3_WOODY_V1` constants are: C3 flag; gas constant; molar mass
of carbon; H2O/CO2 diffusivity factors `1.6` and `1.4`; PAR photon conversion
`4.6 umol J-1`; PSII partition `0.5`; oxygen fraction `0.20`; FvCB
co-limitation curvatures `0.98/0.95` and PSII curvature/yield `0.7/0.85`.
It also freezes `rho_water=1000 kg m-3`, `Cv=0.01 m s-1/2`, von Karman
`0.4`, Stefan--Boltzmann `5.670374419e-8 W m-2 K-4`, dry-air gas constant
`287.05 J kg-1 K-1`, dry-air heat capacity `1004.64 J kg-1 K-1`, liquid
vaporization enthalpy `2.501e6 J kg-1`, and CLM5 Table 5.2 liquid saturation
polynomial coefficients. Air density is derived as `Patm/(Rdry*Tcan)`.
Their exact rational/decimal representation and provenance are serialized in
the model-version definition. All CLM PFT tables, RHESSys parameter rows and
GIS profiles are explicitly `COMPARATOR_ONLY`, not fallbacks.

## Complete Initial State

For every stratum: canopy liquid; previous accepted root, stem, sun-leaf and
shade-leaf equilibrium potentials (warm starts, not hydraulic storage);
displayed, storage and transfer C and N pools
for leaf, fine root, live stem, dead stem, live coarse root and dead coarse
root; C and N retranslocation pools; NSC/storage C; standing-dead C/N/DM;
phenology phase/timers and previous offset flux; persistent `T10_K`; mortality carry; leaf/root/stem area; height/crown
geometry; and last accepted state/transaction identity. For every receiving
biogeochemical column: mineral ammonium/nitrate by soil layer, litter metabolic/
cellulose/lignin C/N/DM, and coarse woody debris C/N/DM. Reconstruction requires
`LAI=leaf_C*SLA`, every pool except signed maintenance reserve `XS_C`
nonnegative, C:N consistency where the pool is
stoichiometric, and identical topology/area/profile digests. Zero pools are
permitted only when all derived quantities and pending transfers agree.

## Executable Field Inventory

The serialized v1 schema has exactly these canonical consumed fields; brackets
are typed repeated records, not wildcard aliases. Every field is required
unless its `required_when` branch says otherwise. Runtime names are identical
snake-case names. RHESSys spellings are raw-source evidence only and have no
consumed alias in v1.

| Required when | Canonical fields | Units/domain |
|---|---|---|
| every stand | `model_definition_sha256`, `configuration_sha256`, `initial_state_sha256`, `area_m2`, `timestamp`, `dt_s`, `topology_tiles[]` | digests exact; area/dt positive; tile fractions positive and sum one |
| every stratum | `stratum_id`, `lifeform=C3_WOODY`, `phenology_type`, `vertical_rank`, `tile_ids[]`, `height_m`, `crown_base_m`, `leaf_dimension_m`, `stem_dimension_m`, `wet_surface_dimension_m`, `sla_m2_per_kg_c`, `sai_relation`, `leaf_angle_chi`, `clumping_index` | finite; geometry/areas/dimensions positive; `leaf_angle_chi in [-0.4,0.6]`; enum exact |
| optics | `leaf_rho_vis`, `leaf_tau_vis`, `leaf_rho_nir`, `leaf_tau_nir`, `stem_rho_vis`, `stem_tau_vis`, `stem_rho_nir`, `stem_tau_nir` | each `[0,1)` and each rho+tau `<1` |
| gas exchange | `g0_umol_h2o_m2_s`, `g1_sqrt_kpa`, `rubisco_n_efficiency`, `electron_n_efficiency`, `tp_vcmax_ratio`, `rd_leaf_n_rate`, `kc25_pa`, `ko25_pa`, `gamma25_pa`, `ha_vcmax`, `hd_vcmax`, `entropy_vcmax`, `ha_jmax`, `hd_jmax`, `entropy_jmax`, `ha_kc`, `ha_ko`, `ha_gamma` | explicit units from contract; nonnegative/positive as mathematically required |
| interception/aero | `alpha_liq`, `p_liq_kg_m2_plant`, `stemflow_fraction`, `z0m_m`, `z0h_m`, `z0q_m`, `displacement_m`, `leaf_emissivity`, `stem_emissivity`, `wet_surface_emissivity` | fractions `[0,1]`; `p_liq>=0`; `zref>d+z0`; emissivities `(0,1]` |
| hydraulics | `k1a_max_s1`, `k1b_max_s1`, `k2_max_m_s`, `k3_max_m_s`, `p50_leaf_mm`, `p50_stem_mm`, `p50_root_mm`, `vulnerability_shape`, `root_to_leaf_area`, `lateral_root_length_m`, `root_layers[]` | conductance/length positive; p50 negative; layer IDs unique; fractions sum one |
| respiration/allocation | `atkin_intercept`, `mr_base_kgc_per_kgn_s`, `mr_q10`, `xs_recovery_days`, `growth_resp_ratio_g1`, `alloc_froot_leaf_a1`, `alloc_croot_stem_a2`, `alloc_stem_leaf_a3`, `livewood_fraction_a4`, `current_growth_fraction` | caller values; `a1..a3>=0`; fractions `[0,1]`; time positive |
| stoichiometry | `cn_leaf`, `cn_leaf_litter`, `cn_froot`, `cn_livewood`, `cn_deadwood`, `drymatter_carbon_fraction`, `mineral_n_root_fraction[layer]`, `nh4_request_fraction`, `litter_metabolic_fraction[tissue]`, `litter_cellulose_fraction[tissue]`, `litter_lignin_fraction[tissue]` | C:N positive; dry C `(0,1]`; root fractions sum one; NH4 fraction `[0,1]`; three litter fractions nonnegative and sum one |
| phenology/turnover | `onset_duration_s`, `offset_duration_s`, `gsi_on_threshold`, `gsi_off_threshold`, `gsi_hysteresis`, `leaf_lifetime_s`, `froot_lifetime_s`, `livewood_turnover_s`, `mortality_rate_s1` | deciduous requires thresholds/durations with on>off; evergreen ignores GSI thresholds but still requires explicit nulls; rates nonnegative |

The initial-state record contains `canopy_liquid`, four warm-start potentials,
signed `XS_C`, nonnegative `NSC_C`, retranslocation N, and for each of six tissue IDs (`leaf`, `froot`,
`livestem`, `deadstem`, `livecroot`, `deadcroot`) the six explicit fields (36 total)
`display_C`, `display_N`, `storage_C`, `storage_N`, `transfer_C`, `transfer_N`,
plus standing-dead C/N/DM where applicable. It also contains phase enum,
onset/offset remaining seconds, previous leaf/root offset fluxes, geometry,
pending-transfer list (normally empty), and last transaction ID. The BGC state
has `NH4_N` and `NO3_N` per layer and C/N/DM for every metabolic, cellulose,
lignin and CWD receiver. Missing, extra consumed, duplicate, sentinel, or
unit-incompatible fields fail before calculation.
