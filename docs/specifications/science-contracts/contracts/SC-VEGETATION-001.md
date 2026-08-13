---
contract_id: SC-VEGETATION-001
title: Native Vegetation State and Cross-Domain Boundary Contract
status: approved
maturity: active
owner: openWEPP maintainers + forest ecohydrology/hydrology reviewer
contract_version: 8
producer_scope:
  - Native vegetation configuration/runtime separation and stratum topology
  - Stage A potential response and Stage C vegetation finalization boundaries
  - Vegetation-owned canopy stores, elemental state, and transfer proposals
  - Read-only aggregate compatibility reduction
consumer_scope:
  - Native management, land-surface energy, soil hydrology, snow/frost, residue/biogeochemistry, and hillslope orchestration
evidence_level: static
last_reviewed: 2026-08-12
supersedes: []
superseded_by: []
---

# SC-VEGETATION-001 Native Vegetation State and Cross-Domain Boundary Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define canonical openWEPP ownership, state, ordering, units, conservation, and
failure semantics for a future native vegetation subsystem. Version 5 admitted
the indivisible `OPENWEPP_C3_WOODY_V1` constitutive stack: direct/diffuse
two-stream radiation, sunlit/shaded FvCB--Medlyn gas exchange, explicit leaf
energy balance, interval-equilibrium plant hydraulics, liquid interception, and persistent
vegetation C/N dynamics. It releases contract-first implementation authority;
it admits no production kernel, runtime selector, cutover, output, calibration,
or recommended site value. Version 6 superseded V1 for heterogeneous topology
by admitting `OPENWEPP_C3_WOODY_V2`: tile-resolved occupancy liquid state,
same-tile column routing, occupancy-local wet-energy/physiology/hydraulics,
occupancy-preserving water identities, exact area conversion, and fail-closed
migration. V1 remains immutable historical authority and is not a V2 runtime
alias. Version 7 admits `OPENWEPP_C3_WOODY_V3` as the executable successor to
the V2 topology: exact mixed leaf/stem radiation preparation and absorption
ownership, neutral canopy-surface wind, height-based stem hydraulics, one common
root node, an uncapped but hydraulically coupled accepted potential pass,
single-source/single-debit leaf respiration, typed numerical failure payloads,
and independent V3 constitutive vectors. V1 and V2 remain immutable historical
identities and neither is a V3 runtime alias. Version 8 admits
`OPENWEPP_C3_WOODY_V4` as the executable successor to V3 for shared-stratum
state: displayed leaf carbon alone owns LAI, derived leaf/stem/root area is
bound to that displayed pool, and the unconsumed previous leaf/root offset-flux
fields are removed rather than assigned invented semantics. V1, V2, and V3
remain immutable historical identities and are not V4 runtime aliases.

The local definition identity for `OPENWEPP_C3_WOODY_V1` is
`sha256:003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`
for
`docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v1_definition.json`.
That digest freezes fixed constants, selected family IDs, and typed unsupported
branches for historical V1. The V2 definition and digest are maintained by
`20260811-c3-woody-tile-liquid-topology-authority-001`; canonical equations and
semantics remain in this contract.

The immutable V2 definition identity is
`sha256:38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`
for
`docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/openwepp_c3_woody_v2_definition.json`.

The immutable V3 definition identity is the SHA-256 of
`docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_definition.json`.
That definition normatively imports the exact V2 digest and supersedes only the
V3 delta stated below. The final digest is bound by the definition artifact,
contract-derived tests, and this contract's Version 7 change record.

The immutable V4 definition identity is
`sha256:8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`
for `docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_definition.json`.
That definition normatively imports the exact V3 digest and supersedes only the
shared-stratum state, area derivation, and V3-to-V4 migration delta in the
Version 8 amendment below.

## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope:

- native stratum identity, explicit horizontal support, vertical overlap, and
  deterministic ordering;
- immutable configuration and initial-state references distinct from evolving
  vegetation state;
- caller-supplied site-specific stratum values, topology, and compatible
  initial state constrained by canonical schema semantics and guards;
- vegetation-owned liquid interception state, live/standing-dead elemental
  state, potential response, and finalization;
- Stage A potential response, Stage B hydrologic arbitration, and Stage C
  finalization with atomic two-owner commit;
- exact shared water, radiation, latent-energy, carbon, nitrogen, litter, and
  canopy-to-ground transfer lineage;
- a future single-owner split in which vegetation owns intercepted canopy snow
  and snow/frost owns ground snow; and
- a named read-only compatibility adapter that cannot feed native state.

Out of scope:

- C4, crops, nonvascular strata, recruitment, succession, fire, catastrophic
  disturbance, canopy snow/ice, and soil decomposition/mineralization;
- source-derived formulas, constants, bounds, defaults, naming, or control
  flow remain out of scope unless independently adjudicated; this includes
  RHESSysEastCoast behavior and any claim that GIS2RHESSys profile values are
  defaults, calibrated, validated, or transferable;
- the agricultural WEPP `Kcb`/LAI PMET partition as a future native-forest
  implementation target;
- soil-layer liquid/frozen storage mutation by vegetation;
- ground snow, litter/residue, soil carbon/nitrogen, infiltration, runoff,
  drainage, percolation, lateral flow, erosion, routing, or publication
  ownership; and
- replacement of current `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`,
  `SC-WATBAL-001`, `SC-SNOWFREEZE-001`, or direct-runtime authority.

## Authority Anchors with Top-Down Citations

| Anchor ID | Authority | Contract use | Evidence |
|---|---|---|---|
| `REF-VEGETATION-001` | Tague and Band (2004), *Earth Interactions* 8 | Ecosystem state/process separation is a scientific architecture precedent, not code authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-002` | Gash (1979), *QJRMS* 105:43-55 | Independent lead for future wet-canopy storage/evaporation authority; no formula admitted here. | `[DIRECT][Static]` |
| `REF-VEGETATION-003` | Shuttleworth and Wallace (1985), *QJRMS* 111:839-855 | Independent precedent for separately constrained canopy and soil fluxes; no formula admitted here. | `[DIRECT][Static]` |
| `REF-VEGETATION-004` | `SC-PLANT-001` CP-GSI01/02 | Current aggregate native phenology, canopy, foliar transfer, and real-consumer authority retained until cutover. | `[DIRECT][Static]` |
| `REF-VEGETATION-005` | `SC-EVAP-001`, `SC-WATBAL-001` | Existing ET demand/uptake lineage and hydrology-owned layer mutation. | `[DIRECT][Static]` |
| `REF-VEGETATION-006` | `SC-RESIDUE-001` authenticated forest-litter boundary | Existing exact-once ground dead-material receipt and custody. | `[DIRECT][Static]` |
| `REF-VEGETATION-007` | `SC-LANDSURFACEENERGY-001` | Exact-one water/energy custody, surface distinction, and authority-missing constitutive posture. | `[DIRECT][Static]` |
| `REF-VEGETATION-008` | `SC-SNOWFREEZE-001` and canopy-snow backlog | Ground snow remains snow/frost-owned; canopy-snow formulas remain non-promotable. | `[DIRECT][Static]` |
| `REF-VEGETATION-009` | Approved sanitized artifact `afd6044612f15ec0838bafd1c3ed63a5e06f912b0dc3224c5249eb656a6e988b` | `CODE-OBSERVED` semantic comparison evidence for strata, stage ordering, and custody only. | `[DIRECT][Static]` |
| `REF-VEGETATION-010` | Physical conservation and dimensional identity | Exact-one mass/energy/elemental transfers, non-negative stores, and no unowned mutation. | `[INFERENCE][Static]` |
| `REF-VEGETATION-011` | ADR-0011, ADR-0017, source firewall compliance PASS | Architecture-first authority; comparator/source behavior is a flag, not a scientific target, and unadjudicated source behavior cannot substitute for science authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-012` | `laurencelin/RHESSysEastCoast` commit `375c75b1cd2202217651dff43aa113d80b9c1118`, MIT license SHA-256 `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` | Licensed implementation provenance; not scientific authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-013` | `laurencelin/GIS2RHESSys` commit `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18`, same MIT license digest | Licensed format/profile provenance; parameter cells remain data rather than constitutive authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-014` | Code-to-literature audit package `20260808-rhessys-east-coast-code-literature-authority-audit-001` | Candidate 71-field/32-profile plus parser-only-default, generator, source-call, concordance, deviation, and authority-gap evidence, governed by its dual-review/disposition/verification cycle. | `[DIRECT][Static]` |
| `REF-VEGETATION-015` | Authority-admission package `20260808-rhessys-east-coast-vegetation-authority-admission-001` | Exact selected-field ledger, targeted replacement-authority attempts, and strict acquisition/schema adjudication. | `[DIRECT][Static]` |
| `REF-VEGETATION-016` | White et al. (2000), *Earth Interactions* 4(3), Appendix A; ORNL DAAC dataset DOI `10.3334/ORNLDAAC/652` | Defines documented BIOME-BGC parameter families and their units/biome domains; does not by itself authorize the selected GIS cells or a stand initial state. | `[DIRECT][Static]` |
| `REF-VEGETATION-017` | Hwang et al. (2009), WRR DOI `10.1029/2009WR007775`, Tables 2-3; Ford et al. (2010), *Ecohydrology*, Tables I-V | Coweeta species/catchment parameter and dated stand-observation evidence; also direct evidence that some allocation/phenology inputs were not species-level and that the pine and hardwood observations are from adjacent, distinct watersheds. | `[DIRECT][Static]` |
| `REF-VEGETATION-018` | wepppy Stevens Canyon peak-flow inversion investigation dated 2026-08-03, including PMET calibration, legacy-ET ablation, and water-balance attribution | Diagnostic mechanism evidence that the agricultural complementary `K_Ep/K_Es` partition structurally donates reduced canopy demand to soil evaporation and that disabling PMET alone does not recover the target. It is not calibration or validation authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-019` | Gash (1979), *QJRMS* 105:43-55, DOI `10.1002/qj.49710544304` | Primary process precedent for finite wet-canopy interception storage and evaporation as a distinct component; no complete formula family is admitted here. | `[DIRECT][Static]` |
| `REF-VEGETATION-020` | Shuttleworth and Wallace (1985), *QJRMS* 111:839-855 | Primary process precedent for separately constrained canopy and soil resistance/flux components; no complete formula family is admitted here. | `[DIRECT][Static]` |
| `REF-VEGETATION-021` | Javaux et al. (2013), DOI `10.2136/vzj2013.02.0042`; Cai et al. (2018), DOI `10.5194/hess-22-2449-2018` | Primary process leads for root-distribution and soil-state controls on root uptake; the selected layer request law remains authority-missing. | `[DIRECT][Static]` |
| `REF-VEGETATION-022` | Verstraete (1988), NASA NTRS `19880062508` | Primary process lead for multilayer canopy radiative transfer; the selected operator remains authority-missing. | `[DIRECT][Static]` |
| `REF-VEGETATION-023` | Medlyn et al. (2002), DOI `10.1046/j.1365-3040.2002.00891.x`; Bernacchi et al. (2013), DOI `10.1111/pce.12118`; Samanta et al. (2008), DOI `10.1029/2007WR006761` | Primary process leads for C3 temperature response and canopy-conductance scale; the selected complete families and site values remain separate decisions. | `[DIRECT][Static]` |
| `REF-VEGETATION-024` | Farquhar, von Caemmerer, and Berry (1980), DOI `10.1007/BF00386231`, reviewed bytes SHA-256 `ce15f7a78456bf8a9153b20204a6a0d51c3e2697c3a03105315634bf1fe05048`, pp. 78--86 | C3 biochemical assimilation equations. | `[DIRECT][Static]` `PRIMARY_PROCESS_AUTHORITY` |
| `REF-VEGETATION-025` | de Pury and Farquhar (1997), DOI `10.1111/j.1365-3040.1997.00094.x`, reviewed bytes SHA-256 `8a847133cf3d546bccd3e2dc076fa3b1e5e6f71edf2dd2efcc32282f3fc41fc6`, pp. 538--543 | Sunlit/shaded canopy scaling. | `[DIRECT][Static]` `PRIMARY_PROCESS_AUTHORITY` |
| `REF-VEGETATION-026` | Medlyn et al. (2011), DOI `10.1111/j.1365-2486.2010.02375.x`, accepted manuscript SHA-256 `57f9754dac8f81f257d819d474f6ed250b801179ceebadbe88c3f9c56cf17623`, eqs. 11--12 | Photosynthesis-linked stomatal conductance. | `[DIRECT][Static]` `PRIMARY_PROCESS_AUTHORITY` |
| `REF-VEGETATION-027` | CLM5 Technical Note, reviewed bytes SHA-256 `9ca0f0e5b7aff712a0ef7f5198f111c4b250cac4417a4f000e36c6c143f2e363`, Chapters 3, 5, 7, 9, 11, 16--21 | Exact established-model radiation, transfer, interception, gas-exchange, hydraulic and C/N definitions. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-028` | BIOME-BGC v4.2 theoretical framework, SHA-256 `476dd8d5606941ccfdd59de277d03671e764ac6ceac44d9bebd68bf61f00be85`, pp. 14--31 | Independent established-model C/N pool, respiration, allocation and turnover architecture. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-029` | `SC-BIOGEOCHEM-001` | Mineral-N arbitration and litter/CWD receiving-owner authority. | `[DIRECT][Static]` |
| `REF-VEGETATION-030` | `SC-VEGETATIONTRANSACTION-001` | Shared V2 occupancy water/energy identity, receiving-owner reconstruction, and all-owner atomicity. | `[DIRECT][Static]` |
| `REF-VEGETATION-031` | CLM5 Technical Note, Surface Albedos, equations 2.3.1--2.3.30, <https://escomp.github.io/CTSM/release-clm5.0/tech_note/Surface_Albedos/CLM50_Tech_Note_Surface_Albedos.html> | Two-stream coordinate over exposed leaf plus stem area, area-weighted leaf/stem reflectance/transmittance, and sunlit/shaded absorption. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-032` | CLM5 Technical Note, Fluxes, equations 2.5.117 and 2.5.122, <https://escomp.github.io/CTSM/release-clm5.0/tech_note/Fluxes/CLM50_Tech_Note_Fluxes.html> | Incident canopy wind equals friction velocity and drives characteristic-dimension-specific boundary resistance. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-033` | CLM5 Technical Note, Plant Hydraulics, Section 2.11.2, <https://escomp.github.io/CTSM/release-clm5.0/tech_note/Plant_Hydraulics/CLM50_Tech_Note_Plant_Hydraulics.html> | Common root-node hydraulic circuit, root-to-stem path, gravity, unstressed maximum evaluation, vulnerability response, and continuity. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-034` | CLM5 Technical Note, Photosynthesis, equations 2.9.10--2.9.17 and Chapters 17/19 in reviewed CLM5 bytes, <https://escomp.github.io/CTSM/tech_note/Photosynthesis/CLM50_Tech_Note_Photosynthesis.html> | Net assimilation subtracts leaf dark respiration; Rd uses its own peaked temperature response and one leaf-N/acclimation basis. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-VEGETATION-035` | ESCOMP/CTSM commit `8e1309ab0db671d884b80746cbae9bbaafbe78a7`, `src/biogeophys/PhotosynthesisMod.F90` SHA-256 `e4c9ad718209af44fcfdfc1d591bd2729d345f9e422cf5d9c8a889525d6a1cdf`, lines 1318--1322 and 1441--1447, <https://raw.githubusercontent.com/ESCOMP/CTSM/8e1309ab0db671d884b80746cbae9bbaafbe78a7/src/biogeophys/PhotosynthesisMod.F90> | Immutable directly resolvable transcription authority for leaf N in `g N m^-2 leaf`, the Atkin coefficients, Celsius `T10`, the positive-leaf-N branch, and `lmr25top` already expressed as the photosynthesis module's `umol CO2 m^-2 leaf s^-1` rate. | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |

Source-reported literature names in `REF-VEGETATION-009`,
`REF-VEGETATION-012`, or `REF-VEGETATION-013` are discovery leads only. They
are not admitted scientific authority until consulted and reviewed
independently. The MIT grants permit inspected/adapted implementation work and
redistribution with notice, but no RHESSys equation, constant, default, or
profile value is promoted by this contract.

## Variables and Units Using Canonical Symbols First

All are per horizontal stand/OFE area unless an intrinsic covered-area basis is
explicitly declared.

