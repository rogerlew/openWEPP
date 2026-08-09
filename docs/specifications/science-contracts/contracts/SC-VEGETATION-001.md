---
contract_id: SC-VEGETATION-001
title: Native Vegetation State and Cross-Domain Boundary Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + forest ecohydrology/hydrology reviewer
contract_version: 4
producer_scope:
  - Native vegetation configuration/runtime separation and stratum topology
  - Stage A potential response and Stage C vegetation finalization boundaries
  - Vegetation-owned canopy stores, elemental state, and transfer proposals
  - Read-only aggregate compatibility reduction
consumer_scope:
  - Native management, land-surface energy, soil hydrology, snow/frost, residue/biogeochemistry, and hillslope orchestration
evidence_level: static
last_reviewed: 2026-08-09
supersedes: []
superseded_by: []
---

# SC-VEGETATION-001 Native Vegetation State and Cross-Domain Boundary Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define canonical openWEPP ownership, state, ordering, units, conservation, and
failure semantics for a future native vegetation subsystem. Version 4 admits
typed boundary architecture, strict local-definition acquisition, typed-schema
rules, caller-owned site configuration, and native-forest component-flux
separation. It admits no vegetation kernel, physiological formula, empirical
default, runtime selector, management schema, compatibility cutover, output,
calibration, or recommended site value.

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

- every radiation, interception-capacity, stemflow, aerodynamic, stomatal,
  transpiration-demand, photosynthesis, respiration, allocation, mortality,
  turnover, rooting-development, canopy-snow, or nutrient-cycle constitutive
  equation not already admitted by a named canonical owner;
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
| `z_s` | `m` | stratum reference height | vegetation |
| `LAI_s`, `WAI_s` | `m^2 m^-2` | leaf and woody area per ground area | vegetation |
| `r_s,l` | fraction | root participation fraction for stratum `s`, soil layer `l` | native management / vegetation |
| `S_liq,s` | `kg m^-2` | liquid water stored on stratum `s` | vegetation |
| `S_snow,s` | `kg m^-2` | future intercepted canopy-snow water-equivalent store | vegetation; constitutive behavior non-promotable in versions 2-4 |
| `P_liq,s` | `kg m^-2` | interval-integrated liquid incident on stratum `s` | upstream canopy/forcing handoff |
| `E_int,s` | `kg m^-2` | interval-integrated actual evaporation from canopy liquid store | vegetation + energy join |
| `R_down,s` | `kg m^-2` | interval-integrated typed total downward liquid release | vegetation |
| `R_stem,s`, `R_drip,s` | `kg m^-2` | future interval-integrated distinct stemflow and drip/drainage terms | authority missing |
| `Q_rad,k,j` | `J m^-2` | interval-integrated radiation energy in band/direction `k` received by component `j` | land-surface energy |
| `D_s,l` | `kg m^-2` | interval-integrated Stage A root-water request | vegetation |
| `U_s,l` | `kg m^-2` | interval-integrated Stage B hydrology-authorized withdrawal | soil hydrology |
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

`kg m^-2` and `mm water` are not silently interchangeable. Any compatibility
conversion requires a named unit helper and water-density authority selected by
the implementation package.

Every transfer above is an amount integrated over `tau`; `interval^-1` is not a
physical unit. A future rate-producing constitutive owner must declare its time
unit and integrate through `dt` before entering these amount ledgers.

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
- Stage B allocation receipt with `U_s,l`, availability lineage, and one reason
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
5. **Assemble Stage B.** Hydrology evaluates all same-`tau` demands and
   competing withdrawals against the same layer snapshot. It constructs a
   candidate soil state and returns `0 <= U_s,l <= D_s,l`. On the same
   horizontal area basis, every layer must also satisfy
   `sum_s U_s,l + W_comp,l <= A_l`; no individually valid request can overbook
   the shared layer snapshot. Each request carries one enumerated reason:
   `fully_supplied`, `zero_demand`, `liquid_storage_limit`,
   `frozen_exclusion`, `rooting_exclusion`, or `competing_demand`. Invalid
   state, missing policy authority, or ambiguous priority is a typed failure,
   not a limitation reason.
