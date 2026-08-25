---
contract_id: SC-SNOWENERGY-001
title: Snow-Surface Energy and Sub-Canopy Longwave Contract
status: approved
maturity: active
owner: openWEPP maintainers + snow-process reviewer
contract_version: 18
released_contract_version: 18
candidate_contract_version: 19
producer_scope:
  - Hourly atmospheric longwave evaluated from hourly temperature and daily vapor/cloud state
  - Native-canopy effective cover to diffuse sky-view translation
  - Complementary sky and canopy longwave incident at the snow surface
consumer_scope:
  - Shared Stage 3 snow-surface energy carrier
  - Snow sublimation and melt components
  - Snow-energy diagnostics and assurance outputs
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-08-24
supersedes: []
superseded_by: []
---

# SC-SNOWENERGY-001 Snow-Surface Energy and Sub-Canopy Longwave Contract

Status: `approved`
Maturity: `active`
Evidence mode: `static + independent oracle + contract vectors`

Lifecycle: version 18 remains `approved / active`; only the version-19
successor amendment is `in_review / draft` until its own dual review,
verification, implementation, and exact-head gates pass.

## Purpose

Define the canonical hourly atmospheric and sub-canopy longwave equations,
using hourly temperature with daily vapor/cloud state, plus operand meanings,
guards, and coupling obligations for openWEPP snow-surface energy
calculations. The contract derives diffuse sky view from existing effective
canopy cover and does not introduce a user-entered sky-view coefficient or a
required remote-sensing input.

Version 3 replaced the failed version-2 snowfall-event top-layer provider with
the Marks/SNOBAL active thermal control volume. The active volume spans the
upper `min(z_s, 0.25 m)` of snow except for the version-4 terminal lower-volume
collapse, crosses depositional-layer boundaries
conservatively, and exchanges conductive heat with the remaining lower pack
inside each stability substep. Marks/SNOBAL mass-dependent timestep
subdivision is required. The production default remains unchanged. EB-03A
real-consumer and terminal workspace evidence pass.

Version 4 defined the terminal resolved thermal-layer domain. The exact
libsnobal `1 kg m^-2` threshold suspends Stage 3 exchange only when total pack
ice mass is at or below the boundary. In a resolved pack, a lower thermal
volume strictly below the same threshold collapses into a one-volume thermal
solve; equality remains a two-volume solve. At that version, CoE remained the
current runtime owner for snow existence and mass, so persistent layer state
was retained rather than converted to water. Version 7 preserves that behavior
only under the implementation hold and does not carry it into target authority.

Version 5 separates represented-layer lifecycle from aggregate residual
acceptance. A density-layer fragment is represented when its converted ice
mass is greater than the existing `1e-9 kg m^-2` zero-mass boundary; with
`rho_w = 1000 kg m^-3`, this is `mass_swe_m > 1e-12 m`. The independent
`1e-9 m` SWE and physical-depth aggregate closure tolerances remain residual
guards and are not layer-deletion thresholds.

Version 7 reconciles melt ownership. The admitted future production target is
one Stage 3 energy-and-mass control volume: after cold-content satisfaction,
bounded positive energy converts available ice to liquid and that liquid enters
the same-substep refreeze, retention, and routing chronology. The current
runtime remains unchanged and therefore is on `IMPLEMENTATION_HOLD`: it still
generates melt with CoE and reports Stage 3 positive excess without converting
it. CoE is a compatibility implementation during the hold, not the target
scientific authority, and simultaneous CoE/Stage 3 melt generation is
prohibited.

Version 8 binds the CLIGEN/openWEPP hourly forcing projection to explicit
virtual transfer geometry for Stage 3 turbulent exchange. The model evaluates
temperature, humidity, and wind with `5 m` transfer heights above the modeled
instantaneous snow surface and exposed-snow aerodynamic roughness `0.005 m`.
Those values are model geometry derived from the pinned libsnobal point-input
contract, not measurements, forcing reference heights, calibration parameters,
or site observations.

Version 9 distinguishes the current evaluation carrier's raw signed vapor
exchange opportunity from canonical bounded vapor transfer. Schema-v6 raw
vapor and latent energy are diagnostic opportunities. Actual sequential snow
mass debit/credit uses bounded transfer. Capacity truncation without a matching
latent-energy bound is a plausibility finding, not production authority.

Version 10 separates forcing-source custody from model transfer geometry.
GRIDMET asset metadata describes daily `vs` as nominal `10 m` wind, but the
initial fixture manifests did not preserve enough generator metadata to
identify their exact asset version/status, pixel, sampling, transformation, or
exposure.
Their CLI `w-vl` therefore remained raw forcing with source/exposure authority
missing at that intake. Stage 3 consumes that raw value while using virtual `z_u=5 m`; PMET's
separate `10 m`-to-`2 m` adjustment is local and never becomes snow forcing.
A reference-height diagnostic cannot establish forest/sub-canopy exposure,
license attenuation, or authorize a production correction.

Version 11 incorporates provider-side recovery from the surviving WEPPpy
source runs. Byte-identical `/wc1` CLIs and run records directly establish
retained watershed centroids, GRIDMET-enabled run intent, complete daily
parquet wind, and equality to one-decimal CLI `w-vl`. The nearest pre-build
WEPPpy revision statically reconstructs a path that would request GRIDMET `vs`
at the watershed centroid and share it across hillslopes. The runs do not
retain the deployed
container/source SHA, raw request/response, asset version/status, server-side
pixel-selection rule, exact pixel, timezone/day boundary, fill policy, or
aerodynamic exposure. Modeled evergreen-forest landuse and `cancov=0.9`
establish target model intent only, not physical exposure or representativeness.

Version 17 defines the precipitation-custody input to the persistent Stage 3
lane. It admits one sealed, canonically ordered phase-parcel set per support;
binds precipitation mass and precipitation-advected heat to that same set;
and makes open raw rain mutually exclusive with covered vegetation terminal
liquid at each ground destination. It imports the interception, throughfall,
stemflow, and drainage chronology from `SC-VEGETATION-001@28` and the
destination topology from `SC-LANDSURFACEENERGY-001`; it adds no interception
equation or canopy-snow process.

Version 18 defines the persistent Stage 3 snow--soil conductive boundary. One
OFE-ground lane column couples its bottom represented snow thermal volume to
the first ordered OFE soil-thermal node. The interface uses the same
center-to-center, two-half-layer series resistance and Crank--Nicolson endpoint
evaluation already admitted by `SC-LANDSURFACEENERGY-001`; tile temperatures
are neither averaged nor assigned duplicate shares of the lane flux.

Version 19 defines the covered terminal chronology successor. It separates
read-only discovery from exact endpoint acceptance, requires every adaptive
and root trial to rebuild the complete covered physical carrier on its exact
projected support, retains the solver-returned dormant endpoint, and stages
terminal liquid as `ProducedUnconsumed` custody inside the canonical snow
owner. It also introduces a terminal-specific snow--soil receipt and physical
ledger; no post-event snow temperature is invented. The successor remains
default-off and in review. Runner construction, liquid receiver consumption,
restart, selectors, activation, CoE retirement, and cutover are not admitted.

## Scientific Scope

In scope:

- Dilley-O'Brien clear-sky longwave from air temperature and vapor pressure.
- Unsworth-Monteith cloud correction derived from daily clearness index.
- An effective-cover-to-diffuse-sky-view transformation derived from the same
  Beer-law canopy extinction basis used by FSM2.
- Complementary atmospheric and canopy longwave at the snow surface.
- Effective-unity canopy and snow emissivity convention; atmospheric
  emissivity remains the variable Dilley-Unsworth result.
- Outgoing snow longwave and the positive-toward-snow net-longwave sign.
- Typed runtime obligations and deterministic analytical test vectors.

Out of scope:

- Default activation, user-facing selectors, or public output-schema changes.
- A prognostic canopy-temperature energy balance.
- Explicit trunks, canopy gaps, terrain horizons, multiple reflections, or
  three-dimensional ray tracing.
- Site fitting, empirical calibration, or a new user radiative coefficient.
- Implementing the version-7 melt-owner target, selectors, defaults, or
  cutover. Shortwave, sensible, latent, ground/conductive, and precipitation-
  advection components are governed here only to make the future complete
  energy ledger and its implementation hold explicit.