| Symbol | Units | Meaning | Owner |
|---|---|---|---|
| `tau` | opaque identity | timestep plus interval identity | orchestrator |
| `A` | `m^2` | strictly positive horizontal transaction area | orchestrator |
| `dt` | `s` | strictly positive interval duration | orchestrator |
| `f_t` | fraction | non-overlapping horizontal topology-tile fraction | native management |
| `C_s` | fraction | projected ground-area cover of stratum `s` | native management / vegetation state |
| `o=(s,t)` | typed identity | V2 occupancy: exact stratum/tile pair where `s` is present in `t` | native management / vegetation |
| `z_s` | `m` | stratum reference height | vegetation |
| `LAI_s`, `WAI_s` | `m^2 m^-2` | leaf and woody area per ground area | vegetation |
| `LAI_s,t`, `WAI_s,t` | `m^2 plant m^-2 tile-ground` | V2 conditional occupancy area, exactly `LAI_s/C_s`, `WAI_s/C_s` | vegetation, derived |
| `L`, `S`, `P=L+S` | `m^2 plant m^-2 tile-ground` | V3 occupancy leaf, stem, and total exposed plant area used by one two-stream layer | vegetation, derived |
| `Omega`, `K_eff` | fraction, `m^2 ground m^-2 plant` | clumping index and direct extinction `K_eff=Omega*K` | vegetation/radiation |
| `rho_lambda`, `tau_lambda` | fraction | V3 area-weighted effective reflectance/transmittance for band `lambda` | vegetation/radiation |
| `A_leaf,sun,lambda,j`, `A_leaf,shade,lambda,j`, `A_stem,lambda,j` | `W m^-2 tile-ground` | physically owned V3 absorbed radiation by band `lambda` and incident component `j` | radiation/energy |
| `r_s,l` | fraction | root participation fraction for stratum `s`, soil layer `l` | native management / vegetation |
| `S_liq,s` | `kg m^-2` | liquid water stored on stratum `s` | vegetation |
| `S_liq,s,t` | `kg H2O m^-2 tile-ground` | V2 sole mutable canopy-liquid store for occupancy `(s,t)`; aggregate is derived only | vegetation |
| `S_snow,s` | `kg m^-2` | future intercepted canopy-snow water-equivalent store | vegetation; constitutive behavior non-promotable in versions 2-4 |
| `P_liq,s` | `kg m^-2` | interval-integrated liquid incident on stratum `s` | upstream canopy/forcing handoff |
| `P_liq,s,t`, `E_int,s,t` | `kg H2O m^-2 tile-ground` | V2 occupancy-local incident liquid and accepted wet evaporation/condensation amount | vegetation + energy join |
| `R_through,s,t`, `R_stem,s,t`, `R_drain,s,t` | `kg H2O m^-2 tile-ground` | V2 local throughfall, stemflow, and explicit initial-plus-second drainage | vegetation |
| `E_int,s` | `kg m^-2` | interval-integrated actual evaporation from canopy liquid store | vegetation + energy join |
| `R_down,s` | `kg m^-2` | interval-integrated typed total downward liquid release | vegetation |
| `R_stem,s`, `R_drip,s` | `kg m^-2` | interval-integrated distinct stemflow and drip/drainage terms | vegetation |
| `Q_rad,k,j` | `J m^-2` | interval-integrated radiation energy in band/direction `k` received by component `j` | land-surface energy |
| `D_s,l` | `kg m^-2` | interval-integrated Stage A root-water request | vegetation |
| `D_W,s,t,l`, `A_W,s,t,l`, `F_W,s,t,l` | `kg H2O m^-2 stand-ground` | V2 occupancy-preserving water request, maximum authorization, and finalized use | vegetation / hydrology |
| `A_W,s,l`, `F_W,s,l` | `kg m^-2` | maximum hydrologic authorization and finalized withdrawal | soil hydrology / vegetation finalization |
| `A_l` | `kg m^-2` | same-snapshot layer liquid admissible to all Stage B withdrawals on the transaction area basis | soil hydrology |
| `W_comp,l` | `kg m^-2` | interval-integrated non-vegetation competing withdrawal accepted from layer `l` | soil hydrology |
| `T_s` | `kg m^-2` | interval-integrated Stage C actual transpiration | vegetation |
| `E_floor,j` | `kg m^-2` | interval-integrated actual evaporation from explicit forest-floor recipient `j`, such as litter or mineral soil | owning forest-floor water/energy component, never a complement of `T_s` |
| `h_v` | `J kg^-1` | authority-tagged vaporization enthalpy for the accepted state | land-surface energy |
| `Q_T,s` | `J m^-2` | interval-integrated latent-energy debit paired with `T_s` | land-surface energy |
| `M_C,p`, `M_N,p` | `kg C m^-2`, `kg N m^-2` | vegetation elemental pool `p` | vegetation |
| `L_DM,c` | `kg dry matter m^-2` | interval-integrated dead-material transfer by class `c` | vegetation to residue/biogeochemistry |
| `L_C,c`, `L_N,c` | `kg C m^-2`, `kg N m^-2` | elemental content of transferred material | vegetation to residue/biogeochemistry |
| `Ep_compat` | `mm` | future interval-integrated aggregate projection of accepted transpiration | read-only adapter |
| `PAR_s,c` | `W m^-2 leaf` | absorbed PAR for sunlit/shaded class `c` | radiation/vegetation handoff |
| `A_n,s,c` | `umol CO2 m^-2 leaf s^-1` | net C3 leaf assimilation | vegetation |
| `g_s,s,c` | `umol H2O m^-2 leaf s^-1` | Medlyn stomatal conductance | vegetation |
| `T_leaf,s,c` | `K` | converged leaf temperature | vegetation/LSE join |
| `psi_s,n` | `mm H2O` | root/xylem/leaf-node water potential; the exact E14/E15 hydraulic basis | vegetation |
| `u_ref`, `u_star` | `m s^-1` | reference wind and neutral friction velocity at the canopy surface | forcing / vegetation, derived |
| `u_leaf`, `u_wet`, `u_stem` | `m s^-1` | distinct semantic V3 surface-wind operands, each exactly equal to `u_star` | vegetation, derived |
| `z2`, `Delta_psi_stem` | `m`, `mm H2O` | root-to-stem conducting length `height_m` and gravitational head `1000*height_m` | vegetation, derived |
| `psi_root` | `mm H2O` | V3 single common root-node potential connected in parallel to every soil layer | vegetation |
| `E_c,max`, `E_c` | `kg H2O m^-2 leaf s^-1` | internal `beta_c=1` class maximum and accepted uncapped coupled class transpiration | vegetation |
| `beta_sun`, `beta_shade` | `1` | accepted class-resolved hydraulic stomatal factors solved inside Stage A or the capped pass | vegetation |
| `beta_hyd` | `1` | persisted Emax-weighted aggregate of accepted class factors; warm-start/diagnostic only | vegetation |
| `Rd25`, `Rd_c` | `umol CO2 m^-2 leaf s^-1` | Atkin leaf-N/T10 respiration basis at 298.15 K and class temperature response | vegetation |
| `GPP_s`, `R_m,s`, `R_g,s` | `kg C m^-2` | interval carbon gain and respiration | vegetation |
| `NSC_C,s` | `kg C m^-2` | nonnegative unallocated carbon carried across intervals | vegetation |
| `D_N,s,l,q`, `A_N,s,l,q`, `F_N,s,l,q` | `kg N m^-2` | mineral-N request, maximum authorization, and finalized use | vegetation / `SC-BIOGEOCHEM-001` |

`kg m^-2` and `mm water` are not silently interchangeable. Any compatibility
conversion uses the version-5 identity `1 mm = 1 kg m^-2`, derived from
`rho_water=1000 kg m^-3`; no other density or unit shortcut is accepted.

Every transfer above is an amount integrated over `tau`; `interval^-1` is not a
physical unit. A future rate-producing constitutive owner must declare its time
unit and integrate through `dt` before entering these amount ledgers.

For V2, tile-local energy is `J m^-2 tile-ground` per interval. Every
stand-ground aggregate is derived exactly as `sum_t(f_t*X_s,t)`. No local and
aggregate store may coexist as independent mutable sources.

## Algorithm State Surfaces

### Required Inputs

- one immutable `tau/A/dt` transaction identity;
- versioned native configuration, coverage topology, parameter-set references,
  initial-state references, and rooting-profile references with digests;
- beginning vegetation state identified by state version and owner;
- meteorological, precipitation-phase, and land-surface radiation handoffs;
- read-only soil-layer potential, temperature, liquid/frozen accessibility,
  thickness, and identity observations; and
- explicit current ground-snow/surface recipient state.

### Required Outputs

- validated topology and deterministic top-to-bottom stratum ordering;
- Stage A potential response with layer requests and reconstructible proposed
  canopy/radiation/elemental transfers;
- Stage B authorization with `A_W,s,l`, finalized `F_W,s,l`, availability lineage, and one reason
  code per request;
- Stage C accepted vegetation state and actual water/energy/elemental ledgers;
- separate actual canopy transpiration, wet-canopy evaporation, and explicit
  forest-floor evaporation ledgers without a complementary-demand identity;
- receiving-owner receipts or typed rejection; and
- optional compatibility values with a field-specific reduction receipt.

### Mutated State Surfaces

Vegetation may mutate only candidate canopy liquid, future canopy snow,
geometry, phenology, live/standing-dead, and internal elemental state. Soil
hydrology alone constructs candidate soil-layer mutations. Receiving owners
alone construct candidate ground-snow, litter/residue, and soil C/N mutations.
The orchestrator commits all accepted candidate states atomically; errors leave
every owner state byte-identical.

## Algorithm Specification with Step Sequence

1. **Validate configuration.** Require unique IDs; immutable version/digest;
   explicit units, area basis, parameter classification, and initial-state
   classification; finite domains; and no hidden fallback.
2. **Validate exact cover topology.** Horizontal tiles are non-overlapping,
   each `f_t > 0`, and `sum_t f_t = 1` within a separately admitted
   representation tolerance. A tile contains at most one stratum at a given
   vertical rank and may contain strata at several ranks. Define
   `C_s = sum(f_t for tiles containing s)` and aggregate compatibility cover as
   `C_union = sum(f_t for tiles containing at least one stratum)`. Thus cover
   closes within a rank while cross-rank cover may sum above one without an
   independence assumption.
3. **Validate ordering.** Sort vertical ranks top to bottom; ties use the stable
   configuration rank and ID. Height inconsistency with declared rank is a
   typed error, never a biomass or cover perturbation.
4. **Assemble Stage A.** Freeze beginning snapshots. Vegetation may compute
   only contract-authorized potential responses. It emits `D_s,l >= 0`, canopy
   transfer proposals, separately identified wet-canopy evaporation and
   canopy-transpiration candidates, and sufficient operands for independent
   reconstruction. Any forest-floor owner constructs its own evaporation
   candidate from its own state and available-energy lineage; it does not
   receive a residual canopy demand.
   It does not mutate hydrologic/frozen state or publish actual transpiration,
   assimilation, or litter receipt.
5. **Authorize water.** Hydrology evaluates all same-`tau` demands and
   competing withdrawals against the same layer snapshot and returns maximum
   `0 <= A_W,s,l <= D_s,l` without mutation. On the same
   horizontal area basis, every layer must also satisfy
   `sum_s A_W,s,l + W_comp,l <= A_l`; no individually valid request can overbook
   the shared layer snapshot. Each request carries one enumerated reason:
   `fully_supplied`, `zero_demand`, `liquid_storage_limit`,
   `frozen_exclusion`, `rooting_exclusion`, or `competing_demand`. Invalid
   state, missing policy authority, or ambiguous priority is a typed failure,
   not a limitation reason.
   Version 5 selects equal-status proportional arbitration within each layer:
   with eligible request sum `R_l`, `A_W,s,l=D_s,l` when `R_l<=A_l-W_comp,l`,
   and otherwise `A_W,s,l=D_s,l*(A_l-W_comp,l)/R_l`. A negative remainder or
   nonfinite operand is a typed error; `R_l=0` returns exact zero. No stratum,
   species, or call-order priority is implicit.
6. **Finalize water and carbon.** Vegetation re-solves hydraulic
   complementarity, energy and gas exchange under the authorization caps and
   emits `0<=F_W,s,l<=A_W,s,l`; hydrology validates and constructs its candidate
   debit. `T_s=sum_l F_W,s,l`. Turnover/retranslocation candidates then precede
   new-growth N demand.
7. **Authorize/finalize N.** Biogeochemistry returns `A_N<=D_N` without
   mutation. After the final water-limited GPP is known, vegetation computes
   final external need and distributes `min(need,sum A_N)` in proportion to
   the authorizations, producing `F_N<=A_N`; unused authorization is not
   debited. Biogeochemistry validates finalized use and forms its candidate.
8. **Join latent energy.** Land-surface energy supplies authority-tagged `h_v`
   and independently reconstructs `Q_T,s = -h_v*T_s`. Missing authority,
   lineage mismatch, or a second latent debit fails the transaction.
9. **Apply receiving-owner proposals.** Ground water/snow and dead-material
   receivers construct candidate receipts. Every transfer has one donor debit
   and one recipient credit or named atmospheric sink on the same basis.
10. **Close and commit.** Both owners independently reconstruct every shared
   transfer. Only after all water, energy, carbon, nitrogen, and material
   identities and typed receipts pass does the orchestrator atomically commit
   candidate states and expose adapter/publication candidates.
11. **Coupled constitutive solve.** Versions 5--7 bind the exact equations,
    branch order, solver tolerances and limits in `INV-VEGETATION-062` through
    `INV-VEGETATION-072`. Radiation precedes the nested FvCB--Medlyn--leaf-
    energy--hydraulic solve. Nonconvergence authorizes no last iterate or
    fallback flux.

### `OPENWEPP_C3_WOODY_V1` Equation Set

1. Direct/diffuse VIS/NIR uses CLM5 two-stream equations 3.1--3.30. Each
   explicit topology tile is traversed top-to-bottom; terminal transmitted
   energy reaches the ground. Sunlit area is
   `Lsun=(1-exp(-kb*L))/kb`, `Lsha=L-Lsun`; zero class area produces zero class
   flux without division.
   Executably, for cumulative plant area `x`, each band solves
   `-mubar*dIup/dx + b*Iup-c*Idown=d*exp(-Kx)` and
   `mubar*dIdown/dx + b*Idown-c*Iup=f*exp(-Kx)`, where
   `G(mu)=phi1+phi2*mu`, `phi1=.5-.633*chi-.33*chi^2`,
   `phi2=.877*(1-2phi1)`,
   `mubar=integral_0^1 mu'/G(mu') dmu'`
   `=[1-(phi1/phi2)ln((phi1+phi2)/phi1)]/phi2`, with the removable
   `phi2=0` limit `mubar=1`; the defining integral uses the same deterministic
   adaptive-Simpson rule and failure semantics as the upscatter integral,
   `K=G/mu`, `omega=rho+tau`,
   `omega*beta=.5[(rho+tau)+(rho-tau)((1+chi)/2)^2]`. Direct upscatter uses
   CLM5 3.15 with the unclamped defining integral from 3.16:
   `as=(omega/2) integral_0^1 [mu'*G(mu)/(mu*G(mu')+mu'*G(mu))]dmu'` and
   `omega*beta0=(1+mubar*K)*as/(mubar*K)`. Version 5 evaluates that smooth
   integral by deterministic adaptive Simpson quadrature to absolute `1e-14`
   in at most 20 bisection levels; failure is typed nonconvergence. When
   `omega=0`, the products `omega*beta` and `omega*beta0` are exact zero and
   neither quotient is evaluated. When direct incident flux is zero, the
   direct branch and its zenith-angle operands are absent.
   The executable coefficients under the displayed sign convention are
   `b=1-(1-beta)omega`, `c=omega*beta`,
   `d=omega*mubar*K*beta0`, and `f=omega*mubar*K*(1-beta0)`: the upward
   equation's right side is `d*exp(-Kx)` and the downward equation's right side
   is `f*exp(-Kx)`. Coefficients are not divided by `mubar` twice.
   Boundary conditions are incident downward
   diffuse at `x=0` and `Iup(X)=ground_albedo*(Idown(X)+direct*exp(-KX))`.
   A real 2x2 matrix exponential plus analytic exponential particular solution
   solves the boundary value problem; singular resonance uses its exact
   matrix-exponential integral, never an exponent clamp. Absorption is incident
   minus top reflection and terminal direct/diffuse transmission. Sunlit
   absorption integrates the direct-beam illuminated fraction `exp(-Kx)`;
   shaded absorption is total minus sunlit. Every band/direction closes before
   the next stratum receives terminal transmission.
2. Liquid interception is ordered exactly as follows:
   `fint=alpha_liq*tanh(L+S)`, `Pint=fint*P`, `Pfree=P-Pint`,
   `stemflow=fstem*Pfree`, `throughfall=(1-fstem)*Pfree`,
   `Sstar=S0+Pint`, `Smax=pliq*(L+S)`, `drip=max(0,Sstar-Smax)`, then
   initial capacity drainage. Then `fwet=(S/Smax)^(2/3)` for `Smax>0` (zero
   otherwise). Wet leaf area is `Awet_leaf,c=fwet*Lclass,c`, wet stem area is
   `Awet_stem=fwet*S`, and their sum is `fwet*(L+S)`; dry transpiring leaf area
   is `Ldry,c=(1-fwet)*Lclass,c`. A positive
   vapor amount is limited by stored liquid and subtracted. A negative vapor
   amount is condensation: add its magnitude, reapply the same capacity and
   drainage law, and expose the second drainage explicitly. The accepted
   `S1` follows that ordered branch. Subfreezing canopy temperature returns
   `VEG-E-040`, never a liquid calculation. CLM5 7.2--7.12
   defines interception/store/drip; the conservative stemflow split is
   `OPENWEPP_CANONICAL_SELECTION`. It must close
   `S0+rain+condensation=S1+Ewet+throughfall+stemflow+drainage`.
3. For each sun/shade leaf class, C3 gross rates are
   `Ac=Vcmax(ci-Gamma)/(ci+Kc*(1+Oi/Ko))`,
   `Aj=J*(ci-Gamma)/(4ci+8Gamma)`, and `Ap=3Tp`. `J` is the smaller root of
   `thetaJ*J^2-(IPSII+Jmax)*J+IPSII*Jmax=0`. CLM5 9.8 quadratic co-limitation
   yields `Ag`, and `An=Ag-Rd`. Temperature responses are exactly CLM5
   9.9--9.11, evaluated without exponent clamps.
4. Potential Medlyn conductance is
   `gs,pot=g0+1.6*(1+g1/sqrt(D_kPa))*An/(cs/Patm)` for positive assimilation;
   otherwise `gs,pot=g0`. Actual conductance is
   `gs=g0+beta_hyd*(gs,pot-g0)`, `0<=beta_hyd<=1`. The unique `beta_hyd` is
   solved with gas, canopy-air, leaf-energy, and hydraulic residuals so
   gas/energy transpiration equals hydraulic leaf flux to the water-residual
   tolerance. `beta_hyd=1` defines unstressed maximum leaf demand; a one-pass
   diagnostic hydraulic reduction is prohibited. `g0` and `g1` are required
   caller parameters. `D<=0`,
   nonpositive surface CO2, or nonpositive pressure is a typed domain error,
   never a hidden floor.
   The coupled diffusion identities, with `An` in `umol CO2 m^-2 leaf s^-1`,
   `rb,rs` in `s m^-1`, and `c` in Pa, are
   `cs=ca-1.4*rb*R*Tleaf*An*1e-6` and
   `ci=ca-(1.4*rb+1.6*rs)*R*Tleaf*An*1e-6`. Surface VPD is derived from the
   solved `Tleaf` and `qcan`; ambient VPD and omission of `rb` are prohibited
   aliases.
