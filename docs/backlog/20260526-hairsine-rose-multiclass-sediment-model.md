# Hairsine-Rose Multi-Class Sediment Model (Concept Backlog)

## Status
- `state`: backlog
- `maturity`: concept / planning only
- `default_path`: not eligible
- `date`: 2026-05-26
- `evidence_mode`: Static (lit review from training-time knowledge; static analysis of
  openWEPP sediment surface — no execution; literature PDFs not yet vendored)

## Why this exists

openWEPP's current sediment kernel inherits the Foster-Meyer (1972, 1989) detachment-
transport-capacity framework via SC-SED-001 and SC-ROUTE-001. The framework is well-
calibrated for the USDA rainfall-simulator and natural-runoff plot datasets of the 1980s
on Midwestern croplands, but is known to perform poorly outside that regime in several
identifiable ways:

- it lacks an explicit **deposited-layer** state that armors the original soil after a
  deposition phase, so re-detachment after deposition is parameterized via enrichment
  ratios rather than physical mass continuity;
- size-class behavior at deposition transitions is enforced via enrichment-ratio
  procedures (SC-SED-001 §11.5) rather than per-class settling-velocity continuity;
- transport-capacity authority (Yalin in legacy; `tc = k·τ^m` Foster power-law in
  openWEPP per SC-SED-001 `INV-SED-006` and `[20260525_water_erosion_kernel_audit.md
  §10.3]`) is shear-stress-based; multiple post-1990 studies show shallow-flow
  unit-stream-power forms better predict hillslope transport.

The Hairsine-Rose (HR) multi-class model (1991-1992, refined through 2010s) was
developed specifically to remedy the deposited-layer and size-class shortcomings via
explicit mass conservation per particle class with an evolving deposited-layer shield.
HR is the most theoretically mature post-Foster-Meyer hillslope erosion framework with
broad independent validation.

This backlog item captures the case for evaluating HR as a future openWEPP physics
authority, the static implications for the existing parameterization surface, and the
design constraint that **sediment class cardinality and composition must be
user-customizable** rather than fixed at the legacy 5-class Foster-Young scheme.

## Scope

Define an experimental erosion-kernel family that adopts the Hairsine-Rose multi-class
formulation with an explicit deposited-layer state, configurable per-class settling
velocities and densities, and a unit-stream-power transport-capacity option.

This backlog item is **concept-stage**. It does **not** authorize implementation. The
promotion path is: this backlog → an audit comparing HR formulation against the
existing SC-SED-001 invariant surface → an ADR on whether to commit to HR as a future
authority arm → a contract-first work-package sequence that authors `SC-SED-HR-001`
(or amends SC-SED-001) before any code changes.

## Non-goals

- No default replacement of the Foster-Meyer authority arm. HR would be experimental
  and disabled by default until validation and governance close.
- No removal of the legacy 5-class Foster-Young primary/aggregate scheme. HR adoption
  must preserve byte/semantic continuity for that class scheme in legacy mode.
- No introduction of finite-element 2D spatial discretization (the LISEM/EUROSEM
  approach). openWEPP's OFE/segment/channel topology is preserved. HR is adopted as
  the constitutive model on the existing spatial grid.
- No machine-learning surrogate sediment models. HR adoption is in service of
  contract-first process-based physics per ADR-0011, not a replacement for it.
- No commitment to vendoring restricted PDFs into this repository. Vendoring decisions
  follow `references/README.md` and `docs/governance/reference-vendoring-policy.md`.

## Governing constraints (openWEPP policy)

- ADR-0011 (architecture-first, top-down science contracts): HR adoption must be
  preceded by canonical contract authority. Concretely: a new `SC-SED-HR-001` or a
  versioned amendment to SC-SED-001 with separated invariant families.
- ADR-0003 (semantic parity, not bit-parity): HR adoption must declare its own
  tolerance bounds; legacy Foster-Meyer parity remains a separate authority arm.
- The comparator-tier policy applies: HR is a forensic-investigation alternative until
  validation against contract-derived vectors is complete.
