# RHESSys-Derived Vegetation Crate

## Status

- `state`: **blocked; integrated authority admission executed-hold** -
  source-aware implementation remains closed on residual
  `AUTH-RHEC-001..011` plus `AUTH-RHEC-014/015`; `AUTH-RHEC-016` authority is
  admitted but its implementation/tests remain a successor obligation; default
  activation remains prohibited
- `date`: 2026-08-06
- `owner`: openWEPP maintainers + forest ecohydrology reviewer
- `working crate name`: `openwepp-vegetation` (the RHESSys lineage is
  provenance, not necessarily the permanent public crate name)
- `origin`: static comparison of openWEPP, WEPP-forest, and RHESSys vegetation,
  ET, radiation, litter, snow, hydrology, and biogeochemistry implementations
- `implementation release trigger`: close every required audit blocker through
  reviewed canonical authority, prospectively reconcile the coupled package,
  and pass its contract-first gate
- `default eligibility`: prohibited until conservation, field-evaluation,
  coupled-response, and real-consumer gates pass
- `completed authority package`:
  [VEGETATION-SOURCE-PROVENANCE-AND-BOUNDARY-AUTHORITY](../work-packages/20260808-vegetation-source-provenance-and-boundary-authority-001/package.md)
- `superseded research package`:
  [VEGETATION-RADIATION-INTERCEPTION-CONDUCTANCE-SLICE](../work-packages/20260808-vegetation-radiation-interception-conductance-slice-001/package.md)
- `completed authority-audit precursor`:
  [RHESSYS-EAST-COAST-CODE-LITERATURE-AUTHORITY-AUDIT](../work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/package.md)
- `executed-hold authority-admission package`:
  [RHESSYS-EAST-COAST-VEGETATION-AUTHORITY-ADMISSION](../work-packages/20260808-rhessys-east-coast-vegetation-authority-admission-001/package.md)
- `queued implementation package`:
  [RHESSYS-EAST-COAST-COUPLED-VEGETATION-SLICE](../work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md)

## Summary

Develop a separately testable Rust vegetation subsystem informed by RHESSys
ecosystem physics. The subsystem would represent one or more canopy strata and
own their coupled phenology, radiation, interception, conductance,
transpiration, photosynthesis, respiration, allocation, turnover, and litter
production. It would not own openWEPP soil-water storage, infiltration, runoff,
percolation, erosion, or watershed routing.

The central design problem is not translating isolated equations. RHESSys gains
much of its vegetation fidelity from coupling energy, water, carbon, nitrogen,
phenology, and vertically organized canopy state. The openWEPP implementation
must preserve those process relationships while establishing a narrow,
conservative boundary with the existing hillslope hydrology.

This concept broadens the
[native-vegetation ET process-model backlog](20260803-native-vegetation-et-process-model.md).
Native ET remains a primary motivation, but it should be delivered as part of a
coherent vegetation state machine rather than as another stand-alone
demand-partition formula.

## Why This Exists

openWEPP now has strong typed architecture, native forest management,
generalized-GSI canopy phenology, foliar-mass and litter closure, soil hydrology,
snow/frost physics, and real-consumer verification. Its forest vegetation
physics nevertheless remains largely an adaptation of an agricultural plant
model.

RHESSys already demonstrates a higher-fidelity ecosystem organization:

- explicit canopy strata with fractional patch coverage;
- vertically ordered radiation and aerodynamic transfer;
- wet-canopy evaporation, transpiration, and soil evaporation as distinct
  fluxes;
- stomatal conductance driven by radiation, vapor-pressure deficit,
  temperature, leaf water potential, and atmospheric CO2, with shared canopy
  state feeding both water flux and photosynthesis;
- layer-aware root water demand and realized uptake;
- evergreen and deciduous phenology with leaf and fine-root allocation and
  turnover;
- carbon and nitrogen pools connecting vegetation, litter, coarse woody debris,
  and soil biogeochemistry;
- daily ecosystem state that persists independently of management-file
  compatibility conventions.

The opportunity is to adopt the useful ecosystem-process architecture and
adjudicated kernels while retaining openWEPP's typed boundaries, conservation
ledgers, tests, native input authority, hillslope hydrology, erosion, routing,
and operational feedback through WEPPcloud.

## Source And Licensing Posture

The selected implementation and compatibility sources are pinned:

| Role | Repository and local checkout | Commit | License evidence |
|---|---|---|---|
| Coupled vegetation implementation provenance | `https://github.com/laurencelin/RHESSysEastCoast`; `/workdir/RHESSysEastCoast` | `375c75b1cd2202217651dff43aa113d80b9c1118` | MIT `LICENSE`, Laurence Lin (2021); SHA-256 `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` |
| Vegetation-profile and file-generation compatibility | `https://github.com/laurencelin/GIS2RHESSys`; `/workdir/GIS2RHESSys` | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | MIT `LICENSE`, Laurence Lin (2021); same SHA-256 |

These grants permit source inspection, adaptation, translation, tests, and
redistribution subject to preservation of the MIT copyright and permission
notice. The successor must vendor or otherwise preserve the notice with any
source-derived material it distributes and record exact file/function lineage.

The separate official checkout formerly inspected at commit
`f9d1bbf8d161aa55b6a51061dc320188ead44962` still lacks a repository license.
It is not interchangeable with the two licensed forks and remains outside the
direct-translation route unless separately licensed.

## Licensed Source-Aware Migration Protocol

The former sanitized clean-room protocol is superseded for the two pinned MIT
repositories. Source-aware implementation is allowed, but licensing does not
turn source behavior into scientific truth. For every migrated surface:

1. Record the exact repository, commit, file, function or data row, and license.
2. Classify its role as `IMPLEMENTATION_PROVENANCE`, `FORMAT_COMPATIBILITY`,
   `PARAMETER_DATA`, `COMPARATOR`, or `SCIENTIFIC_AUTHORITY`.
3. Trace cited literature and physical/conservation invariants for constitutive
   equations and parameter domains; label gaps rather than inventing authority.
4. Reconcile the behavior with `SC-VEGETATION-001` ownership and typed error
   rules before production code.
5. Preserve intentional compatibility while rejecting accidental C-source
   artifacts such as sentinels, implicit defaults, unexplained conductance
   floors, commented experimental branches, and silent canonicalization.
6. Use source differential vectors as comparator evidence, never as the sole
   production-promotion criterion.

This path is source-aware and auditable, not a line-for-line port obligation.
Rust structure may follow openWEPP conventions while behavior and deviations
remain traceable.

## Existing RHESSys Vegetation-File Compatibility

`GIS2RHESSys/vegCollection.csv` is the initial compatibility corpus: 71 fields
across 32 profiles, including generic evergreen and deciduous vegetation plus
East Coast deciduous, evergreen, shrub, and grass taxa. It contains phenology,
optical, interception, aerodynamic, stomatal, hydraulic-stress,
photosynthesis, allocation, turnover, and rooting operands used to generate
`stratum_*.def` files.

The compatibility target is both generated `stratum_*.def` files and the
collection table from the pinned commit. Parsing must be strict and
provenance-preserving: unknown fields may be retained diagnostically, but a
missing required field, duplicate key, invalid unit/domain, or unsupported
sentinel must not silently receive a RHESSys parser default.

Mixed forest is a composition, not a synthetic average profile. Represent it
with two or more explicit strata or cohorts carrying their own parameter-set
identity, cover fraction, LAI/state, height/vertical order, and root profile.
Acceptance must include deciduous-only, evergreen-only, and mixed
deciduous-evergreen cases with overlapping vertical layers.

## Existing Native-Management Fit

The canonical native producer document is already close to the required
configuration boundary:

```text
format: openwepp-management-yaml
datver: ow-lanuse-1
landuse: native_forest
```

It currently supplies:

- aggregate initial canopy cover (`cancov`);
- aggregate initial interrill and rill ground cover (`inrcov`, `rilcov`);
- stand-level generalized-GSI phenology, summer foliar biomass, evergreen
  fraction, structural canopy cover, and structural biomass;
- grass, shrub, and tree community records containing projected-area
  coefficient, canopy diameter, height, and belt-transect population;
- initial root and standing-residue mass;
- decomposition and routing operands;
- scheduled stand identity and management dates.

The grass/shrub/tree community records are parsed and retained but are not
currently consumed by the production plant-community model. They do not carry
an explicit RHESSys-compatible `cover_fraction`. A legacy WEPP horizontal-cover
bridge can be reconstructed as:

```text
cover = min(diameter * projected_area_coefficient * population, 100) / 100
```

That relationship is useful migration evidence, but it is not sufficient
native stratum authority. It assumes legacy geometry and does not resolve
vertical overlap, species identity, per-stratum phenology, leaf area, rooting,
physiology, or carbon/nitrogen initial state.

## Proposed Native Stratum Surface

Extend a later native-management schema with an explicit, versioned vegetation
block. The exact fields require science-contract adjudication; the conceptual
shape is:

```yaml
vegetation:
  model: rhessys_east_coast_v1
  strata:
    - id: overstory
      vertical_layer: 1
      lifeform: tree
      parameter_set: gis2rhessys:chestnut_oak
      cover_fraction: 0.72
      height_m: 18.0
      initial_state: northern_hardwood_mature_v1
      rooting_profile: hardwood_deep_v1
    - id: understory
      vertical_layer: 2
      lifeform: shrub
      parameter_set: gis2rhessys:rhododendron
      cover_fraction: 0.35
      height_m: 1.5
      initial_state: shrub_understory_v1
      rooting_profile: shrub_shallow_v1
```

Required semantics:

- `cover_fraction` is explicit for every stratum and has defined patch-area
  meaning.
- Cover closure is defined within each vertical layer; cover across vertically
  overlapping layers is not rejected merely because its sum exceeds one.
- Stratum ordering and radiation transmission are deterministic.
- Parameter sets and initial-state sets are versioned, checksummed, and carry
  scientific provenance.
- Stand-level aggregate cover, LAI, biomass, and rooting cannot silently
  override inconsistent stratum state.
- Legacy community geometry may be converted only by an explicit migration
  tool that records its equation, assumptions, and source values.
- Missing required physiology or state fails closed. The runtime does not fill
  gaps from hidden RHESSys defaults.
- A mixed stand retains each component profile and stratum; the loader does not
  average deciduous and evergreen parameter rows into a fabricated default.

## Ownership Boundary

| Owner | State and processes |
|---|---|
| Native management | Stand identity, explicit strata and coverage, parameter-set references, initial vegetation pools, disturbances, and management events. |
| Vegetation crate | Per-stratum canopy state; radiation transmission; interception storage; conductance; transpiration demand; photosynthesis and respiration; phenology; allocation; mortality and turnover; litter/CWD production; optional vegetation nutrient demand. |
| openWEPP soil hydrology | Soil-layer liquid/frozen water, matric state, infiltration, redistribution, percolation, drainage, lateral flow, runoff, and admissible layer withdrawals. |
| Snow/frost subsystem | Ground snowpack and soil freeze/thaw. Canopy snow ownership must be adjudicated once, not duplicated between crates. |
| Hillslope orchestrator | Forcing delivery, phase ordering, water-allocation arbitration, typed error propagation, conservation reconciliation, and publication. |
| Residue/biogeochemistry consumers | Ground litter/residue and soil C/N pools after an explicit transfer from vegetation; no duplicate pool ownership. |

The subsystem must never mutate hydrologic soil state directly. It requests
withdrawals; the hydrology owner authorizes and applies them.

## Synthetic Soil-Water Coupling

Use a two-stage daily or subdaily protocol so the crate can be developed and
tested independently without breaking the feedback between vegetation and
hydrology.

### Stage A: potential ecosystem response

Inputs include meteorological forcing, above-canopy radiation, atmospheric
state, current canopy stores, soil-layer temperature and water-potential
observations, frozen-water accessibility, and the beginning-of-step vegetation
state.

The crate returns:

- interception, throughfall, stemflow, drainage, and canopy evaporation;
- radiation transmitted to lower strata and the ground;
- potential transpiration and root-water demand by stratum and soil layer;
- potential nutrient demand when the nutrient extension is active;
- sufficient diagnostics to reconstruct every request.

### Stage B: hydrologic arbitration

openWEPP resolves simultaneous soil evaporation, vegetation uptake,
percolation, drainage, lateral flow, frozen-water exclusion, and storage bounds.
It returns the authorized water withdrawal by stratum and soil layer plus
reason-coded limitations.

### Stage C: vegetation finalization

The crate consumes the authorized withdrawal and finalizes:

- actual transpiration and conductance;
- photosynthesis, respiration, and water stress;
- carbon allocation and pool changes;
- leaf/root turnover, mortality, litter, and coarse-woody-debris transfers;
- exact water, carbon, nitrogen, and energy residuals for the owned state.

If conductance, photosynthesis, and water supply require iteration, the
iteration protocol, convergence criterion, maximum iterations, and failure
behavior must be part of the contract. Non-convergence must not be hidden by a
fallback flux.

## Compatibility Reduction Surface

Existing openWEPP consumers should initially receive a typed reduction of the
richer stratum state:

- aggregate live LAI and woody area;
- aggregate canopy cover under an explicit overlap law;
- effective canopy height and aerodynamic geometry;
- canopy rainfall and snow interception terms;
- actual transpiration, canopy evaporation, litter evaporation, and soil
  evaporation;
