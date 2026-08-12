# Kickoff: Coupled C3 Forest Vegetation Model Stack Authority

Scope: local repository scientific-authority and canonical-contract task in
`/home/workdir/openWEPP`, with read-only inspection of the pinned
`/workdir/RHESSysEastCoast` and `/workdir/GIS2RHESSys` checkouts and read-only
retrieval of permissible scientific literature. Create and execute work package
`docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/`.
Authorized writes are bounded to package documentation and artifacts, the
scientific reference corpus and tracked metadata, canonical science contracts,
contract-derived tests and fixtures, lifecycle records, and prospective
successor-package amendments. Do not author production vegetation Rust, change
runtime selection, activate a new model, deploy, publish, or send external
messages. Read-only literature retrieval is the only authorized network action.

Execution mode: package-end-to-end.

Phase plan: scaffold the package and execute every phase in `package.md`
sequentially through terminal disposition. The package objective is to select
and canonically admit one coherent coupled C3 woody-vegetation model stack,
resolve the existing RHESSys vegetation authority blockers for the supported
domain, and release a contract-first implementation successor. This package is
not another open-ended literature survey or gap inventory.

Autonomy: execute the complete authorized scope and update all required
artifacts without requesting additional user direction unless a load-bearing
primary source remains inaccessible after documented attempts through DOI,
author manuscript, institutional repository, government archive, and existing
local caches. If blocked, request only the exact article, chapter, equation, or
technical specification that prevents admission of a named constitutive family.

## Binding User Direction

The earlier native canopy-phenology work did not carry vegetation state far
enough. Do not repeat that mistake by authorizing only phenology, canopy
geometry, evapotranspiration, or a temporary water-only vegetation model.

The canonical model architecture must include photosynthesis and persistent
carbon and nitrogen state from the beginning. Implementation may later land in
stable phases, but every phase must target one already-admitted coupled state
machine. No temporary model may become an independent authority for final LAI,
foliar mass, roots, litter, transpiration, or plant water stress.

For the selected C3 woody-vegetation domain:

- photosynthesis must causally participate in stomatal conductance, transpiration,
  carbon gain, allocation, respiration, future leaf area, root state, and litter
  production;
- vegetation carbon and nitrogen pools, demands, transfers, and receiving-owner
  boundaries are first-class model state;
- deciduous, evergreen, and mixed multistratum forests must retain distinct
  stratum identities; a mixed forest is never an averaged parameter row;
- site-specific parameter values, topology, and complete compatible initial
  state are caller-supplied configuration, not universal defaults openWEPP must
  select;
- openWEPP owns field meanings, equations, units, scale and area basis,
  mathematical domains, fixed scientific constants, numerical algorithms,
  guards, conservation, state ownership, and test invariants;
- empirical calibration and transferability are separate later claims and may
  not block implementation-authority admission when defensible process science
  exists;
- the agricultural WEPP `Kcb`/LAI PMET partition is prohibited as the native
  forest architecture; reduced canopy demand must not be automatically donated
  to forest-floor or mineral-soil evaporation;
- vegetation must not mutate hydrologic soil storage directly;
- water and nutrient acquisition must use typed request, arbitration, receipt,
  finalization, and atomic-commit boundaries;
- current GSI authority may remain the phenological timing/activity signal, but
  the coupled model’s leaf carbon, SLA, nitrogen state, allocation, and turnover
  must own actual foliar mass and LAI;
- RHESSysEastCoast and GIS2RHESSys are licensed implementation, architecture,
  comparator, and format provenance. Source behavior is not automatically
  scientific authority.

## Required Outcome

The package may close `complete / implementation-authority released` only when
the supported model domain has one complete selected stack rather than a menu
of unresolved alternatives.

The package must resolve, for the selected C3 woody domain:

- `AUTH-RHEC-001`: strict consumed-field schema, aliases, required presence,
  parameter classifications, and no hidden defaults;
- `AUTH-RHEC-002`: caller-value ingestion and domain validation without a
  default, calibration, or transferability claim;
