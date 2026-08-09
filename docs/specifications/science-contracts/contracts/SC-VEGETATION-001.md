---
contract_id: SC-VEGETATION-001
title: Native Vegetation State and Cross-Domain Boundary Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + forest ecohydrology/hydrology reviewer
contract_version: 2
producer_scope:
  - Native vegetation configuration/runtime separation and stratum topology
  - Stage A potential response and Stage C vegetation finalization boundaries
  - Vegetation-owned canopy stores, elemental state, and transfer proposals
  - Read-only aggregate compatibility reduction
consumer_scope:
  - Native management, land-surface energy, soil hydrology, snow/frost, residue/biogeochemistry, and hillslope orchestration
evidence_level: static
last_reviewed: 2026-08-08
supersedes: []
superseded_by: []
---

# SC-VEGETATION-001 Native Vegetation State and Cross-Domain Boundary Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define canonical openWEPP ownership, state, ordering, units, conservation, and
failure semantics for a future native vegetation subsystem. Version 2 admits
typed boundary architecture only. It admits no vegetation kernel,
physiological formula, empirical parameter value, runtime selector, management
schema, compatibility cutover, output, calibration, or default.

## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope:

- native stratum identity, explicit horizontal support, vertical overlap, and
  deterministic ordering;
- immutable configuration and initial-state references distinct from evolving
  vegetation state;
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
  RHESSysEastCoast behavior and GIS2RHESSys profile values;
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
| `S_snow,s` | `kg m^-2` | future intercepted canopy-snow water-equivalent store | vegetation; constitutive behavior non-promotable in version 2 |
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
   transfer proposals, and sufficient operands for independent reconstruction.
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
    failure rollback. Version 2 authorizes no fixed-point iteration and no
    fallback flux.

This sequence is implementation-authoritative for boundary transactionality,
not for any missing constitutive response.

## Branch and Guard Table

| Condition | Required disposition | Failure |
|---|---|---|
| empty stand with zero vegetation stores | valid degenerate; zero vegetation demands/transfers | none |
| invalid/ambiguous topology, units, digest, or parameter class | reject before Stage A | `VEG-E-001/002/003` |
| missing constitutive authority for a requested process | reject; do not substitute proxy physics | `VEG-E-060` |
| stale/mismatched `tau`, state, area, or layer identity | reject before mutation | `VEG-E-010` |
| missing/duplicate shared lineage | reject | `VEG-E-011` |
| Stage A canopy/radiation proposal fails closure | reject | `VEG-E-012` |
| allocation is negative, above demand, or above admissible liquid | reject candidate hydrology state | `VEG-E-020` |
| missing/invalid limitation reason or allocation policy | reject | `VEG-E-021` |
| vegetation attempts soil/frozen-store mutation | reject | `VEG-E-022` |
| `T_s != sum_l U_s,l` | reject both candidate states | `VEG-E-030` |
| missing/mismatched `h_v` or duplicate latent debit | reject | `VEG-E-031` |
| water/energy/carbon/nitrogen/material closure fails | reject atomically | `VEG-E-032` |
| canopy-snow constitutive execution requested under version 2 | reject; boundary concept only | `VEG-E-040` |
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
| `INV-VEGETATION-022` | Vegetation owns intercepted canopy snow; snow/frost owns ground snow; v1 admits no canopy-snow constitutive law, and version 2 preserves that prohibition. | `REF-VEGETATION-008`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | governance | `VEG-E-040` |
| `INV-VEGETATION-030` | Live/standing-dead plant pools remain vegetation-owned until an accepted exact-once material/element transfer. | `REF-VEGETATION-006`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | receipt test | `VEG-E-032` |
| `INV-VEGETATION-031` | Vegetation and residue/biogeochemistry independently reconstruct identical dry-matter, carbon, and nitrogen transfers. | `REF-VEGETATION-006`, `REF-VEGETATION-009`, `REF-VEGETATION-010` | `[INFERENCE][Static]` | dual reconstruction | `VEG-E-032` |
| `INV-VEGETATION-040` | Every compatibility field has an explicit reduction, area basis, unit conversion, missing-state rule, and contributing-strata receipt. | `REF-VEGETATION-004`, `REF-VEGETATION-009` | `[DIRECT][Static] + [INFERENCE][Static]` | adapter test | `VEG-E-050` |
| `INV-VEGETATION-041` | The adapter is read-only, never feeds native state, and cannot support cutover without real downstream consumption. | `REF-VEGETATION-004`, `REF-VEGETATION-011` | `[DIRECT][Static]` | consumer gate | hard `HOLD` |
| `INV-VEGETATION-050` | RHESSys behavior, source-reported citations, and comparator agreement cannot authorize equations, constants, bounds, or defaults. | `REF-VEGETATION-009`, `REF-VEGETATION-011` | `[DIRECT][Static]` | firewall/review | hard `HOLD` |
| `INV-VEGETATION-051` | No production implementation is promotable while a requested process is `AUTHORITY_MISSING`. | `REF-VEGETATION-001` through `REF-VEGETATION-014` | `[INFERENCE][Static]` | gap gate | `NON_PROMOTABLE` |
| `INV-VEGETATION-052` | A compatible vegetation definition preserves every exact input key/value and provenance identity, but selected runtime parameters use a versioned typed schema with explicit aliases; absent, duplicate, non-finite, invalid, or unsupported-sentinel values never receive hidden defaults. | `REF-VEGETATION-012`, `REF-VEGETATION-013`, `REF-VEGETATION-014` | `[DIRECT][Static] + [INFERENCE][Static]` | schema/runtime/test | `VEG-E-003/060` |

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
| `L_DM,c` | not an alias of `L_C,c` or `L_N,c` | dead-material receipt | independent unit/stoichiometry fields | this contract / `SC-RESIDUE-001` |
| `S_snow,s` | not ground SWE or snow depth | future canopy store | `kg m^-2` water equivalent; no runtime alias | this contract / `SC-SNOWFREEZE-001` |
| `Q_rad,k,j` | not a universal ground/net-radiation scalar | energy receipt | interval-integrated `J m^-2`; recipient-specific | `SC-LANDSURFACEENERGY-001` |