Validity is limited to an equivalent, horizontally homogeneous one-layer
canopy with the FSM2 random-orientation angular extinction approximation and
an isotropic diffuse sky hemisphere. The native structural-cover floor is
treated as effective vertical optical depth in that equivalent medium. The
contract does not claim direct validity for directional crowns, explicit
gaps/edges/trunks, terrain-obstructed sky, or anisotropic diffuse radiation.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| `REF-SNOWENERGY-FLERCHINGER` | Flerchinger et al. (2009), *Water Resources Research* 45:W03423, doi: `10.1029/2008WR007394`, corrected Table 1, Tables 2 and 9 | Corrected Dilley-O'Brien clear-sky equation, precipitable-water proxy, Unsworth-Monteith cloud correction, daily clearness bounds, and reported uncertainty. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-ESSERY2008` | Essery et al. (2008), *Hydrological Processes* 22:2788-2800, doi: `10.1002/hyp.6930` | Hemispherical view-factor integration and two-component forest longwave exchange. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-FSM2` | Essery et al. (2025), “FSM2.1.1: an efficient model of snow processes and surface energy balance”, *Geoscientific Model Development* 18:3583-3607, doi: `10.5194/gmd-18-3583-2025`, Eq. 13-14 and §2.3 | Beer-law direct and diffuse canopy transmission, factor `1.6`, and complementary longwave exchange with unity emissivity. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-SVS2` | Leonardini et al. (2025), “SVS2-Crocus”, *Geoscientific Model Development* 18:9119-9154, doi: `10.5194/gmd-18-9119-2025`, Eq. 1 and §2.1.2 | Independent support that canopy sky-view factor is an exponential extinction function rather than direct plan-view cover. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-RUTTER2023` | Rutter et al. (2023), *Journal of Geophysical Research: Atmospheres* 128:e2022JD037980, doi: `10.1029/2022JD037980` | Stand-scale complementary sky/canopy formulation, effective-unity canopy behavior, and canopy-temperature approximation evidence and limits. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PLANT` | `SC-PLANT-001#INV-PLANT-034` and native canopy variables | Canonical openWEPP meanings of effective canopy cover, structural cover floor, LAI, and height. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-EB01A` | `docs/work-packages/20260730-snow-surface-eb-01a-longwave-authority-research-001/` | Package evidence that reconciled atmospheric longwave candidates and admitted the FSM2 canopy route. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-MARKS1999` | Marks et al. (1999), *Hydrological Processes* 13:1935-1959, doi: `10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C` | Two-layer SNOBAL energy balance, active-layer thermal state, conductive exchange, and progressively smaller shallow-layer timesteps. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-LIBSNOBAL` | CC0 libsnobal at `/home/workdir/pysnobal`, commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `_calc_layers.c`, `_adj_layers.c`, `_e_bal.c`, `g_snow.c`, `_divide_tstep.c`, `_below_thold.c`, `snobal.h`, `pysnobal/ipysnobal.py`, and `test_data_point/inheight.input` | Equation-reference implementation for `z_s_0`, `G_0`, harmonic two-layer transfer, the `60/10/1 kg m^-2` mass-dependent `60/15/1 minute` timestep hierarchy, exact total-`<=`/lower-`<` terminal-layer ordering, residual-snow phase disposition, and the point-forcing `5 m` thermodynamic/wind virtual heights plus `0.005 m` snow roughness. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-GRIDMET` | Google Earth Engine asset catalog `IDAHO_EPSCOR/GRIDMET`, accessed 2026-08-07; Abatzoglou (2013), DOI `10.1002/joc.3413`; NASA GSFC NLDAS-2 forcing documentation, accessed 2026-08-07 | General authority that distributed GRIDMET `vs` is daily nominal `10 m` wind on an approximately `4 km` grid and derives from gridded land-data forcing. It does not identify retained fixture pixels, transforms, or exposure. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-WIND-CUSTODY` | `docs/work-packages/20260807-snow-stage3-wind-source-custody-and-exposure-authority-001/`, its result-blind freeze, retained CLI hashes, provider recovery, custody ledger, consumer proof, and exposure matrix | Custody/claim authority proving retained WEPPpy value lineage and distinguishing it from statically reconstructed request/transform semantics, while separating raw CLI/Stage 3 wind, PMET-local `2 m` adjustment, virtual `5 m` transfer geometry, and missing deployed/server/exposure authority without fitting or production correction. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-LUTE2022` | Lute et al. (2022), *Geoscientific Model Development* 15:5045-5071, doi: `10.5194/gmd-15-5045-2022`, section 2.2.7 | Independent documentation that Marks et al. address shallow-snow energy instability with progressively smaller timesteps. SnowClim's alternative temperature replacement and fitted cold-content tax are not admitted. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PHYSICAL` | Stefan-Boltzmann law and bounded-fraction physical invariants | Thermal emission, finite-temperature, and bounded-transmission requirements. | `[INFERENCE][Static]` |
| `REF-SNOWENERGY-21N` | `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/` with frozen 21M evidence and pinned libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6` (`_e_bal.c`, `_snowmelt.c`, `_advec.c`, `_mass_bal.c`, `_runoff.c`, `envphys.h`, `snow.h`) | Result-blind CoE-envelope adjudication; energy-to-melt derivation; exact energy, solid-to-liquid, and liquid-disposition chronology; current-runtime hold. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-USER-OFE-GROUND-V15` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/science-contracts/SC-SNOWENERGY-001/authority-decision.md` | Direct prospective user selection of one persistent Stage 3 column per lane on OFE-ground basis, with complete tile-ground flux summation, no covered-subset renormalization, uniform-depth terminal identity, and topology-bound restart semantics. Repository state/terminal architecture supports the selection; future per-tile/routing-cell ownership requires a new version. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `SC-VEGETATION-001@28`; `SC-LANDSURFACEENERGY-001`; user-directed Child-1 covered physical-custody checkpoint | Imports the admitted vegetation liquid interception/release chronology and typed LSE destination topology, then binds their terminal parcels to the OFE-ground Stage 3 lane without adding canopy-snow or interception physics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-SOIL-BOUNDARY-V18` | pinned `dac3c950...:src/frostn.for`, lines 476-607; `src/tmpadj.for`, lines 266-353; `SC-LANDSURFACEENERGY-001@8` | Legacy WEPP establishes additive snow/soil thermal resistance and harmonic conduction; current LSE supplies the authoritative node-centered two-half-layer interface, Crank--Nicolson endpoint evaluation, and OFE soil-thermal owner. Legacy zero-flux fallbacks, calibrated factors, and frost-front approximations are not imported. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `T_a` | `K` | Hourly above-canopy air temperature. | hourly climate forcing | atmospheric longwave |
| `e_a` | `kPa` | Daily actual vapor pressure held across the declared hourly evaluations. | daily climate forcing | precipitable-water proxy |
| `w` | `kg m^-2` | Precipitable-water proxy, `4650 e_a / T_a`. | atmospheric longwave | clear-sky equation |
| `L_clear` | `W m^-2` | Clear-sky downward longwave irradiance. | atmospheric longwave | clear-sky emissivity |
| `epsilon_clear` | `dimensionless` | Effective clear-sky emissivity. | atmospheric longwave | cloud mixture |
| `R_s` | `MJ m^-2 d^-1` | Daily incident above-canopy shortwave radiation. | climate forcing | clearness index |
| `R_a` | `MJ m^-2 d^-1` | Daily extraterrestrial radiation on the horizontal surface. | solar geometry | clearness index |
| `k_t` | `dimensionless` | Daily clearness index, `R_s/R_a`. | cloud inference | cloud fraction |
| `c` | `fraction` | Effective daily cloud fraction. | cloud inference | all-sky emissivity |
| `epsilon_all` | `dimensionless` | Effective all-sky emissivity. | atmospheric longwave | atmospheric irradiance |
| `L_atm` | `W m^-2` | Hourly all-sky downward atmospheric longwave above canopy. | atmospheric longwave | sub-canopy mixture |
| `C` | `fraction` | Effective plan-view overhead canopy interception fraction. | plant/canopy state | sky-view translation |
| `P_0` | `fraction` | Vertical canopy gap fraction, `1-C`. | sky-view translation | Beer-law elimination |
| `f_sky` | `fraction` | Hemispherical diffuse sky-view/transmission factor. | sky-view translation | sub-canopy mixture |
| `T_c` | `K` | Effective radiating canopy temperature. | EB-03 provider boundary | canopy emission |
| `L_can` | `W m^-2` | Effective canopy longwave emission, `sigma T_c^4`. | canopy emission | sub-canopy mixture |
| `L_sub` | `W m^-2` | Downward longwave incident at the snow surface. | sub-canopy mixture | snow energy carrier |
| `T_s` | `K` | Snow-surface radiating temperature. | EB-03 provider boundary | snow emission |
| `L_out` | `W m^-2` | Upward longwave emitted by snow. | snow emission | net longwave |
| `L_net` | `W m^-2` | Net longwave, positive toward snow. | longwave balance | snow energy carrier |
| `z_0` | `m` | Active thermal-layer depth, normally `min(z_s, 0.25 m)`; whole-pack depth when `INV-SNOWENERGY-026` collapses a lower volume with `0 < m_l < 1 kg m^-2`. | Stage 3 thermal partition | shared energy carrier |
| `z_T` | `m` | Virtual air-temperature transfer height above the instantaneous modeled snow surface; `5 m` for current Stage 3 geometry, not a physical measurement-height claim. | model geometry | turbulent sensible exchange |
| `z_q` | `m` | Virtual humidity/vapor-pressure transfer height above the instantaneous modeled snow surface; `5 m` for current Stage 3 geometry, not a forcing reference-height claim. | model geometry | turbulent latent exchange |
| `z_u` | `m` | Virtual momentum transfer height above the instantaneous modeled snow surface; `5 m` in current Stage 3 geometry and distinct from nominal GRIDMET `10 m` wind reference height. | model geometry | turbulent momentum exchange |
| `z_u,source` | `m` | Physical/source-product wind reference height relative to the source-product land/model surface; nominal `10 m` for GRIDMET `vs` only when exact GRIDMET source identity is established. | forcing-source custody | source applicability |
| `u_cli` | `m s^-1` | Daily CLI `w-vl`, parsed as raw `vwind`/`vwind_m_s` and consumed unchanged by Stage 3. Retained provider evidence proves daily parquet-to-CLI equality; nearby historical code statically reconstructs watershed-centroid request, run-level sharing, and one-decimal formatting. Deployed request and server-side pixel/version/timezone/fill semantics remain `AUTHORITY_MISSING`. | CLI/runtime boundary | Stage 3 and PMET input |
| `u_2,PMET` | `m s^-1` | PMET-local FAO-56 adjustment of raw wind to `2 m`; never a Stage 3 input or exposure authority. | PMET local | evapotranspiration only |
| `z_0,aero` | `m` | Aerodynamic roughness length of the exposed snow surface; fixed `0.005 m` for the admitted Stage 3 snow surface. This is distinct from active-layer depth `z_0`. | snow-surface authority | turbulent exchange |
| `m_0` | `kg m^-2` | Snow-ice mass contained in `z_0`. | Stage 3 thermal partition | active-layer heat capacity |
| `T_0` | `K` | Heat-capacity-weighted active-layer temperature. | active-layer cold content | radiation and turbulent exchange |
| `T_l` | `K` | Heat-capacity-weighted lower-pack temperature when `z_s > z_0`. | lower-pack cold content | interface conduction |
| `G_0` | `W m^-2` | Conductive exchange, positive from the lower pack into the active layer. | coupled thermal provider | active/lower energy balances |
| `p_a` | `Pa` | Atmospheric pressure derived from run elevation. | climate metadata projection | SNOBAL effective snow conductivity |
| `k_d`, `k_eff` | `W m^-1 K^-1` | Dry Yen snow conductivity and Anderson pore-vapor-enhanced effective conductivity. | density, temperature, pressure | active/lower series resistance and `G_0` |
| `Q_cc` | `J m^-2` | Positive active-layer cold-content deficit relative to `0 degC` ice. | Stage 3 thermal partition | shared energy carrier |
| `m_v,raw` | `kg m^-2` | Signed vapor exchange opportunity integrated from turbulent mass flux before snow-ice availability bounding; deposition positive, sublimation negative. | evaluation-only schema-v6 carrier | attribution diagnostic; never actual S/F debit |
| `m_v` | `kg m^-2` | Signed bounded vapor transfer; deposition positive, sublimation negative. | bounded Stage 3 transfer | actual snow mass and canonical latent-energy ledgers |
| `T_ca`, `q_ca` | `K`, `kg kg^-1` | Shared canopy-air temperature and specific humidity solved by the coupled carrier. | shared carrier transaction | V11 canopy and Stage 3 snow turbulent exchange |
| `H_i`, `V_i` | `W m^-2`, `kg m^-2 s^-1` | Sensible and vapor exchange between participant `i` and the shared carrier. | shared carrier transaction | canopy/snow flux ledgers |
| `L_can`, `L_snow<->canopy` | `W m^-2` | Area-weighted component canopy emission and reciprocal canopy/snow longwave exchange. | V11/LSE canopy components | snow longwave and reciprocal ledger |
| `Q_E` | `J m^-2` | Hour-integrated applied surface energy, positive toward snow. | shared energy carrier | cold-content update |
| `Q_complete` | `J m^-2` | Exact-one sum of all admitted external and ground/interlayer energy operands for the declared substep/control volume, before phase-change allocation. | future complete Stage 3 energy carrier | cold-content and phase allocation |
| `Q_cold_required` | `J m^-2` | Non-negative energy required to bring the melt-owning ice to its phase threshold after active/lower allocation. | future Stage 3 thermal state | positive-excess derivation |
| `Q_excess` | `J m^-2` | Non-negative energy remaining after active/lower cold-content satisfaction in the same stability substep. | future complete Stage 3 energy ledger | bounded phase change |
| `delta_E_cold` | `J m^-2` | Signed increase in below-phase-threshold snow internal energy over the substep; positive means warming. | future Stage 3 energy ledger | independent energy closure |
| `Q_refreeze` | `J m^-2` | Latent energy released by same-substep refreeze, exactly `L_f m_refrozen`. | future Stage 3 phase ledger | cold-content/energy closure |
| `L_f` | `J kg^-1` | Latent heat of fusion used by the admitted phase conversion. | fixed physical constant | bounded phase change |
| `m_ice_available` | `kg m^-2` | Ice available for melt after same-substep solid precipitation and reservation of the already evaluated bounded sublimation mass; later deposition does not enlarge it. | future Stage 3 phase state | joint melt/vapor availability bound |
| `m_melt` | `kg m^-2` | Ice converted to liquid by Stage 3 in one substep, bounded by available ice. | future Stage 3 melt owner | same-substep liquid disposition |
| `m_liquid_external_in` | `kg m^-2` | Liquid entering the Stage 3 control volume during the substep, excluding the retained store already present at substep start. | precipitation/upstream handoff | liquid-disposition ledger |
| `delta_m_retained` | `kg m^-2` | Signed retained-liquid store change, `m_liquid_phase_end-m_liquid_phase_start`. | Stage 3 liquid state | liquid-disposition ledger |
| `m_refrozen`, `m_routed` | `kg m^-2` | Same-substep liquid refrozen to ice and liquid exported after holding-capacity disposition. | Stage 3 phase/liquid solve | linked mass ledgers |
| `m_solid_precip`, `m_deposition`, `m_sublimation` | `kg m^-2` | Same-substep solid precipitation, vapor deposition to ice, and sublimation from ice, each non-negative and distinct. | precipitation and signed vapor exchange | complete solid ledger |
| `P_phase` | ordered set | Sealed precipitation phase parcels for one exact support, in canonical destination then phase/source order. Every parcel binds lane/OFE, destination tile, phase, mass basis, support, atmospheric or vegetation producer-state identity, temperature/enthalpy provider, and receipt identity. | atmospheric forcing plus vegetation terminal-liquid owner | Stage 3 precipitation mass and advection consumers |
| `m_precip,p`, `Q_adv,p` | `kg m^-2 tile-ground`, `J m^-2 tile-ground` | Parcel mass and precipitation-advected heat reconstructed from the same parcel identity; OFE-ground lane values are `sum_p(f_destination,p * value_p)` with each parcel consumed exactly once. | sealed phase parcel | Stage 3 mass and complete-energy ledgers |
| `T_sb,0`, `T_sb,1` | `K` | Beginning and candidate-ending temperatures of the bottom represented Stage 3 snow thermal volume. | Stage 3 lane owner | snow side of lower boundary |
| `dz_sb`, `lambda_sb` | `m`, `W m^-1 K^-1` | Positive physical thickness and conductivity of that bottom snow thermal volume. | Stage 3 configuration/state | lower-boundary resistance |
| `T_1,0`, `T_1,1` | `K` | Beginning and candidate-ending temperatures of the first ordered OFE soil-thermal node. | soil-thermal owner | soil side of lower boundary |
| `dz_1`, `lambda_1` | `m`, `W m^-1 K^-1` | Positive thickness and conductivity of the first ordered OFE soil-thermal node. | LSE/soil-thermal configuration | lower-boundary resistance |
| `G_ss` | `W m^-2 OFE-ground` | Snow--soil conductive heat, positive downward from snow to soil; Stage 3 is debited and soil thermal is credited by the identical accepted amount. | joined Stage 3/soil-thermal candidate | complete energy and soil ledgers |
| `Q_unallocated_after_exhaustion` | `J m^-2` | `Q_excess-L_f m_melt`, non-negative energy remaining only when the available-ice bound saturates. | future energy/phase ledger | terminal meltout hold |
| `m_res` | `kg m^-2` | Total ice-mass boundary above which the Marks/SNOBAL Stage 3 thermal layer is resolved. | fixed libsnobal threshold | Stage 3 domain branch |
| `t_unres` | `s` | Duration for which CoE snow exists below the resolved Stage 3 thermal domain. | Stage 3 domain branch | diagnostics and runtime evidence |
| `m_l` | `kg m^-2` | Ice mass in the selected lower thermal volume. | Stage 3 thermal partition | one/two-volume branch |
| `t_collapse` | `s` | Duration for which a resolved pack uses one thermal volume because `0 < m_l < m_res`. | Stage 3 layer branch | diagnostics and runtime evidence |
| `SWE_layer` | `m` | Persistent layer water-equivalent mass depth. | typed `DirectSnowLayerState` | named lifecycle conversion |
| `m_layer` | `kg m^-2` | Persistent layer areal water mass used for lifecycle selection. | named SWE-to-area-mass conversion | density lifecycle |
| `z_layer` | `m` | Persistent layer physical thickness. | typed `DirectSnowLayerState` | density closure and Stage 3 |
| `sigma` | `W m^-2 K^-4` | Stefan-Boltzmann constant. | fixed constant | emission equations |

## Algorithm State Surfaces

### Required inputs

| Surface | Required state |
|---|---|
| Above-canopy meteorology | hourly finite `T_a > 0 K`; daily finite `e_a >= 0 kPa` and `R_s >= 0 MJ m^-2 d^-1` |
| Turbulent forcing geometry | typed positive virtual `z_T`, `z_q`, and `z_u` plus typed positive `z_0,aero`, all relative to the instantaneous modeled snow surface; current geometry is exactly `5 m`, `5 m`, `5 m`, and `0.005 m`; none asserts physical forcing reference height |
| Solar geometry | finite `R_a >= 0 MJ m^-2 d^-1` plus an explicit daylight/polar-night classification |
| Canopy | finite effective daily `C` in `[0, 1)` |
| Thermal provider | supported internal `layered_thermal_liquid_v1`; finite active-layer `T_0 > 0 K`, non-negative finite active/lower cold content, conservative depositional-to-thermal partition, and `T_c=T_a` with the named approximation identity |
| Future melt-owner energy producers | finite, unit-explicit, same-substep net shortwave/longwave radiation, sensible heat, bounded latent exchange, ground/interlayer conduction, and precipitation-advected heat with exact-one lineage; unavailable components block cutover |
| Future phase/liquid inputs | active/lower ice and retained liquid at substep start, solid/liquid precipitation, and one signed vapor exchange, all in explicit area-mass units and chronology |
| Stage 3 precipitation custody | One sealed ordered `P_phase` for every support, including a complete empty set for zero precipitation; exact LSE destination topology and fractions; open raw-liquid or covered vegetation-terminal-liquid source selected exclusively per destination; solid atmospheric precipitation bypasses canopy and remains ground-snow precipitation. |

### Required outputs

`w`, `L_clear`, `epsilon_clear`, `k_t`, `c`, `epsilon_all`, `L_atm`,
`P_0`, `f_sky`, `L_can`, `L_sub`, `L_out`, and `L_net`, each with the
units and lineage declared above.

The future melt-owner path additionally publishes `Q_excess`, `m_melt`,
`m_refrozen`, `delta_m_retained`, `m_routed`, refreeze latent energy, terminal
unallocated energy, and independently reconstructable complete-energy,
solid-to-liquid, and liquid-disposition ledger operands. Their absence is a
cutover blocker, not permission to infer them from adjacent state or output.

### Mutated state surfaces

The longwave evaluator is pure. It may not mutate canopy, snow mass, snow
temperature, cold content, or forcing state. The shared Stage 3 energy carrier
consumes every admitted component exactly once. In the version-7 target it is
solely responsible for mutating cold content and converting bounded positive
excess to melt. Optional sublimation uses the same pre-exchange `T_s`; signed
vapor exchange and latent heat are two views of one transfer.

## Algorithm Specification

Required evaluation order:

1. Validate cadence, units, finiteness, and physical input domains.
2. Once per daylight day, calculate `k_t` and the bounded cloud fraction `c`;
   otherwise take the explicit polar-night unavailable branch.
3. For each hour, evaluate `w`, `L_clear`, `epsilon_clear`,
   `epsilon_all`, and `L_atm` using hourly `T_a` and held daily `e_a`/`c`;
   enforce the no-clamp derived-emissivity guard.
4. Translate the current effective canopy cover to `P_0` and `f_sky`.
5. Partition the current snow column at
   `z_0=min(total_snow_depth, 0.25 m)` except for the `INV-SNOWENERGY-026`
   strict lower-volume collapse, integrating mass, heat capacity, cold
   content, and thermal resistance across depositional-layer boundaries.
   Obtain `T_s=T_0` from the active control volume and set `T_c=T_a` under the
   named homogeneous-stand approximation; stop on missing or invalid state.
6. Evaluate `L_can`, `L_sub`, `L_out`, and `L_net` in the specified order.
7. If sublimation is enabled, evaluate one signed vapor exchange at the same
   `T_s`, bound its loss against post-precipitation ice, derive latent heat from
   that exact bounded exchange, and reserve its sublimation mass before
   calculating melt availability. Define
   `m_deposition=max(m_v,0)`, `m_sublimation=max(-m_v,0)`, and
   `m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)`.
   For current Stage 3 transfer, use the version-10 virtual geometry
   `z_T=z_q=z_u=5 m` and `z_0,aero=0.005 m`; validate each typed length and the
   logarithmic displacement/roughness domain before flux arithmetic.
8. Select the `60`, `15`, or `1 minute` stability substep from the
   Marks/SNOBAL active/lower mass thresholds and reevaluate `T_0`, vapor
   exchange, radiation, and `G_0` at every substep.
9. For the admitted target, sum complete net radiation, sensible heat, latent
   heat, ground/interlayer conduction, and precipitation-advected heat once in
   the applicable active/lower balances. Apply energy to cold content first.
10. Define `Q_excess=max(Q_complete-Q_cold_required,0)` after the active/lower
    allocation and convert only that positive remainder to bounded melt:
    `m_melt = min(Q_excess/L_f, m_ice_available)`. Debit that exact ice mass,
    credit the same liquid mass, and apply same-substep refreeze with its latent
    energy explicitly coupled to cold-content state.
11. Apply the previously evaluated signed vapor mass exchange after
    melt/refreeze, then wet compaction and holding-capacity retention/routing;
    repartition state before the next substep. Publish independently
    reconstructable energy, solid-to-liquid, and liquid-disposition ledgers
    after all guards pass.

The normative mass chronology is precipitation -> energy balance ->
melt/refreeze -> vapor mass mutation -> wet compaction -> retention/runoff.
With all mass operands non-negative except signed store change, the target
identities are:

```text
Q_complete + Q_refreeze - delta_E_cold - L_f*m_melt - Q_unallocated_after_exhaustion = 0
m_ice_start + m_solid_precip + m_deposition - m_ice_end - m_sublimation - m_melt + m_refrozen = 0
m_liquid_external_in + m_melt - m_refrozen - delta_m_retained - m_routed = 0
```

`Q_refreeze=L_f*m_refrozen`. The retained-store operand is
`delta_m_retained=m_liquid_phase_end-m_liquid_phase_start`; its initial level
is not also counted as external input. Define
`Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt`. It must be zero in every
currently admitted resolved substep. A positive value identifies the
unresolved terminal meltout/remaining-energy boundary: it may not be discarded,
carried, or routed by proxy and blocks cutover until a contract amendment
defines its physical recipient and next-state chronology.

Steps 9-11 describe target authority, not current runtime conformance. The
current implementation omits complete sensible and precipitation-advected
heat from this carrier, does not convert `unused_positive_energy_j_m2`, and
keeps CoE as the melt generator. It must remain on `IMPLEMENTATION_HOLD` until
one implementation package closes every component, thin-pack, selector,
default, real-consumer, and rollback gate atomically. A partial energy-melt
path or simultaneous CoE/Stage 3 generation is forbidden.

### Atmospheric longwave

For each hourly evaluation, use hourly air temperature `T_a` in kelvin and
the daily actual vapor pressure `e_a` in kilopascals:

```text
w = 4650 e_a / T_a
L_clear = 59.38
          + 113.7 (T_a / 273.16)^6
          + 96.96 sqrt(w / 25)
epsilon_clear = L_clear / (sigma T_a^4)
```

The constants and units are inseparable from this equation. A vapor pressure
in pascals or temperature in degrees Celsius is invalid.

Evaluate `w`, `L_clear`, `epsilon_clear`, `epsilon_all`, and `L_atm` at the
hourly `T_a`; do not substitute daily mean temperature into the nonlinear
`T_a^6` and `T_a^4` equations. The daily clearness-derived `c` and daily
`e_a` are held constant across that day's hourly evaluations. A future
subdaily humidity/cloud route requires a contract amendment.

When daylight permits a daily clearness index and
`R_a > R_a,min = 1e-9 MJ m^-2 d^-1`:

```text
k_t = R_s / R_a
c = clamp((0.80 - k_t) / (0.80 - 0.15), 0, 1)
epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c
L_atm = epsilon_all sigma T_a^4
```

`R_a,min` is a numeric divide/branch threshold, not a user coefficient or
empirical calibration parameter. The clamp belongs only to the declared
empirical cloud mapping. It must not repair a non-finite input or an invalid
radiation unit.

When `R_a <= R_a,min`, the clearness route is unavailable. Version 2 has no
independently authoritative polar-night cloud producer, so an enabled
longwave cell returns typed `CloudForcingUnavailable`. It must not reuse the
legacy SIMIMPL28 cloud fraction or a prior daylight value. Disabled longwave
cells do not require this operand.

Flerchinger et al. report approximately `24.5 W m^-2` subdaily RMSD and
`14.9 W m^-2` daily RMSD for the Dilley-Unsworth combination across their
development comparison. These are model-form context, not openWEPP
calibration tolerances or guaranteed errors. A daily clearness operator cannot
recover observed subdaily cloud variation, and solar-index inference is
undefined during polar night.

The reviewed authority does not provide a transferable numeric
temperature/humidity envelope for every openWEPP climate. Therefore each
evaluation must require finite `L_clear`, `epsilon_clear`, `epsilon_all`, and
`L_atm`, with `0 <= epsilon_clear <= 1` and `0 <= epsilon_all <= 1`.
Out-of-range derived emissivity is typed `out-of-authority`; it is not clamped
to one. Passing this physical output guard does not assert site validation.

### Effective canopy cover to diffuse sky view

`C` is the effective overhead interception state already produced by the
canopy model. Identify its complement with vertical Beer-law gap fraction:

```text
P_0 = 1 - C = exp(-k_ext VAI_eff)
```

FSM2's hemispherical diffuse transmission is:

```text
f_sky = exp(-1.6 k_ext VAI_eff)
```

Eliminating the unobserved product `k_ext VAI_eff` gives the canonical
openWEPP translation:

```text
f_sky = P_0^1.6 = (1 - C)^1.6
```

This is a model-state translation, not a fitted coefficient. The exponent is
the FSM2 diffuse-transmission factor. The effective extinction coefficient
cancels algebraically, so openWEPP does not request it from the user.

The translation treats the complete effective cover state—including the
native structural floor—as equivalent vertical optical depth under the same
homogeneous, randomly oriented canopy and isotropic diffuse-sky regime. It
does not assert that structural cover is a measured stem-area index.

`LAI`, native structural cover, and canopy height are not separately added:

- daily `C` already includes the leaf-on/leaf-off canopy trajectory and the
  structural-cover floor;
- structural cover is a fraction, not stem-area index, so adding it to LAI
  would be dimensionally and semantically invalid;
- adding LAI again would double count foliage represented in `C`;
- homogeneous Beer-law gap transmission contains no independent height term.

Height and LAI remain diagnostic provenance for `C` and may support future
adjudication; they do not enter this canonical one-layer translation.

### Complementary sub-canopy exchange

`L_atm` arrives as the already evaluated incident atmospheric flux with
variable effective `epsilon_all`. With effective canopy and snow emissivities
fixed to one:

```text
L_can = sigma T_c^4
L_sub = f_sky L_atm + (1 - f_sky) L_can
L_out = sigma T_s^4
L_net = L_sub - L_out
```

Positive `L_net` supplies energy to snow. Negative `L_net` removes energy.
Sky and canopy weights are complementary and must sum to exactly one within
the numeric tolerance.

The admitted first implementation may use above-canopy `T_a` for `T_c` only
as an explicitly named homogeneous-stand approximation. It must not be
described as a prognostic canopy energy balance. Stable nocturnal inversions,
forest edges, large gaps, and strongly sunlit or intercepted-snow canopies are
known limitations.

### Shared Stage 3 active-layer thermal and sublimation composition

The sole version-3 thermal provider is the Marks/SNOBAL active control volume
constructed from the layers carried by
`snow_stage3_liquid_routing_model=layered_thermal_liquid_v1`:

```text
z_0 = min(sum_i(z_i), 0.25 m)
m_0 = integral over [0, z_0] of layer mass
Q_cc,0 = integral over [0, z_0] of layer cold content
T_s = T_0 = 273.15 K - Q_cc,0 / (m_0 c_i)
T_c = T_a
```

The normal `0.25 m` maximum is fixed Marks/SNOBAL structural authority, not a
user coefficient. The sole exception is the exact version-4 lower-volume
collapse, which makes the complete resolved pack active for that substep.
Depositional boundaries do not define the radiating/turbulent
heat capacity. A depositional layer intersected by `z_0` is partitioned
conservatively; mass, depth, liquid, refrozen mass, and cold content must close
before flux evaluation. All material within the active thermal volume shares
the resulting `T_0` after projection. When the pack is no deeper than
`0.25 m`, the complete pack is the active control volume.

When a lower volume exists, derive its aggregate temperature and effective
conductivity from its mass, cold content, and series thermal resistance.
Thermal partitions persist across substeps; a boundary-intersected
depositional layer must not be recombined in a way that erases the current
active/lower temperature gradient.

Use the exact libsnobal `KTS` plus `efcon` conductivity formulation, not the
Sturm density-only frost-insulation relation:

```text
p_a = 101300 Pa * (1 - 0.0065 z_elev / 293 m)^5.26
rho_r = rho_s / (1000 kg m^-3)
k_d = 4.186798188 * 0.0077 * rho_r^2
D_e = 0.65 * (101324.6 Pa / p_a) * (T_s / 273.16 K)^14 * 1e-4 m^2 s^-1
w_s = (18.0153 / 28.9644) * e_si(T_s) / (p_a - e_si(T_s))
k_eff = k_d + L_s(T_s) D_e w_s
```

Here `e_si` is the admitted SNOBAL ice-saturation relation already used by
the vapor provider. Elevation is existing climate metadata; `p_a` is derived
internally and is not a user coefficient. Each volume's series resistance
uses its current shared temperature. The SNOBAL harmonic interface exchange
is:

```text
G_0 = 2 k_0 k_l (T_l - T_0) / (k_l z_0 + k_0 z_l)
```

where positive `G_0` supplies heat to the active layer. The same transfer is
`-G_0` in the lower-layer balance; it cancels exactly in the whole-pack
ledger. A transfer first satisfies the receiving control volume's available
cold content. Under the version-7 target, any positive whole-pack residual then
enters the single bounded phase-change ledger; it is neither discarded nor
also supplied to a CoE generator. The current runtime's reported-but-
unconverted excess is an explicit implementation hold.

Use the minimum active/lower control-volume mass to select the stability
timestep:

```text
m_min >= 60 kg m^-2  -> 60 minute substeps
10 <= m_min < 60     -> 15 minute substeps
m_min < 10           -> 1 minute substeps
```

If no lower volume exists, `m_min=m_0`. The `1 kg m^-2` small-timestep
threshold is also the exact libsnobal terminal resolved-pack boundary. Before
partitioning or constructing `T_0`, conductivity, or the next carrier,
calculate total represented ice mass `m_s`. If
`m_s <= m_res = 1 kg m^-2`, the Stage 3 thermal and exchange domain is
unresolved for the remainder of the hour:

```text
Q_shortwave = Q_longwave = Q_latent = 0
m_v = m_sub = 0
G_0 = Q_E = 0
t_unres += remaining hour duration
```

When `m_s > m_res`, select the normal active/lower partition. If a lower volume
exists and `0 < m_l < m_res`, do not evaluate that sub-resolution volume.
Collapse the thermal partition to one whole-pack active volume for the current
substep and continue the ordinary surface-energy solve. Existing conservative
projection may coalesce thermally identical fragments, but total mass, liquid,
refrozen mass, and cold content must remain closed:

```text
active thermal volume = complete represented pack
lower thermal volume = none
t_collapse += substep duration
```

The lower-volume comparison is strict: `m_l = 1 kg m^-2` remains a resolved
two-volume solve. This reproduces libsnobal `_calc_layers.c` ordering and
branch sides; it is distinct from the `m_s <= 1 kg m^-2` no-layer branch.

The current implementation adopts libsnobal's resolved-layer boundary but not
its residual-snow-to-water conversion. During the implementation hold, CoE
remains the compatibility runtime owner, so persistent layer mass, liquid,
refrozen mass, and cold content remain unchanged by suspended Stage 3
exchange. The version-7 target does not authorize carrying that behavior into
cutover: exact residual-snow disposition at `m_s <= 1 kg m^-2` must be derived,
implemented, and independently closed before activation. Until then the
unresolved branch must not fabricate temperature, conductivity, vapor
exchange, or phase change, and no partial Stage 3 melt path may activate.

All four current `B/L/S/LS` cells use this provider and the same compatibility
CoE melt, density, phase, liquid-routing, forcing, and albedo selections. That
fact records current implementation, not target authority. Longwave and
sublimation are separate default-off selectors. Enabling either without the
Stage 3 provider is a typed missing-provider error. Enabling the new
sublimation selector together with legacy
`coe_open_sublimation_stage_a_v1` or
`coe_open_sublimation_stage_b_v1` is a typed incompatible-selection error,
preventing double mass loss.

For the retained Marks/SNOBAL-lineage neutral exchange, calculate one
loss-positive substep sublimation amount `m_sub >= 0` from the same `T_s` used
by longwave, then define:

```text
m_v = -m_sub
q_v = m_v / delta_t
Q_latent = q_v L_s(T_s)
```

where `delta_t` is the selected substep duration, `q_v` is in
`kg m^-2 s^-1`, and `L_s(T_s)` is the temperature-appropriate latent heat of
sublimation in `J kg^-1`. Thus
`Q_latent <= 0` during sublimation. The implementation must derive both mass
and energy from `q_v`; it may not independently recompute either view.

The current compatibility substep carrier is:

```text
Q_surface = Q_shortwave
            + I_L L_net
            + I_S Q_latent