- Customizable sediment classes must work end-to-end: parser → projection → kernel →
  HBP serialization → routing intake. The HBP `npart: u16` field already supports
  variable cardinality (see Static Analysis §B).

## Background — what Hairsine-Rose actually is

### A. Core formulation (Hairsine & Rose 1991, 1992a, 1992b)

For each particle class `i ∈ {1..N}`, HR tracks two mass-balance equations per unit
plan area:

1. **Sediment in transport** `c_i(x,t)` (mass per unit volume of overland flow):
   ```
   ∂(h c_i)/∂t + ∂(q c_i)/∂x  =
       e_i(rainfall detachment from original soil)
     + e_d_i(rainfall re-detachment from deposited layer)
     + r_i(flow-driven entrainment from original soil)
     + r_d_i(flow-driven re-entrainment from deposited layer)
     - d_i(deposition to bed)
   ```

2. **Deposited-layer storage** `m_d_i(x,t)` (mass per unit plan area):
   ```
   ∂m_d_i/∂t = d_i - e_d_i - r_d_i
   ```

Per-class deposition is governed by **settling velocity** `v_s,i`:
```
d_i = v_s,i · c_i
```

Detachment from the original soil is multiplied by a **shielding factor** that depends
on the deposited-layer mass fraction `H = Σ m_d_i / m_d*` (where `m_d*` is the mass
required to fully cover the soil surface):
```
e_i = (1 - H) · a_i · (1 - p) · I^k  ·  p_i_original
e_d_i = H · a_d_i · (1 - p) · I^k    ·  (m_d_i / Σ m_d_j)
```
with analogous shielded forms for flow-driven entrainment `r_i`, `r_d_i`. The
deposited-layer composition therefore evolves separately from the original-soil
composition, and re-erosion of previously deposited (selectively finer) material is
physically distinct from fresh detachment.

### B. Distinguishing features versus Foster-Meyer / WEPP

| Feature | WEPP / Foster-Meyer | Hairsine-Rose |
|---|---|---|
| Per-class continuity | Single bulk equation with class-fraction enrichment update | Independent per-class continuity equations |
| Deposited layer | Implicit; re-detachment via enrichment ratio | Explicit storage state `m_d_i` per class |
| Size-selective deposition | Empirical enrichment ratio adjustment | Natural consequence of `v_s,i` ordering |
| Re-detachment after deposition | Enrichment-ratio bookkeeping | Mass-conservative re-entrainment of deposited classes with shielding factor `H` |
| Rainfall detachment | `Di = Ki · I · sin(slope) · SDR`-style | Class-resolved with shielding term `(1-H)` |
| Transport capacity authority | Yalin (legacy) / Foster power-law (openWEPP) | Pluggable; commonly Govers stream-power form or class-resolved Tc |
| State per OFE | Single sediment load + class fractions | Sediment load + class fractions + deposited-layer composition per class |

### C. Empirical validation lineage (1992 - present)

Independent validation has come from at least four research groups across three
continents over thirty years. The validation lineage is unusually strong for a
process-based erosion model:

- **Australian flume + plot studies** (Hairsine, Rose, Sander, Hogarth, Yu; 1990s -
  2000s): original calibration on Griffith and Toowoomba rainfall-simulator data,
  multi-class settling-velocity studies, deposited-layer compositional evolution.
- **Belgian-Dutch field plots** (Beuselinck, Govers, Steegen): independent validation
  of the net-deposition equation and size-selective deposition.
- **Indian Ocean / SE Asia tropical plots** (Yu et al.; HR-based GUEST model):
  applications outside the temperate-cropland regime.
- **UK / EU adoption in EUROSEM** (Morgan, Quinton): HR-influenced multi-class
  deposition formulation inside an event-based finite-difference model.
- **Mathematical / analytical** work (Sander, Parlange, Hogarth, Heng; 2000s-2010s):
  closed-form and numerical-scheme analyses, sensitivity studies, settling-velocity
  identifiability.

### D. Where HR is expected to outperform Foster-Meyer (qualitative claims, from the
literature; not yet validated in openWEPP)