- `AUTH-RHEC-003`: multistratum direct/diffuse radiation and canopy scaling;
- `AUTH-RHEC-004`: liquid interception storage, throughfall, stemflow,
  drainage, wet fraction, carry, and evaporation;
- `AUTH-RHEC-005`: aerodynamic and boundary-layer transfer;
- `AUTH-RHEC-006`: photosynthesis-linked stomatal and canopy conductance;
- `AUTH-RHEC-007`: independently owned canopy transpiration, wet-canopy
  evaporation, and forest-floor evaporation with complete resistance, energy,
  phase, area, and interval lineage;
- `AUTH-RHEC-008`: complete bounded C3 photosynthesis;
- `AUTH-RHEC-009`: phenology, leaf-carbon/SLA/LAI ownership, leaf onset,
  senescence, and turnover;
- `AUTH-RHEC-010`: plant hydraulics, layer-resolved root demand, dry/frozen
  exclusions, hydrologic arbitration, and actual uptake;
- `AUTH-RHEC-011`: persistent C/N pools, maintenance and growth respiration,
  allocation, retranslocation, turnover, mortality required for continuous
  state, N demand and uptake, and material-transfer ownership;
- `AUTH-RHEC-014`: canopy/ground available-energy ownership and component
  closure;
- `AUTH-RHEC-015`: complete caller-supplied initial-state schema and
  reconstruction invariants;
- `AUTH-RHEC-016`: implementation-ready digest-bound local-definition
  acquisition authority.

Preserve `AUTH-RHEC-012` as an explicit canopy-snow deferral unless complete
joint vegetation/snow/land-surface-energy authority is admitted in this package.
Preserve `AUTH-RHEC-013` as licensing/provenance authority.

Do not close by repeating that broad process authority is missing. For every
in-domain constitutive family, exhaust primary-source, established
reference-model, physical-conservation, dimensional-derivation, and explicit
openWEPP canonical-selection routes. Choose one defensible formulation and
record why it is selected. Implementation effort, absence of site calibration,
or the existence of multiple defensible published formulations is not a valid
hold boundary.

## Initial Supported Domain

Admit a first canonical model version for:

- vascular C3 woody vegetation;
- evergreen and deciduous tree or shrub strata;
- mixed vertically overlapping multistratum stands;
- explicit direct and diffuse radiation, with PAR distinguished where required;
- sunlit/shaded or another explicitly selected nondegenerate canopy scaling;
- coupled leaf photosynthesis, stomatal conductance, leaf energy balance, and
  transpiration;
- prognostic or otherwise explicitly selected plant hydraulic state;
- layer-resolved root water requests and realized uptake;
- rainfall interception, throughfall, stemflow, drainage, and wet-canopy
  evaporation;
- persistent vegetation carbon and nitrogen pools;
- allocation, storage, retranslocation, respiration, turnover, and litter/CWD
  transfers;
- mineral-N demand, competition/arbitration, and receipts;
- exact water, energy, carbon, nitrogen, and dry-material ledgers.

The package may explicitly reject or defer C4 photosynthesis, nonvascular
strata, crops, canopy snow, recruitment, succession, fire behavior, and
catastrophic disturbance only when each unsupported branch has typed rejection
semantics and cannot silently enter the selected C3 model.

Do not use an immutable-leaf-N or no-material-mutation simplification to declare
the C/N architecture complete. If the soil/litter/mineral-N receiving owner
does not exist canonically, create `SC-BIOGEOCHEM-001` unless an existing
contract demonstrably owns that state without semantic distortion. Define its
boundary sufficiently to support vegetation N requests, authorized uptake,
litter/CWD C/N receipts, exact ownership, and atomic transaction semantics.
Complete soil decomposition/mineralization equations may be a separately named
implementation dependency only when their ownership, inputs, outputs, state,
and release gate are fully specified and no temporary nutrient source is
authorized.

## Model-Stack Selection Rules

Select one coherent canonical stack. Candidate lineages include, but are not
limited to:

- two-stream or multilayer direct/diffuse canopy radiation;
- sunlit/shaded C3 canopy scaling;
- Farquhar-von Caemmerer-Berry C3 photosynthesis;
- independently admitted temperature-response functions;
- a photosynthesis-linked stomatal formulation such as the Medlyn family;
- an explicitly solved leaf energy balance;
- a plant-hydraulics formulation with layer-resolved roots;
- a dynamic Rutter-family interception store, with Gash as comparator or
  limiting/event evidence where appropriate;
- BIOME-BGC/RHESSys-family vegetation C/N pools, allocation, respiration,
  turnover, litter, and nutrient-demand architecture.

These are candidate lineages, not pre-approved equations. Read the primary
sources and exact established-model specifications. Select and admit exact
equations, parameter semantics, branches, units, domains, guards, constants,
and numerical procedures.

Do not create runtime switches among multiple uncalibrated scientific models.
Rejected alternatives belong in the model-selection artifact with reasons.
A later model version may prospectively admit another stack.

A pinned and documented reference-model implementation may establish exact
`REFERENCE_MODEL_DEFINITION` behavior. That is distinct from claiming universal
natural truth or predictive validation. Classify every authority item as one
of:

- `PRIMARY_PROCESS_AUTHORITY`;
- `REFERENCE_MODEL_DEFINITION`;
- `OPENWEPP_CANONICAL_SELECTION`;
- `PHYSICAL_OR_DIMENSIONAL_INVARIANT`;
- `CALLER_PARAMETER`;
- `ASSUMED_FOR_EXECUTION`;
- `EMPIRICAL_CALIBRATION_EVIDENCE`;
- `INDEPENDENT_VALIDATION_EVIDENCE`;
- `COMPARATOR_ONLY`;
- `REJECTED_SOURCE_BEHAVIOR`.

## Reference Acquisition and Rights

Reference acquisition is current package scope and precedes binding equation
admission.

For every load-bearing source:

1. Add or update `references/annotated_bibliography.md` before using the source
   as authority.
2. Record complete citation, DOI/version, acquisition route, access date,
   reference quality, contract/process mapping, rights classification, local
   path, SHA-256 of the exact reviewed bytes, and page/table/equation locators.
3. Place affirmatively redistributable artifacts under
   `references/vendorable/`.
4. Place copyrighted, restricted, or rights-ambiguous artifacts only under
   gitignored `references/copyrighted/`.
5. Never commit restricted full text.
6. Default rights to restricted until redistribution is affirmatively proven.
7. Preserve source/license notices for any vendored source-derived material.
8. Treat abstracts, search snippets, citation strings, inaccessible equations,
   secondary summaries, mutable web pages without captured identity, and remote
   bytes without checksum as discovery evidence, not binding equation
   authority.
9. Prefer stable author manuscripts, institutional repositories, government
   archives, model technical notes, or open-license versions when the version
   can be shown to contain the exact cited equations.
10. Record unsuccessful acquisition attempts and whether another independently
    sufficient source closes the same equation family.

Verify and reuse the existing local/checksummed corpus where exact identities
remain valid, including Gash, Shuttleworth-Wallace, Jarvis, Stewart, Kelliher,
JULES, Forrester, Bonan, and Pereira.

Reacquire and checksum the previously remote-only Farquhar 1980,
de Pury-Farquhar 1997, White 2000, and Wullschleger 1993 materials before using
them as binding authority.

Acquire or verify exact sources for the selected radiation, dynamic
interception, photosynthesis-linked stomatal, temperature-response,
plant-hydraulic, respiration, allocation, turnover, retranslocation,
mineral-N, litter/CWD, and soil-biogeochemical formulations.

## Required Reading

### Core

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/standards/prompt-wording-guidance.md`
- `/home/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/home/workdir/openWEPP/docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`
- `/home/workdir/openWEPP/docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `/home/workdir/openWEPP/docs/governance/reference-vendoring-policy.md`
- `/home/workdir/openWEPP/references/README.md`
- `/home/workdir/openWEPP/references/annotated_bibliography.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/authority-gap-register.md`
- `/home/workdir/openWEPP/docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/package.md`