Q_E,0,potential = (Q_surface + G_0) delta_t
Q_E,l,potential = -G_0 delta_t
```

where `I_L` and `I_S` are the independent longwave and sublimation selector
indicators. Before the atomic version-7 cutover, Stage 3 may apply only the
portion that changes cold content; unused positive potential is reported and
is not converted to melt. Sublimated ice is removed from the
active surface downward after the coupled update. Cold content associated
with that removed ice is a separately reported energy export. Active and
lower ledgers retain `G_0` with opposite signs; it cancels in whole-pack
closure:

```text
Q_surface,applied + Q_refreeze + Q_cc,export
    = Q_cc,before - Q_cc,after
```

and post-CoE vapor mass closure is:

```text
M_ice,before - M_ice,after = M_sublimation
```

Sublimation must not enter routed melt, retained liquid, released liquid, or
refreeze operands.

### Raw opportunity versus bounded vapor transfer

The current evaluation-only schema-v6 carrier exposes
`vapor_mass_exchange_kg_m2 = m_v,raw`. For sequential `Q`, independently derive

```text
m_deposition = max(m_v,raw, 0)
m_sublimation = min(max(-m_v,raw, 0), m_active_ice_before)
m_v = m_deposition - m_sublimation
```

and compare both bounded components to producer fields before aggregation.
Same-state `S` and frozen-active `F` prohibit mutation, so actual bounded
transfer is N/A even when raw opportunity is nonzero. Numeric zero cannot
replace N/A.

The evaluation carrier's `latent_flux_w_m2` is raw turbulent latent-energy
opportunity paired with `m_v,raw`. When the availability bound is inactive,
raw and bounded mass/latent views coincide. When sublimation opportunity
exceeds active ice, raw latent opportunity is not latent energy of actual
bounded transfer. Report that valid capacity truncation as
`VAPOR_OPPORTUNITY_TRANSFER_MISMATCH`; do not hide it in melt, liquid,
deposition, a median, or an endpoint residual. `INV-SNOWENERGY-017/029`
continue to require one bounded `m_v` for the future production target's
exact-one mass and latent-energy ledgers. Version 9 authorizes
characterization only and makes no production correction.

The independent consumer reconstructs the current evaluation chronology per Q
tuple, distinct from the future bounded-latent production target:

```text
Q_latent_raw = latent_flux_w_m2 * duration_seconds = m_v,raw * L_s(T_s)
Q_latent_bounded = m_v * L_s(T_s)
Q_latent_truncation = Q_latent_raw - Q_latent_bounded
C1 = C0 - G
E_raw = independently reconstructed external_flux_w_m2 * duration_seconds
surface_change = if E_raw >= 0 then min(E_raw, C1) else E_raw
active_cold_change = G + surface_change
lower_cold_change = -G
Q_complete_raw = E_raw + G
Q_excess_raw = max(Q_complete_raw - active_cold_change, 0)
m_ice_available = max(m_active_ice_before - m_sublimation, 0)
m_melt_raw_carrier = min(Q_excess_raw / L_f, m_ice_available)
Q_unallocated_raw = max(Q_excess_raw - L_f * m_melt_raw_carrier, 0)
```

Here `C0` is active cold content before the substep and `G` is the emitted
active-side internal-conduction primitive; schema v6 does not independently
recompute layerwise conductivity. The consumer compares each reconstructed
cold, melt, and closure operand to producer fields, then checks mass endpoints.
It preserves that the as-built melt chronology consumes `E_raw`; it must not
relabel `Q_latent_raw` or `m_melt_raw_carrier` as conformance to the future
bounded-latent production target.

## Branch and Guard Table

| Branch/condition | Required behavior | Guard class | Failure class |
|---|---|---|---|
| Any required scalar is non-finite | Reject before arithmetic. | runtime | typed invalid forcing/state |
| Any turbulent height or `z_0,aero` is non-finite/non-positive, or a measurement height does not exceed the displacement/roughness boundary | Reject before logarithms or stability iteration; do not substitute another forcing profile. | runtime | typed invalid turbulent geometry |
| `T_a <= 0 K`, `T_c <= 0 K`, or `T_s <= 0 K` | Reject. | runtime | typed invalid temperature |
| `e_a < 0 kPa`, `R_s < 0`, or `R_a < 0` | Reject. | runtime | typed invalid forcing |
| Derived `L_clear`, `epsilon_clear`, `epsilon_all`, or `L_atm` is non-finite, or either emissivity is outside `[0,1]` | Reject without clamping. | runtime | typed out-of-authority atmospheric state |
| Daylight and `R_a > R_a,min` | Calculate `k_t`, clamp only the empirical cloud mapping, and continue. | runtime | none |
| Polar night or `R_a <= 1e-9 MJ m^-2 d^-1` with longwave enabled | Do not divide, reuse SIMIMPL28 cloud fraction, or carry a prior value; return typed unavailable state. | runtime | typed cloud-forcing unavailable |
| `C < 0` or `C >= 1` | Reject; do not silently clamp. | runtime | typed invalid canopy state |
| `C = 0` | Require `f_sky = 1` and `L_sub = L_atm`. | test | blocked promotion on mismatch |
| `C -> 1` within valid domain | Require `f_sky -> 0` and `L_sub -> L_can`. | test | blocked promotion on mismatch |
| Stage 3 thermal provider absent while longwave or sublimation is enabled | Do not publish or mutate candidate energy/mass state. | runtime | typed missing thermal provider |
| New sublimation selector plus legacy Stage A/B melt variant | Reject before hourly processing. | runtime | typed incompatible selector |
| Sublimation demand exceeds available ice | Bound the transfer to available ice and derive latent energy from the bounded transfer. | runtime | none; bounded physical availability |
| Melt and sublimation both demand the same substep ice | Reserve bounded `m_sublimation` first, bound melt by the remaining `m_ice_available`, and apply the already evaluated vapor mutation after melt/refreeze. Deposition cannot enlarge melt availability. | runtime/test | typed joint-availability or mass-closure failure |
| Total snow depth is `<= 0.25 m` | Use the complete pack as the active thermal control volume. | runtime | none |
| Total represented ice mass is `<= 1 kg m^-2` | Before thermal partition, preserve persistent CoE/layer state and suspend all Stage 3 thermal, radiation, conduction, and vapor exchange without constructing temperature or conductivity. | runtime/model domain | explicit unresolved-duration and total-mass diagnostics; no typed thermal failure |
| Total mass is `> 1 kg m^-2` and `0 < m_l < 1 kg m^-2` | Conservatively project to one whole-pack thermal volume and continue normal exchange; thermally identical fragments may coalesce only with closed aggregate state. | runtime/model domain | explicit collapse-duration and lower-mass diagnostics |
| Lower thermal mass is exactly `1 kg m^-2` | Retain the resolved active/lower two-volume solve. | runtime/model domain | no collapse diagnostic; ordinary coupled-state guards |
| A depositional layer crosses `z_0` | Partition/project conservatively; reject nonclosing state. | runtime | typed thermal-partition closure failure |
| Active/lower mass selects a smaller timestep | Execute every required substep and reevaluate the coupled state; do not retain an hourly energy debit. | runtime | typed cadence/closure failure |
| Coupled update would require `T <= 0 K` | Reject; no clamp, temperature replacement, or cold-content tax is allowed. | runtime | typed invalid thermal state / blocked campaign |
| `T_c = T_a` approximation active | Emit/retain explicit approximation identity in configuration or diagnostics. | profile | blocked promotion if unlabeled |
| Canopy is outside equivalent homogeneous/random-orientation/isotropic-diffuse regime | Do not expand the claim; retain a diagnostic/model-limitation classification. | governance | model limitation |
| Any target energy component is unavailable, duplicated, non-finite, or lacks exact-one lineage | Do not activate the Stage 3 melt owner. | governance before cutover; runtime after cutover | hard `IMPLEMENTATION_HOLD`; typed energy-input failure after cutover |
| `Q_excess > 0` with available ice | Apply `m_melt=min(Q_excess/L_f,m_ice_available)` once after cold-content satisfaction. | runtime/test | typed energy/phase closure failure |
| CoE and Stage 3 melt generation both selected or reachable | Reject the configuration/cutover; never blend outputs. | governance/runtime | hard `IMPLEMENTATION_HOLD`; typed incompatible-owner failure after cutover |
| Any energy, solid, or liquid ledger does not reconstruct from exact operands | Reject without residual acceptance, alias substitution, or clamp. | runtime/test | hard `IMPLEMENTATION_HOLD`; typed closure failure after cutover |
| Available-ice bound saturates and `Q_unallocated_after_exhaustion > 0` | Do not discard, carry, or route the energy by proxy; target cutover remains blocked until canonical physical recipient and next-state chronology are amended. | governance/model boundary | hard `IMPLEMENTATION_HOLD` |
| `m_s <= 1 kg m^-2` or terminal meltout would enter unresolved residual snow | Preserve current compatibility behavior only; do not activate a partial target phase path. | governance/model boundary | hard `IMPLEMENTATION_HOLD` |
| shared carrier lacks sealed exposure, has duplicate flux lineage, or has invalid participant/support receipt | reject before trial state or ledger mutation | runtime | `SNOWENERGY-E-WIND-001` / `SNOWENERGY-E-CARRIER-001` |
| snow flux is requested after the accepted event or in a snow-free regime | reject before evaluation | runtime | `SNOWENERGY-E-REGIME-001` |
| canopy-intercepted snow is supplied to this surface carrier | reject as out of scope | runtime | `SNOWENERGY-E-SCOPE-001` |
| component-weighted canopy longwave or reciprocal ledger does not close | reject without candidate publication | runtime | `SNOWENERGY-E-LW-001` |

`R_a,min` is the numerically explicit `1e-9 MJ m^-2 d^-1`
divide/branch threshold.

## Invariants and Guard Map

### Invariants

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-SNOWENERGY-001` | Temperatures entering fourth-power emission are finite kelvin values greater than zero. | `REF-SNOWENERGY-PHYSICAL` | `[INFERENCE][Static]` | pre-arithmetic temperature guard | typed invalid temperature |
| `INV-SNOWENERGY-002` | Vapor pressure is finite, non-negative, and expressed in `kPa`. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | forcing/unit boundary | typed invalid forcing |
| `INV-SNOWENERGY-003` | `0 <= c <= 1`; clamping occurs only after a valid finite daylight `k_t` exists. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | cloud branch guard | typed invalid or unavailable cloud forcing |
| `INV-SNOWENERGY-004` | `0 <= C < 1`, `P_0=1-C`, and `f_sky=(1-C)^1.6`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PLANT` | `[DIRECT][Static] + [INFERENCE][Static]` | canopy-domain guard and vectors | typed invalid canopy state |
| `INV-SNOWENERGY-005` | `0 < f_sky <= 1` and sky view decreases monotonically with `C` on the valid domain. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | analytical property test | blocked contract/runtime promotion |
| `INV-SNOWENERGY-006` | Sky and canopy weights are complementary: `f_sky+(1-f_sky)=1`. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-RUTTER2023` | `[DIRECT][Static]` | independent reconstruction | blocked contract/runtime promotion |
| `INV-SNOWENERGY-007` | At `C=0`, `L_sub=L_atm`; as `C->1`, `L_sub->sigma T_c^4`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | limiting-case vectors | blocked contract/runtime promotion |
| `INV-SNOWENERGY-008` | `L_net=L_sub-sigma T_s^4`, positive toward snow. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static]` | sign and reconstruction tests | typed energy-evaluation failure |
| `INV-SNOWENERGY-009` | LAI, structural cover, and height are not independently added to effective cover in this translation. | `REF-SNOWENERGY-PLANT`, `REF-SNOWENERGY-FSM2` | `[DIRECT][Static] + [INFERENCE][Static]` | contract/source review | hard `HOLD` on alternate composition |
| `INV-SNOWENERGY-010` | No production snow-energy flux is published before EB-03 supplies coherent `T_c`, `T_s`, and cold-content coupling. | `REF-SNOWENERGY-EB01A` | `[DIRECT][Static]` | runtime-consumer closure gate | hard `HOLD` |
| `INV-SNOWENERGY-011` | Polar-night cloud inference never divides by zero or silently reuses unrelated legacy cloud state. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | daylight/cloud branch guard | typed cloud-forcing unavailable |
| `INV-SNOWENERGY-012` | Package-local analytical code is evidence only and is never imported by production crates. | ADR-0011 and package scope | `[DIRECT][Static]` | source/write-set inventory | blocked package closure |
| `INV-SNOWENERGY-013` | Atmospheric longwave uses hourly `T_a`; daily `e_a` and clearness-derived `c` may be held across the day. Daily-mean-temperature substitution is prohibited. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | cadence/aggregation test | typed cadence mismatch / hard `HOLD` |
| `INV-SNOWENERGY-014` | Derived atmospheric fluxes are finite and both effective emissivities lie in `[0,1]`; out-of-range results fail without clamping. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | derived-domain guard | typed out-of-authority state |
| `INV-SNOWENERGY-015` | All `B/L/S/LS` cells use the same Stage 3 top-layer `T_s`/cold-content provider; `T_c=T_a` is explicitly identified. | `REF-SNOWENERGY-EB01A`, `REF-SNOWENERGY-RUTTER2023`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085` | `[DIRECT][Static] + [INFERENCE][Static]` | selector/provider guard and real-consumer test | typed missing provider / blocked campaign |
| `INV-SNOWENERGY-016` | In the current compatibility `B/L/S/LS` cells, longwave and sublimation are orthogonal default-off selectors and neither changes the CoE melt-model selector. This current-runtime selector invariant does not authorize CoE after the atomic `INV-SNOWENERGY-029` cutover. | snow-surface EB roadmap, `REF-SNOWENERGY-21N` | `[DIRECT][Static]` | selector matrix test | blocked EB-04 admission or partial cutover |
| `INV-SNOWENERGY-017` | Signed vapor mass and latent heat are derived from one bounded exchange at the shared `T_s`; sublimation is negative latent energy and cannot be debited twice. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent latent/mass reconstruction | typed closure failure |
| `INV-SNOWENERGY-018` | Sublimation reduces ice storage only and never aliases routed melt, retained/released liquid, or refreeze. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-073`, `#INV-SNOWFREEZE-076` | `[DIRECT][Static]` | independent mass closure and alias-separation test | typed closure failure |
| `INV-SNOWENERGY-019` | Cold-content change closes from applied surface energy, interlayer conduction, refreeze energy, and exported cold content on the declared control volume. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-080`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent energy reconstruction | typed closure failure |
| `INV-SNOWENERGY-020` | The radiating/turbulent control volume is normally the upper `min(z_s,0.25 m)` of snow and is independent of snowfall-event boundaries; `INV-SNOWENERGY-026` exclusively authorizes whole-pack depth for the strict sub-resolution-lower-volume collapse. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | active-layer partition and anti-alias test | typed partition failure |
| `INV-SNOWENERGY-021` | Active/lower mass, depth, cold content, and thermal resistance reconstruct the persistent column exactly before and after projection. | physical conservation, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static] + [INFERENCE][Static]` | independent partition reconstruction | typed closure failure |
| `INV-SNOWENERGY-022` | `G_0` is positive into the active layer, appears as `-G_0` in the lower balance, and cancels from the whole-pack ledger. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | sign, limiting, and reconstruction tests | typed closure failure |
| `INV-SNOWENERGY-023` | Mass-dependent `60/15/1 minute` substeps are selected from the `60/10/1 kg m^-2` Marks/SNOBAL thresholds; substep fluxes are reevaluated from current state. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-LUTE2022` | `[DIRECT][Static]` | cadence and thin-pack tests | typed cadence failure / blocked campaign |
| `INV-SNOWENERGY-024` | No active/lower update may use an absolute-zero clamp, air-temperature replacement, fitted cold-content tax, or user limiter. | `REF-SNOWENERGY-PHYSICAL`, EB-03A authority envelope | `[INFERENCE][Static]` | source scan and physical-domain tests | hard `HOLD` |
| `INV-SNOWENERGY-025` | Active and lower partitions retain distinct shared temperatures across substeps, and `G_0` uses libsnobal `KTS+efcon` effective conductivity with elevation-derived pressure; the Sturm frost-insulation relation is not an admissible substitute. | `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-ANDERSON1976` | `[DIRECT][Static]` | unequal-temperature persistence and conductivity vectors | typed conductivity/projection failure |
| `INV-SNOWENERGY-026` | At total represented ice mass `m_s <= 1 kg m^-2`, the current Stage 3 thermal/exchange domain is unresolved before partition: compatibility CoE and persistent-layer mass, liquid, refrozen mass, cold content, and topology are preserved; no temperature, conductivity, surface energy, conduction, vapor exchange, sublimation, or melt alias is produced. For `m_s > 1`, `0 < m_l < 1 kg m^-2` collapses to one whole-pack thermal volume and continues exchange, while `m_l = 1` remains two-volume. Version 7 retains this as current-runtime behavior but blocks target cutover until residual-snow phase authority is complete. | `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-21N`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | exact total-mass threshold sides, strict lower-layer collapse/equality, state-preservation, resume, and real-consumer trace tests | typed closure failure above boundary / blocked campaign on alias or mutation |
| `INV-SNOWENERGY-027` | Persistent density layers are retained or removed by the density model's mass-unit lifecycle boundary: `m_layer = rho_w * SWE_layer > 1e-9 kg m^-2` is represented and `m_layer <= 1e-9 kg m^-2` is zero mass. The independent `1e-9 m` SWE and physical-depth aggregate tolerances test residual closure only; neither may delete a represented layer. Retained mass, depth, liquid, refrozen mass, cold content, density, and settle state remain coupled. | `INV-SNOWENERGY-021`, physical conservation, dimensional consistency | `[DIRECT][Static] + [INFERENCE][Static]` | named SWE-to-mass predicate, exact-side tests, independent aggregate reconstruction, typed mismatch outside tolerance | typed aggregate mismatch / blocked campaign on cross-unit filtering or state deletion |
| `INV-SNOWENERGY-028` | A `1e-9 m` SWE closure residual is exactly `1e-6 kg m^-2` when the same residual is expressed as area mass through `rho_w=1000 kg m^-3`; vapor-to-sublimation transfer closure uses that `1e-6 kg m^-2` bound. This conversion does not alter the separately governed `1e-9 kg m^-2` hourly/daily vapor-aggregation reconstruction tolerance or the `1e-9 kg m^-2` represented-layer lifecycle boundary (`1e-12 m` SWE). | `INV-SNOWENERGY-017`, `INV-SNOWENERGY-018`, `INV-SNOWENERGY-027`, named unit conversion, dimensional consistency | `[DIRECT][Static] + [INFERENCE][Static]` | operand-specific independent reconstruction and named SWE-to-area-mass conversion | typed closure failure / blocked adjudication on cross-predicate substitution |
| `INV-SNOWENERGY-029` | The admitted future melt owner is Stage 3 alone. In each resolved stability substep, complete net radiation, sensible heat, latent heat, ground/interlayer conduction, and precipitation-advected heat satisfy cold content first. Split the already bounded signed vapor exchange exactly as `m_deposition=max(m_v,0)` and `m_sublimation=max(-m_v,0)`; reserve sublimation from post-precipitation ice; define `m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)`; then define `Q_excess=max(Q_complete-Q_cold_required,0)` after active/lower allocation and convert only that remainder as `m_melt=min(Q_excess/L_f,m_ice_available)`. `Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt` must be zero; a positive value is an unresolved terminal boundary and blocks cutover. The CoE `A/B/C/D`, `C_canopy`, daily midpoint gate, embedded albedo, and rain-heat terms are compatibility diagnostics only and cannot generate melt after cutover. | `REF-SNOWENERGY-21N`, `REF-SNOWENERGY-LIBSNOBAL`, physical energy/phase conservation | `[DIRECT][Static] + [INFERENCE][Static]` | complete-component, cold-content-first, joint vapor/melt availability, latent-fusion, terminal-energy, and exact-one-owner gates | hard `IMPLEMENTATION_HOLD` until complete; typed energy/mass closure failure after cutover |
| `INV-SNOWENERGY-030` | Stage 3-generated liquid is debited from ice and credited to the single liquid handoff exactly once in the same substep, then passes through refreeze, retention, and routing before thermal repartition. The energy ledger includes latent heat released by refreeze; the solid ledger credits refrozen liquid back to ice; and the liquid ledger debits that same refrozen mass. All three reconstruct independently from exact operands. Simultaneous CoE/Stage 3 melt, discarded positive energy, delayed duplicate routing, or an unresolved `m_s <= 1 kg m^-2` phase proxy is prohibited. | `REF-SNOWENERGY-21N`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-091`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | same-substep chronology, linked-ledger reconstruction, thin-pack authority, and real-consumer cutover gates | hard `IMPLEMENTATION_HOLD`; typed closure failure after cutover |
| `INV-SNOWENERGY-031` | Stage 3 uses explicit virtual transfer heights `z_T=z_q=z_u=5 m` above the instantaneous modeled snow surface and exposed-snow aerodynamic roughness `z_0,aero=0.005 m`. These are model geometry, not physical measurement/reference heights, observations, calibration parameters, or aliases of active thermal-layer depth `z_0`. All four values cross a typed runtime boundary and satisfy the logarithmic displacement/roughness domain before evaluation. | `REF-SNOWENERGY-LIBSNOBAL`, user authority dated 2026-08-05, `INV-SNOWENERGY-033` | `[DIRECT][Static]` | exact-value projection, typed-domain, source-height/geometry non-alias, and sensitivity tests | typed invalid turbulent geometry / blocked cutover |
| `INV-SNOWENERGY-032` | Evaluation schema-v6 preserves `m_v,raw` and `Q_latent_raw=m_v,raw L_s(T_s)` as raw opportunities and actual sequential transfer separately as bounded deposition/sublimation with `Q_latent_bounded=m_v L_s(T_s)`. `S/F` actual transfer is N/A. For Q, the consumer reconstructs bounded transfer plus the exact characterization-only `C0/G/C1/E_raw/surface_change/active_change/lower_change/Q_complete_raw/Q_excess_raw/availability/melt/unallocated` chronology before producer and endpoint checks. Producer disagreement, simultaneous transfer, wrong direction, numeric-zero N/A, melt/liquid aliasing, or nonclosure is invalid evidence. Valid capacity truncation and `Q_latent_truncation` are `VAPOR_OPPORTUNITY_TRANSFER_MISMATCH` and block passage/persistence. The raw-latent chronology is not future bounded-latent target conformance. | `INV-SNOWENERGY-017/018/029`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-096`, physical mass/energy conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent tuple-level raw/bounded mass/latent reconstruction, operator-order chronology, anti-alias tests, and endpoint/energy closure | evidence hard-fail on malformed/alias/nonclosure; governance hold on physical passage |
| `INV-SNOWENERGY-033` | WIND-SOURCE-CUSTODY-AND-EXPOSURE: retained CLI `w-vl` is parsed as raw `vwind` and reaches Stage 3 as raw `vwind_m_s`; PMET alone creates `u_2,PMET` and that value cannot feed snow. GRIDMET `vs` product metadata describes daily nominal `10 m` wind, while Stage 3 `z_u=5 m` is virtual snow-surface-relative transfer geometry. Surviving WEPPpy runs directly prove byte-identical CLI lineage, retained watershed centroids and GRIDMET-enabled flags, complete daily parquet wind, and exact parquet-to-CLI equality. The nearest pre-build code statically reconstructs watershed-centroid GRIDMET `vs` requests, shared run-level wind, and one-decimal formatting; it is not deployed-code or request evidence. Exact deployed identity/request/response, product version/status, server-side pixel/sampling, day boundary, missing policy, source datum, and physical exposure remain `AUTHORITY_MISSING` unless directly retained. Modeled forest/`cancov=0.9` is target intent, not physical exposure or linkage. Neither values, residuals, a height conversion, nor a desired energy balance can establish forest/sub-canopy applicability, fit attenuation, license a canopy operator, or authorize production correction. | `REF-SNOWENERGY-GRIDMET`, `REF-SNOWENERGY-WIND-CUSTODY`, `INV-SNOWENERGY-031`, ADR-0042 | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | literal source/consumer alias tests, provider/fixture hash and parquet/CLI equality, static provider-code reconstruction, custody ledger, independent neutral-height diagnostic, and two-sided exposure-authority matrix | governance `HOLD` on remaining custody/applicability; no production correction |
| `INV-SNOWENERGY-034` | SNOW-TERMINAL-ENTHALPY-EVENT-NUMERICS: only `persistent_accumulation_shadow_v1` may enter an evaluation-only terminal snow domain when post-precipitation represented ice is `0 < m_i <= 1 kg m^-2`. Collapse the complete snow column to one enthalpy-bearing control volume without deleting mass: canonical cold-content deficit `Q_cc >= 0 J m^-2`, retained liquid `m_l >= 0 kg m^-2`, and material enthalpy `H=-Q_cc+L_f m_l` relative to 0 C ice. Use the existing complete bounded Stage 3 carrier and its current-state surface temperature; do not introduce a heat-capacity epsilon, clamp, fitted threshold, cold-content tax, or new flux equation. A deterministic first-order transition map is integrated with step doubling: compare one trial of `h` with two sequential trials of `h/2`, accept the two-half state only when the componentwise scaled ice/liquid/cold-content/energy norm satisfies `TOL-SNOWENERGY-001`, otherwise halve `h`; `h <= 60 s`, `h_min=1e-9 s`, at most 64 consecutive rejections, and any nonfinite/domain/nonconvergence result is typed failure with no state commit. Each trial reevaluates the carrier from its start state. Define `Delta H_cc=Q_cc,start-Q_cc,end`, positive for warming and negative for cooling. Apply energy to `Q_cc` and refreeze first, reserve bounded sublimation before melt availability, define `Q_excess=max(Q_complete+Q_refreeze-Delta H_cc,0)` and `m_melt=min(Q_excess/L_f,max(m_i-m_sublimation,0))`, then apply deposition after same-trial melt availability. Because entry is explicitly post-precipitation, the actual endpoint-solid function is `g(tau)=m_i,start+m_refrozen(tau)+m_deposition(tau)-m_sublimation(tau)-m_melt(tau)`; deposition/refreeze cannot retroactively enlarge same-trial melt availability. Require `g(0)=m_i,start`, bounded `g>=0`, and no event while deposited or refrozen solid remains. When an accepted trial first reaches the mass-root tolerance, replay from the immutable pretrial state and localize the earliest event by safeguarded bisection; preserve the positive/terminal bracket, require monotonically nonincreasing candidate solid, and stop only when both bracket width and endpoint solid satisfy `TOL-SNOWENERGY-001`, otherwise fail after 64 iterations. At the accepted upper endpoint, the complete solid identity—not a debit clamp—must establish zero ice; terminal liquid equals retained/external liquid plus melt less refreeze. The snow energy identity closes through `Delta H_cc`, fusion, refreeze, and `Q_terminal_unallocated=Q_complete+Q_refreeze-Delta H_cc-L_f m_melt >= 0`, which is explicitly censored and may be positive when sublimation exhausts solid first. No snow-domain state receives energy and no snow flux is evaluated after the event. Publish `evaluated_seconds=t_event`, `unevaluated_seconds=requested-t_event`, and censored terminal liquid/energy handoffs; neither is a land-surface recipient. This mechanics-only exception supersedes the `INV-SNOWENERGY-026` no-evaluation branch only for the named operator and terminal domain; compatibility/default paths and historical schemas remain exact. | `REF-SNOWENERGY-LIBSNOBAL`, `INV-SNOWENERGY-017/023/026/029/030`, physical conservation, deterministic numerical analysis | `[DIRECT][Static] + [INFERENCE][Static]` | exact boundary sides, step-doubling refinement, event bracket/order, joint vapor/melt, deposition/refreeze no-false-event, cooling/no-event, typed nonconvergence, atomicity, independent schema-v8 reconstruction, and production isolation | evaluation hard-fail + governance claim limit |

| `INV-SNOWENERGY-035` | A default-off terminal receiver may consume only the earliest closed INV-034 event with in-tolerance unallocated energy, exact half-open support, and one atomic retained/rain/melt/refreeze liquid debit-credit-consumed join; INV-034 remains evaluation-only and CoE remains production owner. | `INV-SNOWENERGY-030/034`, physical conservation | `[INFERENCE][Static]` | receipt, energy, support, and production-isolation guards | typed terminal-receiver failure; no recipient/commit |

### Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-SNOWENERGY-001` | future atmospheric/exchange evaluator; analytical invalid-temperature cases | runtime | typed invalid temperature | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-002` | future forcing boundary; negative/non-finite vapor tests | runtime | typed invalid forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-003` | future daylight cloud operator; endpoint/clamp cases | runtime | typed invalid/unavailable cloud forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-004` | future canopy translator; valid/invalid cover cases | runtime | typed invalid canopy state | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-005` | monotonic analytical vector/property gate | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-006` | independently fixed complementary-mixture vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-007` | open/near-closed canopy limiting vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-008` | future shared energy carrier and fixed net-flux vectors | runtime | typed evaluation failure | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-009` | contract review and production source inspection | governance | hard `HOLD` | EB-02 `canopy-sky-view-derivation.md` |
| `INV-SNOWENERGY-010` | EB-03 provider decision and real-consumer closure gate | governance | hard `HOLD` | `GAP-SNOWENERGY-001` |
| `INV-SNOWENERGY-011` | explicit polar-night unavailable branch | runtime | typed cloud-forcing unavailable | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-012` | intended/exact write-set reconciliation | governance | blocked package closure | EB-02 `exact-diff-reconciliation.md` |
| `INV-SNOWENERGY-013` | hourly-forcing/provider cadence assertion | profile | typed cadence mismatch / hard `HOLD` | EB-02 `operand-lineage.csv`; EB-03 evidence required |
| `INV-SNOWENERGY-014` | future atmospheric derived-domain guard; out-of-authority vector | runtime | typed out-of-authority state | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-015` | Stage 3 selector/provider validation and hourly diagnostics | runtime/profile | typed missing provider or blocked campaign | EB-03 runtime tests |
| `INV-SNOWENERGY-016` | direct-production selector matrix | runtime/test | typed selector failure or blocked EB-04 | EB-03 consumer-path evidence |
| `INV-SNOWENERGY-017` | shared vapor/latent operands and reconstruction residual | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-018` | snow-layer/aggregate mutation and liquid-alias separation | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-019` | Stage 3 cold-content closure ledger | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-020` | active-volume constructor and surface-provider diagnostics | runtime/test | typed partition failure | EB-03A evidence |
| `INV-SNOWENERGY-021` | independent persistent/thermal partition reconstruction | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-022` | coupled active/lower energy update | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-023` | mass-selected substep scheduler | runtime/test | typed cadence failure | EB-03A real-consumer evidence |
| `INV-SNOWENERGY-024` | production-source scan and invalid-state vectors | governance/test | hard `HOLD` | EB-03A review and verification |
| `INV-SNOWENERGY-025` | SNOBAL effective-conductivity primitive and persistent unequal-temperature runtime vector | runtime/test | typed conductivity/projection failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-026` | pre-temperature total-ice-mass branch plus unresolved-domain trace diagnostics | runtime/test/profile | preserve state and emit zero exchange below boundary; typed guards remain above boundary | EB-04C contract, replay, and conservation evidence |
| `INV-SNOWENERGY-027` | multilayer density initialization and typed aggregate replay | runtime/test/profile | retain mass-resolved layers; independently reject mass/depth residual beyond tolerance | EB-04D contract, exact-side tests, replay, and conservation evidence |
| `INV-SNOWENERGY-028` | independent snow-mass, vapor aggregation, and vapor-to-sublimation reconstructions | test/profile/governance | apply each operand's unit-explicit tolerance; reject cross-predicate substitution | EB-04E prospective protocol and EB-04S result-blind authority reconciliation |
| `INV-SNOWENERGY-029` | future Stage 3 complete energy carrier and sole melt generator | runtime/test/governance | no partial activation; typed closure failure after cutover | 21N authority decision and future implementation evidence |
| `INV-SNOWENERGY-030` | future same-substep phase/liquid pipeline and three independently reconstructed ledgers | runtime/test/governance | no dual owner, alias, delay, or unresolved thin-pack proxy | 21N chronology and future real-consumer evidence |
| `INV-SNOWENERGY-032` | package-local independent schema-v6 consumer | test/governance | reject invalid evidence; preserve valid capacity truncation as a physical finding and keep persistence held | Stage 3 evolving-carrier plausibility package |
| `INV-SNOWENERGY-033` | contract-derived alias/source checks and wind-custody package evidence | governance | `AUTHORITY_MISSING` / persistence hold; no production correction | Stage 3 wind source-custody package |
| `INV-SNOWENERGY-034` | terminal one-volume evaluator, adaptive step controller, event bracket/localizer, and schema-v8 consumer | evaluation runtime/test/governance | typed domain, step-underflow, rejection-limit, bracket, iteration, or closure failure; no state commit | terminal enthalpy-event package |
| `INV-SNOWENERGY-035` | terminal receiver event/receipt join and independent reconstruction | default-off runtime/test/governance | typed no-recipient failure and exact rollback | terminal handoff implementation package |
| `INV-SNOWENERGY-036` | One shared canopy-air node jointly closes reference, all V11 canopy, and Stage 3 snow sensible/vapor exchange. | Child 2C carrier authority | `[INFERENCE][Static]` | carrier residual | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-037` | Wind is a sealed exposure-projected operand at declared transfer geometry; raw 10 m wind and fixed attenuation are not substitutes. | Child 2C exposure authority | `[INFERENCE][Static]` | exposure join | `SNOWENERGY-E-WIND-001` |
| `INV-SNOWENERGY-038` | Canopy--snow--sky longwave uses one reciprocal current-trial state and exact-one exchange. | Child 2C longwave authority | `[INFERENCE][Static]` | radiation lineage | `SNOWENERGY-E-LW-001` |
| `INV-SNOWENERGY-039` | Snow fluxes stop at the accepted event and snow-free fluxes begin only on admitted successor support. | Child 2C chronology authority | `[INFERENCE][Static]` | event/regime join | `SNOWENERGY-E-REGIME-001` |
| `INV-SNOWENERGY-040` | Canopy-intercepted snow is outside this carrier and cannot enter its mass or energy ledgers. | Child 2C scope authority | `[INFERENCE][Static]` | scope guard | `SNOWENERGY-E-SCOPE-001` |
| `INV-SNOWENERGY-042` | One persistent Stage 3 lane owner is OFE-ground. Complete typed tile-ground snow-surface fluxes aggregate exactly once as `sum_i(f_i X_i)` over an ordered tile set closing to one within `TOL-SNOWENERGY-002`; the tolerance never authorizes renormalization. Every contribution binds the same beginning lane-state identity, snow-surface temperature, and latent heat. Missing, duplicate, wrong-class, wrong-model, covered-subset-normalized, or restart topology/basis substitutions fail closed. Uniform terminal liquid preserves `sum_i(f_i M_i)=M_lane`; dividing the complete lane amount by every tile fraction is prohibited. | `REF-SNOWENERGY-USER-OFE-GROUND-V15`; single-column Stage 3 and terminal-receiver state semantics | `[DIRECT][Static] + [INFERENCE][Static]` | lane topology/source-set/common-state/restart guards | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-043` | Covered fixed-point acceptance first reconstructs and validates each candidate fingerprint independently. Schema, terminal-event model, lane, interval, layer cardinality/order, density, and stored count-like settling chronology compare exactly. Numeric state compares only under the physical-class absolute bounds in `TOL-SNOWENERGY-003`; candidate fingerprint equality is neither required nor a substitute for those comparisons. | existing Stage 3 closure scales and covered carrier temperature policy | `[DIRECT][Static] + [INFERENCE][Static]` | typed convergence/nonconvergence and stale-fingerprint guards | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-044` | Additive restart must not restore `OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1`. Before restart implementation, a normative V2 successor must exclude numerical initial-guess identity and exactly join configured lane/OFE and tile topology, covered/open final boundaries, component carrier, installed LSE owner, complete snow owner, wet-liquid authorization, and canonical parent/restart framing. Until its exact fields, framing, ordering, and test vectors are admitted, the successor is `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`. | canonical receipt/restart governance; v15 adopter-wire limitation | `[INFERENCE][Static]` | schema/version/topology/owner replay guards | restart hard `HOLD` |
| `INV-SNOWENERGY-045` | Each accepted support owns exactly one sealed, canonically ordered precipitation phase-parcel set. A parcel binds support, lane/OFE, destination tile, phase, tile-ground mass basis, source and destination identities, temperature/enthalpy provider, producer beginning-state identity, and receipt identity. Zero precipitation is a present, complete empty set, never an omitted owner. | `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[DIRECT][Static] + [INFERENCE][Static]` | parcel-set schema/order/cardinality/identity guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-046` | At each ground destination, liquid custody is exclusive: an open destination receives its sealed raw atmospheric rain parcel, while a covered destination receives only terminal throughfall/drainage and stemflow parcels produced under `SC-VEGETATION-001@28`. Raw rain and vegetation release cannot both be delivered to one destination. Solid atmospheric precipitation bypasses vegetation, remains ground-snow precipitation, and canopy-intercepted snow remains excluded. | `SC-VEGETATION-001@28`; `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[DIRECT][Static]` | destination/source/phase exclusivity guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-047` | Stage 3 reconstructs precipitation mass and precipitation-advected heat from the identical accepted parcel identities. Tile-ground parcel operands aggregate once to the lane as `sum_p(f_destination,p * X_p)` on the existing OFE-ground basis, without covered-fraction renormalization. Missing, duplicate, substituted, differently ordered, or mass-only/advection-only parcel use fails before candidate publication. | `INV-SNOWENERGY-042`; `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[INFERENCE][Static]` | same-set/area-basis/exact-once reconstruction guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-048` | The persistent lower boundary is exactly one OFE/lane interface from the bottom represented snow thermal volume to the first ordered OFE soil-thermal node. No tile soil temperature participates; first-tile selection, tile averaging, covered-only averaging, tile-fraction weighting, duplicated lane flux, or silent zero heat is prohibited. | `REF-SNOWENERGY-SOIL-BOUNDARY-V18`; `SC-LANDSURFACEENERGY-001@8` | `[DIRECT][Static] + [INFERENCE][Static]` | topology/node/owner/basis guard | `SNOWENERGY-E-SOIL-HEAT-001` |
| `INV-SNOWENERGY-049` | With positive finite `dz_sb,lambda_sb,dz_1,lambda_1`, `g_ss=1/(dz_sb/(2*lambda_sb)+dz_1/(2*lambda_1))=2/(dz_sb/lambda_sb+dz_1/lambda_1)`. `G_ss,e=g_ss*(T_sb,e-T_1,e)` for endpoint `e in {0,1}` and the accepted support flux is `bar(G_ss)=0.5*(G_ss,0+G_ss,1)`, positive downward. Both ending temperatures participate in the covered fixed point; beginning values come only from sealed beginning owners. | `REF-SNOWENERGY-SOIL-BOUNDARY-V18`; LSE Crank--Nicolson authority | `[DIRECT][Static] + [INFERENCE][Static]` | physical-operand/endpoint/convergence guard | `SNOWENERGY-E-SOIL-HEAT-001` |
| `INV-SNOWENERGY-050` | The Stage 3 candidate records exactly `-bar(G_ss)` and the first-node soil-thermal candidate records exactly `+bar(G_ss)` on the same support and OFE-ground basis. One sealed `SnowSoilHeatReceiptV1` binds support, lane/OFE, topology/configuration digests, both beginning-owner identities, the four resistance operands, both endpoint temperature pairs, accepted flux, both candidate-ending identities, and a reconstructable digest. Independent validation reconstructs the receipt and equal/opposite debits from primitives before atomic publication; any omission, substitution, sign/basis error, nonconvergence, or later failure rolls back both owners and all receipt state. | physical conservation; `SC-LANDSURFACEENERGY-001@8` | `[INFERENCE][Static]` | receipt/reconstruction/atomic-owner guard | `SNOWENERGY-E-SOIL-HEAT-001` |
| `INV-SNOWENERGY-051` | Candidate-v19 covered terminal execution uses the closed three-mode enum and rebuilds every adaptive/root carrier from exact trial-start owners/support; no raw/scaled/fabricated carrier is admissible. | `INV-034/036/038/039`; deterministic numerics | `[INFERENCE][Static]` | terminal carrier/mode guard | `SNOWENERGY-E-TERMINAL-CARRIER-001` |
| `INV-SNOWENERGY-052` | Candidate-v19 admits terminal-domain and resolved crossings; positive endpoints are exact and cursor events only replay a prior accepted exact-zero endpoint. | `INV-026/034`; endpoint-root identity | `[INFERENCE][Static]` | lifecycle/endpoint guard | `SNOWENERGY-E-TERMINAL-ENDPOINT-001` |
| `INV-SNOWENERGY-053` | Dormant endpoint, non-storage cumulative lineage, area-weighted ProducedUnconsumed parcel mass, solid refreeze, and tolerance-bounded terminal energy form one exact custody rule without immediate receiver credit. | `INV-030/035/042`; conservation | `[DIRECT][Static] + [INFERENCE][Static]` | mass/energy/owner guards | `SNOWENERGY-E-TERMINAL-CUSTODY-001`, `SNOWENERGY-E-TERMINAL-ENERGY-001` |
| `INV-SNOWENERGY-054` | Every terminating lane emits a present nonempty independently reconstructable terminal physical ledger, including explicit-zero cursor replay. | independent conservation obligations | `[INFERENCE][Static]` | ledger/receipt guard | `SNOWENERGY-E-TERMINAL-LEDGER-001` |