1. **Sequential storms** with intermediate dry-down: deposited-layer composition
   persists between events, and the next event re-entrains a different size
   distribution than fresh detachment would produce. WEPP's enrichment ratio resets
   between events. Expected improvement: better outlet size-distribution prediction
   for closely-spaced storm sequences.
2. **Long deposition reaches** (footslopes, depressional storage, sediment fans):
   the size-selective deposition emerges from `v_s,i` ranking, not from a fitted
   ratio. Expected improvement: more defensible outlet sediment quality for
   long-deposition watersheds, including reservoir-delivery and water-quality
   contexts.
3. **Post-fire surfaces with ash + soil two-phase composition**: ash is a low-density
   class with distinct settling velocity. HR's class-resolved structure naturally
   represents two-phase transport; WEPP's primary/aggregate scheme cannot. Expected
   improvement: post-fire sediment yield and ash routing where the ash phase
   dominates early-season exports.
4. **Tropical / aggregated soils**: aggregate breakdown and re-entrainment dynamics
   that are out-of-distribution for the Midwestern calibration. Expected improvement:
   broader geographic applicability without per-region recalibration.
5. **Rainfall-dominated vs flow-dominated transitions**: HR's separate rainfall and
   flow-driven terms with explicit shielding give cleaner partitioning of which
   process is active at any time, which can help interpret detachment-vs-routing
   model failures.

### E. Known weaknesses / where HR does not obviously win

- **Parameter identifiability**: HR introduces `a_i`, `a_d_i`, `r_i`, `r_d_i`, `m_d*`
  in addition to per-class `v_s,i`. Multiple authors (Tromp-van Meerveld 2008,
  Beuselinck 2002) document identifiability issues — different parameter sets fit
  similar outlet sediment curves. Mitigation requires explicit prior bounds or
  multi-output (size-distribution + total load) calibration.
- **Computational cost**: per-class state and per-class continuity equations multiply
  the kernel work proportional to `N` classes. For openWEPP's current 5-class scheme
  the cost is bounded; for user-customizable cardinality (say 10-20 classes) it
  matters more.
- **Where Foster-Meyer is already adequate**: short uniform plots, single-event storms,
  Midwestern cropland — HR offers little gain over WEPP. The case for HR is strongest
  where WEPP's assumptions are weakest.

## Static analysis — implications for the openWEPP parameterization surface

### A. Existing constitutive parameters that map cleanly

| openWEPP / SC-SED-001 surface | HR equivalent | Notes |
|---|---|---|
| `Kr` (rill erodibility) | flow-driven detachment coefficient `r_i` (class-resolved) | Single bulk `Kr` would be replaced by per-class `r_i`; class fractions would derive from soil texture + aggregate composition |
| `Ki` (interrill erodibility) | rainfall detachment coefficient `a_i` | Same — bulk-to-per-class refinement |
| `τc` (critical shear) | retained for flow-driven entrainment threshold | Surface unchanged; meaning narrowed to flow-detachment branch |
| `Tc` (transport capacity) | retained as ceiling, optionally replaced by Govers form | The audit §10.3 already flags `tc = k·τ^m` as a contract-sanctioned simplification; HR adoption is a natural moment to evaluate stream-power Tc |
| `particle_class_count` (`npart`) | identical concept; cardinality user-configurable | Already variable in HBP format (`u16`); see §B below |
| `sediment_concentration_kg_m3_i` | identical (concentration `c_i`) | Boundary surface preserved |
| `particle_flow_fraction_i` | identical (class fraction in transport) | Boundary surface preserved |
| Enrichment ratio `ER` | replaced by emergent class fractions | `ER` becomes a diagnostic output of the HR mechanism, not an input |

### B. Particle-class cardinality is already variable