6. **Assemble Stage C.** Vegetation consumes the exact allocation receipt and
   constructs a candidate vegetation state. For every stratum,
   `T_s = sum_l U_s,l`. Any constitutive feedback that cannot accept the exact
   allocation rejects the transaction; it cannot silently use less water.
7. **Join latent energy.** Land-surface energy supplies authority-tagged `h_v`
   and independently reconstructs `Q_T,s = -h_v*T_s`. Missing authority,
   lineage mismatch, or a second latent debit fails the transaction.
8. **Apply receiving-owner proposals.** Ground water/snow and dead-material
   receivers construct candidate receipts. Every transfer has one donor debit
   and one recipient credit or named atmospheric sink on the same basis.
9. **Close and commit.** Both owners independently reconstruct every shared
   transfer. Only after all water, energy, carbon, nitrogen, and material
   identities and typed receipts pass does the orchestrator atomically commit
   candidate states and expose adapter/publication candidates.
10. **Iteration if later authorized.** A successor contract must state iterate
    variables, ordering, norm, dimensional tolerances, maximum iterations, and
    failure rollback. Version 4 preserves the version-2 prohibition on
    fixed-point iteration and authorizes no
    fallback flux. Penman-Monteith is neither required nor prohibited: if a
    component selects it, that component must admit the complete equation,
    constants, units, resistance scale, domains, guards, and limiting vectors.

This sequence is implementation-authoritative for boundary transactionality,
not for any missing constitutive response.

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
| `T_s != sum_l U_s,l` | reject both candidate states | `VEG-E-030` |
| missing/mismatched `h_v` or duplicate latent debit | reject | `VEG-E-031` |
| water/energy/carbon/nitrogen/material closure fails | reject atomically | `VEG-E-032` |
| canopy-snow constitutive execution requested under versions 2-4 | reject; boundary concept only | `VEG-E-040` |
| iterative feedback requested without successor authority | reject without partial publication | `VEG-E-041` |
| compatibility adapter invoked before Stage C/receipt closure | reject | `VEG-E-050` |

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-VEGETATION-001` | Configuration, parameter sets, initial state, and evolving state are distinct versioned objects; missing physiology never defaults. | `REF-VEGETATION-001`, `REF-VEGETATION-009`, `REF-VEGETATION-011` | `[INFERENCE][Static]` | test/governance | hard `HOLD` |
| `INV-VEGETATION-002` | Exact tile topology reconstructs every stratum cover and overlap without state perturbation or implicit independence. | `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | future runtime/test | `VEG-E-002` |
| `INV-VEGETATION-003` | Top-to-bottom order is deterministic and same-rank cover closes at or below unity. | `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | future runtime/test | `VEG-E-002` |
| `INV-VEGETATION-004` | Root participation is explicit by soil layer and separately authoritative; depth alone is not a layer profile. | `REF-VEGETATION-005`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | governance/runtime | `VEG-E-060` |
| `INV-VEGETATION-010` | Stage order is A potential response, B hydrologic arbitration, C vegetation finalization, then closure/atomic commit. | `REF-VEGETATION-005`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | orchestrator test | `VEG-E-010` |
| `INV-VEGETATION-011` | Vegetation never mutates soil-layer liquid/frozen state; hydrology is sole Stage B mutator. | `REF-VEGETATION-005`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | owner guard | `VEG-E-022` |
| `INV-VEGETATION-012` | Each allocation is bounded by its request, and the sum of vegetation plus competing withdrawals is bounded by same-snapshot layer admissibility on one area basis; every request carries one reason code. | `REF-VEGETATION-005`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | hydrology/test | `VEG-E-020/021` |
| `INV-VEGETATION-013` | Actual transpiration for each stratum exactly equals its accepted layer withdrawals. | `REF-VEGETATION-005`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | dual reconstruction | `VEG-E-030` |
| `INV-VEGETATION-014` | Actual transpiration and latent energy share one transaction/stratum/lineage and one `h_v` identity. | `REF-VEGETATION-007`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | energy join | `VEG-E-031` |
| `INV-VEGETATION-015` | Failed or non-converged transactions publish and mutate nothing. | `REF-VEGETATION-007`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | atomic commit | `VEG-E-032/041` |
| `INV-VEGETATION-020` | Canopy liquid start plus interval-integrated incident water equals end storage plus interval-integrated actual evaporation and named releases. | `REF-VEGETATION-002`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[DIRECT][Static] + [INFERENCE][Static]` | dual reconstruction | `VEG-E-012/032` |
| `INV-VEGETATION-021` | Canopy, ground, litter, snow, soil, ponded-water, and atmospheric radiation/latent terms remain distinct. | `REF-VEGETATION-007`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | alias/poison test | `VEG-E-011/032` |
| `INV-VEGETATION-022` | Vegetation owns intercepted canopy snow; snow/frost owns ground snow; v1 admits no canopy-snow constitutive law, and versions 2-4 preserve that prohibition. | `REF-VEGETATION-008`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | governance | `VEG-E-040` |
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

### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-VEGETATION-001` | configuration/parameter classification assertions | test | blocked promotion | focused contract test |
| `INV-VEGETATION-002` | future topology validator | runtime | `VEG-E-002`; currently `HOLD` | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-003` | future order/cover validator | runtime | `VEG-E-002`; currently `HOLD` | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-004` | root-profile authority gate | governance | `VEG-E-060` | `GAP-VEGETATION-002` |
| `INV-VEGETATION-010` | stage-order contract assertions | test | blocked promotion | focused test + coupling artifact |
| `INV-VEGETATION-011` | owner/write-set guard | governance | `VEG-E-022` | focused test + adjacent contracts |
| `INV-VEGETATION-012` | future allocation validator | runtime | `VEG-E-020/021`; currently `HOLD` | `GAP-VEGETATION-003` |
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
| `INV-VEGETATION-052` | strict definition/schema/alias validator | runtime/test | `VEG-E-003/060`; currently `HOLD` | `GAP-VEGETATION-011/012` |
| `INV-VEGETATION-053` | local-byte identity validator | runtime/test | `VEG-E-003`; implementation missing | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-054` | raw/resolved object separation and round-trip vector | runtime/test | `VEG-E-003`; implementation missing | `GAP-VEGETATION-001/008` |
| `INV-VEGETATION-055` | schema-manifest completeness validator | runtime/test | `VEG-E-003/060`; caller values required | `GAP-VEGETATION-011/012/013` |
| `INV-VEGETATION-056` | complete dated-state identity validator | runtime/test | `VEG-E-060`; implementation missing | `GAP-VEGETATION-018/022` |
| `INV-VEGETATION-057` | site-value classification and schema-domain validator | runtime/test | `VEG-E-003/060`; implementation missing | `GAP-VEGETATION-001/012/013` |
| `INV-VEGETATION-058` | caller-state completeness/domain validator | runtime/test | `VEG-E-060`; implementation missing | `GAP-VEGETATION-018/022` |
| `INV-VEGETATION-059` | native-forest component ledger and prohibited-path guard | governance/runtime/test | `VEG-E-061`; implementation missing | `GAP-VEGETATION-004/023` |
| `INV-VEGETATION-060` | independent component reconstruction and canopy-loss poison vector | test | `VEG-E-061`; implementation missing | `GAP-VEGETATION-004/023` |
| `INV-VEGETATION-061` | fixture metadata/claim guard and layer-response vectors | test/governance | blocked promotion on overclaim | future implementation package |

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
| `D_s,l`, `U_s,l` | not aliases of legacy `UPi`, `Ui` | future layer exchange | explicit migration/cutover required | this contract / `SC-EVAP-001` |
| `E_floor,j` | not legacy PMET `Es` and not `Kcb_adjusted - Ep` | future native forest only | explicit recipient, state, resistance, and energy lineage | this contract / `SC-LANDSURFACEENERGY-001` |
| `L_DM,c` | not an alias of `L_C,c` or `L_N,c` | dead-material receipt | independent unit/stoichiometry fields | this contract / `SC-RESIDUE-001` |
| `S_snow,s` | not ground SWE or snow depth | future canopy store | `kg m^-2` water equivalent; no runtime alias | this contract / `SC-SNOWFREEZE-001` |
| `Q_rad,k,j` | not a universal ground/net-radiation scalar | energy receipt | interval-integrated `J m^-2`; recipient-specific | `SC-LANDSURFACEENERGY-001` |

## Constants and Parameters with Provenance Anchors

Version 1 admits no vegetation-process numerical constant or empirical default,
and versions 2-4 preserve that prohibition. No source default, recommended
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

The only equations admitted here are exact configuration definitions and
physical bookkeeping identities. They are not constitutive physiology.

### Definition Acquisition And Typed Schema

The schema-form portion of `AUTH-RHEC-001` and all authority requirements of
`AUTH-RHEC-016` are admitted at the authority level only. The complete selected
consumed-field manifest and aliases remain missing. This admission does not
authorize a profile value or runtime implementation.

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

No runtime symbol, registry, or output metadata changes are authorized here.

## Tolerance and Numeric Notes

- Conservation and representation tolerances are distinct.
- Version 4 preserves the version-2 rule that no numerical tolerance is
  admitted. Exact mathematical identities must
  be tested with separately authorized scale-aware floating predicates in a
  future implementation package.
- Zero snapping, negative-pool clipping, cover perturbation, conductance floors,
  denominator replacement, or unbounded iteration are prohibited absent a
  threshold, units, provenance, tests, and explicit canonical authority.
- Empty vegetation, zero leaf area, zero demand, and zero transfer are valid
  degenerates when all corresponding stores and receipts close.

## Calibration and Identifiability

```text
science_implementation_status = AUTHORITY_MISSING
calibration_evidence_status = NOT_CALIBRATION_READY
identifiability_status = NOT_ASSESSED
```

Physiological and allocation parameters will be calibration-applicable only
after their equations, validity domains, typed parameter surface, and
observation operators are independently admitted. No current parameter,
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
| same-layer aggregate overbooking | reject when `sum_s U_s,l + W_comp,l > A_l` even if each `U_s,l <= D_s,l` | `INV-VEGETATION-012`, `VEG-E-020` |
| stale transaction/layer identity | reject before any candidate commit | `VEG-E-010` |
| allocation above demand or accessible liquid | reject hydrology and vegetation candidates | `VEG-E-020` |
| all-distinct layer requests/allocations | both owners reconstruct `T_s=sum U_s,l` | `INV-VEGETATION-013`, `VEG-E-030` |
| all-distinct water/energy operands | exact `Q_T,s=-h_v*T_s`, no alias/double debit | `INV-VEGETATION-014`, `INV-VEGETATION-021` |
| canopy liquid store | independently reconstruct start + incident - evaporation - every release = end | `INV-VEGETATION-020` |
| canopy/ground/litter/snow/soil poison aliases | omitted, duplicated, or swapped recipient fails | `INV-VEGETATION-021`, `VEG-E-011/032` |
| dry matter/C/N transfer | donor and receiver reconstruct same three distinct operands | `INV-VEGETATION-030`, `INV-VEGETATION-031` |
| canopy snow request | ownership visible but constitutive execution rejected under versions 2-4 | `INV-VEGETATION-022`, `VEG-E-040` |
| unbounded/failed iteration | no partial mutation or publication | `INV-VEGETATION-015`, `VEG-E-041` |
| compatibility adapter | field-specific receipt, read-only, no native feedback | `INV-VEGETATION-040`, `INV-VEGETATION-041` |
| source-derived constant/proxy physiology | `AUTHORITY_MISSING`, `NON_PROMOTABLE` | `INV-VEGETATION-050`, `INV-VEGETATION-051` |
| distinct caller stratum values | both parse and remain separately reconstructible; no averaging or default substitution | `INV-VEGETATION-052`, `INV-VEGETATION-057`, `INV-VEGETATION-061` |
| complete caller initial state / one missing pool | accept the complete state; reject the incomplete state without synthesis | `INV-VEGETATION-056`, `INV-VEGETATION-058` |
| native-forest component poison vector | all-distinct canopy transpiration, wet-canopy evaporation, and forest-floor evaporation close independently | `INV-VEGETATION-059`, `INV-VEGETATION-060` |
| canopy-area reduction with unchanged floor operands | canopy response changes; floor evaporation is not increased by the lost canopy demand | `INV-VEGETATION-060`, `VEG-E-061` |
| two layer-root profiles under one soil snapshot | distinct layer-resolved root requests and hydrology receipts; no single-depth alias | `INV-VEGETATION-004`, `INV-VEGETATION-061` |

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

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-VEGETATION-001` | No implemented native topology/configuration/state surface exists. | Versioned schema, typed state, digest/provenance validation, and topology vectors. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-002` | No selected layer root-uptake response/remapping law exists. Site root-profile values may be caller configuration, but depth alone is not a layer profile. | Admit the response law, units, layer mapping, frozen/dry exclusions, hydrologic arbitration, closure, and tests; require callers to supply profile values. | `CONSTITUTIVE_AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-003` | Stage B competition/fairness/priority policy is unspecified. | Named hydrology policy, reason codes, admissibility, conservation, and adversarial vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-004` | Radiation, liquid interception detail, conductance, separate native-forest component demands, photosynthesis, respiration, allocation, turnover, mortality, rooting, and nutrient constitutive laws are not admitted. | Independent literature authority, complete equations, parameter classifications, units, domains, guards, ownership, and tests per family. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-005` | Canopy snow has a single-owner boundary but no admitted constitutive law or atomic amendment with snow/frost. | Independent authority plus joint vegetation/snow/LSE contract and mass-energy vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-006` | Elemental/dead-material transfer classes and receiving biogeochemistry are incomplete. | Material taxonomy, stoichiometry authority, exact receiver contracts, and closure tests. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-007` | Every compatibility reduction except exact tile-union cover lacks reviewed operator/cutover evidence. | Field-specific reductions, unit helpers, real consumers, negative old-path proof. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-008` | No vegetation crate, scheduler transaction, typed failures, registry entries, fixtures, output, or real consumer exists. | Scoped implementation packages and direct consumer evidence. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-009` | Calibration/identifiability authority and independent observations are absent. | Prospective data roles, observation operators, readiness analysis, calibration, and held-out validation. | `NOT_CALIBRATION_READY`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-010` | The earlier repository-license gap is closed only for the two pinned Laurence Lin repositories in `REF-VEGETATION-012/013`; the separate official RHESSys repository remains outside this route. | Preserve exact commit/file lineage and the MIT notice for distributed source-derived material. Licensing never substitutes for scientific authority; historical `DIRECT_TRANSLATION_PROHIBITED` remains applicable outside the admitted pinned route. | `LICENSE_ADMITTED`, `SCIENCE_AUTHORITY_UNCHANGED` |
| `GAP-VEGETATION-011` | Pinned GIS definitions contain five keys that do not match the pinned C parser: SLA, all-sided LAI ratio, both VPD thresholds, and mortality. | A versioned explicit alias/correction decision, raw-value preservation, strict vectors, and no legacy hidden-default behavior. | `FORMAT_MISMATCH`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-012` | The parser reads 53 parameters absent from all 32 GIS profiles and silently supplies defaults. | Enumerate every consumed dependency in the typed schema with units, basis, parameter class, finite domain, and missing-value failure; require callers to supply every site value. | `SCHEMA_INCOMPLETE`, `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-013` | The minimum generic and East-Coast deciduous/evergreen profile candidates have no cell-level source, calibration-domain, or transferability map. | Do not distribute or recommend them as defaults. Permit explicit caller values after schema/domain validation, preserve stratum identity, and label demonstration fixtures `ASSUMED_FOR_EXECUTION`. | `CALLER_CONFIGURATION`, `DEFAULT_AND_TRANSFERABILITY_CLAIM_PROHIBITED` |
| `GAP-VEGETATION-014` | The audited aerodynamic and Jarvis conductance chain has unresolved equation/scale/domain authority, unsupported Tmin/CO2/Tavg branches, sentinels, and a nonzero floor. | Select and admit the law and leaf/canopy scale; classify site thresholds as caller values where appropriate; reject source sentinels/floors and supply independent vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-015` | The audited Farquhar path recognizes core C3 equations but hardcodes C3 for every profile and lacks complete authority for capacity constants, canopy scaling, and fixed growth-respiration iteration. | Contract the selected C3 route and parameters; explicitly exclude C4 profiles or separately admit a C4 route; define convergence/failure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-016` | Source root demand is a single-depth saturated/unsaturated split with direct patch-store coupling rather than explicit layer requests and hydrology-authorized withdrawals. | Admit the root-response and layer-mapping laws; accept caller-supplied layer-profile values; specify dry/frozen/zero-participation branches, Stage B policy, and dual closure vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-017` | The audited available-energy chain uses a homogeneous air-temperature canopy slab, deletes warm-period negative net longwave, applies dimensionally inconsistent surface-heat storage branches, and contains an erroneous day/night negative-energy assignment. | Admit distinct canopy/ground longwave and storage-heat owners, units, scale, sign and condensation policies, component ledger, and independent limiting/closure vectors before any Penman-Monteith use. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-018` | The audited worldfile generators construct initial C/N pools and root depth with fixed row indices, unproven ratios/constants, contradictory deadwood C:N rules, and an SLA identity that diverges from the runtime parser. | Reject those synthesis paths. Implement complete, versioned, dated caller-state ingestion with exact profile/key identity, area basis, domains, finite guards, and independent mass/LAI reconstruction. | `CALLER_STATE_REQUIRED`, `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-019` | The audited Penman-Monteith routine omits the water/air molecular-mass ratio from the psychrometric constant, despite defining it and using the correct factor in another source routine. | Never port the defective expression. Penman-Monteith is neither required nor prohibited; any component selecting it must independently admit the complete equation, constants, units, resistance scale, phase/enthalpy, guards, and limiting vectors. | `SOURCE_ROUTINE_REJECTED`, `CONSTITUTIVE_AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-020` | Strict local-byte acquisition authority is now admitted by `INV-VEGETATION-053/054`, but no runtime validator implements it. The audited generator still fetches mutable raw `master` parameter collections and is prohibited. | Implement the exact tuple/digest checks and negative vectors without importing the audited fallback path. | `AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-021` | The audited canopy path ignores parsed absorptance/transmittance and diffuse extinction, while nine profile optical triples fail exact unit closure. | Contract an authoritative component operator and error policy with all operands consumed exactly once; preserve raw invalid values but never silently normalize them. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-022` | The inspected Coweeta evidence does not jointly observe every required C/N/root/geometry pool on one compatible state surface. | This does not block caller-supplied state. Require a complete caller state for execution; require observation operators, uncertainty, and compatible measurements only before an empirical calibration, validation, or transferability claim. | `CALLER_STATE_REQUIRED`, `EMPIRICAL_CLAIM_NOT_READY` |
| `GAP-VEGETATION-023` | Existing agricultural PMET couples canopy and soil demand through complementary LAI factors, so canopy-demand loss is structurally donated to soil evaporation; the Stevens Canyon investigation found parameter search and legacy-ET ablation insufficient. | Admit and implement separately owned canopy transpiration, wet-canopy evaporation, forest-floor evaporation, and layer-root response equations with independent operands, resistance/energy lineage, poison vectors, and closure. | `NATIVE_FOREST_PMET_PARTITION_PROHIBITED`, `AUTHORITY_MISSING`, `NON_PROMOTABLE` |

The first safe successor is an authority-and-typed-boundary slice for topology,
caller configuration/state, radiation/interception/conductance inputs,
independent native-forest flux components, and layer-resolved potential demand.
It must independently admit every implemented constitutive relationship, remain
default-off and non-publishing, mutate no soil store, and make no runtime or
cutover claim.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-08 | 2 | Codex | Admitted the exact licensed-source provenance boundary without promoting source science; added strict-definition invariant `INV-VEGETATION-052` and audit-proven format, hidden-default, parameter, conductance, photosynthesis, root-demand, available-energy, and initialization gaps. |
| 2026-08-08 | 3 | Codex | Admitted strict caller-supplied local acquisition, immutable raw/resolved separation, typed schema-form requirements, and dated initial-state identity; retained every selected value, alias, initializer, constitutive, implementation, and cutover gap. |
| 2026-08-09 | 4 | Codex | Reclassified site-specific values and complete compatible state as caller configuration, bounded demonstration claims, prohibited agricultural PMET redistribution in the native-forest target, and required independent canopy/wet-canopy/forest-floor/root component closure. |
| 2026-08-08 | 1 | Codex | Initial native-stratum, Stage A/B/C, ownership, transaction, conservation, compatibility, firewall, and non-promotable-gap authority. |