## Producer and Consumer Obligations

| Obligation ID | Role | Requirement |
|---|---|---|
| `OBL-SNOWENERGY-P-001` | climate producer | Publish hourly `T_a` plus daily `e_a` and `R_s` with declared units, cadence, and finite-domain validation. |
| `OBL-SNOWENERGY-P-002` | solar-geometry producer | Publish `R_a` and explicit daylight/polar-night classification. |
| `OBL-SNOWENERGY-P-003` | canopy producer | Publish one effective daily plan-view canopy cover `C`; preserve its leaf-on/leaf-off and structural-floor semantics. |
| `OBL-SNOWENERGY-P-004` | Stage 3 thermal producer | Above `m_res`, publish active-layer `T_s`, mass, depth, cold content, lower state when present, and explicitly identified `T_c=T_a`, or a typed unavailable result. At or below `m_res`, publish unresolved duration/mass without fabricating thermal state. |
| `OBL-SNOWENERGY-P-005` | sublimation exchange | Publish one bounded signed vapor mass exchange and derive its latent heat using the same `T_s`. |
| `OBL-SNOWENERGY-P-006` | complete energy producer | Before melt-owner cutover, publish finite, unit-explicit, same-substep net radiation, sensible heat, latent heat, ground/interlayer conduction, and precipitation-advected heat with exact-one composition and independently reconstructable lineage. |
| `OBL-SNOWENERGY-P-007` | evaluation evidence producer | Preserve raw vapor/latent opportunity separately from bounded deposition/sublimation and state endpoints; use N/A for S/F actual transfer; do not relabel raw opportunity as actual snow loss. |
| `OBL-SNOWENERGY-P-008` | terminal event producer | For enabled persistent evaluation only, publish request/version, start/end enthalpy state, accepted/rejected trials, error norm, bracket bounds, event/evaluated/unevaluated seconds, complete carrier, `Delta H_cc`, refreeze, deposition, sublimation, melt, endpoint-solid identity, terminal liquid/energy handoffs, and scale-aware closure without naming a receiving surface. |
| `OBL-SNOWENERGY-P-013` | staged terminal probe/endpoint producer | Publish only immutable bracket evidence or exact endpoint result; retain no probe mutation. |
| `OBL-SNOWENERGY-P-014` | staged terminal custody producer | Publish one terminal ledger and ProducedUnconsumed parcel set per terminating lane joined to the canonical owner/event chain. |
| `OBL-SNOWENERGY-C-001` | longwave evaluator | Apply the equations and guards in the specified order without silent unit conversion or fallback. |
| `OBL-SNOWENERGY-C-002` | shared energy carrier | Consume `L_net` exactly once with the positive-toward-snow convention. |
| `OBL-SNOWENERGY-C-003` | sublimation/melt consumers | Use the same EB-03 snow state as longwave; do not reconstruct an independent surface temperature. |
| `OBL-SNOWENERGY-C-004` | diagnostics | Preserve component operands sufficient to reconstruct `L_atm`, `f_sky`, `L_sub`, `L_out`, and `L_net`. |
| `OBL-SNOWENERGY-C-005` | configuration/reporting | Identify when `T_c=T_a` is used and communicate its approximation limits. |
| `OBL-SNOWENERGY-C-006` | runtime implementation package | Prove the real common `B/L/S/LS` Stage 3 consumer reads this path before claiming activation. |
| `OBL-SNOWENERGY-C-007` | snow state | Remove sublimated mass from layer and aggregate ice state only, publish cold-content export, and reject double-selector composition. |
| `OBL-SNOWENERGY-C-008` | thermal partition | Reconstruct the Marks/SNOBAL active volume independently of depositional boundaries and conserve projection operands. |
| `OBL-SNOWENERGY-C-009` | coupled solver | Apply equal-and-opposite `G_0` within each selected stability substep and close active/lower and whole-pack energy. |
| `OBL-SNOWENERGY-C-010` | stability scheduler | Select `60/15/1 minute` substeps from the fixed mass thresholds and reevaluate fluxes from current substep state. |
| `OBL-SNOWENERGY-C-011` | conductive provider | Publish current active/lower state, applied `G_0`, cadence, and separate active/lower/cancellation residuals from the production solve. |
| `OBL-SNOWENERGY-C-012` | density-layer handoff | Convert layer SWE to `kg m^-2` before applying the density model's zero-mass lifecycle boundary; preserve all coupled state for retained layers and keep aggregate residual tolerances independent. |
| `OBL-SNOWENERGY-C-013` | melt-owner implementation and cutover | Atomically replace CoE generation with bounded Stage 3 positive-energy phase conversion; close same-substep refreeze/retention/routing, residual-snow and terminal-unallocated-energy authority, selectors/defaults/rollback, real-consumer use, and independent energy plus linked mass ledgers before claiming conformance. |
| `OBL-SNOWENERGY-C-014` | evaluation evidence consumer | Reconstruct raw vapor from turbulent primitives; verify `Q_latent_raw=m_v,raw L_s(T_s)`; derive bounded transfer and `Q_latent_bounded=m_v L_s(T_s)`; report their difference; reconstruct the exact characterization-only raw-latent cold/melt chronology; and reject producer disagreement plus all vapor/melt/liquid/N/A aliases before reduction. Never claim the as-built raw-latent chronology conforms to the future bounded-latent target. |
| `OBL-SNOWENERGY-C-015` | terminal event evidence consumer | Independently reconstruct endpoint solid, terminal liquid, enthalpy/energy, event support, and bracket/error acceptance from schema-v8 primitives. Reject full-step melt/sublimation, omitted deposition/refreeze, post-event snow flux, poisoned producer residuals, request/state mismatch, terminal recipient claims, and any event outside the terminal domain. |
| `OBL-SNOWENERGY-C-023` | staged terminal carrier consumer | Independently reconstruct every trial support and covered forcing component; reject raw/scaled carrier paths. |
| `OBL-SNOWENERGY-C-024` | staged terminal custody consumer | Independently reconstruct terminal mass, energy, snow--soil disposition, dormant state, parcel set and acyclic receipt chain. |
| `OBL-SNOWENERGY-P-010` | shared carrier producer | Emit one carrier candidate with complete operand lineage, residuals, current-trial temperatures, and owner/support identities. |
| `OBL-SNOWENERGY-C-017` | shared carrier consumer | Independently reconstruct snow, vapor, liquid, energy, longwave, and event-time closure and reject any alias or duplicate flux. |
| `OBL-SNOWENERGY-C-018` | OFE-ground lane-boundary consumer | Independently reconstruct the complete ordered typed tile contribution set, all retained source-receipt-set identities, common Stage 3 state/temperature/latent heat, OFE-ground flux sums, terminal-liquid handoff, and topology/basis identity; reject omission, duplication, class/model substitution, covered-subset normalization, or restart topology substitution. |
| `OBL-SNOWENERGY-C-019` | covered fixed-point consumer | Reconstruct each candidate fingerprint, compare structural and count-like state exactly, and apply only the physical-class absolute convergence bounds admitted by `TOL-SNOWENERGY-003`; reject stale fingerprints and nonconvergence without state repair. |
| `OBL-SNOWENERGY-C-020` | additive-restart consumer | Refuse V1 lane receipts and any inferred successor wire. Restore only after a normative V2 schema and vectors exist and the restored receipt is rejoined to static topology, destination/lane/component receipts, installed LSE and snow owners, and wet-liquid authorization. |
| `OBL-SNOWENERGY-P-011` | precipitation parcel-set producer | Seal the complete ordered phase-parcel set, including a present empty set, only after joining atmospheric phase custody, vegetation terminal-liquid custody, support, topology, and producer beginning-state identities. |
| `OBL-SNOWENERGY-C-021` | Stage 3 precipitation consumer | Independently validate destination exclusivity and reconstruct OFE-ground precipitation mass and precipitation-advected heat from the same exact parcel set before accepting a physical candidate. |
| `OBL-SNOWENERGY-P-012` | joined snow/soil boundary producer | Build one lane-level lower-boundary receipt from immutable Stage 3 and OFE soil-thermal beginnings plus the accepted coupled candidate; publish neither candidate independently. |
| `OBL-SNOWENERGY-C-022` | snow/soil boundary consumer | Independently reconstruct half-layer series conductance, both endpoint fluxes, Crank--Nicolson accepted heat, exact snow debit/soil credit, receipt digest, and candidate-ending joins before the complete-owner commit. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract | Required interpretation |
|---|---|---|---|---|---|
| `T_a` | `air_temperature_k` | hourly climate to longwave | `K` -> `K` | `SC-CLIMATE-001` | Hourly above-canopy air temperature. |
| `e_a` | `vapor_pressure_kpa` | daily climate to hourly longwave | `kPa` -> `kPa` | `SC-CLIMATE-001` | Daily actual vapor pressure held across the day; not pascals or relative humidity. |
| `R_s` | `solar_radiation_mj_m2_day` | climate to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` | Incident daily shortwave. |
| `R_a` | `extraterrestrial_radiation_mj_m2_day` | solar geometry to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` / `SC-SNOWENERGY-001` | Same daily energy units as `R_s`. |
| `C` | `canopy_cover_fraction` | canopy to sky-view translation | `fraction` -> `fraction` | `SC-PLANT-001` | Effective overhead interception; not LAI or sky-view factor. |
| `f_sky` | `subcanopy_sky_view_fraction` | sky view to longwave mixture | `fraction` -> `fraction` | `SC-SNOWENERGY-001` | Derived diffuse transmission `(1-C)^1.6`; never alias directly to `1-C`. |
| `T_c` | `canopy_temperature_k` | thermal provider to canopy emission | `K` -> `K` | `SC-SNOWENERGY-001` | Effective radiating canopy temperature selected by EB-03. |
| `T_s`, `T_0` | `snow_surface_temperature_k` | active thermal provider to snow emission | `K` -> `K` | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Heat-capacity-weighted Marks/SNOBAL active-layer temperature. |
| `G_0` | `snow_active_lower_conduction_w_m2` | lower pack to active thermal volume | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive into active; equal negative lower operand. |
| `L_atm` | `atmospheric_longwave_w_m2` | atmosphere to sub-canopy mixture | `W m^-2` -> same | `SC-SNOWENERGY-001` | Hourly all-sky downward longwave above canopy. |
| `L_sub` | `subcanopy_longwave_w_m2` | mixture to snow energy | `W m^-2` -> same | `SC-SNOWENERGY-001` | Downward longwave incident at snow. |
| `L_net` | `net_longwave_w_m2` | longwave to shared energy carrier | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive toward snow. |
| `Q_cc` | `cold_content_j_m2` | Stage 3 layer to shared energy carrier | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive energy deficit relative to `0 degC` ice. |
| `m_v,raw` | `vapor_mass_exchange_kg_m2` | evaluation turbulent opportunity | `kg m^-2` -> same | `SC-SNOWENERGY-001` | Raw signed opportunity; never actual S/F transfer or bounded snow loss. |
| `m_v` | `deposition_kg_m2 - sublimation_kg_m2` | bounded Q transfer reconstructed from separate schema-v6 fields | `kg m^-2` -> same | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Actual signed bounded transfer; not the raw vapor field. |
| `Q_E` | `applied_surface_energy_j_m2` | shared carrier to cold content | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive toward snow. |
| `Q_complete`, `Q_cold_required`, `Q_excess`, `delta_E_cold`, `Q_refreeze`, `Q_unallocated_after_exhaustion` | future typed Stage 3 energy-ledger fields | substep energy/phase closure | `J m^-2` -> same | `SC-SNOWENERGY-001` / `SC-SNOWFREEZE-001` | Exact named operands; `Q_refreeze=L_f m_refrozen` and none may be inferred from residual output. |
| `m_ice_available`, `m_melt`, `m_liquid_external_in`, `delta_m_retained`, `m_refrozen`, `m_routed`, `m_solid_precip`, `m_deposition`, `m_sublimation` | future typed Stage 3 phase/liquid-ledger fields | same-substep mass closure | `kg m^-2` -> same | `SC-SNOWENERGY-001` / `SC-SNOWFREEZE-001` | Exact named operands with retained change defined as end minus start. |
| `SWE_layer` | `snow.layer.mass_swe_m` | persistent snow layer to density lifecycle | `m` -> `m` | `SC-SNOWENERGY-001` | Typed vector element converted to `m_layer`; strict mass-unit lifecycle boundary. |
| `z_layer` | `snow.layer.thickness_m` | persistent snow layer to density closure | `m` -> `m` | `SC-SNOWENERGY-001` | Typed vector element; physical-depth aggregate only. |
| `T_ca`, `q_ca` | `SharedCanopyAirNodeV1.temperature_k`, `.specific_humidity` | shared carrier trial state | `K`, `kg kg^-1` | `SC-VEGETATIONTRANSACTION-001` | no independent canopy-air alias |
| `H_i`, `V_i` | typed carrier turbulent flux entries | surface/node exchange | `W m^-2`, `kg m^-2 s^-1` | Child 2C carrier | exact-once flux lineage |
| `L_can`, `L_snow<->canopy` | typed longwave ledger entries | component emission/reciprocal exchange | `W m^-2` | V11/LSE and Stage 3 | equal/opposite closure |