Static read of openWEPP confirms `npart` is `u16` in the HBP parser
([`crates/openwepp-input-contract/src/parsers/hbp.rs:178, 798, 1684, 1701`])
and `particle_class_count` is a runtime scalar
([`crates/openwepp-hillslope-orchestrator/src/constants.rs:378`]). Class properties
(`fall[i]`, `tcf1[i]`, `ssa_class[i]`) are retrieved per class index in
[`run_erod14_wave2`](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs#L6192).
The five-class Foster-Young scheme is therefore a **convention enforced upstream
in the projection layer**, not a hard structural constraint in the kernel or HBP
format.

This is a structural advantage for HR adoption: user-customizable class definitions
require only:
1. an input-file class table (settling velocity, density, specific surface area,
   composition mapping from soil texture), and
2. a projection module that produces the per-class properties from that table.

The kernel and HBP serialization need no changes for cardinality. They need changes
to add per-class deposited-layer storage state (`m_d_i`).

### C. Surfaces that would require new state and new contract authority

| New surface | Type | Owner |
|---|---|---|
| `m_d_i` per class | runtime state, persists across events | new SC-SED-HR-001 |
| `H = Σ m_d_i / m_d*` | derived state diagnostic | new SC-SED-HR-001 |
| `m_d*` (cover threshold) | input parameter | new SC-INFILE-SOIL-HR amendment to soil parser |
| `v_s,i`, `a_i`, `a_d_i`, `r_i`, `r_d_i` per class | input parameter table | new SC-INFILE-PARTICLES-001 (user-customizable class table) |
| `tc_class_i` (optional class-resolved transport capacity) | derived state | extension to SC-SED-001 |

### D. Surfaces that would require breaking changes (and therefore careful contract
sequencing)

- **HBP event payload schema**: the deposited-layer state is per-OFE, per-class, and
  persistent across events — it must serialize across the hillslope/watershed
  boundary if watershed-scale routing is HR-aware, and across day boundaries within
  a hillslope HBP shard. This is a schema extension, not a break, but it requires
  versioned schema migration per the HBP contract.
- **Enrichment-ratio downstream consumers**: `ER` is currently a hillslope export
  consumed by watershed sediment-quality interpretation. Under HR, `ER` becomes
  derived from `c_i` and `v_s,i`. Either retain `ER` as a diagnostic export for
  backward compatibility, or break with a contract amendment.
- **Comparator metadata**: HR's outlet sediment numbers will not match legacy
  Foster-Meyer numbers even when both models are "correct" within their own physics.
  The comparator tier for HR-mode outputs must be classified as
  `Investigation`-tier vs. legacy, not as a parity check (per ADR-0011 comparator
  policy).

### E. Customizable sediment classes — separate value beyond HR

Even without HR adoption, user-customizable class tables have standalone value:

1. **Post-fire ash phase**: a sixth or seventh class with very low density (~0.5 g/cm³)
   and small settling velocity can be added to the existing 5-class Foster-Young
   scheme to represent ash transport, without HR adoption.
2. **Site-specific soil characterization**: users with measured class composition
   (e.g., laser-diffraction PSD, aggregate-stability data) can replace the
   regression-based default classes.
3. **Sensitivity / uncertainty studies**: investigators can vary class cardinality
   and properties without code changes.

If HR adoption is deferred, **class-table customization should be promoted as a
separate, smaller backlog item** — it does not depend on HR.

## Proposed phased numerical plan (conditional on promotion)

### Phase A — paper study and reference acquisition
- Vendor the foundational HR papers (1991, 1992a, 1992b, Sander 1996, Beuselinck
  2002) per `references/README.md` workflow; record rights classification.
- Author an audit (`docs/audits/`) comparing the SC-SED-001 invariant surface
  against the HR governing equations row-by-row.
- Authoring of `SC-SED-HR-001` draft against HR governing equations with explicit
  `INV-SED-HR-*` invariants and tolerance bounds.

### Phase B — isolated kernel implementation
- Implement an experimental `run_erod_hairsine_rose` kernel arm gated by
  `erod_hr_enabled` runtime flag, default off.
- Implement class-table parser (`SC-INFILE-PARTICLES-001`) with default values that
  reproduce the Foster-Young 5-class scheme exactly.
- Add per-class deposited-layer state to the hillslope state-surface, with HBP
  schema extension.

### Phase C — single-OFE single-event validation
- Implement contract-derived vectors from the HR analytical solutions (Sander 1996
  steady-state, Hogarth 2004 unsteady).
- Verify local mass closure per class and total mass closure across the kernel.