5. Version 5 selects neutral aerodynamic transfer on its stated forcing domain:
   `rah=ln((zref-d)/z0m)*ln((zref-d)/z0h)/(kappa^2*u)`, with the analogous
   `raw` using `z0q`; CLM5 5.122 gives
   `gb=Cv*sqrt(u_leaf/dleaf)`, `Cv=0.01 m s^-1/2`, and `rb=1/gb`. Required
   caller roughness/displacement values satisfy `u,u_leaf>0`,
   `zref>d+max(z0m,z0h,z0q)`, and `dleaf>0`; nonneutral/calm forcing is typed
   unsupported, not floored. For leaf class `c`,
   `g_ah=1/rah`, `g_aw=1/raw`, and canopy-air nodes solve
   `g_ah*(Tcan-Tair)=sum_c gb_c*L_c*(Tleaf_c-Tcan)` and
   `g_aw*(qcan-qair)=sum_c[gb_c*Awet_c+(Ldry_c/(rb_c+rs_c))]
   *(qsat(Tleaf_c)-qcan)`. For each class,
   `LWnet=eps*L_c[Lin_down+Lin_up-2*sigma*Tleaf^4]`,
   `H=rho*cp*gb*L_c*(Tleaf-Tcan)`, and
   `Etrans=rho*(qsat(Tleaf)-qcan)/(rb+rs)*Ldry`; the solved residual is
   `SWabs+LWnet-H-lambda*Etrans=0` for dry leaf area. Wet leaves and stems use
   a common stratum wet-surface node `Twet`, caller `wet_surface_dimension_m`,
   `gb_wet=Cv*sqrt(u_wet/wet_surface_dimension_m)`, and
   `Ewet=rho*(qsat(Twet)-qcan)*gb_wet*(Awet_leaf+Awet_stem)`. Its separate
   signed energy residual uses the wet-supported absorbed shortwave/longwave,
   sensible heat over the same total wet plant area, and `lambda*Ewet`.
   Version 5 partitions stratum absorbed energy between wet and dry surfaces
   in exact proportion to their plant-area fractions before either residual;
   this is an `OPENWEPP_CANONICAL_SELECTION` and closes back to the unpartitioned
   stratum energy. Negative vapor flux is condensation
   only when the liquid store can receive it; otherwise it is typed unsupported.
   Positive wet evaporation is capped by `S/dt` inside the wet energy solve;
   an active cap re-solves `Twet` using the capped latent term, and the identical
   interval amount `Ewet*dt` debits the store. Dry stems use a separate
   nontranspiring `Tstem` node, caller `stem_dimension_m` and
   `stem_emissivity`, and a shortwave/longwave/sensible residual with zero
   latent flux. Wet leaf, wet stem, dry leaf, and dry stem energy operands sum
   exactly to the unpartitioned stratum operand.
   Here `qsat=0.622*esat/[Patm-0.378*esat]`; liquid `esat` is exactly the
   CLM5 5.154/Table 5.2 eighth-order polynomial over `0..100 degC`. The exact
   coefficients, `sigma`, `cp_air`, and liquid `lambda` are fixed
   model-definition constants; leaf and wet `eps` are their distinct caller
   emissivities and
   `rho_air=Patm/(Rdry*Tcan)` is derived at the solved node. The ground owner independently solves its own residual and never consumes a
   canopy-demand complement.
6. Hydraulic path equations are exactly CLM5 11.8--11.18 and 11.27--11.28:
   leaf, stem and layer root fluxes include their LAI/SAI/RAI area factors,
   path length, gravity, series soil/root conductance, and
   `v(psi)=2^[-(psi/p50)^ck]`. Unstressed maxima are the `beta_hyd=1`
   gas/energy solution. Demand is
   `E_sun=E_sun,max*v(psi_sun;p50e,ck)` and
   `E_shade=E_shade,max*v(psi_shade;p50e,ck)`, with unstressed maxima from the
   common gas/energy solve. The four CLM5 11.25 continuity equations enforce
   `E_sun=q1a`, `E_shade=q1b`, `q1a+q1b=q2`, and `q2=sum_i q3_i`, selecting
   interval-equilibrium potentials; previous potentials are warm starts only.
   The common coupled residual also enforces equality of total hydraulic leaf
   flux and actual gas/energy transpiration by solving `beta_hyd`; no relative
   mismatch allowance or one-pass diagnostic flux is admitted.
   Version 5 does not admit hydraulic redistribution: any accessible-layer
   `q3_i<0` rejects the candidate with `VEG-E-063`; it is never silently
   projected to zero. Nonnegative `q3_i*dt` are Stage-A requests. Stage-C imposes
   `0<=q3_i*dt<=A_W,i` with hydraulic-law equality unless capped and cap equality
   when constrained. Frozen, dry/inaccessible and zero-root layers have zero
   flux. Finalized `F_W=q3_i*dt` alone is debited and transpiration is its sum.
7. Leaf N determines capacity before FvCB:
   `Vcmax25=Nleaf_area*rubisco_N_efficiency`,
   `Jmax25=Nleaf_area*electron_N_efficiency`,
   `Tp25=tp_vcmax_ratio*Vcmax25`, and
   `Rd25=Nleaf_area*rd_leaf_N_rate`; `Nleaf_area` is the accepted leaf-N pool
   divided by accepted `LAI`, with sun/shade N allocated proportional to their
   leaf areas, in `kg N m^-2 leaf`. Zero LAI takes the exact zero-capacity
   branch without division. All four coefficients are required caller
   parameters with declared units. Gas resistance converts by
   `g_mol=g_ms*Patm/(R*T)` and `g_umol=1e6*g_mol`, preventing Pa/ppm or
   mol/micromol aliasing. Gross assimilation becomes interval GPP using exact leaf-class areas, `dt`,
   molar carbon conversion, and topology weights. Stomata use net `An=Ag-Rd`,
   but interval `GPP` uses gross `Ag`; leaf maintenance respiration owns `Rd`
   exactly once downstream. CLM5 Chapters 17 and 19
   define maintenance respiration, growth respiration, relative allocation,
   storage/transfer pools and stoichiometric N demand. A single receipt-bounded
   scale applies to all proposed tissue growth.
8. Maintenance respiration and allocation use the following selected CLM5
   sequence. Leaf respiration is
   `mr_leaf=(i_atkin+0.2061*Nleaf_area-0.0402*T10)*leaf_area*dt` after the
   source g-to-kg conversion. The persistent ten-day temperature state follows
   `T10_1=Tair+(T10_0-Tair)*exp(-dt/864000 s)` before that evaluation;
   live-stem/coarse-root respiration is
   `Npool*MRbase*MRQ10^((Tair-293.15)/10)*dt`; fine-root respiration sums the
   same expression over root-N layer fractions and soil temperatures. With
   their sum `mr`, `GPP_mr=min(GPP,mr)`, `XS_mr=mr-GPP_mr`,
   `GPP_XS=min(max(-XS_C/(86400*tau_XS),0),GPP-GPP_mr)`, and
   `XS_C'=XS_C-XS_mr+GPP_XS`. Then `Cavail=GPP-GPP_mr-GPP_XS`,
   `Callom=(1+g1)*(1+a1+a3*(1+a2))`, and the leaf, fine-root, live-stem,
   dead-stem, live-coarse-root, and dead-coarse-root tissue coefficients are
   respectively `1,a1,a3*a4,a3*(1-a4),a2*a3*a4,a2*a3*(1-a4)` times
   `Cleaf_tot`. `Nallom=1/CNleaf+a1/CNfroot+
   a3*a4*(1+a2)/CNlivewood+a3*(1-a4)*(1+a2)/CNdeadwood` and
   carbon offered for new growth is `Coffer=Cavail+NSC_C0`. Its potential
   N demand is `Ndem_pot=Coffer_pot*Nallom/Callom`. Before external arbitration,
   let `Nint_offer=Nretrans_pool0+Nretrans_generated`; external shortfall is
   `Dext_pot=max(0,Ndem_pot-Nint_offer)`. Required caller
   `n_root_fraction[l]` sums to one and `nh4_request_fraction` is in `[0,1]`;
   requests are `D_N,l,NH4=Dext_pot*n_root_fraction[l]*nh4_request_fraction` and
   `D_N,l,NO3=Dext_pot*n_root_fraction[l]*(1-nh4_request_fraction)`.
   After final water-limited GPP recomputes
   `Ndem_final=Coffer_final*Nallom/Callom` and
   `Dext_final=max(0,Ndem_final-Nint_offer)`, let
   `Asum=sum_l,q A_N,l,q`, `Fext=min(Dext_final,Asum)`, and, when `Asum>0`,
   `F_N,l,q=Fext*A_N,l,q/Asum` (all zero when `Asum=0`). Internal use is
   `Nint_use=min(Nint_offer,Ndem_final)`, `Nused=Nint_use+Fext`, and the unique
   receipt-bounded common scale is `eta=1` for zero demand, otherwise
   `min(1,Nused/Ndem_final)`. Then
   `Cleaf_tot=eta*Coffer_final/Callom`, `Rg=g1*Cleaf_tot*(1+a1+a3*(1+a2))`,
   and each tissue amount splits `fcur` to display and `1-fcur` to storage.
   Each tissue N credit is its tissue C allocation divided by the exact tissue
   C:N; their sum equals `Nused`. Debit `Nint_use` once from retranslocation and
   `F_N` once from mineral pools. Carry unused carbon exactly as
   `NSC_C1=(1-eta)*Coffer_final`; it is neither respiration nor a sink.
   Every caller ratio/fraction is finite and in its schema domain; a negative
   proposed pool or failed C/N closure is typed failure.
   Turnover/retranslocation precedes external demand; the N transaction is
   `request -> authorization -> finalized_use -> validation -> commit`, with
   `0<=use<=authorization<=request` and mineral pools debited only by use.
9. Current GSI supplies only deterministic onset/offset edges. Deciduous state
   changes `dormant->onset` on an upward crossing of caller `gsi_on_threshold`
   and `active->offset` on a downward crossing of lower
   `gsi_off_threshold`; equality retains phase, crossings require the previous
   accepted signal, and re-entry is prohibited until the current timed phase
   completes. Onset transfer is `F=r_on*Stransfer`, with
   `r_on=2/t_remaining` except the final interval uses `1/dt`; every C and N
   transfer is donor-bounded. Deciduous offset computes leaf litter
   `Nlit=Cfall/CNleaf_litter`, retranslocates
   `Nret=Cfall/CNleaf-Nlit`, then debits leaf C/N once. Evergreen tissue `p`
   turns over by the bounded amount `F_p=(1-exp(-dt/lifetime_p))*S_p`.
   Onset continues until its donor transfer pools are exactly empty; the final
   shortened interval transfers the full remainder and enters `active`.
   Offset continues analogously until the accepted leaf C/N donor is empty,
   then enters `dormant`. Ordered after phenological leaf transfer, every
   fine-root C/N subpool turns over by
   `F_fr=(1-exp(-dt/froot_lifetime))*S_fr` and routes by the fine-root litter
   fractions. Every live-stem/live-coarse-root C/N subpool transfers internally
   to its paired deadwood subpool by
   `F_lw=(1-exp(-dt/livewood_turnover))*S_lw`. Finally, every remaining tissue
   subpool undergoes background mortality
   `F_m=(1-exp(-mortality_rate*dt))*S_after`; leaf/fine-root mortality routes to
   litter and all stem/coarse-root mortality routes to CWD. C and N use the
   same bounded fraction; external `DM=C/drymatter_carbon_fraction`. Each donor
   is debited once in that order. Actual
   `LAI=leaf_C*SLA`. Mortality routes exact C/N/dry material to typed litter or
   CWD proposals; `SC-BIOGEOCHEM-001` receives them.

Quadratics use the cancellation-safe smaller root. `ci` uses Brent on
`[Gamma,ca]` to `1e-6 Pa`/`1e-10` relative or gas residual `1e-8 umol m-2 s-1`,
64 evaluations. Leaf energy uses damped Newton to `1e-6 W m-2` plus `1e-10`
scale and `1e-8 K` step, at most 50 steps/20 halvings. Hydraulic
complementarity uses damped semismooth Newton with pivoted LU, residual
`1e-12 mm s-1` plus `1e-9` scale, step `1e-7 mm`, and the same limits. A trial
step is accepted only when the infinity norm strictly decreases; otherwise it
is halved. A pivot below `64*epsilon*matrix_inf_norm`, bracket failure, or limit
exhaustion is typed nonconvergence. Bounds/tolerances are
`OPENWEPP_CANONICAL_SELECTION` representation constants serialized in the
model-definition digest; no last iterate is usable.

This sequence is implementation-authoritative for boundary transactionality,
not for any missing constitutive response.

### `OPENWEPP_C3_WOODY_V2` Occupancy Topology Amendment

V2 retains every V1 equation and numerical constant except that all
heterogeneous-topology E04 state and every nonlinear downstream consumer is
resolved by occupancy. An occupancy is the exact typed pair `o=(s,t)`.

1. Validate positive, nonoverlapping `f_t` with `sum_t f_t=1`, exact unique
   occupancies, and `C_s=sum_{t in T_s} f_t>0`. Reject missing, duplicate, extra,
   or nonmember occupancy state. Do not normalize inconsistent topology.
2. `LAI_s` and `WAI_s` remain stand-ground shared stratum state. V2 canonically
   selects uniform conditional density across occupied tiles:
   `LAI_s,t=LAI_s/C_s`, `WAI_s,t=WAI_s/C_s`. Using stand-area LAI directly in a
   tile is invalid.
3. Every occupancy owns exactly one `S_liq,s,t` in
   `kg H2O m^-2 tile-ground`, its derived wet fraction, accepted local numerical
   warm starts, and last accepted transaction identity. Aggregated liquid is a
   read-only diagnostic `S_liq,s,agg=sum_t f_t*S_liq,s,t`.
4. For each tile, traverse occupancies in deterministic top-to-bottom rank.
   Begin with tile-local top rain. Execute the complete V1 E04 sequence at each
   occupancy using local `S0`, incident `P`, conditional `L/S`, and local canopy
   temperature. Free throughfall and both initial and post-condensation drainage
   proceed to the next lower occupancy in the same tile. Stemflow bypasses lower
   foliage and routes directly to that tile's ground liquid recipient. Terminal
   throughfall/drainage and rain on an empty tile route to that tile's ground.
   No release crosses tile identity and no stand aggregation precedes terminal
   routing.
5. Solve each potential tile column once, top-to-bottom, with `beta_hyd=1` and
   no owner authorization cap. At each occupancy, solve the complete local
   gas/energy/hydraulic state, finalize its potential E04 vapor amount and both
   drainage terms, route those final releases, and only then solve its
   descendant. The resulting stand-ground layer demands are immutable potential
   requests `D_W,s,t,l`.
6. Hydrology arbitrates all potential occupancy requests and competitor demands
   once against the immutable beginning snapshot. Authorizations remain fixed;
   no candidate debit occurs.
7. Rebuild every tile column from the original beginning occupancy states and
   forcing, again top-to-bottom. Each occupancy solves under its fixed
   `A_tile=A_W/f_t` caps, finalizes vapor/store/drainage, and routes those newly
   finalized releases before its descendant is solved. A descendant never uses
   potential-column release. Its use stays within fixed authorization even if
   changed upstream liquid would raise unconstrained demand. There is no
   reauthorization or outer column iteration: this final capped pass is the
   accepted column. Any infeasible cap or local nonconvergence atomically rejects
   the transaction.
8. Local canopy-liquid closure is
   `S0+P+condensation=S1+wet_evaporation+throughfall+stemflow+initial_drainage+second_drainage`.
   After internal releases cancel, each tile column and the weighted stand must
   close independently from exposed operands. A producer-supplied residual is
   not acceptance evidence.
9. Radiation remains column-local. Wet/dry area, leaf/stem/wet energy,
   canopy-air state, sun/shade FvCB--Medlyn exchange, hydraulics, condensation,
   and active store-cap re-solve are occupancy-local whenever local forcing,
   radiation, wetness, or authorization differs. PAR, wet fraction,
   temperature, conductance, or another nonlinear operand is never averaged
   before its solve. Accepted occupancy fluxes are weighted by `f_t` only after
   the local solve and then update the single shared stratum C/N state once.
10. Water request identity is
   `(tau,stratum,occupancy,layer,resource,amount_basis)`. From tile-ground local
   demand, `D_W,s,t,l=f_t*D_tile,s,t,l`. Hydrology arbitrates stand-ground
   amounts against one same-layer snapshot. For the capped local re-solve,
   `A_tile,s,t,l=A_W,s,t,l/f_t`; finalized use returns as
   `F_W,s,t,l=f_t*F_tile,s,t,l`, with
   `0<=F_W<=A_W<=D_W`. Missing/double `f_t`, identity swaps, stale transactions,
   and duplicate request identities fail before owner mutation.
11. Occupancy-local warm starts are exactly:
   `sun_leaf_temperature_k`, `shade_leaf_temperature_k`,
   `dry_stem_temperature_k`, `wet_surface_temperature_k`,
   `canopy_air_temperature_k`, `canopy_air_specific_humidity_kg_kg`,
   `sun_ci_pa`, `shade_ci_pa`, `beta_hyd`, `stem_potential_mm`,
   `sun_leaf_potential_mm`, `shade_leaf_potential_mm`, and sorted
   `root_potential_mm_by_layer[]=(layer_id,potential_mm)`. Temperatures and `ci`
   are finite and positive, humidity is finite/nonnegative, `beta_hyd` is in
   `[0,1]`, potentials are finite, and every configured root layer occurs once.
   `last_accepted_transaction_id` is exact null for an initial lane or the
   immediately preceding accepted transaction. Recursively lexicographic field
   keys, root-layer ID order, the null marker, and all vector entries define
   canonical serialization and enter the state digest; displayed prose order is
   non-normative. Hydraulic potentials use `mm H2O` only, and MPa input is a
   wrong-unit typed failure rather than an implicit conversion.
12. Warm starts affect initialization only;
   alternate valid starts converge to the same accepted state within canonical
   tolerance. A failed transaction preserves every warm-start byte, and one
   occupancy's accepted solution is never broadcast to another.
13. V2 initialization requires exactly one lane per occupancy. Every V1
   migration requires caller-supplied complete V2 warm starts with null
   transaction identity; no V1 warm start is copied or broadcast. Given those
   lanes, V1 exact-zero liquid expands to zero stores, and a V1 store with one
   occupied tile maps as `S_V2=S_V1/C_s`. A nonzero V1 store over multiple tiles
   returns an exhaustive unresolved-liquid-lane report. Missing warm starts,
   uniform distribution, or parser defaults fail.
14. Shared stratum C/N advances once after final capped columns finish:
   `GPP_s=sum_t(f_t*GPP_s,t)`,
   `R_leaf,s=sum_t(f_t*R_leaf,s,t)`, and
   `T_s=sum_t sum_l F_W,s,t,l`. Shared tissue maintenance, turnover, allocation,
   and growth respiration execute once on stand-ground pools. Mineral-N demand
   is stratum/layer/species-level after aggregation, preserves NH4/NO3 and layer
   identity, and is never duplicated per occupancy. Preaggregating nonlinear
   physiology operands is prohibited.
15. V2 model-definition digest and contract-section hashes are exact. V1
    definition bytes remain immutable and historical; V1 state cannot execute
    under V2 identity except through the explicit migration operation.

### `OPENWEPP_C3_WOODY_V3` Potential-Pass Authority Amendment

V3 normatively imports the exact V2 definition digest and all unchanged V1/V2
equations, topology, routing, transaction, ownership, and numerical rules. The
following clauses exclusively supersede the conflicting V1/V2 preparation,
state, respiration, and potential-pass text. No other V1/V2 byte is rewritten.

1. **Mixed leaf/stem radiation preparation.** For occupancy `o=(s,t)`, set
   `L=LAI_s,t`, `S=WAI_s,t`, and `P=L+S`. Require finite `L,S>=0`, finite
   `0<Omega<=1`, and the existing leaf-angle and optical domains. For `P>0`,
   `w_leaf=L/P`, `w_stem=S/P`, and, independently for
   `lambda in {VIS,NIR}`,
   `rho_lambda=w_leaf*rho_leaf,lambda+w_stem*rho_stem,lambda` and
   `tau_lambda=w_leaf*tau_leaf,lambda+w_stem*tau_stem,lambda`. For `P=0`,
   transport and every absorbed owner term are exact zero. The complete tile
   column is one two-stream boundary-value problem over actual conditional
   plant-area coordinates; lower-canopy and ground upward diffuse flux traverse
   every overlying layer. Solving layers independently with a zero lower
   boundary, directly summing lower reflection at the top, or averaging
   different strata is prohibited.