## Constants and Parameters

| Name | Value | Units | Status/provenance |
|---|---:|---|---|
| `sigma` | `5.670374419e-8` | `W m^-2 K^-4` | fixed SI Stefan-Boltzmann constant |
| Dilley intercept | `59.38` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature coefficient | `113.7` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature reference | `273.16` | `K` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water coefficient | `96.96` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water reference | `25` | `kg m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| vapor-pressure conversion | `4650` | `K kg m^-2 kPa^-1` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| clear clearness bound | `0.80` | `dimensionless` | EB-01A mapping |
| overcast clearness bound | `0.15` | `dimensionless` | EB-01A mapping |
| cloud mixture weight | `0.84` | `dimensionless` | `REF-SNOWENERGY-FLERCHINGER` |
| diffuse extinction multiplier | `1.6` | `dimensionless` | `REF-SNOWENERGY-FSM2`, Eq. 14 |
| canopy/snow emissivity | `1` | `dimensionless` | effective-unity exchange convention admitted by EB-01A/FSM2; atmospheric emissivity remains variable |
| `R_a,min` | `1e-9` | `MJ m^-2 d^-1` | EB-03 numeric divide/branch threshold; not a fit parameter |
| hourly duration | `3600` | `s` | typed hourly forcing cadence and named time conversion |
| latent heat of fusion, `L_f` | `333600` | `J kg^-1` | exact pinned libsnobal `LH_FUS(FREEZE)=3.336e5 J kg^-1` phase-change constant; not a calibration coefficient |
| normal maximum active-layer depth | `0.25` | `m` | fixed Marks/SNOBAL `max_z_s_0`; exceeded only by the exact `INV-SNOWENERGY-026` lower-volume collapse; not a user coefficient |
| libsnobal sea-level pressure | `101324.6` | `Pa` | fixed `SEA_LEVEL` constant used by `efcon` |
| libsnobal dry-snow conductivity factor | `4.186798188 * 0.0077` | `W m^-1 K^-1` | exact `CAL_TO_J(0.0077)` factor in `KTS`; density enters as `(rho/1000)^2` |
| normal mass threshold | `60` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| medium mass threshold | `10` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| minimum resolved thermal mass, `m_res` | `1` | `kg m^-2` | exact libsnobal threshold: total mass `<=` suspends; lower-volume mass `<` collapses to one volume; lower-volume equality remains two-volume |
| density-layer zero mass | `1e-9` | `kg m^-2` | existing density-model lifecycle boundary; equivalent to `1e-12 m` SWE through `rho_w`, not the aggregate closure tolerance |
| layer aggregate closure tolerance | `1e-9` | `m` | existing independent SWE and physical-depth residual bound; never a layer-deletion threshold |
| same-residual area-mass closure tolerance | `1e-6` | `kg m^-2` | exact area-mass equivalent of `1e-9 m` SWE through `rho_w=1000 kg m^-3`; applies to vapor-to-sublimation transfer closure |
| hourly/daily vapor-aggregation tolerance | `1e-9` | `kg m^-2` | separate aggregation reconstruction predicate; not the vapor-to-sublimation transfer tolerance or layer lifecycle boundary |
| medium duration | `900` | `s` | fixed `15 minute` Marks/SNOBAL level |
| small duration | `60` | `s` | fixed `1 minute` Marks/SNOBAL level |

None of these constants is a user calibration coefficient in the admitted
model.

## Unit Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_a` | `K` | typed `TemperatureCelsius` boundary with named kelvin conversion | temperature helper | none | internal diagnostic only |
| `e_a` | `kPa` | typed non-negative vapor-pressure wrapper | daily dewpoint-to-vapor helper | none | internal diagnostic only |
| `R_s`, `R_a` | `MJ m^-2 d^-1` | typed non-negative daily-radiation wrapper | named hourly-sum and solar producer | none | internal diagnostic only |
| `k_t`, `c` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `C` | `dimensionless fraction` | existing native canopy state; registry disposition owned by `SC-PLANT-001` | none | existing typed producer semantics; no new exception | existing canopy publication unchanged |
| `P_0`, `f_sky` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `w` | `kg m^-2` | internal derived operand; no boundary entry in EB-02 | equation-local `4650 e_a/T_a` only | package evidence scalar only; future typed internal value or recorded exception | not published by EB-02 |
| `epsilon_clear`, `epsilon_all` | `dimensionless` | typed bounded fraction | none | none | internal diagnostic only |
| `T_c`, `T_s` | `K` | typed `TemperatureCelsius` provider | named kelvin conversion | none | internal diagnostic only |
| `L_clear`, `L_atm`, `L_can`, `L_sub`, `L_out`, `L_net` | `W m^-2` | typed energy/radiative flux wrappers | named hourly integration helper | none | internal diagnostic only |
| `Q_cc`, `Q_E`, `Q_complete`, `Q_cold_required`, `Q_excess`, `delta_E_cold`, `Q_refreeze`, `Q_unallocated_after_exhaustion` | `J m^-2` | Stage 3 scalar with contract-bound guard | no conversion | retained/future scalar exception: internal area-normalized energy ledger | required cutover evidence for future operands |
| `m_melt` | `kg m^-2` | future Stage 3 solid-to-liquid ledger | named `Q_excess/L_f` conversion bounded by `m_ice_available` | internal scalar with linked-ledger guards | required cutover evidence |
| `m_ice_available`, `m_liquid_external_in`, `delta_m_retained`, `m_refrozen`, `m_routed`, `m_solid_precip`, `m_deposition`, `m_sublimation` | `kg m^-2` | future Stage 3 phase/liquid ledgers | named phase-state and exact handoff conversions only | internal scalars with linked-ledger guards | required cutover evidence |
| `m_v,raw` | `kg m^-2` | schema-v6 `vapor_mass_exchange_kg_m2` diagnostic scalar | named turbulent mass-flux duration integration | retained evaluation-only scalar exception | internal diagnostic only |
| `m_v` | `kg m^-2` | schema-v6 bounded `deposition_kg_m2` and `sublimation_kg_m2` fields or future typed Stage 3 transfer | named availability bound and signed recomposition | internal scalar with mass/latent ledger guards | required bounded-transfer evidence |
| `m_res` | `kg m^-2` | fixed internal model-domain constant | named SWE-to-mass conversion using `rho_w` | no user boundary or scalar exception | contract and environment-gated trace metadata |
| `m_layer` | `m` SWE at the typed state seam; `kg m^-2` for lifecycle comparison | `SWE_layer` (`snow.layer.mass_swe_m`) | `snow_water_equivalent_meters_to_area_mass_kg_m2` | typed `DirectSnowLayerState` vector-element scalar exception | internal state only |
| `z_layer` | `m` | `z_layer` (`snow.layer.thickness_m`) | identity | typed `DirectSnowLayerState` vector-element scalar exception | internal state only |
| `t_unres` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `m_l` | `kg m^-2` | Stage 3 lower-volume diagnostic scalar | named SWE-to-mass conversion using `rho_w` | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `t_collapse` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `p_a` | `Pa` | typed positive pressure wrapper | named elevation-to-pressure projection | none | environment-gated research trace |
| `k_d`, `k_eff` | `W m^-1 K^-1` | typed positive thermal-conductivity wrapper | exact named `KTS+efcon` helper | none | environment-gated research trace operands |
| `T_ca`, `q_ca` | `K`, `kg kg^-1` | typed shared carrier node | identity at trial state | none | carrier receipt |
| `H_i`, `V_i` | `W m^-2`, `kg m^-2 s^-1` | typed flux wrappers | named duration integration | no raw 10 m wind alias | carrier ledger |
| `L_can`, `L_snow<->canopy` | `W m^-2` | typed longwave ledger | Stefan--Boltzmann and complementary exchange | none | reciprocal ledger |