### Phase D — sequential-storm and multi-OFE validation
- Sequential-storm vectors that exercise the deposited-layer persistence.
- Multi-OFE vectors that exercise the per-segment composition transport.
- Comparator-tier classification of HR-mode outputs versus legacy Foster-Meyer arm
  outputs as Investigation-tier divergence.

### Phase E — post-fire scenario evaluation
- ERMiT / Wagenbrenner / Robichaud post-fire datasets (acquisition and rights
  classification TBD).
- Evaluate HR's two-phase (ash + soil) transport prediction versus a Foster-Meyer
  baseline with ash added as a sixth class but no deposited-layer state.

## Acceptance criteria to promote from backlog

1. **Reference closure**: foundational HR papers (1991, 1992a, 1992b, plus at least
   one independent-group validation: Beuselinck 2002 or Heng 2009) are vendored or
   metadata-tracked per `references/README.md`.
2. **Contract draft**: `SC-SED-HR-001` (or amendment) is authored with at least the
   continuity, settling-velocity, shielding-factor, and conservation invariants
   stated as `INV-SED-HR-*` rows.
3. **Comparator-tier policy stated**: HR-mode outputs are explicitly classified as
   Investigation-tier vs. legacy Foster-Meyer outputs, not as parity claims.
4. **Customizable-class architecture decided**: a separate backlog/decision determines
   whether class-table customization ships independently of HR or coupled to it.
5. **Local closure target**: per-class mass closure `|d(h c_i)/dt + d(q c_i)/dx -
   sources_i + sinks_i| ≤ ε`, with `ε` stated in the contract.
6. **Identifiability framework**: a written position on how the additional HR
   parameters (`a_i`, `a_d_i`, `r_i`, `r_d_i`, `m_d*`) will be constrained — prior
   bounds, multi-output calibration, or explicit non-identifiability acknowledgment.
7. **Governance**: ADR authored on the relationship between HR and Foster-Meyer
   authority arms; whether one supersedes the other, both coexist as user-selectable,
   or HR is offered only as an investigation tool.

## Key risks

- **Identifiability**: HR's extra parameters can fit similar outlet curves with
  different per-class allocations. Without prior bounds and multi-output calibration,
  HR may give "right answer for wrong reasons" parity to Foster-Meyer.
- **Validation-data scarcity**: the strongest HR validations are Australian and
  European; openWEPP's target scenarios (wepp-forest post-fire, Western US
  rangeland) have less HR-specific published validation. A site-specific validation
  campaign may be required before promotion to a default authority arm.
- **Schema-extension drift**: deposited-layer state in HBP must be designed once and
  not iterated. A premature schema commit before the per-OFE persistence semantics
  are clear risks a costly migration.
- **Class-table customization scope creep**: customization needs careful
  parameter-table provenance and unit handling. Without governance, it becomes a
  vector for unvalidated user-tunable knobs.
- **Adoption-without-contract drift**: implementing HR as an "experimental switch"
  without a contract authority would violate ADR-0011 and create a hidden physics
  arm. Contract-first sequencing is non-negotiable here.

## Open questions

1. Does HR-mode require its own ADR, or is a SC-SED-001 amendment with an
   alternate authority arm sufficient?
2. Should class-table customization ship as a standalone backlog item before any HR
   work, or be bundled with HR Phase B?
3. What is the per-OFE persistence semantics for `m_d_i` across the daily/event
   simulation tick? Within an event: clearly stateful. Across events: yes (the whole
   point of HR). Across days within the same event: yes. Across years: probably yes,
   with a documented reset condition for tillage events.
4. Should the unit-stream-power transport capacity (Govers) be bundled with HR
   adoption or kept as a separate SC-SED-001 amendment? Bundling tests both changes
   simultaneously, which complicates attribution; separating them risks adopting one
   without the other.
5. What is the minimum acceptable post-fire validation dataset before HR is
   promoted from Investigation-tier to a co-equal authority arm for wepp-forest
   scenarios?