## Constants and Parameters with Provenance Anchors

Version 1 admits no vegetation-process numerical constant, empirical default,
and version 2 preserves that prohibition. No source default, profile value,
physiological bound, or parameter set. Every later parameter entry must be one
of `fixed_science`, `calibratable`, `external_configuration`, or
`initial_state`; carry units, validity domain, evidence bounds, version,
SHA-256, authority, ecosystem applicability, and prohibited extrapolations; and
distinguish any `ASSUMED_FOR_EXECUTION` value from science or calibration.

The only equations admitted here are exact configuration definitions and
physical bookkeeping identities. They are not constitutive physiology.

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
- Version 2 admits no numerical tolerance. Exact mathematical identities must
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
| canopy snow request | ownership visible but constitutive execution rejected under version 2 | `INV-VEGETATION-022`, `VEG-E-040` |
| unbounded/failed iteration | no partial mutation or publication | `INV-VEGETATION-015`, `VEG-E-041` |
| compatibility adapter | field-specific receipt, read-only, no native feedback | `INV-VEGETATION-040`, `INV-VEGETATION-041` |
| source-derived constant/proxy physiology | `AUTHORITY_MISSING`, `NON_PROMOTABLE` | `INV-VEGETATION-050`, `INV-VEGETATION-051` |