Energy-carrier integration from `W m^-2` uses the explicit typed hourly
duration of `3600 s`; a hidden daily factor of `86400` is prohibited.

## Tolerance and Numeric Notes

INV-035 inherits the INV-034 event tolerances for numeric closure only.
Fingerprint, half-open support, receipt cardinality, debit/credit/consumed
state, CoE exclusion, and recipient absence on failure are exact.

- `TOL-SNOWENERGY-001` governs terminal numerics only. The step-doubling LTE
  norm is `max_i |fine_i-coarse_i| / (a_i + 1e-8 *
  max(|fine_i|,|coarse_i|))`, with `a_mass=1e-9 kg m^-2` and
  `a_energy=1e-6 J m^-2`; acceptance requires norm `<=1`. Proposed duration is
  at most `60 s` and at least `1e-9 s`; 64 consecutive rejections fail typed.
  Event bisection requires bracket width `<=1e-6 s`, endpoint solid
  `<=max(1e-12 kg m^-2,1e-12*m_i,start)`, and at most 64 iterations. Snow-side
  mass/energy closure remains independently scale-aware at
  `max(1e-12 kg m^-2,1e-12*sum_abs_mass_operands)` and
  `max(1e-6 J m^-2,1e-12*sum_abs_energy_operands)`. LTE, event-root, and
  closure tolerances are not interchangeable and authorize no state clamp.
  Candidate-v19 terminal acceptance names the unit-bearing components
  `a_terminal_mass=1e-9 kg m^-2` and
  `a_terminal_energy=1e-6 J m^-2`; these are comparison tolerances only and
  authorize no normalization, clamp, or residual deletion.

| Tolerance ID | Binding rule | Guard |
|---|---|---|
| `TOL-SNOWENERGY-001` | Terminal step-doubling, event-root, and independent mass/energy closure tolerances are distinct and never repair identity or state. | typed numerical failure |
| `TOL-SNOWENERGY-002` | OFE tile-fraction closure residual `abs(sum_i(f_i)-1) <= 1e-12` (dimensionless) admits only floating-point summation roundoff; it never changes, rescales, or renormalizes any fraction or flux. Identity, ordering, cardinality, duplication, area basis, boundary class, model definition, and state joins remain exact. | typed topology failure |
| `TOL-SNOWENERGY-003` | Covered fixed-point state convergence uses absolute bounds only: `1e-9 m` for SWE, thickness, liquid, and refrozen depth; `1e-8 K` for temperature difference; `1e-6 kg m^-2` for cumulative/detached mass; and `1e-6 J m^-2` for cold content and cumulative energy. Density and all structural/count-like fields are exact. No relative term, clamp, canonicalization, or cross-unit substitution is admitted. | typed nonconvergence |

- Analytical evidence uses an absolute tolerance of `1e-9` for dimensionless
  identity checks and `1e-6 W m^-2` for independently reconstructed fluxes.
- Runtime snow-mass closure uses `1e-9 m` water equivalent. Through the named
  `snow_water_equivalent_meters_to_area_mass_kg_m2` conversion and
  `rho_w=1000 kg m^-3`, the same residual is exactly `1e-6 kg m^-2`; the
  vapor-to-sublimation transfer identity uses this area-mass bound. Energy
  closure uses `1e-6 J m^-2`. These tolerances do not relax physical domains.
- Hourly/daily vapor-aggregation reconstruction separately uses
  `1e-9 kg m^-2`. The density-layer lifecycle boundary also has the numeric
  value `1e-9 kg m^-2` (`1e-12 m` SWE), but it is a representation predicate,
  not a residual-acceptance threshold. Neither predicate may be substituted
  for the `1e-6 kg m^-2` vapor-to-sublimation transfer closure, and that
  transfer tolerance may not be generalized to other mass checks.
- Evaluate fourth powers in finite `f64`; reject non-finite intermediate
  values.
- The canonical canopy producer currently caps effective cover below one.
  The longwave consumer nevertheless must guard `C >= 1` rather than inventing
  its own epsilon clamp.
- The limit `C -> 1` is a scientific test; `C=1` is outside the admitted
  finite inversion domain.

## Calibration and Identifiability

Disposition: `CALIBRATION_NOT_APPLICABLE`.

```text
science_implementation_status = IMPLEMENTED
stage3_melt_owner_status = AUTHORITY_ADMITTED_IMPLEMENTATION_HOLD
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`IMPLEMENTED` applies to the canonical longwave equations, their default-off
diagnostic/reproduction seam, and the version-3 active-layer coupled provider.
EB-03A production, analytical reconstruction, and real B/L/S/LS consumer gates
pass. The two `NOT_APPLICABLE` fields reflect that this contract defines no
empirically estimated parameter surface.

`AUTHORITY_ADMITTED_IMPLEMENTATION_HOLD` applies specifically to the
version-7 melt-owner target. It is not a runtime-conformance claim. Current
CoE melt remains byte-identical compatibility behavior until
`GAP-SNOWENERGY-011` is closed.

The admitted equations use fixed literature constants and existing forcing or
state variables. EB-03 performs no fitting and introduces no tunable
sky-view, extinction, canopy-temperature, or emissivity coefficient.

| Candidate | Calibration status | Reason |
|---|---|---|
| `1.6` diffuse multiplier | fixed | literature equation constant |
| `0.15`, `0.80`, `0.84` cloud mapping | fixed for canonical route | changing them would require a new authority/calibration package |
| `C` | externally produced state | governed by `SC-PLANT-001`, not fit here |
| `T_c`, `T_s` | provider states | Stage 3 / air-temperature approximation, not fitted |
| `R_a,min` | fixed numeric guard | divide/branch threshold, not empirical calibration |

Identifiability warning: fitting an extinction coefficient after deriving
`f_sky` from effective cover would confound canopy-state calibration with the
radiative translation and is prohibited in this version.

## Test Vector Obligations

The implementation increment must reproduce package artifact
`analytical-test-vectors.csv` and include at least:

1. open canopy: `C=0`, `f_sky=1`, `L_sub=L_atm`;
2. intermediate cover values independently evaluated as `(1-C)^1.6`;
3. near-closed-canopy limiting behavior;
4. monotonic decrease of `f_sky` with increasing `C`;
5. clear and overcast clearness endpoints and both clamp sides;
6. independent reconstruction of Dilley clear-sky and all-sky fluxes;
7. complementary sky/canopy mixing and net-longwave sign;
8. typed rejection of invalid temperatures, cover, and forcing;
9. typed unavailable polar-night cloud inference;
10. distinct hourly-temperature fluxes under held daily vapor/cloud state and
    an explicit nonzero daily-mean-temperature substitution bias; and
11. typed failure when the Stage 3 thermal provider is absent;
12. all four orthogonal selector cells with identical non-candidate settings;
13. independent latent/mass equivalence and wrong-sign rejection;
14. sublimation mass closure with explicit non-aliasing to liquid/melt; and
15. cold-content energy closure including exported cold content;
16. active depth and mass reconstructed across at least three depositional
    layers with a boundary-crossing split;
17. a shallow pack whose complete mass, rather than a thin snowfall-event
    layer, supplies the active heat capacity;
18. `G_0` sign, equal-and-opposite active/lower ledgers, isothermal zero-flux,
    and harmonic-conductivity reconstruction;
19. exact `60/15/1 minute` transitions around `60/10/1 kg m^-2`; and
20. a thin-pack chronology proving substep reevaluation and rejecting hourly
    debit, absolute-zero clamp, air-temperature replacement, and cold-content
    tax alternatives; and
21. unequal depositional temperatures proving one shared active `T_0`, a
    distinct persistent lower temperature, nonzero correctly signed `G_0`,
    exact active/lower cancellation, and exact libsnobal `KTS+efcon`
    conductivity rather than the Sturm frost relation; and
22. exact total `m_s < 1`, `m_s = 1`, and `m_s > 1 kg m^-2` vectors proving
    pre-partition zero exchange and unchanged persistent state below/at the
    boundary; plus lower-volume `m_l < 1` collapse, `m_l = 1` two-volume
    equality, normal evaluation/resume, explicit runner diagnostics, and
    rejection of forced melt, deletion, temperature clamp, epsilon vapor
    pressure, and one-more-flux aliases.
23. exact density-layer mass-boundary vectors below, at, and above
    `1e-9 kg m^-2`; captured fragment vectors proving state retention and
    independent mass/depth reconstruction; and negative vectors proving
    cross-unit filtering, tolerance inflation, fragment deletion, and a
    material aggregate mismatch are rejected.
24. a complete-component energy vector proving cold-content-first application,
    `m_melt=min(Q_excess/L_f,m_ice_available)`, and exact energy closure;
25. same-substep solid debit, liquid credit, refreeze, retention, and routing
    vectors with three independent ledger reconstructions; and
26. negative cutover vectors rejecting simultaneous CoE/Stage 3 generation,
    omitted sensible or precipitation-advection terms, discarded positive
    energy, delayed/duplicate liquid disposition, and an unauthorized
    `m_s <= 1 kg m^-2` proxy.
27. raw-versus-bounded vapor vectors covering inactive and active availability
    bounds; `S/F` N/A; independent Q deposition/sublimation; same-sign wrong
    magnitude; direction reversal; simultaneous deposition/sublimation;
    endpoint-preserving melt/vapor aliases; active/total endpoint substitution;
    and raw latent opportunity versus bounded-transfer latent energy.
28. terminal-domain exact sides at total ice below, equal to, and above
    `1 kg m^-2`; cold/warming no-event; pure melt; pure sublimation; joint
    melt/sublimation; positive deposition and refreeze no-false-event; and an
    exact hour-end event.
29. first-order step-doubling discrepancy/refinement and event-time convergence
    against an independent constant-carrier analytical or tighter bisection
    oracle, plus typed nonfinite, step-underflow, rejection-limit, invalid
    bracket, nonmonotone candidate, and iteration-limit failures with atomic
    state identity.
30. schema-v8 independent reconstruction of endpoint solid, enthalpy, liquid,
    terminal energy, evaluated/unevaluated time, and bracket/error evidence;
    negative vectors reject full-step flux, omitted deposition/refreeze,
    post-event snow energy, producer residuals, request/state mismatch, and any
    receiving-surface interpretation.

Producer-only analytical vectors cannot close runtime activation. EB-03 must
prove the real shared Stage 3 snow-energy consumer reads the contracted
operands.

### Child 2C canonical obligation IDs

| ID | Binding requirement | Enforcement |
|---|---|---|
| `TOL-SNOWENERGY-001` | Terminal step-doubling, event-root, and independent mass/energy closure tolerances are distinct and never repair identity or state. | typed numerical failure |
| `TOL-SNOWENERGY-002` | OFE tile fractions close within `1e-12` dimensionless summation residual without normalization; every nonnumeric topology join remains exact. | typed topology failure |
| `TOL-SNOWENERGY-003` | Covered Stage 3 fixed-point state uses physical-class absolute bounds only; fingerprints are reconstructed per candidate, while density and structural/count-like state remain exact. | typed nonconvergence |
| `OBL-SNOWENERGY-P-009` | Emit one immutable terminal receiver receipt or a typed failure; never partially commit. | receiver transaction |
| `OBL-SNOWENERGY-C-016` | Reconstruct snow, liquid, vapor, fusion energy, and time without post-event snow operands or aliases. | receiver validator |
| `INV-SNOWENERGY-041` | Terminal numerical tolerances are typed, distinct, and never repair identity, support, or state. | numerical validator |
| `INV-SNOWENERGY-043` | Covered convergence reconstructs candidate fingerprints and separates exact structure/count/density from unit-specific absolute numeric residuals. | convergence validator |
| `INV-SNOWENERGY-044` | Lane receipt V1 is non-restorable; V2 restart schema remains undefined and blocked pending exact normative framing and replay joins. | restart schema/version guard |
| `OBL-SNOWENERGY-C-019` | Apply only `TOL-SNOWENERGY-003` after independent fingerprint validation. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-020` | Reject V1/inferred restart wire and require the complete V2 topology/owner replay join. | additive-restart consumer |

## Binding Exposure Index

The package rows below map active package-local binding residue through version
16 to authority promoted into this canonical core.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SNOWENERGY-CHILD2C-CARRIER` | `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-036, INV-SNOWENERGY-037, INV-SNOWENERGY-038, INV-SNOWENERGY-039, INV-SNOWENERGY-040, OBL-SNOWENERGY-P-010, OBL-SNOWENERGY-C-017` | `flagged-binding-addition` | Shared carrier topology, sealed exposure, weighted component longwave, typed flux lineage, and wrong-regime/scope rejection. |
| `SNOWENERGY-V15-OFE-GROUND-LANE` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-042, OBL-SNOWENERGY-C-018` | `flagged-binding-addition` | Direct user selection of one-column-per-lane OFE-ground storage under `TOL-SNOWENERGY-002`, complete typed tile-surface flux aggregation without covered-subset renormalization, common lane snow state, terminal identity, and topology-bound restart posture; dual review and verification required. |
| `SNOWENERGY-V16-COVERED-CONVERGENCE-RESTART` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-043, INV-SNOWENERGY-044, OBL-SNOWENERGY-C-019, OBL-SNOWENERGY-C-020` | `flagged-binding-addition` | Separates covered fixed-point comparisons under `TOL-SNOWENERGY-003`, reconstructs candidate fingerprints, and holds additive restart until a normative lane-receipt V2 wire and complete topology/owner replay join are admitted. |
| `SNOWENERGY-V17-PRECIPITATION-CUSTODY` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `approved_active` | `maps-to-existing-INV` | `INV-SNOWENERGY-045, INV-SNOWENERGY-046, INV-SNOWENERGY-047, OBL-SNOWENERGY-P-011, OBL-SNOWENERGY-C-021` | `dual-review+exact-clean-gates` | Seals the ordered precipitation phase-parcel set, binds open-versus-covered liquid exclusivity and solid bypass, and requires mass/advection same-set reconstruction on the OFE-ground lane basis. |
| `SNOWENERGY-V18-SNOW-SOIL-BOUNDARY` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `approved_active` | `maps-to-existing-INV` | `INV-SNOWENERGY-048, INV-SNOWENERGY-049, INV-SNOWENERGY-050, OBL-SNOWENERGY-P-012, OBL-SNOWENERGY-C-022` | `dual-review+exact-clean-gates` | Binds one OFE/lane bottom-snow-to-first-soil-node Crank--Nicolson interface, exact equal/opposite custody, reconstructable receipt, and atomic rollback without tile aggregation or duplication. |
| `SNOWENERGY-V19-COVERED-TERMINAL-CHRONOLOGY` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-051, INV-SNOWENERGY-052, INV-SNOWENERGY-053, INV-SNOWENERGY-054, OBL-SNOWENERGY-P-013, OBL-SNOWENERGY-P-014, OBL-SNOWENERGY-C-023, OBL-SNOWENERGY-C-024` | `flagged-binding-addition` | Candidate v19 binding is under mandatory review; it covers exact per-trial carrier, dormant endpoint, pending parcels, terminal snow--soil disposition and terminal ledger. Receiver consumption and cutover remain held. |
| `SNOWENERGY-EB02-AUTHORITY` | `docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-001, INV-SNOWENERGY-002, INV-SNOWENERGY-003, INV-SNOWENERGY-004, INV-SNOWENERGY-005, INV-SNOWENERGY-006, INV-SNOWENERGY-007, INV-SNOWENERGY-008, INV-SNOWENERGY-009, INV-SNOWENERGY-010, INV-SNOWENERGY-011, INV-SNOWENERGY-012, INV-SNOWENERGY-013, INV-SNOWENERGY-014` | `none` | Package-local source reconciliation and analytical artifacts are evidence; all binding equations, guards, and obligations are in this canonical contract. |
| `SNOWENERGY-EB03-COMPOSITION` | `docs/work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-015, INV-SNOWENERGY-016, INV-SNOWENERGY-017, INV-SNOWENERGY-018, INV-SNOWENERGY-019` | `none` | Package evidence binds the Stage 3 provider, orthogonal selectors, and mass/energy composition implemented by version 2. |
| `SNOWENERGY-EB03A-COUPLING` | `docs/work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-020, INV-SNOWENERGY-021, INV-SNOWENERGY-022, INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-025` | `none` | Package evidence must implement and verify the version-3 active thermal control volume and coupled substep solver. |
| `SNOWENERGY-EB04C-THERMAL-DOMAIN` | `docs/work-packages/20260731-snow-surface-eb-04c-thin-pack-thermal-domain-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-026` | `dual review and verification required` | Package evidence must implement and verify the exact minimum-resolved-mass branch without importing libsnobal's phase conversion or weakening typed guards. |
| `SNOWENERGY-EB04D-LAYER-RECONCILIATION` | `docs/work-packages/20260731-snow-surface-eb-04d-layer-thickness-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-021, INV-SNOWENERGY-027` | `dual review and verification required` | Package evidence must separate mass-unit lifecycle selection from meter-unit aggregate residual tolerances and preserve coupled layer state. |
| `SNOWENERGY-EB04S-TOLERANCE-RECONCILIATION` | `docs/work-packages/20260801-snow-surface-eb-04s-authority-reconciliation-retained-adjudication-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-017, INV-SNOWENERGY-018, INV-SNOWENERGY-027, INV-SNOWENERGY-028` | `dual review and verification required` | Result-blind authority reconciliation binds the SWE-to-area-mass equivalence while preserving distinct vapor-aggregation and layer-lifecycle predicates. |
| `SNOWENERGY-21N-MELT-OWNER` | `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-029, INV-SNOWENERGY-030, OBL-SNOWENERGY-P-006, OBL-SNOWENERGY-C-013` | `dual review and verification required` | Stage 3 is the sole future melt owner; the unchanged CoE runtime remains compatibility-only until complete energy, residual-snow, same-substep liquid, real-consumer, and cutover gates pass atomically. |
| `SNOWENERGY-STAGE3-COMPLETE-CARRIER` | `docs/work-packages/20260805-snow-stage3-complete-carrier-shadow-melt-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-029, INV-SNOWENERGY-030, INV-SNOWENERGY-031` | `dual review and verification required` | User authority binds explicit CLIGEN virtual-instrument geometry and lifts the turbulent-input authority hold; carrier and shadow evidence remain required before atomic cutover. |
| `SNOWENERGY-STAGE3-EVOLVING-CARRIER-PLAUSIBILITY` | `docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-017, INV-SNOWENERGY-029, INV-SNOWENERGY-032, OBL-SNOWENERGY-P-007, OBL-SNOWENERGY-C-014` | `dual review and verification required` | Distinguishes evaluation-only raw vapor/latent opportunity from actual bounded sequential transfer; no production correction or persistence authority. |
| `SNOWENERGY-STAGE3-WIND-SOURCE-CUSTODY` | `docs/work-packages/20260807-snow-stage3-wind-source-custody-and-exposure-authority-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-031, INV-SNOWENERGY-033` | `dual review and verification required` | Separates nominal source height, raw CLI wind, PMET-local adjustment, and virtual Stage 3 geometry; provider recovery directly proves retained output equality and statically reconstructs the local path while deployed/server and exposure authority remain missing. |
| `SNOWENERGY-TERMINAL-ENTHALPY-EVENT` | `docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-034, INV-SNOWENERGY-041, OBL-SNOWENERGY-P-008, OBL-SNOWENERGY-C-015` | `flagged-binding-addition` | Admits evaluation-only shallow-pack enthalpy/error-control/event mechanics while keeping liquid, energy, and remaining-time receiving-surface custody censored. |
| `SNOWENERGY-TERMINAL-RECEIVER-TRANSACTION` | `docs/work-packages/20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-035, OBL-SNOWENERGY-P-009, OBL-SNOWENERGY-C-016` | `flagged-binding-addition` | Admits a fresh default-off terminal receiver transaction; it does not alter evaluation-only `INV-SNOWENERGY-034` or authorize production cutover. |