6. How does HR interact with the existing rill-detach-deposit segment topology
   (`route.for` MSHEAR cases) discussed in
   [`20260525_water_erosion_kernel_audit.md` §10.2 row 214](../audits/20260525_water_erosion_kernel_audit.md)?
   HR's continuous formulation may obviate the segment-classification step or
   require its own segment-handling rules.

## References (for promotion-stage vendoring)

The following papers are the minimum corpus for promotion from backlog to active
implementation. Citations are recorded from training-time knowledge of the
literature; exact volume/issue/page numbers and DOIs should be re-verified during the
references-intake step (`references/README.md`). Local copies are not yet acquired;
rights classification per `references/rights_classification_first_pass_2026-05-11.md`
workflow is required before vendoring.

### Foundational (primary, must vendor)

- **Hairsine, P. B., & Rose, C. W. (1991).** Rainfall detachment and deposition:
  Sediment transport in the absence of flow-driven processes. *Soil Science Society
  of America Journal*, 55(2), 320-324.
  - Establishes rainfall-detachment with deposited-layer shielding; the rainfall arm
    of HR.
- **Hairsine, P. B., & Rose, C. W. (1992a).** Modeling water erosion due to overland
  flow using physical principles: 1. Sheet flow. *Water Resources Research*, 28(1),
  237-243.
  - Sheet-flow extension; introduces the flow-driven entrainment terms.
- **Hairsine, P. B., & Rose, C. W. (1992b).** Modeling water erosion due to overland
  flow using physical principles: 2. Rill flow. *Water Resources Research*, 28(1),
  245-250.
  - Rill-flow extension; directly comparable to the WEPP rill/interrill split.
- **Rose, C. W., Williams, J. R., Sander, G. C., & Barry, D. A. (1983).** A
  mathematical model of soil erosion and deposition processes: I. Theory for a
  plane land element. *Soil Science Society of America Journal*, 47(5), 991-995.
  - Pre-HR foundation; multi-class continuity framework that HR builds on.

### Independent validation (must include at least one for promotion)

- **Sander, G. C., Hairsine, P. B., Rose, C. W., Cassidy, D., Parlange, J.-Y.,
  Hogarth, W. L., & Lisle, I. G. (1996).** Unsteady soil erosion model, analytical
  solutions and comparison with experimental results. *Journal of Hydrology*,
  178(1-4), 351-367.
  - Analytical solutions used for kernel verification vectors.
- **Beuselinck, L., Hairsine, P. B., Sander, G. C., & Govers, G. (2002).**
  Evaluating a multiclass net deposition equation in overland flow conditions.
  *Water Resources Research*, 38(7), 14-1 to 14-11.
  - Independent (Belgian) field validation of the multi-class deposition equation;
    identifiability discussion.
- **Hogarth, W. L., Rose, C. W., Parlange, J.-Y., Sander, G. C., & Carey, G. (2004).**
  Soil erosion due to rainfall impact with no inflow: A numerical solution with
  spatial and temporal effects of sediment settling velocity characteristics.
  *Journal of Hydrology*, 294(4), 229-240.
  - Settling-velocity sensitivity; useful for class-cardinality choice.
- **Heng, B. C. P., Sander, G. C., & Scott, C. F. (2009).** Modeling overland flow
  and soil erosion on nonuniform hillslopes: A finite volume scheme. *Water
  Resources Research*, 45(5), W05423.
  - Non-uniform slope numerical scheme; relevant to openWEPP segment topology.

### Model-comparison and review (recommended)

- **Misra, R. K., & Rose, C. W. (1996).** Application and sensitivity analysis of
  process-based erosion model GUEST. *European Journal of Soil Science*, 47(4),
  593-604.
  - GUEST is the HR-family model; sensitivity context.
- **Tromp-van Meerveld, H. J., Parlange, J.-Y., Barry, D. A., Tromp, M. F., Sander,
  G. C., Walter, M. T., & Parlange, M. B. (2008).** Influence of sediment settling
  velocity on mechanistic soil erosion modeling. *Water Resources Research*, 44(6),
  W06401.
  - Identifiability and settling-velocity choice; directly relevant to per-class
    parameterization.