2. **Clumping and actual sunlit leaf area.** The V3 canonical selection applies
   clumping exactly once to direct extinction, `K_eff=Omega*K`, while the
   transport coordinate remains actual `P`. Every direct exponential and the
   direct source in the admitted E01 solution uses `K_eff`; the diffuse
   coefficients otherwise retain the admitted two-stream construction. For
   positive direct incident radiation and `L,P>0`,
   `L_sun=w_leaf*[1-exp(-K_eff*P)]/K_eff` and `L_shade=L-L_sun`.
   The removable `K_eff=0` limit is exactly `L_sun=L`. For zero direct
   radiation, `L_sun=0` and `L_shade=L`; for `L=0` both are zero. A two-stream
   sunlit *plant* area is never passed to photosynthesis as leaf area.
3. **Physical absorption ownership.** For each band and incident component
   independently, set
   `a_leaf=w_leaf*(1-rho_leaf-tau_leaf)` and
   `a_stem=w_stem*(1-rho_stem-tau_stem)`. When
   `a_sum=a_leaf+a_stem>0`, define
   `f_abs_leaf=a_leaf/a_sum` and `f_abs_stem=a_stem/a_sum`; when `a_sum=0`,
   require exact zero plant absorption and return exact-zero owner terms.
   Partition the analytically solved plant absorption as
   `A_leaf,sun=f_abs_leaf*A_plant,sun`,
   `A_leaf,shade=f_abs_leaf*A_plant,shade`, and
   `A_stem=f_abs_stem*(A_plant,sun+A_plant,shade)`. Direct/diffuse and VIS/NIR
   identities never alias. Only leaf-owned VIS absorption enters FvCB PAR;
   stem absorption enters the stem energy owner. The owner terms must sum
   exactly to solved plant absorption within the radiation closure tolerance.
4. **Neutral canopy-surface wind.** On the admitted neutral domain, require
   finite `u_ref>0`, `z0m>0`, and `z_ref>displacement+z0m`. Derive
   `u_star=kappa*u_ref/ln((z_ref-displacement)/z0m)` and require finite
   `u_star>0`. V3 defines three distinct semantic operands by authority:
   `u_leaf=u_wet=u_stem=u_star`. Their conductances remain distinct through
   their positive characteristic dimensions:
   `gb_leaf=Cv*sqrt(u_leaf/d_leaf)`,
   `gb_wet=Cv*sqrt(u_wet/d_wet)`, and
   `gb_stem=Cv*sqrt(u_stem/d_stem)`. Reference wind used directly as surface
   wind, a hidden minimum wind, independent undocumented wet/stem wind, or
   heat/vapor roughness in the momentum logarithm is invalid.
5. **Height-based stem hydraulics.** V3 selects
   `z2_m=height_m` and `Delta_psi_stem_mm=1000*height_m`; require finite
   `height_m>0`. Thus
   `q2=[k2_max/height_m]*v(psi_root)*SAI*
   (psi_root-psi_stem-1000*height_m)`. Crown base, half height, omitted gravity,
   reversed gravity, metres used as millimetres, and any stem-to-leaf gravity
   term are prohibited. The existing explicit layer soil-to-root path length
   and layer gravitational head remain unchanged.
6. **One common root-node state.** The V3 hydraulic unknown vector is exactly
   `[psi_sunleaf,psi_shadeleaf,psi_stem,psi_root]`. All accessible soil layers
   connect in parallel to the single `psi_root`; layer identity remains in
   configuration, soil forcing, `q3_i`, request, authorization, and finalized
   use, but not in the root warm start. V3 replaces
   `root_potential_mm_by_layer[]` with one finite
   `root_node_potential_mm`. The complete occupancy state is exactly the
   lexicographically serialized fields `beta_hyd`,
   `canopy_air_specific_humidity_kg_kg`, `canopy_air_temperature_k`,
   `canopy_liquid_kg_h2o_m2_tile_ground`, `dry_stem_temperature_k`,
   `last_accepted_transaction_id`, `root_node_potential_mm`, `shade_ci_pa`,
   `shade_leaf_potential_mm`, `shade_leaf_temperature_k`, `stem_potential_mm`,
   `sun_ci_pa`, `sun_leaf_potential_mm`, `sun_leaf_temperature_k`, and
   `wet_surface_temperature_k`. Every field enters the state digest.
7. **Exact V2-to-V3 migration.** Because V2 never became an executable accepted
   public state machine, migration of a V2 occupancy succeeds only when every
   entry in its ordered root-potential vector is present, finite, and bitwise
   identical. The common bits become `root_node_potential_mm`. An empty vector
   or any unequal bits returns an exhaustive unresolved field for that
   occupancy with reason `ambiguous_v2_layer_root_warm_starts`. Averaging,
   first-layer selection, root-fraction weighting, broadcast, or synthesis is
   forbidden. All other V2-to-V3 fields map by exact identity after the V3
   model/configuration identity is supplied and validated.
8. **Internal maximum-demand evaluation.** `beta_sun=beta_shade=1` is used only inside each
   outer residual evaluation to solve sun and shade FvCB--Medlyn--energy and
   obtain `E_sun,max`, `E_shade,max`, `gs_sun,max`, and `gs_shade,max`. These
   values are diagnostic operands, not an accepted state, request, finalized
   use, or publishable flux. A `beta=1` column result must never be sent to the
   hydrology owner.
9. **Accepted uncapped potential pass.** "Potential" means uncapped by the
   hydrology owner, not hydraulically unstressed. From the original beginning
   occupancy state, solve one common residual system for sun/shade `ci`, leaf,
   wet, stem, and canopy-air energy states, the four hydraulic potentials, and
   accepted `beta_sun,beta_shade in [0,1]`. The exact hydraulic subsystem has
   six unknowns
   `[psi_sunleaf,psi_shadeleaf,psi_stem,psi_root,beta_sun,beta_shade]`
   and six independent residuals. At every iterate, re-solve each class's gas
   exchange and energy with its own `beta_c`; a common post-hoc scalar
   multiplication is prohibited. Acceptance requires separately
   `E_sun,gas-energy(beta_sun)=E_sun,max*v(psi_sunleaf)=q1a`,
   `E_shade,gas-energy(beta_shade)=E_shade,max*v(psi_shadeleaf)=q1b`,
   `q1a+q1b=q2`, and `q2=sum_i q3_i` under the admitted scaled residual
   tolerances. These are six equations: two class loss-function equalities, two
   class stem-to-leaf equalities, and two downstream continuity equalities.
   Aggregate equality cannot mask a failed class equality. The persisted
   15-field state does not add a seventh unknown: after acceptance, if
   `E_sun,max+E_shade,max>0`, set
   `beta_hyd=(E_sun,max*beta_sun+E_shade,max*beta_shade)/
   (E_sun,max+E_shade,max)`; for exact zero maximum demand set `beta_hyd=1` and
   require every accepted class demand and hydraulic flux to be exact zero.
   The persisted aggregate is a diagnostic and the next solve's initial value
   for both class factors; it is never reapplied as a common constitutive
   stress. This class-resolved solve and aggregate persistence rule are an
   `OPENWEPP_CANONICAL_SELECTION` over the CLM distinct sun/shade water-stress
   factors. The
   accepted potential request is `D_W,o,i=f_t*q3_i*dt` in stand-ground basis,
   without authorization operands. It is valid and expected that the accepted
   uncapped state has either class factor and aggregate `beta_hyd<1` when
   soil/plant hydraulics constrain demand.
10. **Capped pass relationship.** The final authorization pass solves the same
    coupled residual system from original beginning state with fixed layer caps
    added. It neither reuses the internal maximum state nor sequentially clamps
    the accepted potential flux. All V2 two-pass column routing, identity,
    weighting, no-reauthorization, and rollback rules remain binding.
11. **One leaf-respiration identity.** For accepted leaf N area
    `N_leaf_area_kg`, form source units once as
    `N_leaf_area_g=1000*N_leaf_area_kg` and `T10_C=T10_K-273.15`. The Atkin
    source rate is
    `Rd25=i_atkin+0.2061*N_leaf_area_g-0.0402*T10_C`, where `Rd25` and
    `i_atkin` are in `umol CO2 m^-2 leaf s^-1`, the nitrogen coefficient is
    `0.2061 umol CO2 g^-1 N s^-1`, and the temperature coefficient is
    `0.0402 umol CO2 m^-2 leaf s^-1 degC^-1`. For positive leaf area, a nonfinite or
    nonpositive source rate is typed invalid state/configuration; it is never
    clamped. Zero leaf area takes the exact zero-capacity/zero-Rd branch without
    division. The Atkin result is already `Rd25`; no carbon/day conversion is
    applied at this point.
12. **Rd temperature response and debit.** For class temperature `T`, use the
    stable log-domain normalized response
    `ln(f_Rd)=Ha_Rd*(T-298.15)/(R*T*298.15)
    +ln1p(exp((298.15*S_Rd-Hd_Rd)/(R*298.15)))
    -ln1p(exp((T*S_Rd-Hd_Rd)/(R*T)))`, with
    `Ha_Rd=46390 J mol^-1`, `Hd_Rd=150650 J mol^-1`, and
    `S_Rd=490 J mol^-1 K^-1`; `Rd(T)=Rd25*exp(ln(f_Rd))`.
    Exponent clamps and last-finite substitutes are prohibited. The identical
    class-resolved `Rd(T)` is subtracted once in `An=Ag-Rd` and, after exact
    class-area/topology/time/carbon conversion, debited once in the leaf
    maintenance carbon ledger. No independent second leaf-maintenance
    expression is evaluated. `rd_leaf_n_rate` is not a V3 consumed field.
13. **Typed numerical failure payload.** Every failed sun `ci`, shade `ci`,
    canopy-energy, hydraulic-system, or outer gas--energy--hydraulic solve
    returns a typed `NumericalFailureDiagnostics` containing model-definition
    digest, transaction, occupancy, `potential|capped` pass, solve identity,
    completed iterations, every normalized residual available at failure,
    optional step norm, backtracking count, active bounds, active water-cap
    layer IDs, optional bracket, optional pivot magnitude, and optional matrix
    infinity norm. All numeric payload values must be finite when present and
    serialize deterministically. Domain, bracket, singular-pivot, and iteration
    failures have deterministic precedence in that order after identity/schema
    validation. An opaque `nonconvergence` string, a last iterate, or partial
    candidate is prohibited.
14. **V3 identity and exact fixtures.** V3 canonical JSON imports the immutable
    V2 digest, declares every override above, binds exact contract-section and
    fixture digests, and uses recursively lexicographic compact JSON plus one
    trailing newline. Independent Python fixtures must expose inputs, operands,
    expected outputs, closures, diagnostics, and numerically distinct named
    poison alternatives for radiation, potential E11--E15, migration, and
    respiration. Expected values are never generated by Rust.

## Branch and Guard Table