### Conditional

Read these before any corresponding canonical or validation edit:

- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-spec.md`
- `/home/workdir/openWEPP/docs/specifications/unit-governance.md`
- `/home/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
- `/home/workdir/openWEPP/references/rights_classification_first_pass_2026-05-11.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`
- `/home/workdir/openWEPP/tests/integration/vegetation_boundary_authority_contract.rs`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/source-function-state-inventory.md`
- `/home/workdir/openWEPP/docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/literature-acquisition-log.md`
- `/home/workdir/openWEPP/docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/artifacts/primary-source-ledger.md`

### On Demand

- `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, `SC-WATBAL-001`,
  `SC-PLANT-001`, `SC-RESIDUE-001`, `SC-SNOWFREEZE-001`,
  native-management/input contracts, and any new `SC-BIOGEOCHEM-001`.
- Exact RHESSysEastCoast source functions and GIS2RHESSys profile/generator
  surfaces implicated by each selected process family.
- Established-model source and technical documentation used as
  `REFERENCE_MODEL_DEFINITION`.
- Primary literature for each selected equation, parameter family, numerical
  algorithm, and validity domain.
- Current roadmap, backlog, contract indexes, authority registries, external
  authority suites, and successor implementation packages when their lifecycle
  is amended.
- Existing canopy-phenology implementation and contracts when resolving GSI,
  foliar carbon, SLA, LAI, allocation, and turnover ownership.

Required-reading budget: initial carry-forward estimate `708970` local Core
bytes, `WARN`; recompute the exact package-specific total immediately after
scaffolding and before authority amendments. Update this prompt and
`artifacts/required-reading-map.md` with the measured total and threshold
disposition. External literature and source-checkout bytes remain on-demand and
are excluded until their process trigger applies.

## Required Package Artifacts

Scaffold at least:

- `artifacts/required-reading-map.md`
- `artifacts/reference-acquisition-ledger.md`
- `artifacts/reference-rights-and-checksum-disposition.md`
- `artifacts/supported-domain-and-exclusions.md`
- `artifacts/model-stack-selection.md`
- `artifacts/equation-authority-ledger.md`
- `artifacts/parameter-and-configuration-manifest.md`
- `artifacts/state-ownership-and-transaction-ledger.md`
- `artifacts/numerical-solver-and-convergence-contract.md`
- `artifacts/rhessys-source-deviation-disposition.md`
- `artifacts/authority-gap-disposition.md`
- `artifacts/test-vector-ledger.md`
- `artifacts/calibration-readiness-matrix.md`
- `artifacts/contract-amendment-evidence.md`
- `artifacts/contract-test-evidence.md`
- `artifacts/pre-implementation-authority-gate.md`
- `artifacts/successor-release-decision.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-finding-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

Artifacts must use truthful `Static`, `Ran`, or combined evidence labels.

## Execution

1. Scaffold the package under the exact ID above, including package, prompt,
   active/archive prompt directories, artifact placeholders, lifecycle catalog
   entry, declared write set, implementation intent, security-impact gate,
   review requirements, and terminal criteria.

2. Freeze exact Git HEAD, worktree status, source-checkout commits/licenses,
   instruction chain, current authority-gap population, predecessor identities,
   intended write set, and recomputed required-reading budget.

3. Complete the reference-acquisition and rights phase. Reuse valid existing
   local sources, reacquire non-binding remote-only sources, obtain missing
   load-bearing sources, checksum exact bytes, update the canonical
   bibliography and rights records, and record exact supporting locators.

4. Define the supported model domain and select one coherent stack. Compare
   candidate formulations using scientific scope, coupling completeness,
   dimensional consistency, numerical behavior, parameter observability,
   mixed-stratum composition, source defects, and compatibility with openWEPP
   ownership. Select one formulation per family. Reject alternatives explicitly
   rather than retaining an unresolved menu.

5. Author the complete equation-authority ledger. For every equation and
   algorithm record:
   - canonical symbol and runtime alias;
   - exact equation;
   - units and dimensional check;
   - leaf, stratum-covered-area, ground-area, soil-layer, or transaction-area
     basis;
   - rate versus interval-integrated amount;
   - forcing and state operands;
   - fixed constants versus caller parameters;
   - mathematical and supported-model domains;
   - branch order;
   - zero and limiting behavior;
   - guard and typed failure;
   - source locator and authority classification;
   - numerical method, convergence, tolerance, and iteration limit;
   - independent reconstruction vector.

6. Author the canonical parameter and state manifests. Include all vegetation
   water, hydraulic, carbon, nitrogen, geometry, phenology, interception,
   litter/CWD, and mineral-N surfaces. Distinguish configuration, initial state,
   evolving state, physical constants, model-version constants, caller values,
   calibratable values, diagnostic values, and prohibited hidden defaults.

7. Define complete ownership and transaction sequencing for radiation, canopy
   liquid water, soil water requests, plant hydraulic state, actual
   transpiration, carbon gain, respiration, allocation, N demand and uptake,
   litter/CWD transfers, and recipient state. Preserve exact-one mutation and
   atomic failure across vegetation, hydrology, land-surface energy, residue,
   and biogeochemistry owners.

8. Amend canonical contracts before implementation authority is released.
   Upgrade `SC-VEGETATION-001`; create `SC-BIOGEOCHEM-001` when required; make
   bounded amendments to adjacent contracts where their ownership or handoff is
   affected. Package artifacts are evidence and cannot substitute for canonical
   contract text.

9. Add contract-derived tests and an independently executable package-local
   reference calculation surface for nondegenerate numerical vectors. The
   reference calculator is an authority/test oracle, not production code.
   Include vectors that distinguish plausible wrong equations, source defects,
   area-basis aliases, rate/amount errors, sign errors, dropped coefficients,
   hidden floors, incorrect stratum aggregation, and water/C/N double counting.

10. Required integrated vector families include:
    - zero light, zero leaf carbon, zero LAI, saturated light, Rubisco-limited,
      electron-transport-limited, and transition cases;
    - coupled assimilation, stomatal conductance, leaf temperature, VPD, and
      transpiration convergence;
    - wet versus dry canopy and finite interception-store closure;
    - deciduous, evergreen, and vertically overlapping mixed strata;
    - two distinct root profiles under the same soil-layer state;
    - dry, frozen, fully supplied, and competing water-withdrawal cases;
    - carbon gain, maintenance respiration, growth respiration, allocation,
      storage, and turnover closure;
    - nitrogen demand, retranslocation, uptake limitation, competing demand, and
      C:N closure;
    - leaf fall and root turnover with exact donor/receiver C, N, and dry-matter
      reconstruction;
    - canopy reduction with unchanged floor operands, proving lost canopy demand
      is not donated to floor evaporation;
    - non-convergence and invalid-state cases proving byte-identical rollback.

11. Reconcile every existing `AUTH-RHEC-*` and `GAP-VEGETATION-*` row against
    the selected model. Historical predecessor artifacts remain unchanged.
    Record prospective supersession in the new package and canonical contracts.

12. Amend the held
    `20260808-rhessys-east-coast-coupled-vegetation-slice-001` package or
    prospectively replace it with a clearly named implementation successor.
    The successor must target the entire admitted coupled state machine even if
    code delivery is phased. It must not authorize a permanent water-only,
    phenology-only, immutable-N, or diagnostic-photosynthesis endpoint.

13. Update `docs/ROADMAP.md`, `docs/backlog/TRACKER.md`, the RHESSys vegetation
    backlog note, the work-package catalog, contract index, and all affected
    lifecycle records to reflect the exact release or hold condition.

14. Reconcile the exact terminal diff and run every applicable documentation,
    rights, checksum, contract schema, unit-governance, contract-derived test,
    authority-suite anti-evasion, focused nextest, formatting, lint, and
    critical full-workspace requirement directly. Record exact commands,
    commit/blob identities, results, durations, and limitations.

15. Complete two independent science reviews:
    - reviewer A: radiation, interception, energy balance, photosynthesis,
      stomatal conductance, aerodynamics, hydraulics, root uptake, water/energy
      closure, and numerics;
    - reviewer B: phenology ownership, vegetation C/N pools, respiration,
      allocation, retranslocation, turnover, nutrient arbitration,
      litter/CWD/biogeochemistry boundaries, parameter authority, and
      calibration claims.

16. Disposition every review finding as `accepted`, `rejected`, `deferred`, or
    `follow-up`, with rationale. Fix accepted findings and rerun every
    invalidated gate. No finding may remain undispositioned.

17. Complete two independent terminal verifications against the final exact
    bytes, archive the kickoff prompt byte-for-byte, perform line-count
    governance, and disposition the package truthfully.

## Hard Release Criteria

Do not release the implementation successor unless:

- every required in-domain constitutive family has a complete canonical
  equation, state, parameter, numerical, guard, and test-vector definition;
- the source set has exact identity, rights disposition, checksums, and
  supporting locators;
- all selected parameter fields are typed and no hidden default remains;
- photosynthesis is causally coupled to stomatal conductance, water use, carbon
  gain, and future vegetation state;
- actual LAI derives from coupled vegetation state rather than an independent
  terminal GSI realization;
- vegetation C/N, nutrient requests and receipts, litter/CWD transfers, and
  receiving-owner boundaries are included;
- water, energy, carbon, nitrogen, and dry-material closure are independently
  reconstructible;
- the agricultural PMET redistribution is explicitly impossible in the native
  path;
- the schema supports deciduous, evergreen, and mixed multistratum stands
  without profile averaging;
- unsupported lifeforms and processes fail explicitly;
- contract-derived and independent numerical vectors pass;
- dual reviews have no unresolved material findings;
- all selected validation requirements pass on the final exact diff.

`HOLD` is legitimate only for a named, load-bearing source or contradictory
authority that cannot be resolved within the authorized acquisition and
canonical-selection routes. If `HOLD` is claimed, create a hold-legitimacy
artifact naming the exact missing equation or specification, acquisition
attempts, why no independently sufficient authority exists, the selected
boundary it blocks, and the first concrete lift action. Package size,
implementation effort, absent site calibration, unavailable universal default
values, or the existence of reasonable alternatives are not hold boundaries.

No surrogate physics: do not authorize source-observed defects, convenient
defaults, guessed constants, empirical stand-ins, temporary total-ET scalars,
uncoupled diagnostic photosynthesis, hidden conductance floors, arbitrary
clamps, fixed unbounded iterations, or canonicalize-and-proceed behavior.
Missing required authority must remain explicit until resolved.

Conservation/output acceptance: record operand lineage and area/interval bases;
use poison vectors separating every plausible alias; explicitly reject known
wrong formulas; independently reconstruct water, energy, C, N, and dry-material
ledgers; and require real closure. Producer self-consistency and one-sided
bounds are supporting evidence only.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to:
- one canopy-energy-gas-exchange-hydraulics science reviewer, with bounded write
  access only to `artifacts/review_agent_a.md`;
- one carbon-nitrogen-biogeochemistry science reviewer, with bounded write
  access only to `artifacts/review_agent_b.md`;
- one `comparator_suite_runner` for heavy full-workspace, authority-suite, and
  broad validation commands, with bounded write access only to package log
  artifacts;
- two independent terminal verifiers, with bounded write access only to their
  respective verification artifacts.

Subagent requirement: REQUIRED. Spawn the two independent science reviewers,
the comparator suite runner for all selected heavy batch/closure commands, and
the two independent terminal verifiers. The parent agent must not run heavy
full-workspace commands itself when the comparator runner is available. If
delegation is genuinely blocked by tool policy or spawn failure, record
command-level evidence; do not claim the missing independent gate or close the
package.

Truthfulness: distinguish `Static` from `Ran` evidence. A source citation,
abstract, equation name, model reputation, source-code match, compile check,
self-consistency identity, or passing documentation assertion is not by itself
evidence that the complete model-stack authority gate passed.

Do not push or create a remote branch unless separately authorized by the
invoking user/session.