- layer-resolved root withdrawals and aggregate `Ep` compatibility output;
- leaf-on allocation and leaf-off litter transfer;
- litter mass, cover, moisture, and residue-depth inputs;
- vegetation roughness/drag operands needed by overland-flow routing;
- diagnostics identifying the contributing strata.

The reduction must be a named adapter, not a second vegetation model. Current
generalized-GSI phenology remains active authority until a real downstream run
is proven to consume the new crate. Cutover must retire duplicate ownership of
the same canopy, litter, and ET states.

## Candidate Process Scope

### Initial crate scope

- typed stratum and stand state;
- canopy layering and shortwave/longwave transmission;
- rainfall interception and wet-canopy evaporation;
- stomatal and aerodynamic conductance;
- layer-resolved transpiration demand and realized uptake reconciliation;
- evergreen/deciduous phenology;
- photosynthesis, maintenance respiration, and growth respiration;
- carbon allocation and leaf/fine-root turnover;
- litter and coarse-woody-debris transfer ledgers;
- aggregate compatibility adapter and complete diagnostics.

### Deferred extensions

- full soil carbon and nitrogen mineralization/immobilization;
- nitrification, denitrification, and dissolved nutrient transport;
- dynamic competition, recruitment, and species succession;
- disturbance recovery and mortality beyond explicitly authorized events;
- canopy snow interception until ownership is reconciled with the existing
  snow backlog and `SC-SNOWFREEZE-001`;
- fire combustion/emissions and live-fire behavior;
- watershed-scale lateral nutrient routing.

Deferral must preserve extension points but must not introduce placeholder
physics into active paths.

## Provenance and Kernel Selection

Before code authoring, build a function-level RHESSysEastCoast process inventory
and a field-level GIS2RHESSys profile inventory. For
each candidate kernel record:

- repository, file/function or CSV field/profile, and pinned commit;
- scientific citation or lack of one;
- inputs, outputs, units, state mutation, and call ordering;
- dependencies on patch, zone, stratum, soil, carbon, nitrogen, and snow state;
- known code defects, ambiguous branches, and numerical safeguards;
- applicable openWEPP contract and existing consumer;
- disposition: adopt, re-derive from literature, adapt, reject, or defer;
- licensing disposition and required MIT notice custody.

RHESSys behavior is evidence, not automatic scientific authority. Static
agreement with RHESSys is a comparator signal. Production promotion requires
literature, physical invariants, conservation, observed behavior, or an
otherwise adjudicated authority appropriate to the claim.

## Validation Strategy

### Crate-local gates

- Unit and property tests for every selected equation and branch.
- Dimensional and domain tests for every public boundary.
- Water, energy, carbon, and nitrogen ledger closure at each step.
- Limiting cases: bare ground, zero LAI, full evergreen, full deciduous,
  leaf-on/off transition, saturated/dry/frozen soil, zero radiation, extreme
  VPD, and overlapping strata.
- Golden vectors independently reconstructed from cited equations.
- Static RHESSys differential vectors where licensing permits execution and
  comparison; differences are classified, not blindly eliminated.

### Coupling gates

- Demand cannot withdraw water that hydrology did not authorize.
- Every actual flux withdraws from exactly one named store.
- Soil and vegetation independently reconstruct the shared withdrawal ledger.
- Radiation and latent-energy allocation are not double counted across canopy,
  snow, litter, and soil surfaces.
- Leaf and root turnover appear exactly once in receiving litter/C/N pools.
- Existing erosion, frost, snow, routing, and output consumers read the new
  compatibility adapter in a real run before any cutover claim.

### Empirical gates

Admit multiple independent ecosystem classes and observations before default
activation:

- northeastern deciduous and mixed forest;
- evergreen forest with substantial winter canopy;
- shrubland and grassland;
- disturbed/burned forest gradients;
- observed ET partition where available, not total ET alone;
- flux-tower energy/carbon exchange, soil moisture, LAI/phenology, streamflow,
  litterfall, and snow-under-canopy observations as applicable.

Calibration and evaluation sites must be separated. A site-specific parameter
fit cannot become a vegetation-class default without out-of-sample evidence.

## Phased Delivery

1. **Code-to-literature authority audit precursor** - freeze both MIT license
   grants; produce the RHESSysEastCoast function/state/call inventory and
   GIS2RHESSys 71-field/32-profile matrix; verify equations, parameters, units,
   domains, defaults, sentinels, and branches against primary literature;
   classify deviations; close admissible gaps; and amend the implementation
   package before Rust work.
2. **Boundary contract** - define native strata, units, state ownership,
   coupling cadence, allocation arbitration, errors, and conservation ledgers.