| Condition | Required disposition | Failure |
|---|---|---|
| empty stand with zero vegetation stores | valid degenerate; zero vegetation demands/transfers | none |
| invalid/ambiguous topology, units, digest, or parameter class | reject before Stage A | `VEG-E-001/002/003` |
| missing constitutive authority for a requested process | reject; do not substitute proxy physics | `VEG-E-060` |
| agricultural `Kcb`/LAI PMET partition requested for native forest | reject before component flux construction | `VEG-E-061` |
| canopy-demand loss is reassigned to forest-floor evaporation | reject the coupled candidates | `VEG-E-061` |
| stale/mismatched `tau`, state, area, or layer identity | reject before mutation | `VEG-E-010` |
| missing/duplicate shared lineage | reject | `VEG-E-011` |
| Stage A canopy/radiation proposal fails closure | reject | `VEG-E-012` |
| allocation is negative, above demand, or above admissible liquid | reject candidate hydrology state | `VEG-E-020` |
| missing/invalid limitation reason or allocation policy | reject | `VEG-E-021` |
| vegetation attempts soil/frozen-store mutation | reject | `VEG-E-022` |
| `T_s != sum_l F_W,s,l` | reject both candidate states | `VEG-E-030` |
| missing/mismatched `h_v` or duplicate latent debit | reject | `VEG-E-031` |
| water/energy/carbon/nitrogen/material closure fails | reject atomically | `VEG-E-032` |
| canopy-snow constitutive execution requested under versions 2-7 | reject; boundary concept only | `VEG-E-040` |
| iterative feedback requested without successor authority | reject without partial publication | `VEG-E-041` |
| compatibility adapter invoked before Stage C/receipt closure | reject | `VEG-E-050` |
| unsupported lifeform/process or canopy snow | reject before solve | `VEG-E-062` |
| FvCB/Medlyn/energy/hydraulic domain violation | reject without canonicalization | `VEG-E-063` |
| any nested solve exceeds its admitted limit | discard all candidates | `VEG-E-064` |
| missing C/N pool, parameter, initial state, or BGC receipt | reject; no synthesis/source | `VEG-E-065` |
| invalid tile fractions, coverage, membership, or duplicate occupancy | reject before radiation/E04 | `VEG-E-070` |
| missing, duplicate, extra, or stale V2 occupancy state lane | reject before calculation | `VEG-E-071` |
| stand-area plant area used as tile-local area, or `f_t` omitted/applied twice | reject candidate | `VEG-E-072` |
| liquid release crosses tile or stemflow enters lower foliage | reject routing/ground receipt | `VEG-E-073` |
| nonlinear occupancy operand aggregated before local solve | reject candidate | `VEG-E-074` |
| occupancy/layer/transaction/resource/amount-basis request identity mismatch or duplicate | reject before arbitration | `VEG-E-075` |
| nonzero multi-tile V1 store requested for automatic migration | return exhaustive unresolved occupancy lanes | `VEG-E-076` |
| invalid V3 mixed optics, clumping, owner partition, band, or incident-component identity | reject before column radiation | `VEG-E-080` |
| reference wind used as canopy-surface wind, invalid logarithmic geometry, or nonpositive `u_star` | reject before conductance construction; no floor | `VEG-E-081` |
| nonpositive height, wrong stem path/gravity identity, or per-layer root warm start presented as V3 | reject before hydraulic solve | `VEG-E-082` |
| V2 root warm starts are empty or not bitwise identical during V3 migration | return every unresolved occupancy root-node field | `VEG-E-083` |
| `beta_hyd=1` maximum evaluation published/requested, authorization used in potential pass, or class/total coupling equality fails | reject all candidates | `VEG-E-084` |
| positive leaf area with nonfinite/nonpositive Atkin rate, invalid Rd temperature, or nonfinite peaked response | reject without clamp or fallback | `VEG-E-085` |
| numerical failure lacks its typed deterministic diagnostic payload | discard all candidates and reject implementation conformance | `VEG-E-086` |
| V1, V2, or V3 model digest/schema/execution identity mixed | reject before calculation | `VEG-E-077` |
| V4 shared field/tissue set is incomplete, duplicated, unknown, nonfinite, or retains either legacy offset-flux field | reject before calculation | `VEG-E-087` |
| V4 LAI/cache uses storage or transfer leaf C, another tissue, or a producer-supplied independent area | reject shared candidate identity | `VEG-E-088` |
| V3-to-V4 source cache is inconsistent or migration synthesizes/remaps state | reject migration without partial V4 output | `VEG-E-089` |
| displayed leaf N is positive at zero displayed leaf C, or storage/transfer N enters leaf physiology | reject before gas exchange | `VEG-E-090` |

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-VEGETATION-001` | Configuration, parameter sets, initial state, and evolving state are distinct versioned objects; missing physiology never defaults. | `REF-VEGETATION-001`, `REF-VEGETATION-009`, `REF-VEGETATION-011` | `[INFERENCE][Static]` | test/governance | hard `HOLD` |
| `INV-VEGETATION-002` | Exact tile topology reconstructs every stratum cover and overlap without state perturbation or implicit independence. | `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | future runtime/test | `VEG-E-002` |
| `INV-VEGETATION-003` | Top-to-bottom order is deterministic and same-rank cover closes at or below unity. | `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | future runtime/test | `VEG-E-002` |
| `INV-VEGETATION-004` | Root participation is explicit by soil layer and separately authoritative; depth alone is not a layer profile. | `REF-VEGETATION-005`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | governance/runtime | `VEG-E-060` |
| `INV-VEGETATION-010` | Resource order is request, maximum authorization, finalized use, owner validation, then atomic commit. | `REF-VEGETATION-005`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | orchestrator test | `VEG-E-010` |
| `INV-VEGETATION-011` | Vegetation never mutates soil-layer liquid/frozen state; hydrology alone validates finalized use and forms the soil candidate. | `REF-VEGETATION-005`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | owner guard | `VEG-E-022` |
| `INV-VEGETATION-012` | For each water or N request, `0<=finalized<=authorization<=request`; aggregate authorization is bounded by same-snapshot availability and every request carries one reason code. | `REF-VEGETATION-005`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | owner/test | `VEG-E-020/021` |
| `INV-VEGETATION-013` | Actual transpiration exactly equals finalized layer withdrawals; unused authorization is never debited. | `REF-VEGETATION-005`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | dual reconstruction | `VEG-E-030` |
| `INV-VEGETATION-014` | Actual transpiration and latent energy share one transaction/stratum/lineage and one `h_v` identity. | `REF-VEGETATION-007`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | energy join | `VEG-E-031` |
| `INV-VEGETATION-015` | Failed or non-converged transactions publish and mutate nothing. | `REF-VEGETATION-007`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | atomic commit | `VEG-E-032/041` |
| `INV-VEGETATION-020` | Canopy liquid start plus interval-integrated incident water equals end storage plus interval-integrated actual evaporation and named releases. | `REF-VEGETATION-002`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[DIRECT][Static] + [INFERENCE][Static]` | dual reconstruction | `VEG-E-012/032` |
| `INV-VEGETATION-021` | Canopy, ground, litter, snow, soil, ponded-water, and atmospheric radiation/latent terms remain distinct. | `REF-VEGETATION-007`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | alias/poison test | `VEG-E-011/032` |
| `INV-VEGETATION-022` | Vegetation owns intercepted canopy snow; snow/frost owns ground snow; versions 1-7 admit no canopy-snow constitutive law. | `REF-VEGETATION-008`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | governance | `VEG-E-040` |
| `INV-VEGETATION-030` | Live/standing-dead plant pools remain vegetation-owned until an accepted exact-once material/element transfer. | `REF-VEGETATION-006`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | receipt test | `VEG-E-032` |
| `INV-VEGETATION-031` | Vegetation and residue/biogeochemistry independently reconstruct identical dry-matter, carbon, and nitrogen transfers. | `REF-VEGETATION-006`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | dual reconstruction | `VEG-E-032` |
| `INV-VEGETATION-040` | Every compatibility field has an explicit reduction, area basis, unit conversion, missing-state rule, and contributing-strata receipt. | `REF-VEGETATION-004`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | adapter test | `VEG-E-050` |
| `INV-VEGETATION-041` | The adapter is read-only, never feeds native state, and cannot support cutover without real downstream consumption. | `REF-VEGETATION-004`, `REF-VEGETATION-011` | `[DIRECT][Static]` | consumer gate | hard `HOLD` |
| `INV-VEGETATION-050` | RHESSys behavior, source-reported citations, and comparator agreement cannot authorize equations, constants, bounds, or defaults. | `REF-VEGETATION-009`, `REF-VEGETATION-011` | `[DIRECT][Static]` | firewall/review | hard `HOLD` |
| `INV-VEGETATION-051` | No production implementation is promotable while a requested process is `AUTHORITY_MISSING`. | `REF-VEGETATION-001` through `REF-VEGETATION-014` | `[INFERENCE][Static]` | gap gate | `NON_PROMOTABLE` |
| `INV-VEGETATION-052` | A compatible vegetation definition preserves every exact input key/value and provenance identity, but selected runtime parameters use a versioned typed schema with explicit aliases; absent, duplicate, non-finite, invalid, or unsupported-sentinel values never receive hidden defaults. | `REF-VEGETATION-012`, `REF-VEGETATION-013`, `REF-VEGETATION-014` | `[DIRECT][Static] + [INFERENCE][Static]` | schema/runtime/test | `VEG-E-003/060` |
| `INV-VEGETATION-053` | Definition acquisition accepts caller-supplied local bytes only when repository, immutable commit, repository-relative path, and SHA-256 all match; mutable references, network schemes, and runtime fallback acquisition are rejected before parsing. | `REF-VEGETATION-013`, `REF-VEGETATION-014`, `REF-VEGETATION-015` | `[DIRECT][Static] + [INFERENCE][Static]` | acquisition/runtime/test | `VEG-E-003` |
| `INV-VEGETATION-054` | The immutable raw definition and resolved typed parameter set are distinct objects. Raw bytes and exact lexical key/value records remain reconstructible; resolution cannot rewrite the evidence object. | `REF-VEGETATION-013`, `REF-VEGETATION-014`, `REF-VEGETATION-015` | `[DIRECT][Static] + [INFERENCE][Static]` | schema/runtime/test | `VEG-E-003` |
| `INV-VEGETATION-055` | Every consumed field is declared by a versioned schema entry with canonical symbol, source key, explicit aliases, type, units, cadence, area/scale basis, parameter class, finite domain, missing/sentinel policy, authority, ecosystem domain, and prohibited extrapolations. Schema admission does not admit an empirical value. | `REF-VEGETATION-014`, `REF-VEGETATION-015`, `REF-VEGETATION-016` | `[DIRECT][Static] + [INFERENCE][Static]` | schema/runtime/test | `VEG-E-003/060` |
| `INV-VEGETATION-056` | Initial state is a distinct, complete, versioned site object tied to date, area, topology, profile identity, units, and every required pool/geometry field; it is never intrinsic to a species-profile label. | `REF-VEGETATION-015`, `REF-VEGETATION-017` | `[DIRECT][Static] + [INFERENCE][Static]` | initialization/runtime/test | `VEG-E-060` |
| `INV-VEGETATION-057` | Site-specific parameter values are caller-supplied `external_configuration`. A0 authority governs field meaning, units, cadence, basis, mathematical domain, process role, required presence, and guards; accepting a finite in-domain value makes no calibration, validation, ecosystem applicability, or transferability claim. | `REF-VEGETATION-004`, `REF-VEGETATION-010`, `REF-VEGETATION-015` | `[INFERENCE][Static]` | schema/runtime/test | `VEG-E-003/060` |
| `INV-VEGETATION-058` | A compatible initial state may be caller-supplied `initial_state` without being an observation or openWEPP synthesis. It must be complete, finite, domain-valid, dated, area/topology/profile-bound, and versioned; empirical provenance is additionally required only for an observation, calibration, validation, or transferability claim. | `REF-VEGETATION-001`, `REF-VEGETATION-010`, `REF-VEGETATION-017` | `[INFERENCE][Static]` | initialization/runtime/test | `VEG-E-060` |
| `INV-VEGETATION-059` | The future native-forest path represents canopy transpiration, wet-canopy evaporation, and forest-floor evaporation as separately owned, independently reconstructible component fluxes. The Agricultural `Kcb`/LAI PMET partition is not an admissible native-forest implementation target. | `REF-VEGETATION-003`, `REF-VEGETATION-018`, `REF-VEGETATION-019`, `REF-VEGETATION-020` | `[DIRECT][Static] + [INFERENCE][Static]` | governance/runtime/test | `VEG-E-061` |
| `INV-VEGETATION-060` | A reduction in canopy area, conductance, or energy must not automatically reassign lost canopy demand to forest-floor evaporation. Each component responds only through its own admitted operands, state, resistances, and energy/water limits. | `REF-VEGETATION-007`, `REF-VEGETATION-010`, `REF-VEGETATION-018`, `REF-VEGETATION-020` | `[DIRECT][Static] + [INFERENCE][Static]` | independent reconstruction/poison test | `VEG-E-061` |
| `INV-VEGETATION-061` | Contract demonstrations use deliberately distinct `ASSUMED_FOR_EXECUTION` fixtures to prove schema rejection, stratum separation, component independence, layer-resolved root requests, limiting behavior, and closure. Such fixtures make no site-suitability claim and cannot be distributed as recommended defaults. | `REF-VEGETATION-010`, `REF-VEGETATION-011`, `REF-VEGETATION-021` | `[INFERENCE][Static]` | test/governance | hard `HOLD` on overclaim |
| `INV-VEGETATION-062` | One indivisible model version binds two-stream direct/diffuse radiation, sunlit/shaded FvCB--Medlyn exchange, explicit leaf energy, interval-equilibrium hydraulics, liquid interception and persistent C/N dynamics. | `REF-VEGETATION-024`--`029` | `[DIRECT+INFERENCE][Static]` | schema/test | `VEG-E-062` |
| `INV-VEGETATION-063` | Radiation conserves each band/direction through explicit top-to-bottom strata and never averages mixed-stratum parameters. | `REF-VEGETATION-025/027` | `[DIRECT+INFERENCE][Static]` | reconstruction | `VEG-E-032` |
| `INV-VEGETATION-064` | FvCB assimilation causally enters Medlyn conductance, transpiration, carbon gain, allocation and future LAI. | `REF-VEGETATION-024/026/027` | `[DIRECT][Static]` | coupled vector | `VEG-E-063` |
| `INV-VEGETATION-065` | Liquid interception preserves finite store, throughfall, stemflow, drainage, wet fraction/carry and evaporation closure. | `REF-VEGETATION-002/019/027` | `[DIRECT+INFERENCE][Static]` | reconstruction | `VEG-E-032` |
| `INV-VEGETATION-066` | Canopy transpiration, wet-canopy evaporation and forest-floor evaporation have independent resistance, energy, phase, area and interval lineage. | `REF-VEGETATION-003/007/027` | `[DIRECT+INFERENCE][Static]` | poison vector | `VEG-E-061` |
| `INV-VEGETATION-067` | Hydraulic potentials are interval-equilibrium diagnostics; requests exclude frozen/dry/nonrooted layers; finalized use is bounded by authorization and final transpiration equals finalized uptake. | `REF-VEGETATION-021/027` | `[DIRECT+INFERENCE][Static]` | coupled vector | `VEG-E-063` |
| `INV-VEGETATION-068` | All required C/N pools persist, and GPP, respiration, allocation, storage, retranslocation, turnover and mortality conserve C and N. | `REF-VEGETATION-027/028/029` | `[DIRECT+INFERENCE][Static]` | elemental reconstruction | `VEG-E-032` |
| `INV-VEGETATION-069` | Leaf C and caller SLA own actual LAI; GSI owns timing/activity only. | `REF-VEGETATION-004/027/028` | `[DIRECT+INFERENCE][Static]` | state vector | `VEG-E-065` |
| `INV-VEGETATION-070` | Mineral-N demand/receipt and litter/CWD C/N/dry material cross only the atomic `SC-BIOGEOCHEM-001` transaction. | `REF-VEGETATION-029` | `[DIRECT][Static]` | dual reconstruction | `VEG-E-032/065` |
| `INV-VEGETATION-071` | Every site parameter and complete initial state is caller supplied; model constants are versioned/digest bound; no hidden default exists. | `REF-VEGETATION-010/027/028` | `[INFERENCE][Static]` | exhaustive schema | `VEG-E-003/065` |
| `INV-VEGETATION-072` | `ci`, energy and hydraulic solves use finite, scale-aware, safeguarded algorithms; nonconvergence rolls back byte-identically. | `REF-VEGETATION-010/027` | `[DIRECT+INFERENCE][Static]` | numeric/rollback vector | `VEG-E-064` |
| `INV-VEGETATION-073` | V2 has exactly one persistent liquid/numerical lane per valid occupancy `(s,t)`; the stand aggregate is derived and never independently mutable. | `REF-VEGETATION-010/027` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | schema/runtime/test | `VEG-E-070/071/077` |
| `INV-VEGETATION-074` | Conditional occupancy plant area is exactly `LAI_s/C_s` and `WAI_s/C_s`; local E04 and every nonlinear wet-energy/physiology/hydraulic consumer execute before `f_t` weighting. | `REF-VEGETATION-025/027` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | area/poison reconstruction | `VEG-E-072/074` |
| `INV-VEGETATION-075` | Throughfall and both drainage terms remain in the same tile and enter the next lower occupancy; stemflow bypasses foliage to same-tile ground; no lateral mixing or preterminal aggregation occurs. | `REF-VEGETATION-010/027` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | column-routing reconstruction | `VEG-E-073` |
| `INV-VEGETATION-076` | Every occupancy and tile column closes canopy liquid locally, and the stand closes only as the weighted sum of independently exposed local operands. | `REF-VEGETATION-010/027` | `[DIRECT+INFERENCE][Static]` | independent local/stand reconstruction | `VEG-E-032/073` |
| `INV-VEGETATION-077` | V2 water identity preserves transaction, stratum, occupancy, layer, resource, and amount basis; tile demand/authorization/final use cross the stand boundary with exactly one `f_t` conversion each way. | `REF-VEGETATION-005/010/030` plus `OPENWEPP_CANONICAL_SELECTION` | `[INFERENCE][Static]` | hydrology/vegetation dual reconstruction | `VEG-E-072/075` |
| `INV-VEGETATION-078` | Occupancy warm starts are deterministic numerical state only; alternative valid starts converge equivalently, and failure rolls back every lane byte-identically. | `REF-VEGETATION-010/027` | `[DIRECT+INFERENCE][Static]` | convergence/rollback vectors | `VEG-E-064/071` |
| `INV-VEGETATION-079` | V1 zero and single-tile stores have only the specified unique migrations; nonzero multi-tile stores require caller-supplied V2 lanes and never receive a silent distribution. | `REF-VEGETATION-010` plus `OPENWEPP_CANONICAL_SELECTION` | `[INFERENCE][Static]` | migration/schema test | `VEG-E-076/077` |
| `INV-VEGETATION-080` | V3 radiation transports over actual `L+S`, applies clumping exactly once through `K_eff`, preserves whole-column upward diffuse coupling, and partitions every band/component absorption to leaf sun, leaf shade, and stem by area-times-absorptivity without admitting stem PAR. | `REF-VEGETATION-025/027/031` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | independent reconstruction/poison test | `VEG-E-080` |
| `INV-VEGETATION-081` | V3 derives positive neutral `u_star` from reference wind and momentum roughness; leaf, wet, and stem wind are distinct semantic operands equal to `u_star` and retain distinct characteristic dimensions. | `REF-VEGETATION-027/032` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | domain/conductance vector | `VEG-E-081` |
| `INV-VEGETATION-082` | V3 root-to-stem path and gravity are exactly vegetation height and `1000*height` mm H2O, and all layer fluxes connect to one persistent common root node. | `REF-VEGETATION-027/033` | `[DIRECT][Static]` | hydraulic/schema vector | `VEG-E-082` |
| `INV-VEGETATION-083` | V2-to-V3 migration constructs a common root warm start only from a nonempty bitwise-identical V2 vector; ambiguous lanes are exhaustively unresolved and never averaged, selected, weighted, or broadcast. | `REF-VEGETATION-010/033` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | migration/digest poison test | `VEG-E-083` |
| `INV-VEGETATION-084` | Class `beta=1` exists only inside maximum-demand evaluation; the accepted Stage-A potential pass is owner-uncapped but jointly solves distinct accepted sun/shade beta factors, gas exchange, energy, and four-node hydraulics with both class loss-function equalities, both class flux equalities, and total continuity. Persisted scalar `beta_hyd` is only the exact Emax-weighted aggregate/warm start. | `REF-VEGETATION-026/027/033` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | independent coupled vector | `VEG-E-084` |
| `INV-VEGETATION-085` | One Atkin leaf-N/T10 source supplies `Rd25`; its Rd-specific peaked response is subtracted once from gross assimilation and the identical amount is debited once in the carbon ledger, with no nonpositive clamp or second leaf-maintenance formula. | `REF-VEGETATION-027/034` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | respiration/ledger poison test | `VEG-E-085` |
| `INV-VEGETATION-086` | Every numerical failure carries deterministic typed solve/pass/identity, residual, iteration, step/backtracking/bound, cap, bracket, pivot, and matrix evidence applicable at failure; no last iterate or opaque fallback is usable. | `REF-VEGETATION-010/027` plus `OPENWEPP_CANONICAL_SELECTION` | `[DIRECT+INFERENCE][Static]` | diagnostic schema/error-precedence test | `VEG-E-086` |

### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-VEGETATION-001` | configuration/parameter classification assertions | test | blocked promotion | focused contract test |
| `INV-VEGETATION-002` | future topology validator | runtime | `VEG-E-002`; currently `HOLD` | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-003` | future order/cover validator | runtime | `VEG-E-002`; currently `HOLD` | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-004` | root-profile authority gate | governance | `VEG-E-060` | `GAP-VEGETATION-002` |
| `INV-VEGETATION-010` | stage-order contract assertions | test | blocked promotion | focused test + coupling artifact |
| `INV-VEGETATION-011` | owner/write-set guard | governance | `VEG-E-022` | focused test + adjacent contracts |
| `INV-VEGETATION-012` | proportional per-layer allocation validator | runtime/test | `VEG-E-020/021`; implementation missing | coupled reference calculator |
| `INV-VEGETATION-013` | independent hydrology and vegetation reconstruction | test | `VEG-E-030` | future implementation package |
| `INV-VEGETATION-014` | latent mass/energy lineage join | test | `VEG-E-031` | future LSE implementation package |
| `INV-VEGETATION-015` | candidate-state atomicity/rollback test | test | `VEG-E-032/041` | future implementation package |
| `INV-VEGETATION-020` | canopy-water operand reconstruction | test | `VEG-E-012/032` | future implementation package |
| `INV-VEGETATION-021` | all-distinct operand poison vectors | test | `VEG-E-011/032` | operand-lineage artifact |
| `INV-VEGETATION-022` | canopy-snow ownership/gap assertion | governance | `VEG-E-040` | canopy-snow disposition |
| `INV-VEGETATION-030` | transfer custody assertion | test | `VEG-E-032` | focused test + residue amendment |
| `INV-VEGETATION-031` | independent donor/receiver reconstruction | test | `VEG-E-032` | future implementation package |
| `INV-VEGETATION-040` | reduction-receipt assertion | test | `VEG-E-050` | future adapter package |
| `INV-VEGETATION-041` | real-consumer and no-feedback gate | governance | blocked cutover | future cutover package |
| `INV-VEGETATION-050` | digest-bound firewall review | governance | blocked promotion | compliance review |
| `INV-VEGETATION-051` | gap-label assertion | governance | `NON_PROMOTABLE` | focused test + gap register |
| `INV-VEGETATION-052` | strict definition/schema/alias validator | runtime/test | `VEG-E-003/060`; implementation missing | exhaustive v5 schema + focused test |
| `INV-VEGETATION-053` | local-byte identity validator | runtime/test | `VEG-E-003`; implementation missing | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-054` | raw/resolved object separation and round-trip vector | runtime/test | `VEG-E-003`; implementation missing | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-055` | schema-manifest completeness validator | runtime/test | `VEG-E-003/060`; caller values required | `GAP-VEGETATION-011/012/013` |
| `INV-VEGETATION-056` | complete dated-state identity validator | runtime/test | `VEG-E-060`; implementation missing | `GAP-VEGETATION-018/022` |
| `INV-VEGETATION-057` | site-value classification and schema-domain validator | runtime/test | `VEG-E-003/060`; implementation missing | `GAP-VEGETATION-001/012/013` |
| `INV-VEGETATION-058` | caller-state completeness/domain validator | runtime/test | `VEG-E-060`; implementation missing | `GAP-VEGETATION-018/022` |
| `INV-VEGETATION-059` | native-forest component ledger and prohibited-path guard | governance/runtime/test | `VEG-E-061`; implementation missing | `GAP-VEGETATION-004/023` |
| `INV-VEGETATION-060` | independent component reconstruction and canopy-loss poison vector | test | `VEG-E-061`; implementation missing | `GAP-VEGETATION-004/023` |
| `INV-VEGETATION-061` | fixture metadata/claim guard and layer-response vectors | test/governance | blocked promotion on overclaim | future implementation package |
| `INV-VEGETATION-062` | model-version member/schema guard | runtime/test | `VEG-E-062` | coupled package oracle |
| `INV-VEGETATION-063` | band/direction/stratum reconstruction | test | `VEG-E-032` | coupled package oracle |
| `INV-VEGETATION-064` | coupled gas-exchange causal vectors | test | `VEG-E-063` | coupled package oracle |
| `INV-VEGETATION-065` | finite-store closure vectors | test | `VEG-E-032` | coupled package oracle |
| `INV-VEGETATION-066` | independent component poison vectors | test | `VEG-E-061` | coupled package oracle |
| `INV-VEGETATION-067` | hydraulic/root-profile vectors | test | `VEG-E-063` | coupled package oracle |
| `INV-VEGETATION-068` | C/N ledger reconstruction | test | `VEG-E-032` | coupled package oracle |
| `INV-VEGETATION-069` | leaf-C/SLA/GSI ownership vector | test | `VEG-E-065` | coupled package oracle |
| `INV-VEGETATION-070` | BGC receipt/atomicity vectors | test | `VEG-E-032/065` | `SC-BIOGEOCHEM-001` |
| `INV-VEGETATION-071` | exhaustive schema/state validation | governance/test | `VEG-E-003/065` | parameter manifest |
| `INV-VEGETATION-072` | convergence and rollback vectors | test | `VEG-E-064` | numerical-solver artifact |
| `INV-VEGETATION-073` | exact occupancy-lane schema/digest validator | runtime/test | `VEG-E-070/071/077` | V2 definition + schema vectors |
| `INV-VEGETATION-074` | local/stand area-basis and nonlinear-preaggregation poisons | runtime/test | `VEG-E-072/074` | V2 independent oracle |
| `INV-VEGETATION-075` | same-tile top-to-bottom routing validator | runtime/test | `VEG-E-073` | V2 routing vectors |
| `INV-VEGETATION-076` | independent occupancy, tile-column, and weighted-stand water reconstruction | test | `VEG-E-032/073` | V2 oracle + owner ledger |
| `INV-VEGETATION-077` | typed occupancy water request/authorization/final-use validator | runtime/test | `VEG-E-072/075` | `SC-WATBAL-001` join vectors |
| `INV-VEGETATION-078` | occupancy warm-start equivalence and byte rollback | runtime/test | `VEG-E-064/071` | V2 convergence/rollback vectors |
| `INV-VEGETATION-079` | explicit V1-to-V2 migration operation | runtime/test | `VEG-E-076/077` | migration poison vectors |
| `INV-VEGETATION-080` | V3 whole-column radiation and physical-owner reconstruction | test | `VEG-E-080` | independent V3 radiation fixture and named poisons |
| `INV-VEGETATION-081` | neutral friction-velocity and surface-conductance guards | runtime/test | `VEG-E-081` | independent V3 wind/conductance fixture |
| `INV-VEGETATION-082` | common-root hydraulic schema and height/gravity continuity | runtime/test | `VEG-E-082` | independent V3 hydraulic fixture |
| `INV-VEGETATION-083` | exact V2-to-V3 migration and state-digest guards | runtime/test | `VEG-E-083` | V3 migration fixture and poisons |
| `INV-VEGETATION-084` | maximum-evaluation versus accepted-potential coupled residual reconstruction | runtime/test | `VEG-E-084` | independent V3 E11--E15 fixture |
| `INV-VEGETATION-085` | Atkin/Rd25/class-response/carbon-debit reconstruction | runtime/test | `VEG-E-085` | independent V3 respiration fixture |
| `INV-VEGETATION-086` | typed numerical-failure payload and deterministic precedence | runtime/test | `VEG-E-086` | V3 failure-diagnostics fixture |

