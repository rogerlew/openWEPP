# RHESSys-Derived Vegetation Crate

## Status

- `state`: **staged; boundary authority admitted** - high-value
  ecosystem-process architecture; not yet authorized for kernel implementation
  or default activation
- `date`: 2026-08-06
- `owner`: openWEPP maintainers + forest ecohydrology reviewer
- `working crate name`: `openwepp-vegetation` (the RHESSys lineage is
  provenance, not necessarily the permanent public crate name)
- `origin`: static comparison of openWEPP, WEPP-forest, and RHESSys vegetation,
  ET, radiation, litter, snow, hydrology, and biogeochemistry implementations
- `promotion trigger`: **complete** - the process/state provenance ledger,
  sanitized source-analysis rules, explicit native-management stratum
  contract, and synthetic soil-water coupling contract are admitted by
  `SC-VEGETATION-001`; the next slice still requires independent constitutive
  authority, and unresolved RHESSys licensing blocks direct/code-derived
  translation but not literature-derived independent implementation
- `default eligibility`: prohibited until conservation, field-evaluation,
  coupled-response, and real-consumer gates pass
- `completed authority package`:
  [VEGETATION-SOURCE-PROVENANCE-AND-BOUNDARY-AUTHORITY](../work-packages/20260808-vegetation-source-provenance-and-boundary-authority-001/package.md)

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
- stomatal conductance coupled to radiation, vapor-pressure deficit,
  temperature, leaf water potential, and photosynthetic state;
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

## Source and Licensing Posture

The static reference inspected for this concept is:

- repository: `https://github.com/RHESSys/RHESSys`
- local checkout: `/Users/roger/src/RHESSys`
- inspected commit: `f9d1bbf8d161aa55b6a51061dc320188ead44962`