## Gap Register

| Gap ID | Gap | Owner | Required closure | Current disposition |
|---|---|---|---|---|
| `GAP-SNOWENERGY-001` | Shared `T_s` and cold-content provider selection. | `SNOW-SURFACE-EB-03` | Stage 3 top-layer provider plus common-consumer proof. | candidate selected; common-consumer proof failed / runtime `HOLD` |
| `GAP-SNOWENERGY-002` | Canopy temperature uses no prognostic canopy energy balance. | future adjudication after EB-04 | Assess factorial sensitivity and field/literature evidence; retain `T_c=T_a` only within stated homogeneous-stand limits. | accepted approximation |
| `GAP-SNOWENERGY-003` | Polar-night cloud state cannot be inferred from daily clearness index. | future authoritative cloud producer | Version 2 returns typed unavailable when longwave is enabled and `R_a <= R_a,min`; a future producer requires contract amendment. | bounded runtime limitation |
| `GAP-SNOWENERGY-004` | `R_a,min` numeric threshold. | `SNOW-SURFACE-EB-03` | Unit-explicit `1e-9 MJ m^-2 d^-1` and transition tests. | resolved in version 2 |
| `GAP-SNOWENERGY-005` | Effective-cover translation has not been evaluated against hemispherical photography across heterogeneous stands. | future validation campaign | Compare without making observations a runtime prerequisite. | non-blocking research gap |
| `GAP-SNOWENERGY-006` | The Dilley-Unsworth review does not establish a transferable numeric meteorological input envelope for every openWEPP climate. | future validation | Enforce the no-clamp derived-emissivity guard; report extrapolation diagnostics and evaluate climate-envelope adequacy. | implemented guard / validation gap |
| `GAP-SNOWENERGY-007` | The Stage 3 cold-content carrier used a snowfall-event top layer instead of the Marks/SNOBAL active thermal control volume and applied hourly surface energy outside a mass-dependent coupled active/lower substep. | `SNOW-SURFACE-EB-03A` | Implement and independently test version-3 active-volume construction, `G_0`, conservative projection, and stability substeps without a clamp, fitted limiter, new user coefficient, or changed frozen controls. | resolved in version 3; real B/L/S/LS and rollback cells pass |
| `GAP-SNOWENERGY-008` | Stage 3 continued thermal/exchange evaluation below libsnobal's minimum resolved layer mass, producing 17 impossible temperatures and five valid-Kelvin vapor-pressure underflows. | `SNOW-SURFACE-EB-04C` | Apply the exact fixed resolved-layer boundary before temperature/conductivity evaluation while preserving CoE mass and persistent cold content; prove all 22 captured thermal failures pass their original boundary. | resolved in version 4; 22/22 captured failures pass their formerly rejected processing day with zero forbidden thermal errors |
| `GAP-SNOWENERGY-009` | Multilayer density initialization used the `1e-9 m` aggregate SWE tolerance as a layer-deletion threshold, omitting represented fragments whose physical depth remained in the expected aggregate. | `SNOW-SURFACE-EB-04D` | Apply the existing `1e-9 kg m^-2` density-model zero-mass boundary after named SWE-to-mass conversion; preserve coupled state and prove both captured geometry failures pass. | resolved in version 5; both 16,437-day trajectories complete with independently reconstructed layer mass/depth closure |
| `GAP-SNOWENERGY-010` | EB-04R transcribed the `1e-9 m` SWE-equivalent vapor-to-sublimation closure as `1e-9 kg m^-2`, conflating it with separate mass-unit predicates. | `SNOW-SURFACE-EB-04S` | Reconcile from pre-result authority, state every operand-specific tolerance in canonical units, and preserve EB-04R as an unchanged HOLD. | resolved in version 6; result-blind dimensional authority frozen before retained-output adjudication |
| `GAP-SNOWENERGY-011` | CoE remains the production melt generator. Version 12 admits evaluation-only residual-snow enthalpy and localized solid exhaustion, but receiving-surface liquid/energy custody and post-event flux chronology remain unresolved for the target. | land-surface authority plus future atomic cutover | Implement a first-class receiving surface, exact-one ownership, linked ledgers, selectors/defaults/rollback, and real downstream consumption. | terminal snow numerics admitted; production/cutover `IMPLEMENTATION_HOLD` remains |
| `GAP-SNOWENERGY-012` | Current evaluation schema-v6 can apply raw latent-energy opportunity while actual sublimation is availability-bounded. | this plausibility package and future production implementation | Quantify tuple-level capacity truncation without aliasing; production target must derive latent energy and mass from one bounded `m_v`. | characterization admitted; physical passage and persistence held when active |
| `GAP-SNOWENERGY-013` | Surviving WEPPpy runs directly recover retained centroids, GRIDMET-enabled intent, daily parquet wind, and exact CLI equality; nearby historical code only statically reconstructs the likely centroid/GRIDMET/share/format path. Deployed identity/request/response, exact GRIDMET asset version/status, server pixel/sampling, missing/day-boundary policy, and physical exposure linkage remain absent. | source generator owner / future authority package | Supply immutable deployed/server receipt fields and two-sided forcing-to-target exposure authority; a height conversion or modeled forest class alone is insufficient. | narrowed `AUTHORITY_MISSING`; persistence held; no canopy or production correction authorized |

## Default-Off Terminal Receiver Transaction Amendment

`INV-SNOWENERGY-035` — `terminal_receiver_v1` is a fresh, internal,
default-off authority layered after, and not inside, the evaluation-only
`INV-SNOWENERGY-034` event solver. From immutable interval-beginning snow and
receiver snapshots, it consumes the earliest complete solid-exhaustion event
`t*`, where `0 < t* <= dt_interval`, and exposes exactly one receipt containing
the event fingerprint, `t*`, `dt_remaining = dt_interval - t*`, zero terminal
represented ice and cold content, bounded vapor transfer, and all retained plus
newly generated liquid. Liquid temperature is exactly `273.15 K`; sensible
enthalpy relative to the contract reference is exactly zero. Fusion energy
remains closed by the snow ledger and is neither liquid sensible enthalpy nor a
soil-energy credit.

The receipt transfers liquid and zero sensible enthalpy exactly once to
`SC-SURFACELIQUID-001#INV-SURFACELIQUID-010`. After `t*`, no snow albedo,
surface temperature, roughness, radiation, turbulence, evaporation,
precipitation heat, soil heat, or unallocated terminal energy is admissible.
The actual snow-free receiver rebuilds every flux over only `dt_remaining`.
Zero remaining duration commits closure/receipt without a receiver physics
step; all other cases have neither time overlap nor gap.

The transferred amount is explicitly
`m_terminal_liquid = m_liquid,retained,start + m_rain,snow_support +
m_melt,new - m_refreeze`, with every term nonnegative,
finite, on the same OFE-ground basis, and independently present in the event
ledger. `m_rain,snow_support` is the only authorized external snow-support
liquid and contains only rain whose absolute support is
`[wall_start,wall_t*)`; receiver-side rain begins at `wall_t*` and is excluded.
Runon is receiver-side only and is prohibited on snow support in this
transaction because no existing snow authority admits it.
Snow candidate commit sets retained liquid to zero and records
`terminal_receipt_consumed=true` under the event fingerprint while the surface-
liquid candidate credits exactly `m_terminal_liquid`. Debit, credit, and the
consumed marker are one atomic join; any omission, duplication, stale marker,
negative result, or receipt replay is typed failure and restores all three
beginning values.

Handoff additionally requires
`Q_terminal_unallocated <= TOL-SNOWENERGY-001` on the independently reconstructed
event ledger. A larger positive value is `SNOWENERGY-E-TERMINAL-UNALLOCATED` and
rejects the receiver transaction with no recipient, disposition, carry, soil
assignment, or state commit. This does not relax the schema-v8 event-only
censoring: INV-034 still reports the value only as evaluation evidence.

This mechanics authority is unreachable from production defaults/selectors;
CoE remains the sole production snow mass/melt generator. Turbulent-carrier and
forest-exposure authority, physical efficacy, qualification, assurance
approval, production ownership, and cutover remain held. `INV-SNOWENERGY-034`
retains schema-v8 evaluation-only, no-recipient, no-commit semantics.

- `OBL-SNOWENERGY-P-009`: emit one immutable fingerprint-bound receipt or a
  typed snow-side failure; never partially commit.
- `OBL-SNOWENERGY-C-016`: independently reconstruct snow mass, liquid, vapor,
  fusion energy, and time, rejecting duplicates, aliases, post-event snow
  operands, or nonzero liquid sensible enthalpy.

| Obligation ID | Binding requirement | Enforcement |
|---|---|---|
| `OBL-SNOWENERGY-P-009` | Emit one immutable terminal receiver receipt or a typed failure; never partially commit. | receiver transaction |
| `OBL-SNOWENERGY-C-016` | Reconstruct snow, liquid, vapor, fusion energy, and time without post-event snow operands or aliases. | receiver validator |

Required unequal-operand vectors distinguish retained liquid, new melt, rain,
runon, vapor, and receiver storage and poison full-step flux reuse,
liquid/store aliases, fusion-energy-to-soil assignment, and dual melt owners.

| Canonical surface | INV-035 binding |
|---|---|
| Algorithm | localize earliest `t*`; reconstruct event; reject unallocated energy; form explicit liquid debit/credit/consumed join; invoke receiver for remaining half-open support |
| Branch/guard | no event: remain snow; `t*=wall_end`: zero receiver step; positive out-of-tolerance unallocated energy: typed no-recipient failure |
| Invariant/alias | event fingerprint, retained/rain/melt/refreeze/vapor terms are distinct; fusion energy is not liquid sensible enthalpy or soil heat |
| Unit/tolerance | liquid `kg m^-2 OFE-ground`; energy `J m^-2`; time `s`; only named event closure tolerance applies, never to identity/cardinality |
| Tests/gap | unequal operands, endpoint allocation, replay/alias poisons, unallocated-energy rejection; production/carrier/efficacy/cutover gaps stay held |

## Child 2C shared snow--canopy turbulent carrier amendment

This amendment binds one default-off carrier for a forest-covered V11 canopy
and Stage 3 ground snow. It does not activate the carrier, select a production
default, admit canopy-intercepted snow, or qualify an exposure or seasonal
consumer. The carrier fails closed when any required sealed forcing, exposure,
thermal, or support receipt is missing.

### Carrier topology, inputs, and ownership

The topology is:

```text
sealed reference atmosphere -> shared canopy-air node
shared canopy-air node -> V11 canopy surfaces
shared canopy-air node -> Stage 3 ground snow surface
```

The sealed half-hour forcing owns reference wind, temperature, humidity, and
pressure. It must provide an exposure-projected wind at the admitted virtual
transfer geometry; a nominal raw `10 m` wind is not a subcanopy operand. V11
owns canopy structure, leaf/stem surfaces, their temperatures and transfer
conductances. Stage 3 owns snow surface temperature, SWE, liquid, cold
content, roughness, emissivity, and albedo. The carrier transaction owns one
shared canopy-air temperature/humidity node and its coupled residual. Coupled
time owns the segment support receipt and one complete-owner parent commit.

Each production lane owns exactly one persistent Stage 3 snow column whose
mass, depth, liquid, cumulative mass terms, cold content, energy, fluxes, and
terminal liquid are expressed per unit OFE ground. Tile-level surface operands
remain per unit tile ground and enter that column exactly once as
`X_lane = sum_i(f_i * X_i)`, where the ordered complete snow-surface tile set
closes to `sum_i(f_i) = 1` within the admitted topology tolerance. The sum is
not divided by the covered fraction. A mixed open/covered OFE therefore
requires both covered-canopy and open-snow boundary receipts; a missing,
duplicate, or incomplete tile contribution is typed failure. Lane-wide
precipitation enters once on the OFE-ground basis and any tile partition must
independently reconstruct it. The area basis, lane/OFE identity, ordered tile
fractions, and topology digest are restart identity. A future per-tile or
per-routing-cell snow owner requires a new versioned topology and cannot
reinterpret this lane state.

The source authority is `SC-VEGETATION-001@26` for V11 shared-tile air,
neutral momentum and canopy-surface conductances; `SC-SNOWENERGY-001` for
Stage 3 virtual `z_T=z_q=z_u=5 m`, exposed-snow `z_0,aero=0.005 m`, and
reciprocal sky/canopy/snow longwave; `SC-COUPLEDTIME-001@3` for support and
event chronology; and `SC-LANDSURFACEENERGY-001@7` for the snow-free
successor. No fixed forest attenuation multiplier or fitted proxy is
authority.

### Turbulent carrier equations

All turbulent fluxes use the same current trial shared-node state. Define
surface-to-node fluxes as positive from a canopy or snow surface into the
shared node:

```text
H_i = rho_a * c_p * g_H,i * (T_i - T_ca)
V_i = rho_a * g_q,i * (q_i - q_ca)
H_ref = rho_a * c_p * g_H,ref * (T_ref - T_ca)
V_ref = rho_a * g_q,ref * (q_ref - q_ca)
R_T = H_ref + sum_i(H_i) = 0
R_q = V_ref + sum_i(V_i) = 0
```

The unknown shared node is solved or iterated from these complete residuals;
the linear trial update, when every conductance is fixed, is the weighted
closure `T_ca=(g_H,ref*T_ref+sum(g_H,i*T_i))/(g_H,ref+sum(g_H,i))` and the
same expression for `q_ca`. Nonlinear conductances, saturation, and V11
surface temperatures are reevaluated until both residuals and the owning
constitutive tolerances pass. The carrier exports snow sensible and vapor
terms as `H_snow=-H_s` and `V_snow=-V_s`, positive into snow. It exports
canopy terms with the same sign reversal into each canopy owner. There is one
reference exchange and one snow exchange; no flux is copied into a second
owner.

Reference wind is first converted through the sealed exposure/transfer
receipt and the named neutral log-law in V11. Surface conductances remain
owner-specific. Raw `10 m` wind, a fixed attenuation factor, a hidden wind
floor, or independent canopy-air nodes is typed failure.

### Canopy--snow--sky longwave

The carrier imports the reciprocal V11 rank recurrence and the Stage 3
complementary longwave boundary. For the one-layer snow boundary:

```text
L_can       = sum_j(w_j * sigma*T_can,j^4)
L_snow,down = f_sky * L_atm + (1-f_sky) * L_can
L_snow,net  = L_snow,down - sigma*T_s^4
L_snow<->canopy = (1-f_sky) * (sigma*T_s^4 - L_can)
```

The canopy-side term is the equal-and-opposite exchange in the V11 recurrence;
atmospheric and canopy component terms remain separate operands. `L_can` is
reconstructed from the current V11 canopy component temperatures and
emissive-area weights; it is not the shared-air temperature and cannot be
copied from a stale or aggregate diagnostic. `f_sky` is
the existing effective-cover Beer-law translation, not a new input. Canopy
and snow emissivity are exactly one under the admitted one-layer domain.
Longwave is evaluated from the same current trial temperatures as the
turbulent residual; a stale or post-event snow temperature is invalid.

### Carrier algorithm and guards

### Lane-boundary receipt identity

`OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1` is an adopter-specific,
deterministic receipt wire and is not a coupled-time parent/restart identity.
Its SHA-256 preimage is the ASCII domain including its trailing NUL, followed
positionally by: little-endian `u32 lane_id`; `u64`-length-framed UTF-8 OFE ID;
little-endian `u128 start_ns,end_ns`; one byte area basis (`0=OfeGround`); five
raw 32-byte aggregate source-set digests; little-endian `u64` contribution
count; then each tile in strict ID order as `u64`-length-framed UTF-8 tile ID,
little-endian IEEE-754 fraction bits, one-byte boundary class
(`0=V11CanopyCovered`, `1=OpenSnow`), raw 32-byte model definition, beginning
Stage 3 state, provisional carrier, optical, reciprocal-longwave, and final
boundary digests, followed by little-endian IEEE-754 bits for the seven named
physical operands in schema order. The seven aggregate physical operands
follow in the same scalar encoding. No optional or extra fields exist.

Each aggregate source-set digest uses domain
`OPENWEPP_LANE_STAGE3_SOURCE_SET_V1\0`, one byte source index (`0` provisional,
`1` optical, `2` reciprocal longwave, `3` final), little-endian `u64` count,
and for each ordered contribution its framed tile ID, fraction bits, class,
model-definition digest, and selected raw source digest. Validation reconstructs
all four sets. This explicit alternative wire may not enter additive restart
or a coupled parent receipt; that requires a future contract amendment adopting
the repository canonical framed helper/domain and fixed vectors.

1. Validate forcing, exposure, virtual transfer geometry, active participant
   set, owner state, and support receipts without mutating owners.
2. Construct one current-trial shared node from all active V11 surfaces and
   the Stage 3 snow surface; do not split the node by stratum or regime.
3. Evaluate reciprocal canopy--snow--sky longwave and all turbulent residuals
   from the same trial state.
4. Iterate the complete coupled system until the carrier, V11, and Stage 3
   tolerances pass; reject incomplete or wrong-regime operands.
5. Independently reconstruct vapor mass, sensible/latent energy, longwave,
   cold-content, liquid, and event-time ledgers before producing a typed
   owner candidate. Commit only through the coupled-time complete-owner
   transaction.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-SNOWENERGY-036` | One shared canopy-air node jointly closes reference, all V11 canopy, and Stage 3 snow sensible/vapor exchange. | carrier residual / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-037` | Wind is a sealed exposure-projected operand at declared transfer geometry; raw 10 m wind and fixed attenuation are not substitutes. | forcing/exposure join / `SNOWENERGY-E-WIND-001` |
| `INV-SNOWENERGY-038` | Canopy--snow--sky longwave uses one reciprocal current-trial state and exact-one exchange. | radiation lineage / `SNOWENERGY-E-LW-001` |
| `INV-SNOWENERGY-039` | Snow fluxes stop at the accepted event and snow-free fluxes begin only on admitted successor support. | chronology / `SNOWENERGY-E-REGIME-001` |
| `INV-SNOWENERGY-040` | Canopy-intercepted snow is outside this carrier and cannot enter its mass or energy ledgers. | scope guard / `SNOWENERGY-E-SCOPE-001` |
| `INV-SNOWENERGY-042` | The single persistent Stage 3 lane owner is OFE-ground. Complete typed tile-ground snow-surface fluxes aggregate only as `sum_i(f_i X_i)` over an ordered tile set closing to one under `TOL-SNOWENERGY-002`; covered-subset renormalization, inconsistent common snow state, missing/open-surface omission, duplicate tiles, class/model substitution, or restart topology/basis substitution is prohibited. Uniform terminal-liquid projection preserves `sum_i(f_i M_i) = M_lane`; dividing the complete lane amount by every tile fraction is prohibited. | topology/area/source/state/restart guard / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-043` | Covered fixed-point acceptance reconstructs each candidate fingerprint independently; schema, terminal-event model, lane, interval, layer cardinality/order, density, and count-like settling state compare exactly. Numeric state uses only `TOL-SNOWENERGY-003` by physical class; fingerprints need not equal when admitted numeric state differs. | typed convergence/nonconvergence guard / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-044` | Lane receipt V1 is non-restorable. Additive restart is blocked until a normative V2 schema excludes initial-guess identity and defines exact canonical framing, topology/owner joins, and test vectors. | restart schema/version guard / hard `HOLD` |

`TOL-SNOWENERGY-003` governs only covered outer fixed-point convergence. SWE,
physical thickness, liquid depth, and refrozen depth use `1e-9 m` absolute;
temperature difference uses `1e-8 K` absolute; cumulative/detached mass uses
`1e-6 kg m^-2` absolute; cold-content and cumulative energy use `1e-6 J m^-2`
absolute. The depth and corresponding water-equivalent area-mass scales, energy
reconstruction scale, and covered-carrier temperature scale already exist in
this contract or implementation authority. Density is bitwise exact. The
stored `settle_day_count` chronology operand is bitwise exact despite its
`f64` representation. No relative tolerance, state repair, or cross-unit
substitution is admitted.