## Producer Obligations and Consumer Obligations

- `OBL-VEGETATION-P-001`: native management supplies explicit topology and
  digest-bound configuration/initial-state/parameter/rooting references.
- `OBL-VEGETATION-P-002`: vegetation emits Stage A requests and Stage C
  candidates without cross-owner mutation or hidden fallback.
- `OBL-VEGETATION-P-003`: hydrology returns one same-transaction allocation and
  limitation reason per request and constructs all soil-layer mutations.
- `OBL-VEGETATION-P-004`: every water, radiation, latent, element, and material
  transfer includes owner, recipient, interval, area, units, and lineage.
- `OBL-VEGETATION-P-005`: receiving owners accept/reject immutable proposals
  and independently reconstruct their side before commit.
- `OBL-VEGETATION-P-006`: callers supply every required site value and initial
  state explicitly; openWEPP validates the canonical schema and never replaces
  missing values with profile, parser, or biome defaults.
- `OBL-VEGETATION-P-007`: native-forest canopy, wet-canopy, forest-floor, and
  layer-root components preserve distinct operands, state, lineage, and closure.
- `OBL-VEGETATION-P-008`: V2 producers preserve occupancy identity through
  local interception, wet-energy/physiology/hydraulics, water arbitration,
  same-tile descendant routing, weighted stand aggregation, serialization, and
  rollback; no aggregate mutable store or lateral mixing exists.
- `OBL-VEGETATION-C-001`: the orchestrator preserves stage order and commits or
  rolls back all owner candidates atomically.
- `OBL-VEGETATION-C-002`: land-surface energy supplies the authorized latent
  conversion and prevents a second energy debit.
- `OBL-VEGETATION-C-003`: residue/biogeochemistry receives dead material once
  and never treats dry matter, carbon, and nitrogen as aliases.
- `OBL-VEGETATION-C-004`: ground snow consumes only typed canopy release; it
  never shares the canopy store.
- `OBL-VEGETATION-C-005`: current GSI/ET/litter/runtime consumers remain active
  until a later real-consumer cutover proves the adapter and retires duplicate
  ownership atomically.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `C_union` | future aggregate `cancov` adapter | compatibility only | fraction; exact tile union, not summed `C_s` | this contract / `SC-PLANT-001` |
| `LAI_s` | future aggregate `lai` adapter | compatibility only | field-specific ground-area sum required | this contract / `SC-PLANT-001` |
| `z_s` | future `canhgt` adapter | compatibility only | `m`; reduction authority missing | this contract / `SC-PLANT-001` |
| `T_s` | future `Ep_compat` | compatibility only | named `kg m^-2` to `mm water` conversion | this contract / `SC-EVAP-001` |
| `D_s,l`, `A_W,s,l`, `F_W,s,l` | not aliases of legacy `UPi`, `Ui` | future layer exchange | explicit migration/cutover required | this contract / `SC-EVAP-001` |
| `E_floor,j` | not legacy PMET `Es` and not `Kcb_adjusted - Ep` | future native forest only | explicit recipient, state, resistance, and energy lineage | this contract / `SC-LANDSURFACEENERGY-001` |
| `L_DM,c` | not an alias of `L_C,c` or `L_N,c` | dead-material receipt | independent unit/stoichiometry fields | this contract / `SC-RESIDUE-001` |
| `S_snow,s` | not ground SWE or snow depth | future canopy store | `kg m^-2` water equivalent; no runtime alias | this contract / `SC-SNOWFREEZE-001` |
| `Q_rad,k,j` | not a universal ground/net-radiation scalar | energy receipt | interval-integrated `J m^-2`; recipient-specific | `SC-LANDSURFACEENERGY-001` |
| `S_liq,s,t` | not V1 `S_liq,s` and not a mutable weighted aggregate | V2 occupancy state | `kg H2O m^-2 tile-ground`; exact `(s,t)` identity | this contract |
| `D_W,s,t,l`, `A_W,s,t,l`, `F_W,s,t,l` | not stratum-only or tile-ground owner debits | V2 water exchange | stand-ground interval amount with explicit one-time `f_t` conversion | this contract / `SC-WATBAL-001` |
| `u_ref` | `reference_wind_m_s` | V3 forcing boundary | `m s^-1`; input to neutral momentum logarithm, never leaf wind directly | this contract / meteorology |
| `u_star` | `u_leaf`, `u_wet`, `u_stem` | V3 derived surface operands | same `m s^-1` value by explicit authority; distinct semantic fields | this contract |
| `psi_root` | `root_node_potential_mm` | V3 occupancy state | `mm H2O`; one common node, not MPa or a per-layer vector | this contract |
| `root_potential_mm_by_layer[]` | no V3 runtime alias | historical V2 migration input only | bitwise-identical-only migration; no averaging or implicit conversion | this contract |
| `i_atkin`, `Rd25`, `Rd(T)` | no `rd_leaf_n_rate` alias | V3 gas/carbon join | source equation is already `umol CO2 m^-2 leaf s^-1`; carbon conversion occurs only in interval ledger integration | this contract |

## Constants and Parameters with Provenance Anchors

Versions 1--4 admitted no vegetation-process numerical constant or empirical
default. Version 5 superseded only the constant prohibition by admitting the
explicit fixed constants in its equation set and numerical section. No source default, recommended
profile value, physiological bound, or parameter set is admitted. Every later parameter entry must be one
of `fixed_science`, `calibratable`, `external_configuration`, or
`initial_state`; carry units, validity domain, evidence bounds, version,
SHA-256, authority, ecosystem applicability, and prohibited extrapolations; and
distinguish any `ASSUMED_FOR_EXECUTION` value from science or calibration.

Site-specific parameter values are caller-supplied `external_configuration`;
their admissibility is schema- and domain-based, not a claim that openWEPP chose
the appropriate value for the site. A compatible initial state may be
caller-supplied `initial_state` under `INV-VEGETATION-058`. Empirical authority
is required when openWEPP distributes a recommended default, assigns an
observation role, or makes a calibration, validation, ecosystem applicability,
or transferability claim. `ASSUMED_FOR_EXECUTION` fixtures demonstrate typed
behavior only and make no calibration, validation, ecosystem applicability, or transferability claim.

V2 preserves every version-5 fixed constitutive constant and numerical
tolerance. Its new topology/routing/state/resource rules contain no tunable
constant. Version-5 fixed constants are: `R=8.31446261815324 J mol^-1 K^-1`, carbon
molar mass `0.012011 kg mol^-1`, stomatal/boundary H2O:CO2 ratios `1.6/1.4`,
PAR conversion `4.6 umol photon J^-1`, PSII partition `0.5`, oxygen mole
fraction `0.20`, co-limitation curvatures `0.98/0.95`, electron curvature/yield
`0.7/0.85`, `rho_water=1000 kg m^-3`, `Cv=0.01 m s^-1/2`, `kappa=0.4`,
`sigma=5.670374419e-8 W m^-2 K^-4`, `Rdry=287.05 J kg^-1 K^-1`,
`cp_air=1004.64 J kg^-1 K^-1`, and `lambda_vap=2.501e6 J kg^-1`. CLM5
Table 5.2 liquid saturation coefficients `a0..a8` are respectively
`6.11213476, 4.44007856e-1, 1.43064234e-2, 2.64461437e-4,
3.05903558e-6, 1.96237241e-8, 8.92344772e-11, -3.73208410e-13,
2.09339997e-16`, with the equation's factor `100 Pa`.

V3 retains all unchanged V2 constants and adds the fixed Rd response constants
`Ha_Rd=46390 J mol^-1`, `Hd_Rd=150650 J mol^-1`, and
`S_Rd=490 J mol^-1 K^-1`. Its neutral wind derivation reuses digest-bound
`kappa=0.4`; no minimum wind is admitted. Immutable CTSM source binds the Atkin
coefficients and establishes that its result is already `Rd25` in
`umol CO2 m^-2 leaf s^-1`; carbon molar mass is used only when integrating the
identical class-resolved Rd into the carbon ledger. The numbers
`1000 mm m^-1` and `1000 g kg^-1` are explicit directional unit conversions,
not numerical floors.

The exact v1 consumed configuration fields are:
`model_definition_sha256`, `configuration_sha256`, `initial_state_sha256`,
`area_m2`, `timestamp`, `dt_s`, `topology_tiles[]`, `stratum_id`, `lifeform`,
`phenology_type`, `vertical_rank`, `tile_ids[]`, `height_m`, `crown_base_m`,
`leaf_dimension_m`, `stem_dimension_m`, `wet_surface_dimension_m`,
`sla_m2_per_kg_c`, `sai_relation`, `leaf_angle_chi`,
`clumping_index`, all leaf/stem VIS/NIR `rho/tau` pairs,
`g0_umol_h2o_m2_s`, `g1_sqrt_kpa`, `rubisco_n_efficiency`,
`electron_n_efficiency`, `tp_vcmax_ratio`, `rd_leaf_n_rate`, `kc25_pa`,
`ko25_pa`, `gamma25_pa`, the Vcmax/Jmax/Kc/Ko/Gamma
activation/deactivation/entropy fields, `alpha_liq`,
`p_liq_kg_m2_plant`, `stemflow_fraction`, `z0m_m`, `z0h_m`, `z0q_m`,
`leaf_emissivity`, `stem_emissivity`, `wet_surface_emissivity`,
`displacement_m`, the four path `kmax` fields, three `p50` fields,
`vulnerability_shape`, `root_to_leaf_area`, `lateral_root_length_m`,
`root_layers[]`, `atkin_intercept`, `mr_base_kgc_per_kgn_s`, `mr_q10`,
`xs_recovery_days`, `growth_resp_ratio_g1`, allocation `a1/a2/a3/a4`,
`current_growth_fraction`, five tissue/litter C:N fields,
`drymatter_carbon_fraction`, `mineral_n_root_fraction[]`,
`nh4_request_fraction`, three litter chemistry fractions per donor tissue,
onset/offset durations and GSI thresholds/hysteresis, four tissue lifetimes/
mortality rates. Their exact units and domains
are those in Variables, the equations, and the package field table; runtime
aliases are identical. Every field is required, with explicit null GSI fields
for evergreen strata. Missing, extra consumed, duplicate, sentinel, nonfinite,
or incompatible fields fail before calculation. The definition serialization
is canonical UTF-8, lexicographically ordered keys, decimal numbers in shortest
round-trip form, and SHA-256 over those bytes.

The CLM leaf-angle approximation is admitted only for
`-0.4<=leaf_angle_chi<=0.6`; values outside that interval are typed unsupported
configuration, not extrapolated or clamped.

The exact V3 consumed configuration is the V2 field inventory with
`rd_leaf_n_rate` removed. `atkin_intercept` remains required and has units
`umol CO2 m^-2 leaf s^-1`; `Ha_Rd`, `Hd_Rd`, and `S_Rd` are fixed model-definition
constants rather than caller parameters. The V3 occupancy state has the exact
15-field schema in the V3 amendment and replaces the V2 root-layer warm-start
vector with `root_node_potential_mm`. Numerical failure DTO fields and their
enumerated pass/solve identities are definition schema, not optional logging.

Version 7 admits the constitutive equations, topology inheritance, and V3
amendment above. Earlier-version statements limiting admission to
configuration/bookkeeping or treating V1/V2 as executable successors are
historical and superseded only for V3 execution.

### Definition Acquisition And Typed Schema

The schema-form portion of `AUTH-RHEC-001` and all authority requirements of
`AUTH-RHEC-016` are admitted at the authority level. Version 5's exact consumed
field inventory is the canonical field list in this contract's Constants and
Parameters section and the package manifest; runtime names equal canonical
snake-case names, and no RHESSys spelling is a consumed alias. This admission
does not authorize a profile value or runtime implementation.

1. A definition reference is the tuple `(repository, immutable_commit,
   repository_relative_path, sha256)`. All four members are required and are
   checked against caller-supplied local bytes before parsing.
2. HTTP, HTTPS, FTP, mutable branch names such as `master`, search paths, and
   fallback downloads are invalid runtime acquisition modes. Failure leaves no
   parsed or resolved object.
3. The evidence object retains the exact source bytes and an ordered lexical
   parse containing every key spelling, occurrence, and value token. Duplicate
   keys are evidence and a resolution error; they are never last-write-wins.
4. A separate versioned schema manifest resolves raw keys to canonical fields.
   Aliases are explicit, one-to-one, unit- and semantic-authority-backed, and
   versioned. Unknown, missing, duplicate, non-finite, invalid, and unsupported
   sentinel values fail resolution unless the schema explicitly classifies an
   unknown key as diagnostic-only and non-consumed.
5. Each consumed-field declaration contains the metadata required by
   `INV-VEGETATION-055`. No parser default fills an absent field. A schema can
   admit the type and meaning of an external input while requiring the caller to
   provide its site-specific value. The value is `CALLER_CONFIGURATION`, not a
   source default or openWEPP transferability claim.
6. Initial state may be caller-supplied site state and uses the same immutable
   identity discipline while remaining separate from parameter definitions. It
   requires date, horizontal area and topology, profile identity, units, and
   complete pool/geometry coverage. Stand/plot identity, observation operator,
   and uncertainty become mandatory when the caller or openWEPP assigns an
   observational, calibration, validation, or transferability role.
7. Mixed vegetation is a topology containing separately identified resolved
   strata. It cannot be created by averaging raw or resolved parameter records.

Required contract-derived vectors are: valid local digest; one-bit digest
mismatch; mutable reference; network URI; duplicate raw key; absent required
key; unsupported sentinel; unknown diagnostic key; explicit accepted alias;
rejected cadence/unit alias; raw round trip after resolution; and two-stratum
composition proving that neither parameter record was averaged.

## Unit-Governance Map

| Symbol family | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| topology fractions | fraction | future vegetation registry | none | typed fraction required | none |
| heights/root geometry | `m` | future vegetation registry | named geometry conversion only | none | none |
| water stores/transfers | interval-integrated `kg m^-2` | future vegetation registry | named depth/area-mass or rate-time integration only | no final scalar exception | none |
| radiation/latent energy | `J m^-2` per interval | future LSE registry | named flux-duration integration | no final scalar exception | none |
| dry matter / C / N | `kg dry matter m^-2`, `kg C m^-2`, `kg N m^-2` | future vegetation/residue registry | no implicit stoichiometric conversion | no final scalar exception | none |
| compatibility `Ep` | `mm` per declared interval | existing daily water family only after named conversion | `kg_m2_to_mm_water` or successor | none | no publication authorized |
| V2 occupancy water stores/releases | `kg H2O m^-2 tile-ground` | future vegetation occupancy registry | named tile-to-stand weighting only | no scalar exception | none |
| V2 occupancy water owner exchange | interval `kg H2O m^-2 stand-ground` | future typed resource registry | exactly one multiply/divide by positive `f_t` at boundary | no scalar exception | none |
| V2 occupancy energy | interval `J m^-2 tile-ground` | future LSE occupancy registry | named `f_t` weighting after local solve | no scalar exception | none |
| V3 mixed plant area and absorbed owner energy | `m^2 m^-2 tile-ground`, `W m^-2 tile-ground` | future vegetation/LSE occupancy registry | named flux-duration integration only after physical owner partition | no scalar exception | none |
| V3 wind operands | `m s^-1` | future vegetation boundary registry | `reference_wind_to_neutral_friction_velocity` | no scalar exception | none |
| V3 hydraulic path/head/potentials | `m`, `mm H2O` | future vegetation hydraulic registry | `height_m_to_gravity_head_mm_h2o` | no scalar exception | none |
| V3 Atkin/Rd operands | `umol CO2 m^-2 leaf s^-1`, `kg N m^-2 leaf`, `g N m^-2 leaf`, `degC` | future vegetation gas/carbon registry | `kg_n_to_g_n`, Kelvin-to-Celsius difference, and exact interval carbon integration only | no scalar exception | none |
| V3 numerical failure diagnostics | typed identities plus contract-declared units per numeric field | future vegetation diagnostic registry | none beyond named dimensional helpers above | enum/count fields only | no publication authorized |

No runtime symbol, registry, or output metadata changes are authorized here.

## Tolerance and Numeric Notes

- Conservation and representation tolerances are distinct.
- Version 5 numerical tolerances and solver limits are the explicit
  `OPENWEPP_CANONICAL_SELECTION` values in the equation set above.
- Version 7 retains all unchanged V2 tolerances. Topology fractions use the existing
  separately admitted `1e-12` representation tolerance only for validation;
  no constitutive operand, negative store, or closure residual is silently
  normalized through it.
- V3 mixed-optics preparation, neutral wind, Atkin conversion, height/gravity,
  common-root state, and class/total potential residuals use the inherited
  scale-aware tolerances of their owning solve; no new flux or identity
  tolerance is introduced. Identity, model digest, band, direction, owner,
  amount basis, and migration bit equality are exact and never tolerance-fixed.