RHESSys is publicly available, and the official RHESSys site lists
[`Completely open source`](https://rhessys.github.io/) under "Advantages of
RHESSys." The inspected repository nevertheless has no formal `LICENSE`,
`COPYING`, or equivalent grant. External registries have reported uncertain or
historical MIT status, but those reports are not a license attached to the
current source.
The repository's public [License? issue #150](https://github.com/RHESSys/RHESSys/issues/150),
opened 2021-08-07, remains open with no maintainer response as of 2026-08-08.
This demonstrates that the ambiguity has been visible for years without a
project-level resolution. It neither supplies a license nor makes a second
inquiry a useful prerequisite for scientific work.

RHESSys has also accumulated contributions from researchers at multiple
institutions. The absence of a formal license is plausibly an institutional and
contributor-provenance problem rather than evidence that the community intends
to prohibit scientific reuse. Community practice and statements are relevant
context, but they are not substitutes for a copyright grant.

Consequences:

- Static inspection, sanitized process mapping, and scientific comparison may
  inform planning and contract authoring.
- Literature equations may be implemented independently under their scientific
  provenance after contract adjudication. This work does not wait for a RHESSys
  repository license.
- RHESSys may be used as a behavioral comparator and source of black-box test
  vectors without making its implementation the target authority.
- Direct or closely translated RHESSys code must not enter a distributable
  openWEPP crate until maintainers or institutional counsel establish an
  adequate license or permission grant.
- Every promoted kernel must distinguish `RHESSys-code-derived`,
  `literature-derived`, `openWEPP-derived`, and `independently re-derived`
  provenance.
- Absence of a formal license blocks only direct/code-derived translation and
  incorporation. It is not a blanket blocker on the vegetation crate.

## Sanitized Source-Analysis Protocol

Use a lightweight agent-operated provenance firewall when RHESSys source is
consulted. This is an independent-authorship control, not a claim of a legally
dispositive clean room and not a human-in-the-loop workflow.

1. The implementation agent authors a bounded source-analysis request.
2. A source-aware analyst writes a quarantined artifact and returns only its
   path and digest.
3. An independent compliance agent reviews the request and response.
4. Only a passing, sanitized artifact is handed to the implementation agent.
5. A separate source-aware reviewer may compare the completed implementation
   behavior but must not patch or mechanically translate it.

Allowed requests and approved artifacts may describe:

- process inputs, outputs, state, units, cadence, and ownership;
- scientific equations with external literature citations;
- observable state transitions, conservation identities, and process ordering;
- boundary conditions, limiting cases, defects, and black-box vectors;
- source coordinates and commit hashes for auditability;
- evidence explicitly labeled `LITERATURE`, `CODE-OBSERVED`, or `INFERENCE`.

They must not contain or request:

- source excerpts or statement-by-statement descriptions;
- line-by-line translation, mechanically reversible pseudocode, or patches;
- distinctive comments, naming, or nonessential control-flow structure;
- code-only formulas or constants presented as scientific authority;
- instructions for reproducing a named RHESSys function exactly.

The auditable handoff bundle is intentionally small:

```text
request.md
approved-spec.md
compliance-review.md
provenance-manifest.md
implementation-prompt.md
```

The compliance decision asks whether the request targets behavior rather than
translation, whether the response conveys semantics rather than source
expression, whether equations/constants have correctly labeled authority, and
whether the artifact permits an independently structured Rust implementation.
No human prompt adjudication is required for the routine path.

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
  model: rhessys_ecophysiology_v1
  strata:
    - id: overstory
      vertical_layer: 1
      lifeform: tree
      parameter_set: northern_hardwood_deciduous
      cover_fraction: 0.72
      height_m: 18.0
      initial_state: northern_hardwood_mature_v1
      rooting_profile: hardwood_deep_v1
    - id: understory
      vertical_layer: 2
      lifeform: shrub
      parameter_set: temperate_deciduous_shrub
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

Before code authoring, build a function-level RHESSys process inventory. For
each candidate kernel record:

- RHESSys file/function and inspected commit;
- scientific citation or lack of one;
- inputs, outputs, units, state mutation, and call ordering;
- dependencies on patch, zone, stratum, soil, carbon, nitrogen, and snow state;
- known code defects, ambiguous branches, and numerical safeguards;
- applicable openWEPP contract and existing consumer;
- disposition: adopt, re-derive from literature, adapt, reject, or defer;
- licensing disposition.

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

1. **Source and provenance audit** - record the unresolved license posture,
   produce the RHESSys function/state inventory, and identify whether each
   candidate is literature-derived, independently re-derived, or prohibited
   code-derived translation. Do not make a definitive repository license a
   prerequisite for literature-derived work.
2. **Boundary contract** - define native strata, units, state ownership,
   coupling cadence, allocation arbitration, errors, and conservation ledgers.
3. **Independent crate skeleton** - typed state and synthetic soil boundary
   with no production activation and no placeholder physics.
4. **Vertical slices** - implement one contract-authorized process group at a
   time, beginning with radiation/interception and conductance/water demand.
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

Promote this concept to work-package planning only when:

- the proposed implementation method is literature-derived or independently
  re-derived, or RHESSys licensing/permission is sufficient for any explicitly
  proposed code-derived material;
- the sanitized source-analysis request, compliance, and handoff rules are
  bound into the work package when source-aware agents are used;
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
- Which RHESSys revision and configuration represent the reference behavior?
- Will RHESSys issue #150 eventually produce an explicit license or permission
  grant for direct translation and redistribution? This remains useful but is
  not a prerequisite for independent implementation.
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

- RHESSys source at commit
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- RHESSys [License? issue #150](https://github.com/RHESSys/RHESSys/issues/150),
  opened 2021-08-07 and still unanswered as of 2026-08-08.
- Tague, C. L., Band, L. E. (2004). RHESSys: Regional Hydro-Ecologic Simulation
  System - An object-oriented approach to spatially distributed modeling of
  carbon, water, and nutrient cycling.
- RHESSys/BGC vegetation, canopy-stratum, radiation, hydrology, and carbon-
  nitrogen process implementations, subject to the licensing posture above.
- `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`, `SC-WATBAL-001`,
  `SC-SNOWFREEZE-001`, and native management input contracts.
- `crates/openwepp-management-schema`, `crates/openwepp-plant-phenology`,
  `crates/openwepp-meteorology`, and the direct hillslope runtime.
- [Native-vegetation ET process-model backlog](20260803-native-vegetation-et-process-model.md).