- **Morgan, R. P. C., Quinton, J. N., Smith, R. E., Govers, G., Poesen, J. W. A.,
  Auerswald, K., Chisci, G., Torri, D., & Styczen, M. E. (1998).** The European Soil
  Erosion Model (EUROSEM): A dynamic approach for predicting sediment transport from
  fields and small catchments. *Earth Surface Processes and Landforms*, 23(6),
  527-544.
  - EUROSEM as an HR-influenced multi-class model; comparison context.

### Adjacent transport-capacity work (bundle-decision input)

- **Govers, G. (1990).** Empirical relationships for the transport capacity of
  overland flow. *IAHS Publication*, 189, 45-63.
- **Govers, G. (1992).** Evaluation of transporting capacity formulae for overland
  flow. In: A. J. Parsons & A. D. Abrahams (eds), *Overland Flow: Hydraulics and
  Erosion Mechanics*, UCL Press, 243-273.
  - Stream-power transport-capacity formulations for hillslope flow; candidate
    replacement for the Foster power-law `tc = k·τ^m` regardless of whether HR is
    adopted.
- **Prosser, I. P., & Rustomji, P. (2000).** Sediment transport capacity relations
  for overland flow. *Progress in Physical Geography*, 24(2), 179-193.
  - Compendium of Tc formulations with hillslope-regime applicability assessment.

### Post-fire / openWEPP-target scenarios (acquisition lower priority but high value)

- **Wagenbrenner, J. W., MacDonald, L. H., & Rough, D. (2006).** Effectiveness of
  three post-fire rehabilitation treatments in the Colorado Front Range.
  *Hydrological Processes*, 20(14), 2989-3006.
- **Robichaud, P. R., Elliot, W. J., Pierson, F. B., Hall, D. E., & Moffet, C. A.
  (2007).** Predicting postfire erosion and mitigation effectiveness with a web-based
  probabilistic erosion model. *Catena*, 71(2), 229-241.
  - ERMiT — the WEPP-family post-fire predictive tool; baseline for any
    post-fire HR comparison.

### Citation-verification notes

The above citations are reconstructed from training-time knowledge. The promotion
gate requires:
1. DOI / publisher-link verification for each entry.
2. Page-range verification for any equation citations used in `SC-SED-HR-001`.
3. Rights-classification per
   `references/rights_classification_first_pass_2026-05-11.md` schema.
4. Local cache placement in `references/vendorable/` (redistributable) or
   `references/copyrighted/` (restricted) per `references/README.md`.

A bibliography-only entry in `references/annotated_bibliography.md` should be added
**only at promotion time**, not as part of this backlog filing.

## Related work and cross-references

- [`docs/audits/20260525_water_erosion_kernel_audit.md`](../audits/20260525_water_erosion_kernel_audit.md)
  §10.2 row 214 and §10.3 — current Foster-Meyer authority and known divergences.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` — current sediment
  authority; the contract HR would either supersede with `SC-SED-HR-001` or amend.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` — channel-side
  consumer of hillslope sediment payloads; HR adoption affects what payload schema
  routing must accept.
- `docs/decisions/0003-parity-semantic-not-bit.md` — tolerance posture, applies to
  HR vs Foster-Meyer arm comparison.
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md` —
  contract-first sequencing for HR adoption.

## Work-package linkage

| WP | Closes acceptance-criterion | Status |
|---|---|---|
| [`20260526-hrref01-hairsine-rose-references-intake-001/`](../work-packages/20260526-hrref01-hairsine-rose-references-intake-001/) | #1 (Reference closure) — adds R-17+ entries to `references/annotated_bibliography.md`, performs first-pass rights classification, best-effort artifact acquisition | active (2026-05-26) |

Downstream WPs (not yet authored) will address remaining promotion-from-backlog
acceptance criteria:

- #2 Contract draft (`SC-SED-HR-001`) — authored after HRREF-01 closes so that
  bibliography entries are citable.
- #3 Comparator-tier policy — authored alongside #2.
- #4 Customizable-class architecture decision — may ship as a standalone WP per
  static-analysis §E note.
- #5-#7 Local closure target, identifiability framework, governance ADR —
  sequenced after the contract draft.