- V3 centered finite-difference state unit scales are exact model constants:
  `1 Pa` for `ci`, `1 K` for every temperature, `0.001 kg kg^-1` for
  canopy-air specific humidity, `1000 mm H2O` for every hydraulic potential,
  and `1` for each class beta factor. The component perturbation is
  `sqrt(epsilon)*max(abs(x),unit_scale)`; no other hidden scale is allowed.
- Energy-component normalization uses
  `scale_E=max(1 W m^-2,sum(abs(each incident, emitted, sensible, and latent
  operand in that residual)))` with the inherited
  `1e-6 W m^-2+1e-10*scale_E` threshold. Canopy-air vapor continuity uses the
  water-flux threshold with
  `scale_W=max(1e-12 mm s^-1,abs(each vapor or transpiration flux in that
  residual))`. Each hydraulic or class-continuity residual uses one shared
  `scale_W=max(1e-12 mm s^-1,E_sun,max,E_shade,max,abs(q1a),abs(q1b),
  abs(q2),max_i(abs(q3_i)))` and the inherited
  `1e-12 mm s^-1+1e-9*scale_W` threshold. A reported normalized residual is
  the signed residual divided by its exact owning threshold and retains its
  component identity. The post-acceptance beta aggregation is exact algebra,
  not a seventh convergence residual.
- `NumericalFailureDiagnostics` is canonical transaction evidence. Optional
  fields are null only when the solve failed before that quantity existed;
  nonfinite JSON numbers are forbidden. Diagnostic serialization never makes a
  rejected iterate eligible for state, request, or ledger construction.
- Zero snapping, negative-pool clipping, cover perturbation, conductance floors,
  denominator replacement, or unbounded iteration are prohibited absent a
  threshold, units, provenance, tests, and explicit canonical authority.
- Empty vegetation, zero leaf area, zero demand, and zero transfer are valid
  degenerates when all corresponding stores and receipts close.

## Calibration and Identifiability

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_CALIBRATION_READY
identifiability_status = NOT_ASSESSED
```

Version 7 admits equations, domains, occupancy semantics, and a typed parameter
surface, while the
canonical runtime remains absent; therefore `science_implementation_status =
NOT_IMPLEMENTED`.
No current parameter,
dataset, observation operator, objective, calibration, validation, synthetic
recovery, identifiability result, or transferability claim is admitted.
Comparator agreement and source-reported defaults are prohibited evidence.
Caller-supplied configuration is usable before calibration readiness because it
makes no suitability claim; it must not be mislabeled as calibrated or validated.

## Test-Vector Obligations

| Vector family | Expected observable/result | Bound invariant/failure |
|---|---|---|
| empty stand | valid zero response and no invented state | `INV-VEGETATION-001`, `INV-VEGETATION-015` |
| one stratum/one occupied tile | exact `C_s=C_union` and stable order | `INV-VEGETATION-002`, `INV-VEGETATION-003` |
| two disjoint strata at one rank | rank cover closes and union equals sum | `INV-VEGETATION-002`, `INV-VEGETATION-003` |
| vertically overlapping strata | sum of stratum covers may exceed one while exact tile union does not | `INV-VEGETATION-002` |
| duplicate same-rank occupancy / tile sum error | typed rejection without state repair | `VEG-E-002` |
| missing digest/parameter class/root profile | typed failure; no hidden default | `VEG-E-003/060` |
| Stage B fully supplied / dry / frozen / competing | bounded allocation plus exact reason code | `INV-VEGETATION-010`, `INV-VEGETATION-012` |
| same-layer aggregate overbooking | reject when `sum_s A_W,s,l + W_comp,l > A_l` even if each authorization is within demand | `INV-VEGETATION-012`, `VEG-E-020` |
| stale transaction/layer identity | reject before any candidate commit | `VEG-E-010` |
| allocation above demand or accessible liquid | reject hydrology and vegetation candidates | `VEG-E-020` |
| all-distinct request/authorization/finalized use | both owners reconstruct `T_s=sum F_W,s,l` and the ordering inequalities | `INV-VEGETATION-013`, `VEG-E-030` |
| all-distinct water/energy operands | exact `Q_T,s=-h_v*T_s`, no alias/double debit | `INV-VEGETATION-014`, `INV-VEGETATION-021` |
| canopy liquid store | independently reconstruct start + incident - evaporation - every release = end | `INV-VEGETATION-020` |
| canopy/ground/litter/snow/soil poison aliases | omitted, duplicated, or swapped recipient fails | `INV-VEGETATION-021`, `VEG-E-011/032` |
| dry matter/C/N transfer | donor and receiver reconstruct same three distinct operands | `INV-VEGETATION-030`, `INV-VEGETATION-031` |
| canopy snow request | ownership visible but constitutive execution rejected under versions 1-7 | `INV-VEGETATION-022`, `VEG-E-040` |
| unbounded/failed iteration | no partial mutation or publication | `INV-VEGETATION-015`, `VEG-E-041` |
| compatibility adapter | field-specific receipt, read-only, no native feedback | `INV-VEGETATION-040`, `INV-VEGETATION-041` |
| source-derived constant/proxy physiology | `AUTHORITY_MISSING`, `NON_PROMOTABLE` | `INV-VEGETATION-050`, `INV-VEGETATION-051` |
| distinct caller stratum values | both parse and remain separately reconstructible; no averaging or default substitution | `INV-VEGETATION-052`, `INV-VEGETATION-057`, `INV-VEGETATION-061` |
| complete caller initial state / one missing pool | accept the complete state; reject the incomplete state without synthesis | `INV-VEGETATION-056`, `INV-VEGETATION-058` |
| native-forest component poison vector | all-distinct canopy transpiration, wet-canopy evaporation, and forest-floor evaporation close independently | `INV-VEGETATION-059`, `INV-VEGETATION-060` |
| canopy-area reduction with unchanged floor operands | canopy response changes; floor evaporation is not increased by the lost canopy demand | `INV-VEGETATION-060`, `VEG-E-061` |
| two layer-root profiles under one soil snapshot | distinct layer-resolved root requests and hydrology receipts; no single-depth alias | `INV-VEGETATION-004`, `INV-VEGETATION-061` |
| zero/saturated/Rubisco/electron/transition light cases | independently reconstructed limiting branch and co-limitation root | `INV-VEGETATION-062/064/072` |
| coupled assimilation-conductance-temperature-VPD-transpiration | converged common-state result; poison equation differs | `INV-VEGETATION-064/072` |
| deciduous/evergreen/mixed C/N cycles | persistent distinct pools, leaf-C-owned LAI and exact ledgers | `INV-VEGETATION-063/068/069` |
| N limitation/retranslocation/competition | request-bounded growth and exact vegetation/BGC receipt | `INV-VEGETATION-068/070` |
| nonconvergence/invalid state | typed error and byte-identical state | `INV-VEGETATION-071/072` |
| one stratum on unequal tiles with distinct rain/stores | distinct local E04 states and exact weighted aggregate | `INV-VEGETATION-073/074/076` |
| same stratum below different upper columns | same-tile throughfall/drainage only; no lateral mixing | `INV-VEGETATION-075/076` |
| two ranks plus stemflow | free liquid/drainage enters lower occupancy; stemflow bypasses to same-tile ground | `INV-VEGETATION-075` |
| condensation and second drainage in one occupancy | second drainage is explicit and local closure passes | `INV-VEGETATION-074/076` |
| empty tile and single-tile reduction | rain reaches same-tile ground; V2 local result reduces to V1 E04 | `INV-VEGETATION-075/079` |
| homogeneous two-tile and tile permutation | expected weighted result and byte-stable order invariance | `INV-VEGETATION-073/076/078` |
| occupancy water weighting/back-conversion | exactly one `f_t` conversion, bounded finalized use | `INV-VEGETATION-077` |
| aggregate-first nonlinear poison | at least one accepted operand differs from weighted local execution | `INV-VEGETATION-074` |
| V1 migration zero/single/nonzero-multitile | exact expansion/conversion/unresolved-lane error | `INV-VEGETATION-079` |
| tile-local failure after candidate work | every owner and every occupancy warm start byte-identical | `INV-VEGETATION-078` |
| two-rank mixed leaf/stem VIS/NIR direct/diffuse column | whole-column boundary solution, effective optics, non-unit clumping, ground/upward reflection, physical owner absorption, leaf-only sun/shade area, and directional closure match the independent fixture | `INV-VEGETATION-080`, `VEG-E-080` |
| leaf-only / stem-only / identical-optics / zero-leaf / zero-stem / zero-direct / zero-plant reductions | exact admitted degenerate or reduction; zero-leaf stem-only canopy has zero photosynthesis | `INV-VEGETATION-080` |
| radiation poison alternatives | leaf-only optics, stem-only optics, arithmetic mean, area-only absorption, double/omitted clumping, sunlit plant as leaf, stem PAR, VIS/NIR swap, direct/diffuse swap, zero lower boundary, and direct-summed reflection differ and are rejected | `INV-VEGETATION-080`, `VEG-E-080` |
| neutral reference wind | exact `u_star`, three semantic wind identities, and distinct leaf/wet/stem conductances | `INV-VEGETATION-081` |
| wind domain poisons | direct `u_ref`, hidden floor, wrong roughness, nonpositive wind, and invalid `z_ref<=d+z0m` reject | `VEG-E-081` |
| height/gravity and common root node | exact `z2=height`, `Delta_psi=1000*height`, four potentials, and every accessible `q3_i` join one root node | `INV-VEGETATION-082` |
| V2 root warm-start migration | bitwise-identical vector maps exactly; empty/unequal vectors exhaustively name `root_node_potential_mm`; average/first/weighted alternatives reject | `INV-VEGETATION-083`, `VEG-E-083` |
| uncapped potential E11--E15 | explicit reference wind/`u_star`, conductances, class Emax, accepted `beta_sun/beta_shade`, persisted aggregate `beta_hyd`, four potentials, every `q1/q2/q3`, both class loss/flux equalities, total continuity, accepted transpiration, and stand-ground requests match independent expected values | `INV-VEGETATION-084` |
| potential numerical/physical variants | alternate warm starts converge equivalently; dry/frozen/two-layer/redistribution/singular/iteration-limit cases return exact result or typed diagnostics | `INV-VEGETATION-082/084/086`, `VEG-E-082/084/086` |
| potential-pass poisons | published beta-one demand, sequential hydraulics, post-hoc scalar stress, aggregate-only equality, authorization in potential, and external supply clamp differ and reject | `INV-VEGETATION-084`, `VEG-E-084` |
| Atkin/Rd identity | exact `Rd25` from leaf N/T10, class peaked response, `An=Ag-Rd`, class-area scaling, and one identical carbon debit | `INV-VEGETATION-085` |
| respiration poisons | nonpositive Atkin, clamp, `rd_leaf_n_rate`, wrong temperature response, sun/shade swap, and double debit reject or numerically differ | `INV-VEGETATION-085`, `VEG-E-085` |
| numerical failure payload family | each solve identity preserves deterministic pass/occupancy/iteration/residual/step/backtracking/bound/cap/bracket/pivot/matrix evidence and byte-identical rollback | `INV-VEGETATION-086`, `VEG-E-086` |
| V4 displayed/storage/transfer leaf C are all distinct and nonzero | LAI/SAI/RAI use displayed C only; total-pool and partial-pool poisons differ | V4 successor amendment |
| V4 zero display with nonzero leaf storage/transfer | exact zero leaf/stem/root area and zero photosynthetic area | V4 successor amendment |
| distinct displayed/storage/transfer leaf N | FvCB/Atkin capacity uses displayed N divided by displayed LAI only; total-N and donor-N poisons differ | V4 successor amendment |
| complete V4 six-tissue shared state | exact field/tissue/subpool set, recursive-order-stable digest, and every retained scalar mutation changes digest | V4 successor amendment |
| V3-to-V4 shared migration | exactly two offset fields removed, every retained shared/occupancy byte preserved, mismatched source cache rejects | V4 successor amendment |

Future fixtures must use deliberately distinct canopy, ground, litter, snow,
soil, ponded-water, layer, dry-matter, carbon, and nitrogen operands so wrong
aliases cannot equal the expected result. Producer self-consistency alone is
insufficient; both owners reconstruct from independent state/output surfaces.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-VEGETATION-001` | Version 1 native vegetation boundary admission | `active` | `maps-to-existing-INV` | `INV-VEGETATION-001, INV-VEGETATION-010, INV-VEGETATION-011, INV-VEGETATION-013, INV-VEGETATION-014, INV-VEGETATION-022, INV-VEGETATION-041, INV-VEGETATION-050, INV-VEGETATION-051` | `flagged-binding-addition` | Initial authority is consolidated in this contract; package artifacts remain evidence rather than separate binding authority. |
| `BEI-VEGETATION-002` | `20260808-rhessys-east-coast-code-literature-authority-audit-001` audit sidecar population | `active` | `maps-to-existing-INV` | `INV-VEGETATION-050, INV-VEGETATION-051, INV-VEGETATION-052` | `flagged-binding-addition` | Version 2 admits licensed provenance and a strict definition/schema obligation only; the audit's constitutive findings remain explicit gaps and require the package's dual review/disposition/verification cycle. |
| `BEI-VEGETATION-003` | `20260808-rhessys-east-coast-vegetation-authority-admission-001` strict acquisition/schema admission | `active` | `maps-to-existing-INV` | `INV-VEGETATION-052, INV-VEGETATION-053, INV-VEGETATION-054, INV-VEGETATION-055, INV-VEGETATION-056` | `flagged-binding-addition` | Version 3 closes acquisition and schema-form authority only. Selected values, aliases lacking unit/semantic proof, initial state, constitutive science, implementation, and cutover remain non-promotable. |
| `BEI-VEGETATION-004` | `20260809-native-forest-ecohydrology-authority-reframe-001` | `active` | `maps-to-existing-INV` | `INV-VEGETATION-055, INV-VEGETATION-056, INV-VEGETATION-057, INV-VEGETATION-058, INV-VEGETATION-059, INV-VEGETATION-060, INV-VEGETATION-061` | `flagged-binding-addition` | Version 4 assigns site values/state to caller configuration, constrains demonstration claims, and prohibits the agricultural PMET partition as the native-forest target while retaining complete constitutive-authority requirements. |
| `BEI-VEGETATION-005` | `20260811-coupled-c3-forest-vegetation-model-stack-authority-001` | `active` | `maps-to-existing-INV` | `INV-VEGETATION-062, INV-VEGETATION-063, INV-VEGETATION-064, INV-VEGETATION-065, INV-VEGETATION-066, INV-VEGETATION-067, INV-VEGETATION-068, INV-VEGETATION-069, INV-VEGETATION-070, INV-VEGETATION-071, INV-VEGETATION-072` | `flagged-binding-addition` | Version 5 releases the indivisible contract-first C3 woody stack; implementation and empirical claims remain separate. |
| `BEI-VEGETATION-006` | `20260811-c3-woody-tile-liquid-topology-authority-001` | `active` | `maps-to-existing-INV` | `INV-VEGETATION-073, INV-VEGETATION-074, INV-VEGETATION-075, INV-VEGETATION-076, INV-VEGETATION-077, INV-VEGETATION-078, INV-VEGETATION-079` | `flagged-binding-addition` | Version 6 releases immutable V2 occupancy state, routing, area/resource identity, migration, and nonlinear local-solve implementation authority while preserving V1 as historical bytes. |
| `BEI-VEGETATION-007` | `20260812-c3-woody-potential-pass-authority-001` | `active` | `maps-to-existing-INV` | `INV-VEGETATION-080, INV-VEGETATION-081, INV-VEGETATION-082, INV-VEGETATION-083, INV-VEGETATION-084, INV-VEGETATION-085, INV-VEGETATION-086` | `flagged-binding-addition` | Version 7 releases immutable V3 radiation preparation/ownership, surface wind, hydraulic geometry/common-root state, uncapped coupled potential semantics, exact leaf respiration, diagnostics, and independent fixture authority while preserving V1/V2 historical bytes. |
| `BEI-VEGETATION-008` | `20260812-c3-woody-shared-state-authority-001` | `active` | `maps-to-existing-INV` | `INV-VEGETATION-069, INV-VEGETATION-087, INV-VEGETATION-088, INV-VEGETATION-089, INV-VEGETATION-090, INV-VEGETATION-091` | `flagged-binding-addition` | Version 8 releases immutable V4 exact shared-state schema, displayed-leaf C/N area/capacity ownership, derived area caches, and exact V3-to-V4 removal migration while preserving V1/V2/V3 historical bytes. |

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-VEGETATION-001` | No implemented native topology/configuration/state surface exists. | Versioned schema, typed state, digest/provenance validation, and topology vectors. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-002` | Layer root/hydraulic law was missing. | Version 5 selects CLM5 hydraulic nodes/vulnerability and explicit layer requests; implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-003` | Stage B competition/fairness/priority policy was unspecified. | Version 5 selects equal-status proportional per-layer arbitration with exact zero and oversubscription branches; implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-004` | Coupled constitutive stack was missing. | Version 5 selects the complete indivisible stack; production implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-005` | Canopy snow has a single-owner boundary but no admitted constitutive law or atomic amendment with snow/frost. | Independent authority plus joint vegetation/snow/LSE contract and mass-energy vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-006` | Elemental/dead-material receiver was missing. | `SC-BIOGEOCHEM-001` admits N and litter/CWD receiving transactions; soil transformations remain explicit dependency. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-007` | Every compatibility reduction except exact tile-union cover lacks reviewed operator/cutover evidence. | Field-specific reductions, unit helpers, real consumers, negative old-path proof. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-008` | The vegetation crate and default-off diagnostic scaffolding exist, but the public V3 transaction, complete typed numerical failures, owner candidates, atomic commit, output, and real consumer remain incomplete/fail-closed. | Resume the existing implementation package against immutable V3, pass both column solves and independent ledgers, then require direct-consumer evidence before cutover. | `IMPLEMENTATION_INCOMPLETE`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-009` | Calibration/identifiability authority and independent observations are absent. | Prospective data roles, observation operators, readiness analysis, calibration, and held-out validation. | `NOT_CALIBRATION_READY`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-010` | The earlier repository-license gap is closed only for the two pinned Laurence Lin repositories in `REF-VEGETATION-012/013`; the separate official RHESSys repository remains outside this route. | Preserve exact commit/file lineage and the MIT notice for distributed source-derived material. Licensing never substitutes for scientific authority; historical `DIRECT_TRANSLATION_PROHIBITED` remains applicable outside the admitted pinned route. | `LICENSE_ADMITTED`, `SCIENCE_AUTHORITY_UNCHANGED` |
| `GAP-VEGETATION-011` | Pinned GIS definitions contain five keys that do not match the pinned C parser. | Version 5 consumes no RHESSys alias: canonical snake-case fields are exclusive, raw source keys remain evidence, and every mismatch fails ingestion. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-012` | The parser reads 53 parameters absent from all 32 GIS profiles and silently supplies defaults. | Version 5's exhaustive consumed-field schema makes every field caller-required and rejects missing/extra consumed fields; implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-013` | The minimum generic and East-Coast deciduous/evergreen profile candidates have no cell-level source, calibration-domain, or transferability map. | Do not distribute or recommend them as defaults. Permit explicit caller values after schema/domain validation, preserve stratum identity, and label demonstration fixtures `ASSUMED_FOR_EXECUTION`. | `CALLER_CONFIGURATION`, `DEFAULT_AND_TRANSFERABILITY_CLAIM_PROHIBITED` |
| `GAP-VEGETATION-014` | RHESSys aerodynamic/Jarvis chain rejected. | Version 7 selects neutral friction-velocity-derived leaf/wet/stem winds, dimension-specific boundary transfer, and FvCB-linked Medlyn; implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-015` | RHESSys C3 path incomplete. | Version 7 selects C3-only FvCB, Atkin-owned `Rd25`, one exact Rd debit, canopy scaling, and finite solvers. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-016` | RHESSys root demand rejected. | Version 7 selects one common-root CLM5 equilibrium hydraulic system, height/gravity geometry, uncapped accepted potential coupling, and layer authorization/finalization. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-017` | RHESSys available-energy chain rejected. | Version 5 selects signed leaf energy and independent ground-owner residual. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-018` | The audited worldfile generators construct initial C/N pools and root depth with fixed row indices, unproven ratios/constants, contradictory deadwood C:N rules, and an SLA identity that diverges from the runtime parser. | Reject those synthesis paths. Implement complete, versioned, dated caller-state ingestion with exact profile/key identity, area basis, domains, finite guards, and independent mass/LAI reconstruction. | `CALLER_STATE_REQUIRED`, `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-019` | The audited Penman-Monteith routine omits the water/air molecular-mass ratio from the psychrometric constant, despite defining it and using the correct factor in another source routine. | Never port the defective expression. Penman-Monteith is neither required nor prohibited; any component selecting it must independently admit the complete equation, constants, units, resistance scale, phase/enthalpy, guards, and limiting vectors. | `SOURCE_ROUTINE_REJECTED`, `CONSTITUTIVE_AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-020` | Strict local-byte acquisition authority is now admitted by `INV-VEGETATION-053/054`, but no runtime validator implements it. The audited generator still fetches mutable raw `master` parameter collections and is prohibited. | Implement the exact tuple/digest checks and negative vectors without importing the audited fallback path. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-021` | RHESSys optical bypass rejected. | Version 7 selects explicit whole-column two-stream band/direction optics, exact leaf/stem preparation and physical absorption ownership with strict closure. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-022` | The inspected Coweeta evidence does not jointly observe every required C/N/root/geometry pool on one compatible state surface. | This does not block caller-supplied state. Require a complete caller state for execution; require observation operators, uncertainty, and compatible measurements only before an empirical calibration, validation, or transferability claim. | `CALLER_STATE_REQUIRED`, `EMPIRICAL_CLAIM_NOT_READY` |
| `GAP-VEGETATION-023` | Agricultural PMET donation is prohibited. | Version 5 structurally separates canopy/wet/floor components; implementation remains. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING`, `NATIVE_FOREST_PMET_PARTITION_PROHIBITED` |
| `GAP-VEGETATION-024` | V1 lacked persistent liquid distribution and descendant routing for a stratum spanning heterogeneous tiles. | Version 6 selects exact occupancy state, same-tile routing, local nonlinear solves, area conversion, migration, and closure. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-025` | V2 did not define the mixed leaf/stem radiation reduction/partition, local wind derivation, stem geometry/gravity, common-root warm start, accepted uncapped potential semantics, one Rd identity, complete numerical failure payload, or independent potential fixtures. | Version 7 selects every missing rule and binds independent V3 radiation, coupling, migration, respiration, and failure vectors; implementation remains in the existing state-machine package. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-026` | V3 did not identify the displayed leaf C/N subpools owning LAI and photosynthetic capacity and carried two offset-flux state fields with no admitted units, update law, or consumer. | Version 8 selects displayed leaf C/N only, derived area caches, exact V4 shared schema, and removal-only V3 migration; implementation remains in the existing state-machine package. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING` |
| `GAP-VEGETATION-027` | Imported E19 credits noncurrent growth to storage while E20 onset debits transfer, but no admitted equation moves storage to transfer. | Admit a complete ordered storage-to-transfer C/N preparation equation and independent conservation vectors before executing an onset that requires this bridge. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |

The first safe successor is an authority-and-typed-boundary slice for topology,
caller configuration/state, radiation/interception/conductance inputs,
independent native-forest flux components, and layer-resolved potential demand.
It must independently admit every implemented constitutive relationship, remain
default-off and non-publishing, mutate no soil store, and make no runtime or
cutover claim.

## `OPENWEPP_C3_WOODY_V4` Shared-State Authority Amendment

V4 normatively imports the exact V3 definition digest and every unchanged
V1/V2/V3 equation, configuration field, occupancy state, topology, transaction,
numerical rule, and unsupported branch. This section exclusively supersedes the
conflicting shared-stratum state, displayed-leaf C/N area/capacity ownership,
state-digest encoding, and migration text. Historical V3 metadata used
`contract_version: 7`; that literal is retained here solely as immutable-history
evidence, not as the current contract version.

### V4 shared-stratum state schema

For every configured stratum, the executable V4 shared state contains exactly:

- `tissues`, a map with exactly the six typed identities `leaf`, `fine_root`,
  `live_stem`, `dead_stem`, `live_coarse_root`, and `dead_coarse_root`; each
  tissue contains exactly `display`, `storage`, and `transfer`, and each of
  those contains exactly finite nonnegative `carbon` and `nitrogen` amounts in
  `kg element m^-2 stand-ground`;
- finite nonnegative `retranslocation_n` and `nsc_c`, and finite signed `xs_c`,
  all in `kg element m^-2 stand-ground`;
- `standing_dead`, containing finite nonnegative `carbon` and `nitrogen`, plus
  finite nonnegative `standing_dead_dm`, each on stand-ground basis;
- the exact `phase` enum `dormant|onset|active|offset`, finite nonnegative
  `onset_remaining_s` and `offset_remaining_s`, and finite
  `previous_gsi` in `[0,1]`;
- `pending_transfers`, whose complete typed transaction, proposal, donor,
  receiver, carbon, nitrogen, and dry-matter identity remains binding. Every
  entry has a positive nonzero `transaction_id: u128`, nonempty typed owner
  identity, positive nonzero `proposal_id: u64`, donor exactly one of
  `leaf|fine_root|live_stem|dead_stem|live_coarse_root|dead_coarse_root`,
  receiver exactly one of
  `metabolic|cellulose|lignin|coarse_woody_debris`, and finite nonnegative
  carbon, nitrogen, and dry-matter operands;
- finite positive `t10_k`;
- finite nonnegative derived caches `leaf_area`, `stem_area`, and `root_area`,
  each in `m2 m^-2 stand-ground`; and
- `last_transaction_id`, equal to the owning whole-state accepted transaction
  identity under the existing initial/accepted lineage rules.

Unknown, missing, duplicate, wrong-unit, nonfinite, or out-of-domain fields
reject before calculation. Map order is non-semantic. The V4 state digest uses
`OPENWEPP_V4_STATE_CANONICAL_V1`, a typed UTF-8 line encoding. It recursively
walks object keys by unsigned UTF-8 bytewise lexicographic order and arrays by
index, emitting one line per container and scalar as
`encoded_path<TAB>type<TAB>value<LF>`. The root path is empty. Each object-key
path component is `/k<decimal_UTF8_byte_count>:<lowercase_UTF8_hex>` and each
array component is `/i<unsigned_decimal_index>`; therefore arbitrary admitted
IDs, including whitespace and control characters, cannot collide with framing.
Object/array container values are their decimal member counts. Types and scalar
values are: `f64be`
plus exactly 16 lowercase hexadecimal digits of the IEEE-754 binary64
big-endian bits; `u128` plus unsigned base-10 digits with no leading zero except
zero; `string` plus `<decimal_UTF8_byte_count>:<lowercase_UTF8_hex>`; `null` plus an empty value;
and `bool` plus `true|false`. No locale, whitespace, exponent, shortest-decimal,
or host-language JSON-number formatting enters this preimage. Every listed
value, including every pending-transfer operand and transaction identity,
enters the V4 state digest. The digest excludes only the whole-state
`state_sha256` member and includes the root object container line after that
member is omitted.

Whole-state occupancy identity is structural, never a delimiter-composed object
key. The canonical `occupancies` node is an array sorted lexicographically by
the typed pair `(stratum_id UTF-8 bytes, tile_id UTF-8 bytes)`. Each element is
exactly an object with `identity` and `state`; `identity` is exactly an object
with independent `stratum_id` and `tile_id` strings, and `state` is the complete
occupancy lane. The existing length-prefixed string and object-key framing
applies independently to both ID values. Concatenating, joining, splitting, or
otherwise flattening the pair with `@`, NUL, whitespace, a path separator, or
any other delimiter is prohibited. Distinct admitted UTF-8 pairs must have
distinct canonical preimages even when a delimiter-based rendering would be
identical. Duplicate structural pairs reject before digest calculation.

The executable V4 schema does **not** contain
`previous_leaf_offset_flux` or `previous_root_offset_flux`. No admitted V1--V3
equation consumes either value, and no source reviewed for this amendment
establishes complete units, update law, interval semantics, or a downstream
consumer. Retaining, defaulting, accumulating, differentiating, or assigning a
new meaning to either field is prohibited. Phenology continues from the owned
phase, timers, previous accepted GSI, and explicit display/storage/transfer
pools under the unchanged admitted E20 equations.

This V4 correction does not claim that the imported E19 storage-to-E20 transfer
preparation is complete. E19 can credit noncurrent growth to `storage`, while
the admitted E20 onset equation debits `transfer`; no reviewed equation yet
moves storage into transfer. Execution requiring that bridge remains typed
fail-closed under `VEG-E-060` until successor authority supplies its complete
ordering, amounts, and C/N conservation. This bounded pre-existing omission is
not filled by either removed offset-flux scalar.

### Displayed-leaf C/N area and photosynthetic-capacity ownership

Let `C_leaf,display` be the carbon amount in exactly
`tissues[leaf].display.carbon`. V4 defines:

```text
LAI_s = C_leaf,display * SLA_s
SAI_s = LAI_s * sai_relation_s
RAI_s = (LAI_s + SAI_s) * root_to_leaf_area_s
```

The caller configuration supplies finite positive `SLA_s`, finite nonnegative
`sai_relation_s`, and finite positive `root_to_leaf_area_s` under the inherited
configuration domains. The serialized `leaf_area`, `stem_area`, and `root_area`
are derived integrity caches and must equal `LAI_s`, `SAI_s`, and `RAI_s`,
respectively, by exact IEEE-754 binary64 bits after evaluating the displayed
equations in their stated left-to-right operation order. V4 admits no cache
tolerance or normalization. The caches are not independent biomass or geometry
owners.

Leaf `storage.carbon` is a non-displayed allocation pool under the unchanged
E19 allocation identity. Leaf `transfer.carbon` is the explicit E20 onset donor.
This amendment admits no new storage-to-transfer movement. Neither pool is
displayed tissue, exposed leaf area,
photosynthetic capacity, interception area, hydraulic area, or radiation area
until an accepted E20 transition debits the source pool and credits
`leaf.display` in the same candidate. Consequently, a state with zero displayed
leaf C and positive leaf storage or transfer C has exact zero LAI and cannot
photosynthesize. Summing display, storage, and transfer C for LAI is a typed
state-identity failure, not an alternative reduction.

Let `N_leaf,display=tissues[leaf].display.nitrogen`. For positive `LAI_s`,
V4 exclusively defines
`Nleaf_area=N_leaf,display/LAI_s` in `kg N m^-2 displayed leaf`. The unchanged
V3 FvCB capacity and Atkin/Rd equations consume this value. Leaf
`storage.nitrogen` and `transfer.nitrogen` contribute exactly zero to Vcmax,
Jmax, TPU, Rd25, maintenance respiration, or any other displayed-leaf
physiology until an accepted state transition credits leaf display N. For exact
zero `LAI_s`, displayed leaf N must also be exact zero and the existing
zero-capacity/zero-Rd branch executes without division; storage and transfer N
remain non-displayed pools. Positive displayed leaf N at zero displayed leaf C,
or any capacity formed from total leaf N, is a typed shared-state identity
failure.

Each accepted state independently reconstructs the three cached areas from the
accepted displayed leaf C and configuration. Candidate construction updates C/N
pools first, derives the caches once from the resulting displayed leaf C, then
performs state validation. No producer-supplied area can override that
reconstruction.

### Exact V3-to-V4 migration

V3 never became an accepted public runtime state machine. Migration therefore
first validates complete V3 model/configuration/state identity, including the
two finite legacy offset-flux values. For each stratum it then:

1. copies every V4-listed shared field bit-for-bit;
2. removes only `previous_leaf_offset_flux` and
   `previous_root_offset_flux` without mapping their values anywhere;
3. independently evaluates `leaf_area`, `stem_area`, and `root_area` from
   displayed leaf C and V4 configuration in the stated operation order,
   requires exact binary64 equality with the V3 caches, and then copies those
   already-valid cache bytes unchanged;
4. copies every V3 occupancy lane bit-for-bit; and
5. binds the caller-supplied V4 model/configuration identity and recomputes the
   complete V4 state digest.

Migration never sums leaf pools, invents offset-flux consumers, copies an
offset flux into `previous_gsi`, or silently repairs a mismatched area cache.
Because the removed fields have no admitted consumer or material ownership,
their removal creates no carbon, nitrogen, water, energy, or dry-material
transfer.

### V4 invariants, guards, and independent vectors

| ID | Binding invariant or guard |
|---|---|
| `INV-VEGETATION-087` | The V4 shared-stratum record has exactly the listed fields and six tissue identities; every listed operand enters deterministic whole-state serialization and digest. |
| `INV-VEGETATION-088` | `LAI_s=tissues[leaf].display.carbon*SLA_s`; storage and transfer leaf C contribute exactly zero until accepted transfer into display. |
| `INV-VEGETATION-089` | `leaf_area`, `stem_area`, and `root_area` are reconstructed caches equal to the three V4 displayed-leaf equations and cannot override them. |
| `INV-VEGETATION-090` | V3-to-V4 migration removes exactly the two unconsumed offset-flux fields, copies all other shared and occupancy state exactly, and rejects mismatched source area caches. |
| `INV-VEGETATION-091` | Displayed leaf N alone supplies `Nleaf_area` and every FvCB/Atkin capacity; storage/transfer leaf N contributes exact zero before accepted transfer to display. |
| `INV-VEGETATION-092` | Every whole-state occupancy identity is encoded as an independently framed structural `(stratum_id,tile_id)` pair; delimiter flattening is forbidden and cannot determine ordering, equality, membership, or digest bytes. |
| `VEG-E-087` | Missing/extra/duplicate shared field or tissue, legacy offset-flux field in V4 input, nonfinite/domain-invalid amount, or state-digest mismatch rejects before calculation. |
| `VEG-E-088` | LAI or any derived cache formed from total leaf C, storage C, transfer C, or another tissue rejects as a shared-state identity mismatch. |
| `VEG-E-089` | V3 source identity invalid, source cache inconsistent, or any migration synthesis beyond exact removal/rebinding rejects without a partial V4 state. |
| `VEG-E-090` | Positive displayed leaf N at zero displayed leaf C, or photosynthetic/respiration capacity formed from storage/transfer leaf N, rejects before gas exchange. |

The independent V4 fixture family binds at least: positive and zero displayed
leaf C with distinct nonzero storage/transfer poisons; exact leaf/stem/root area
reconstruction; all six tissue identities; recursive-order-invariant canonical
serialization; one-bit mutation of every shared-state field changing the
digest; unknown/missing/duplicate fields; retained legacy offset fields;
display-plus-storage-plus-transfer LAI; wrong cached area; exact V3-to-V4 field
removal; and byte preservation of every unchanged shared and occupancy field.
Expected bytes and values come from the package Python reference calculator,
never from Rust or from the implementation under test.

### V4 scope and claims

V4 corrects authority and schema only. It authorizes the existing implementation
package to replace the V3 shared-state surface and LAI consumer, but does not
implement Rust, activate a runtime selector, complete E01--E22, cut over a real
consumer, admit canopy snow or soil transformations, or establish calibration,
identifiability, empirical validity, or transferability.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-12 | 8 | Codex | Admitted `OPENWEPP_C3_WOODY_V4` exact shared-stratum schema, displayed-leaf-C/N-only area and capacity ownership, derived area caches, and exact removal of two unconsumed V3 offset-flux fields; preserved V1/V2/V3 bytes as historical authority. |
| 2026-08-12 | 7 | Codex | Admitted `OPENWEPP_C3_WOODY_V3` mixed leaf/stem radiation ownership, neutral surface wind, height/gravity and common-root hydraulics, coupled uncapped potential semantics, one Atkin/Rd identity, typed numerical failure payloads, exact migration, and independent V3 fixtures; preserved V1/V2 bytes as historical authority. |
| 2026-08-08 | 2 | Codex | Admitted the exact licensed-source provenance boundary without promoting source science; added strict-definition invariant `INV-VEGETATION-052` and audit-proven format, hidden-default, parameter, conductance, photosynthesis, root-demand, available-energy, and initialization gaps. |
| 2026-08-08 | 3 | Codex | Admitted strict caller-supplied local acquisition, immutable raw/resolved separation, typed schema-form requirements, and dated initial-state identity; retained every selected value, alias, initializer, constitutive, implementation, and cutover gap. |
| 2026-08-09 | 4 | Codex | Reclassified site-specific values and complete compatible state as caller configuration, bounded demonstration claims, prohibited agricultural PMET redistribution in the native-forest target, and required independent canopy/wet-canopy/forest-floor/root component closure. |
| 2026-08-11 | 5 | Codex | Admitted `OPENWEPP_C3_WOODY_V1`, its complete coupled equation/state/numerical authority, caller-only parameter/state posture, and `SC-BIOGEOCHEM-001` transaction boundary; released implementation authority without runtime or empirical claims. |
| 2026-08-12 | 6 | Codex | Admitted `OPENWEPP_C3_WOODY_V2` tile-resolved occupancy liquid state, same-tile column routing, occupancy-local nonlinear wet-energy/physiology/hydraulics, occupancy-preserving water transactions, exact migration, and local/stand closure; preserved V1 bytes as historical authority. |
| 2026-08-08 | 1 | Codex | Initial native-stratum, Stage A/B/C, ownership, transaction, conservation, compatibility, firewall, and non-promotable-gap authority. |