Future fixtures must use deliberately distinct canopy, ground, litter, snow,
soil, ponded-water, layer, dry-matter, carbon, and nitrogen operands so wrong
aliases cannot equal the expected result. Producer self-consistency alone is
insufficient; both owners reconstruct from independent state/output surfaces.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-VEGETATION-001` | Version 1 native vegetation boundary admission | `active` | `maps-to-existing-INV` | `INV-VEGETATION-001, INV-VEGETATION-010, INV-VEGETATION-011, INV-VEGETATION-013, INV-VEGETATION-014, INV-VEGETATION-022, INV-VEGETATION-041, INV-VEGETATION-050, INV-VEGETATION-051` | `flagged-binding-addition` | Initial authority is consolidated in this contract; package artifacts remain evidence rather than separate binding authority. |
| `BEI-VEGETATION-002` | `20260808-rhessys-east-coast-code-literature-authority-audit-001` audit sidecar population | `active` | `maps-to-existing-INV` | `INV-VEGETATION-050, INV-VEGETATION-051, INV-VEGETATION-052` | `flagged-binding-addition` | Version 2 admits licensed provenance and a strict definition/schema obligation only; the audit's constitutive findings remain explicit gaps and require the package's dual review/disposition/verification cycle. |

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-VEGETATION-001` | No implemented native topology/configuration/state surface exists. | Versioned schema, typed state, digest/provenance validation, and topology vectors. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-002` | No independently authorized layer root profile or dynamic remapping law exists. | Literature/observed authority, units, layers, frozen exclusions, remapping closure, tests. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-003` | Stage B competition/fairness/priority policy is unspecified. | Named hydrology policy, reason codes, admissibility, conservation, and adversarial vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-004` | Radiation, liquid interception detail, conductance, transpiration demand, photosynthesis, respiration, allocation, turnover, mortality, rooting, and nutrient constitutive laws are not admitted. | Independent literature authority, domains, parameters, units, guards, and tests per family. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-005` | Canopy snow has a single-owner boundary but no admitted constitutive law or atomic amendment with snow/frost. | Independent authority plus joint vegetation/snow/LSE contract and mass-energy vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-006` | Elemental/dead-material transfer classes and receiving biogeochemistry are incomplete. | Material taxonomy, stoichiometry authority, exact receiver contracts, and closure tests. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-007` | Every compatibility reduction except exact tile-union cover lacks reviewed operator/cutover evidence. | Field-specific reductions, unit helpers, real consumers, negative old-path proof. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-008` | No vegetation crate, scheduler transaction, typed failures, registry entries, fixtures, output, or real consumer exists. | Scoped implementation packages and direct consumer evidence. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-009` | Calibration/identifiability authority and independent observations are absent. | Prospective data roles, observation operators, readiness analysis, calibration, and held-out validation. | `NOT_CALIBRATION_READY`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-010` | The earlier repository-license gap is closed only for the two pinned Laurence Lin repositories in `REF-VEGETATION-012/013`; the separate official RHESSys repository remains outside this route. | Preserve exact commit/file lineage and the MIT notice for distributed source-derived material. Licensing never substitutes for scientific authority; historical `DIRECT_TRANSLATION_PROHIBITED` remains applicable outside the admitted pinned route. | `LICENSE_ADMITTED`, `SCIENCE_AUTHORITY_UNCHANGED` |
| `GAP-VEGETATION-011` | Pinned GIS definitions contain five keys that do not match the pinned C parser: SLA, all-sided LAI ratio, both VPD thresholds, and mortality. | A versioned explicit alias/correction decision, raw-value preservation, strict vectors, and no legacy hidden-default behavior. | `FORMAT_MISMATCH`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-012` | The parser reads 53 parameters absent from all 32 GIS profiles and silently supplies defaults. | Enumerate every selected dependency in the typed schema with units, domain, authority, and missing-value failure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-013` | The minimum generic and East-Coast deciduous/evergreen profile candidates have no cell-level source, calibration-domain, or transferability map. | Admit every selected value independently or replace it with an authority-backed value; do not average profiles into a mixed default. | `PARAMETER_AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-014` | The audited aerodynamic and Jarvis conductance chain has unresolved scale/domain authority, unsupported Tmin/CO2/Tavg branches, sentinels, and a nonzero floor. | Select leaf/canopy scale and domain; admit or reject every factor, threshold, sentinel, and floor; supply independent vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-015` | The audited Farquhar path recognizes core C3 equations but hardcodes C3 for every profile and lacks complete authority for capacity constants, canopy scaling, and fixed growth-respiration iteration. | Contract the selected C3 route and parameters; explicitly exclude C4 profiles or separately admit a C4 route; define convergence/failure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-016` | Source root demand is a single-depth saturated/unsaturated split with direct patch-store coupling rather than explicit layer requests and hydrology-authorized withdrawals. | Layer-profile authority, observation mapping, dry/frozen and zero-participation branches, Stage B policy, and dual closure vectors. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-017` | The audited available-energy chain uses a homogeneous air-temperature canopy slab, deletes warm-period negative net longwave, applies dimensionally inconsistent surface-heat storage branches, and contains an erroneous day/night negative-energy assignment. | Admit distinct canopy/ground longwave and storage-heat owners, units, scale, sign and condensation policies, component ledger, and independent limiting/closure vectors before any Penman-Monteith use. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-018` | The audited worldfile generators construct initial C/N pools and root depth with fixed row indices, unproven ratios/constants, contradictory deadwood C:N rules, and an SLA identity that diverges from the runtime parser. | Define a versioned, dated, typed initializer with exact profile/key identity, area basis, admitted equations/values/domains, finite guards, and independent mass/LAI reconstruction. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-019` | The audited Penman-Monteith routine omits the water/air molecular-mass ratio from the psychrometric constant, despite defining it and using the correct factor in another source routine. | Independently re-derive the complete PM equation, constants, units, resistance scale, phase/enthalpy, and limiting vectors; never port the defective expression. | `SILENT_DEVIATION`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-020` | The audited generator can fetch mutable raw `master` parameter collections, bypassing the pinned evidence identity. | Require explicit local bytes with repository, commit, path, and content digest; prohibit network fallback in compatibility/runtime paths. | `PROVENANCE_VIOLATION`, `NON_PROMOTABLE` |
| `GAP-VEGETATION-021` | The audited canopy path ignores parsed absorptance/transmittance and diffuse extinction, while nine profile optical triples fail exact unit closure. | Contract an authoritative component operator and error policy with all operands consumed exactly once; preserve raw invalid values but never silently normalize them. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |

The first safe successor is an authority-and-typed-boundary slice for topology,
radiation/interception/conductance inputs and layer-resolved potential demand.
It must independently admit every implemented constitutive relationship, remain
default-off and non-publishing, mutate no soil store, and make no runtime or
cutover claim.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-08 | 2 | Codex | Admitted the exact licensed-source provenance boundary without promoting source science; added strict-definition invariant `INV-VEGETATION-052` and audit-proven format, hidden-default, parameter, conductance, photosynthesis, root-demand, available-energy, and initialization gaps. |
| 2026-08-08 | 1 | Codex | Initial native-stratum, Stage A/B/C, ownership, transaction, conservation, compatibility, firewall, and non-promotable-gap authority. |