A future `OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V2` shall be the
parent/restart successor. It must use repository canonical framing, omit all
initial numerical guess identity, bind static topology context and ordered
covered/open final-boundary sources, and require exact lane/OFE, map-key,
class/model, fraction, and complete-destination joins. V1 remains historical
adopter evidence and must never be restored. Restart replay must join V2,
destination and component receipts, installed LSE and complete snow owners,
static tile/occupancy topology, and wet-liquid authorization. The V2 schema is
currently `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`: no implementation may
infer its exact fields, framing, ordering, or vectors from in-memory V1 types.

### Child 2C obligations and gaps

`OBL-SNOWENERGY-P-010`: emit one carrier candidate with complete operand
lineage, residuals, current-trial temperatures, and owner/support identities.
`OBL-SNOWENERGY-C-017`: independently reconstruct snow, vapor, liquid, energy,
longwave, and event-time closure and reject any alias or duplicate flux.
`OBL-SNOWENERGY-C-018`: independently reconstruct each lane boundary and
terminal-liquid handoff on OFE-ground basis from the complete ordered
tile-ground contribution set, and reject any non-closing topology, covered-
subset normalization, or restart area/topology substitution.
`OBL-SNOWENERGY-C-019`: independently reconstruct every candidate fingerprint,
then compare exact structure/count/density and only the unit-specific absolute
state residuals admitted by `TOL-SNOWENERGY-003`.
`OBL-SNOWENERGY-C-020`: reject V1 and inferred successor restart wires; require
the normative V2 schema and the complete static-topology and ending-owner join
before additive restore.

The carrier is `AUTHORITY_ADMITTED / IMPLEMENTATION_MISSING` until the later
default-off implementation package proves a real V11/Stage 3 consumer. The
existing wind-custody/exposure gap remains a typed precondition, not a license
for attenuation. Calibration is `CALIBRATION_NOT_APPLICABLE` here; no
efficacy, qualification, or empirical claim follows.

| ID | Gap | Disposition |
|---|---|---|
| `GAP-SNOWENERGY-014` | Default-off runtime carrier and actual V11 snow-covered consumer are not implemented. | later implementation package; current authority remains promotable |
| `GAP-SNOWENERGY-015` | Deployed/server exposure receipt is not available for every retained forcing value. | typed runtime precondition; no proxy or attenuation admitted |
| `GAP-SNOWENERGY-016` | Exact lane-receipt V2 fields, canonical framing, ordering, and test vectors are not yet defined. | `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`; V1 is never restart authority |

## Version 17 precipitation phase-parcel custody amendment

This approved amendment governs only the mass/enthalpy handoff into the
persistent Stage 3 support. `SC-VEGETATION-001@28` remains sole authority for
liquid interception, persistent canopy storage, initial and second drainage,
throughfall, and stemflow. `SC-LANDSURFACEENERGY-001` remains authority for
the ordered destination topology. No raw precipitation amount may be passed
around the parcel set, and no rule here intercepts solid precipitation in a
canopy.

The canonical parcel key is
`(lane_id, destination_topology_index, phase_rank, source_rank,
semantic_receipt_ordinal)`.
`phase_rank` is `0=solid`, `1=liquid`; `source_rank` is
`0=atmospheric_ground_snow`, `1=open_raw_rain`,
`2=vegetation_terminal_throughfall`,
`3=vegetation_initial_drainage`,
`4=vegetation_second_drainage`, and
`5=vegetation_terminal_stemflow`. Each vegetation route remains a distinct
parcel with its own mass, enthalpy provider, producer-state identity, and
receipt identity; aggregating drainage into throughfall is prohibited. Keys
are unique and strictly increasing.
The semantic ordinal distinguishes repeated parcels from the same route in
their producer order. Receipt hashes authenticate already ordered semantic
records; hash value order is never the semantic or arithmetic order. The
sealed set binds its schema, exact half-open
support, lane/OFE, OFE-ground basis, ordered destination topology/fractions,
parcel count, ordered keys and parcel receipts, and producer beginning-state
identities. An empty ordered vector sealed under the same set schema and
support/topology identity is the only representation of zero precipitation.

For each open destination, atmospheric solid and liquid are separately sealed:
solid uses `atmospheric_ground_snow`, and liquid uses `open_raw_rain`. For each
covered destination, solid likewise uses `atmospheric_ground_snow`, while all
ground-reaching liquid is imported only from the vegetation terminal release
owner as the distinct admitted throughfall, initial-drainage, second-drainage,
and stemflow parcels. A covered raw
rain parcel, an open vegetation-release parcel, or both liquid source classes
at one destination is invalid. Persistent canopy storage is not ground
precipitation. Canopy-intercepted snow remains outside the supported domain.

Each parcel carries finite non-negative tile-ground mass and a finite
temperature or specific-enthalpy provider sufficient to reconstruct its
advected heat. The Stage 3 candidate independently validates the set and then
uses its parcel identities exactly once for both mass and advected energy.
It aggregates each physical value to the lane with the destination fractions
from the sealed topology. The mass consumer set and advection consumer set
must be identical in key order and cardinality; equality of aggregate numbers
or set digests cannot excuse a missing, duplicated, or substituted parcel.
Validation completes before snow or soil owner mutation, and any failure uses
`SNOWENERGY-E-PRECIP-001` with complete rollback.

## Version 19 Covered Terminal Chronology Amendment

`CoveredTerminalExecutionMode` is a closed enum:

1. `PersistentReject` preserves the released covered path and rejects every
   unexpected terminal event before owner publication.
2. `DiscoveryProbe` begins from immutable complete owners at the current
   accepted cursor and may return only canonical bracket/candidate evidence.
   It cannot accept a slab or event, stage an ending, execute or retain WB14,
   publish a receipt or owner, mutate a controller, or retain any `last_*`
   field.
3. `ExactEndpoint { expected_tick }` evaluates the exact projected half-open
   support ending at `expected_tick`. For positive support it accepts only when
   the event is exactly at that endpoint, evaluated time equals support
   duration, and unevaluated time is zero. When `expected_tick` equals the
   current cursor, it uses only the sealed zero-support beginning-root branch
   defined below and performs no positive endpoint solve.

For every full step, first and second half step, rejected retry, bracket
endpoint, and bisection/root trial, the covered carrier is reconstructed from
the immutable complete-owner beginning plus that trial's start snow state and
exact absolute support. Reconstruction includes the sealed precipitation
parcel set and advection, V11 canopy/atmospheric identities, reciprocal
radiation, sensible exchange, bounded vapor/latent exchange, terminal
snow--soil receipt, current physical-child/WB14 identity, receiver topology,
and complete forcing digest. A sealed whole-support boundary cannot be scaled
or reused. Raw cloud fraction, fabricated clearness, compatibility longwave,
and `boundary=None` are inadmissible in covered terminal execution. Schema-v2
resolved beginnings with the admitted terminal model may cross into the
terminal domain; beginning-terminal and resolved-to-terminal events use the
same modes and guards.

The accepted positive-duration endpoint remains the exact dormant Stage-3
state returned by the solver. Represented layers and detached retained liquid
are zero, and the increment of `cumulative_unresolved_liquid_kg_m2` equals the
event terminal liquid. The zero-duration event does not consume layer liquid,
detached liquid, or `refrozen_liquid_m`; refrozen material remains solid
custody. Instead it adds one canonically keyed `ProducedUnconsumed` parcel per
receiver destination to the versioned snow owner. Surface-liquid and WB14
owners remain byte-identical. Only a later separately authorized receiver
checkpoint may atomically credit surface liquid and change parcel posture to
`Consumed`. This successor supersedes version-13 immediate-consumption wording
only when execution mode is the version-19 staged covered chronology. The
historical `terminal_receiver_v1` lane retains version-13 authority under its
distinct selector but is not evidence for this checkpoint; the two selectors
are mutually exclusive and staged chronology has no immediate-consumption
fallback.

`cumulative_unresolved_liquid_kg_m2` is a monotone lineage diagnostic, not a
physical storage term. It is excluded from the V4 owner stored-mass sum; its
event increment must satisfy
`abs(delta_cumulative_unresolved_liquid-m_terminal_liquid) <=
a_terminal_mass`, where `a_terminal_mass=1e-9 kg m^-2`. Physical
pending-liquid custody exists only in the V4
parcel set. If destination `i` has canonical OFE-ground fraction `f_i`, every
parcel carries uniform lane depth/mass `m_i=m_terminal_liquid` on its tile-ground
basis and `sum_i(f_i m_i)=m_terminal_liquid`, `sum_i f_i=1` within
`TOL-SNOWENERGY-002`. The same equation applies to zero sensible enthalpy.
No remainder redistribution or renormalization is allowed.

Before endpoint acceptance, independently reconstructed
`Q_terminal_unallocated` must satisfy `0 <= Q_terminal_unallocated <=
a_terminal_energy`, where `a_terminal_energy=1e-6 J m^-2`. A larger value
remains censored evaluation evidence and
fails the physical chronology with `SNOWENERGY-E-TERMINAL-ENERGY-001`; it is
not deleted, assigned to soil, or hidden in the parcel.

Terminal snow--soil custody uses
`SC-LANDSURFACEENERGY-001#INV-LANDSURFACEENERGY-127` and never supplies a
fabricated post-event snow node. Every terminating lane also emits a nonempty,
independently reconstructed physical ledger binding beginning ice, liquid and
enthalpy; projected precipitation/advection; covered radiation, sensible,
bounded vapor/latent and soil heat; melt, refreeze, deposition and sublimation;
event time and evaluated/unevaluated support; terminal liquid and energy; the
dormant endpoint; ProducedUnconsumed parcel set; and event-result, group and
owner-transition digests. An empty terminal ledger set is invalid.

An event exactly at the current search cursor is admitted only as replay of an
already accepted immediately preceding positive-support endpoint event whose
canonical result proves exact zero endpoint solid, dormant lane bytes, and an
unapplied coupled event receipt predecessor. A fresh beginning with any
positive represented solid, including a tolerance-sized residue, cannot create
a zero-duration event and remains fail-closed. The replay result has
`evaluated_seconds=0`, `unevaluated_seconds` equal the search duration, and no
new forcing integral. It has no positive-duration
snow--soil receipt and `Q_ss=0`, but still emits one present, nonempty terminal
physical ledger whose flux operands are explicit zeros and whose mass,
enthalpy, event, parcel, group, and owner-transition fields are complete.

### Version 19 kernel-profile integration

Authority anchors are released `INV-SNOWENERGY-034/036/038/039/042`,
`SC-LANDSURFACEENERGY-001` candidate v9, `SC-COUPLEDTIME-001` candidate v4,
mass/energy conservation, and deterministic step-doubling/root analysis.
`[DIRECT][Static]` applies to imported released fields and equations;
`[INFERENCE][Static]` applies to their staged covered composition.

| Profile surface | Version-19 binding |
|---|---|
| Inputs | immutable seven-owner beginning, enclosing/current support, current physical-child identity, exact covered forcing and topology, schema-v2 Stage-3 lane |
| Outputs | read-only bracket evidence or exact terminal result, terminal snow--soil disposition, nonempty terminal ledger, preaccept/accepted group identities, V4 pending parcel set |
| Mutated state | none in discovery; exact endpoint candidate in positive solve; only canonical snow owner among physical owners at the zero-duration event; coupled chronology separately appends its receipt |
| Branch priority | request/schema/identity -> mode -> cursor-root or positive support -> carrier/trial -> terminal energy -> ledger/parcel -> coupled event -> snow owner; no immediate-receiver fallback |
| Variables/units | tick/support `ns`; ice/liquid `kg m^-2 OFE-ground`; energy `J m^-2`; flux `W m^-2`; temperature `K`; fractions dimensionless |
| Aliases | `current_search_support=prepared.support`; `physical_child_ordinal=accepted positive-slab count`; `pending_terminal_parcels=V4 parcel map`; cumulative unresolved liquid is lineage only |
| Constants | Existing `TOL-SNOWENERGY-001/002/003`, `L_f`, water density, terminal event-model ID; no new physical constant or fit |
| Unit governance | Existing typed mass/energy/support wrappers apply; `f64` wire values use exact bit encoding; no scalar canonicalization is admitted |
| Calibration | `CALIBRATION_NOT_APPLICABLE`; no parameter, observation operator, fitting, efficacy, or transferability claim |
| Gaps | implementation, runner, receiver consumption, restart, activation and cutover remain `IMPLEMENTATION_MISSING / NON_PROMOTABLE` |

The algorithm is the numbered mode/trial/endpoint/owner sequence above. Its
degenerate state is the explicit cursor-root branch. Conservation equations
are the event mass/energy identities, `sum_i f_i=1`,
`sum_i(f_i m_i)=m_terminal_liquid`, and the equal/opposite snow--soil equation
from candidate v9. Every invalid branch maps to the typed errors below and
rolls back immutable beginnings.

Canonical invariant and guard-map rows `INV-SNOWENERGY-051..054` are integrated
in `## Invariants and Guard Map`; canonical producer/consumer rows
`OBL-SNOWENERGY-P-013/P-014/C-023/C-024` are integrated in the obligations
table. The remaining text is algorithm and test-vector detail, not a second
binding surface.

Required vectors include unavailable raw-cloud forcing with successful covered
probing and negative raw-evaluator reachability; beginning-terminal and
resolved-crossing events; event at start, interior and end; event after a prior
60- or 900-second child; same-tick coalescing, different-tick sequencing and a
surviving covered lane; no-event byte compatibility; exact child/WB14 identity;
nonempty terminal ledgers; canonical pending-parcel bytes; event, group,
topology, ordinal, owner, parcel and forcing poisons; and rollback at discovery,
endpoint solve, zero-duration event, successor execution and final publication.

## Change Log

| Version | Date | Change | Evidence |
|---:|---|---|---|
| 19 | 2026-08-24 | Defined covered terminal execution modes, exact per-trial covered-carrier reconstruction, resolved crossing, dormant endpoint and ProducedUnconsumed snow-owner custody, terminal snow--soil receipt join, and mandatory terminal physical ledger. Immediate receiver consumption, restart, activation and cutover remain excluded. | Static WIP review at `3fda26f0`; Child-1 contract-first correction |
| 18 | 2026-08-24 | Admitted persistent snow--soil boundary authority: one OFE/lane interface, bottom snow volume to first OFE soil node, two-half-layer series resistance, Crank--Nicolson beginning/end evaluation inside the covered fixed point, exact equal/opposite candidate custody, reconstructable receipt, and atomic rollback. | Pinned `frostn.for`/`tmpadj.for` series-resistance provenance; `SC-LANDSURFACEENERGY-001@8`; Child-1 contract-derived guards |
| 17 | 2026-08-24 | Admitted persistent-support precipitation custody: sealed ordered phase-parcel sets, present empty-set zero, open raw-rain versus covered route-distinct vegetation-terminal-liquid exclusivity, solid ground-snow bypass, OFE-ground aggregation, and exact same-set mass/advection consumption. No interception or canopy-snow physics was added. | Direct Child-1 checkpoint authority; `SC-VEGETATION-001@28`; `SC-LANDSURFACEENERGY-001`; contract-derived source guards |
| 13 | 2026-08-19 | Added fresh default-off `terminal_receiver_v1` authority (`INV-SNOWENERGY-035`) for earliest-event closure, exact-one 0 C liquid/enthalpy custody, remaining-time support, and post-event snow-operand exclusion while preserving all carrier/efficacy/production holds and evaluation-only INV-034 semantics. | Contract-first terminal handoff package |
| 12 | 2026-08-07 | Admitted a persistent-evaluation-only one-volume shallow-snow enthalpy solve, deterministic step-doubling policy, safeguarded earliest solid-exhaustion event, schema-v8 reconstruction, and explicit censoring of terminal liquid, energy, and remaining time. | CC0 libsnobal shallow-pack/cadence/phase chronology, conservation, and independent numerical review |
| 11 | 2026-08-07 | Recovered byte-identical WEPPpy lineage, retained centroids/flags, and exact parquet-to-CLI equality; separately labeled the nearby historical centroid/GRIDMET/share/format code path as static reconstruction and narrowed missing authority to deployed request/server semantics and two-sided aerodynamic exposure linkage. | Surviving `/wc1` run records, provider source/history, retained parquet/CLI equality, and independent custody/exposure reviews |
| 10 | 2026-08-07 | Separated nominal GRIDMET `10 m` source height, raw CLI/Stage 3 wind, PMET-local `2 m` adjustment, and virtual Stage 3 `5 m` transfer geometry; retained missing source/exposure authority and prohibited fitted attenuation or production correction. | Stage 3 wind source-custody result-blind authority reconciliation |
| 9 | 2026-08-07 | Distinguished evaluation-only raw vapor/latent opportunity `m_v,raw` from actual bounded transfer `m_v`; bound independent tuple-level transfer reconstruction, N/A and alias rejection, and capacity-truncation plausibility hold without changing production physics. | Stage 3 evolving-state carrier result-blind authority reconciliation |
| 8 | 2026-08-05 | Bound typed CLIGEN/openWEPP virtual-instrument heights `z_T=z_q=z_u=5 m` above the instantaneous modeled snow surface and exposed-snow aerodynamic roughness `z_0,aero=0.005 m`; distinguished aerodynamic roughness from active-layer depth and retained all carrier/cutover gates. | Direct user authority plus pinned libsnobal point-input defaults and fixture |
| 7 | 2026-08-04 | Admitted Stage 3 as the sole future melt owner after CoE failed the frozen specific-validation and enforceable-envelope predicates. Bound cold-content-first complete energy, bounded latent-fusion conversion, same-substep linked mass/liquid ledgers, no-dual-owner guards, and an atomic implementation hold covering incomplete fluxes and residual snow. Runtime CoE behavior remains unchanged as compatibility implementation, not target authority. | `SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION` frozen adjudication and pinned libsnobal chronology |
| 6 | 2026-08-01 | Made closure tolerances operand- and unit-explicit: `1e-9 m` SWE equals `1e-6 kg m^-2` for the same residual and governs vapor-to-sublimation transfer closure; hourly/daily vapor aggregation and represented-layer lifecycle retain their distinct `1e-9 kg m^-2` predicates. | `SNOW-SURFACE-EB-04S` result-blind authority freeze and independent authority reviews |
| 5 | 2026-07-31 | Separated represented density-layer mass lifecycle from aggregate SWE/depth residual tolerances. Layers above `1e-9 kg m^-2` after named SWE conversion remain represented with all coupled state; the independent `1e-9 m` closure guards remain unchanged. | `SNOW-SURFACE-EB-04D` authority reconciliation and required runtime replay |
| 4 | 2026-07-31 | Defined the exact libsnobal `1 kg m^-2` branches. Total mass `<=1` suspends before partition while CoE retains snow state; in a resolved pack, lower mass `<1` collapses to one thermal volume and continues, while lower equality remains two-volume. Both branches publish explicit diagnostics. | `SNOW-SURFACE-EB-04C` authority reconciliation and required runtime replay |
| 3 | 2026-07-30 | Replaced the failed snowfall-event top-layer provider with the Marks/SNOBAL upper-`0.25 m` active thermal control volume, harmonic active/lower `G_0`, conservative depositional-layer projection, and mass-dependent `60/15/1 minute` substeps. The amendment retains CoE snow existence/melt authority and prohibits shallow-pack temperature replacement, cold-content tax, fitted limiter, or new user coefficient. | `SNOW-SURFACE-EB-03A` contract-first authority trace |
| 2 | 2026-07-30 | Selected the Stage 3 top-layer thermal provider; bound `T_c=T_a`, polar-night typed unavailability, `R_a,min`, orthogonal default-off selectors, exact-one vapor/latent composition, snow-state mutation, and mass/energy closure obligations. Real S/LS execution then retained the seam as diagnostic/reproduction-only and opened `GAP-SNOWENERGY-007` because the common provider reaches `0 K` with material SWE remaining. | `SNOW-SURFACE-EB-03` contract-first implementation and terminal consumer evidence |
| 1 | 2026-07-30 | Initial contract: atmospheric longwave, effective-cover-derived diffuse sky view, complementary canopy exchange, runtime hold, and analytical obligations. | `SNOW-SURFACE-EB-01A` and `SNOW-SURFACE-EB-02` static/analytical evidence |
| 14 | 2026-08-20 | Bound the default-off shared V11/Stage 3 canopy-air carrier, complete turbulent residuals, reciprocal canopy--snow--sky longwave, sealed exposure wind, wrong-regime guards, and implementation-only disposition. | Child 2C authority package |
| 15 | 2026-08-22 | Selected the prospective one-column-per-lane OFE-ground area basis, complete tile-set weighted boundary, uniform-depth terminal identity, and topology-bound restart rule; prohibited covered-subset renormalization. | Direct user authority in Stage-3/V11 covered consumer package |
| 16 | 2026-08-22 | Admitted unit-specific covered fixed-point comparisons with independently reconstructed fingerprints and exact structural/density/count fields; reserved canonical lane receipt V2 without initial-guess identity and froze restart semantic joins. | Direct user authority and covered replay review |