3. **Independent crate skeleton** - typed state and synthetic soil boundary
   with no production activation and no placeholder physics.
4. **Coupled vertical slices** - implement one contract-authorized process group
   at a time without severing required conductance, Penman-Monteith,
   photosynthesis, phenology, multistratum, or root-demand feedback merely to
   preserve the superseded narrow scope.
5. **Carbon and phenology integration** - reconcile or replace the existing GSI
   canopy state without duplicate leaf/litter ownership.
6. **Hydrology integration** - activate the two-stage soil-water protocol in a
   diagnostic lane and prove shared withdrawal closure.
7. **Real-consumer shadow** - run full hillslopes with complete diagnostics and
   compare current native vegetation, RHESSys vectors, and observations.
8. **Opt-in candidate** - permit explicit native-management selection only
   after science and coupling gates pass.
9. **Default consideration** - require a separate adjudication based on broad
   out-of-sample ecosystem and hydrologic-response evidence.

Each implementation phase requires an authorized work package and prior
amendment or creation of the applicable `SC-*` contracts.

## Promotion Criteria

Promote a source-aware implementation increment only when:

- the code-to-literature precursor has a passing reviewed/verified disposition,
  has amended the exact successor boundary, and leaves no required
  `BLOCK_SUCCESSOR` row;
- every source-derived surface is tied to one of the two pinned MIT repositories
  and its notice-custody path is declared;
- the function/state and field/profile inventories identify the exact coupled
  boundary and distinguish compatibility from scientific authority;
- the function/state provenance inventory identifies a bounded first vertical
  slice;
- `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`, management-input authority,
  and water-balance ownership impacts are mapped;
- native stratum cover and overlap semantics are independently specified;
- the synthetic soil-water exchange has a typed conservation protocol;
- at least one northeastern deciduous/mixed-forest evaluation dataset and one
  contrasting vegetation class are admitted;
- the package explicitly declares what existing canopy/ET/litter authority is
  retained, shadowed, or replaced.

Default promotion additionally requires real downstream consumption,
out-of-sample improvement, no material regression in snow/frost/runoff/erosion,
and complete water/energy/carbon ledger evidence.

## Open Questions

- Should the public crate be `openwepp-vegetation`, with RHESSys-derived kernels
  as modules, rather than a source-branded crate?
- Which RHESSysEastCoast executable configuration and GIS2RHESSys-generated
  definition set will become the first pinned comparator fixture?
- Which of the 32 profile columns are internally complete for the first coupled
  slice, and which cited parameter values need independent source verification?
- Which process slice provides the most value without prematurely importing the
  full C/N system?
- Should photosynthesis and stomatal conductance be finalized before or after
  soil-water allocation, and is iteration necessary at openWEPP's timestep?
- How should multiple strata share soil-layer water when their root systems
  overlap?
- Which overlap law converts stratum covers to aggregate canopy cover for
  existing openWEPP consumers?
- Which current generalized-GSI states should become initialization inputs,
  retained adapters, or retired authority?
- Does canopy snow belong in the vegetation crate, the snow crate, or a shared
  interception component?
- How much C/N state is necessary to preserve credible phenology and allocation
  without immediately implementing RHESSys soil biogeochemistry?
- How will WEPPcloud failed runs and user interpretation be converted into
  reproducible vegetation evaluation cohorts and adversarial feedback?

## Initial References

- RHESSysEastCoast source at commit
  `375c75b1cd2202217651dff43aa113d80b9c1118` (MIT).
- GIS2RHESSys source and `vegCollection.csv` at commit
  `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` (MIT).
- The separately inspected official RHESSys source at commit
  `f9d1bbf8d161aa55b6a51061dc320188ead44962` remains licensing context only,
  not a direct-translation source.
- Tague, C. L., Band, L. E. (2004). RHESSys: Regional Hydro-Ecologic Simulation
  System - An object-oriented approach to spatially distributed modeling of
  carbon, water, and nutrient cycling.
- RHESSysEastCoast canopy-stratum, radiation, interception, conductance,
  Penman-Monteith, Farquhar photosynthesis, phenology, rooting, and allocation
  implementations, subject to contract adjudication above.
- `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`, `SC-WATBAL-001`,
  `SC-SNOWFREEZE-001`, and native management input contracts.
- `crates/openwepp-management-schema`, `crates/openwepp-plant-phenology`,
  `crates/openwepp-meteorology`, and the direct hillslope runtime.
- [Native-vegetation ET process-model backlog](20260803-native-vegetation-et-process-model.md).
