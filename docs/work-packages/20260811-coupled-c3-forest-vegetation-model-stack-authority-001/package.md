# Coupled C3 Forest Vegetation Model-Stack Authority

Status: `complete / implementation-authority released`

Date: `2026-08-11`

Package ID: `20260811-coupled-c3-forest-vegetation-model-stack-authority-001`

Plan class: `Critical contract-first scientific-authority admission`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Select and canonically admit one coherent, implementation-ready coupled model
stack for vascular C3 woody vegetation. The admitted state machine must cover
distinct evergreen, deciduous, and vertically overlapping mixed strata;
direct/diffuse radiation; liquid interception; aerodynamic and boundary-layer
transfer; bounded C3 photosynthesis; photosynthesis-linked stomata; solved leaf
energy balance and transpiration; plant hydraulics and layer-root acquisition;
persistent vegetation carbon and nitrogen; allocation, respiration,
retranslocation, turnover, litter/CWD transfers, mineral-N requests/receipts;
and exact water, energy, carbon, nitrogen, and dry-material ledgers.

The package closes the selected scientific-authority boundary and releases a
contract-first implementation successor. It does not author production Rust,
select runtime behavior, calibrate a site, deploy, publish, or message external
parties.

## User Outcome

At terminal completion, one exact model version—not a menu of alternatives—has
canonical equations, typed schema, state ownership, numerical algorithms,
guards, test vectors, and digest-bound source authority sufficient for a later
implementation package to build the entire coupled state machine in stable
phases. Caller-supplied values and initial state remain separate from openWEPP's
ownership of meanings, equations, domains, constants, algorithms, and closure.

## Implementation Intent And Risk

Intent is `science-authority admission + contract-derived test oracle +
calibration-readiness assessment only`. Calibration readiness is not a current
closure objective or claim. This package makes no empirical calibration or
independent-validation claim. Risk is `Critical` because it changes canonical
kernel authority and releases a successor for an interdependent water, energy,
carbon, nitrogen, and dry-material state machine.

## Supported Domain

The selected version supports vascular C3 woody evergreen and deciduous tree or
shrub strata, including explicit non-averaged mixed vertically overlapping
stands. It includes direct/diffuse radiation and PAR, nondegenerate canopy
scaling, coupled leaf gas exchange/energy balance, interval-equilibrium plant hydraulic
state, layer-resolved water acquisition, rainfall interception, persistent C/N
state, and exact transfer ledgers.

Typed rejection is required for C4 vegetation, nonvascular strata, crops,
canopy snow, recruitment, succession, fire, and catastrophic disturbance.
`AUTH-RHEC-012` remains an explicit canopy-snow deferral;
`AUTH-RHEC-013` remains licensing/provenance authority.

## Binding Architecture

- Photosynthesis causally participates in stomatal conductance, transpiration,
  carbon gain, allocation, respiration, future leaf area, roots, and litter.
- Current GSI may supply timing/activity, while leaf C, SLA, N, allocation, and
  turnover own foliar mass and LAI.
- Every stratum retains identity, cover, vertical position, state, parameters,
  radiation, roots, and transactions; mixed forests are never averaged rows.
- Site parameters, topology, and complete compatible initial state are required
  caller configuration with no openWEPP universal defaults.
- Vegetation never mutates hydrologic soil storage directly. Water and nutrient
  acquisition use typed request, arbitration, receipt, finalization, and atomic
  commit.
- Native forest forbids agricultural WEPP `Kcb`/LAI PMET redistribution.
  Reduced canopy demand is not donated to forest-floor or soil evaporation.
- Canopy transpiration, wet-canopy evaporation, and forest-floor evaporation
  retain independent energy, resistance, phase, area, and interval lineage.
- Package artifacts are evidence; canonical `SC-*` contracts own authority.

## Authority Classifications

Every authority row uses exactly one of `PRIMARY_PROCESS_AUTHORITY`,
`REFERENCE_MODEL_DEFINITION`, `OPENWEPP_CANONICAL_SELECTION`,
`PHYSICAL_OR_DIMENSIONAL_INVARIANT`, `CALLER_PARAMETER`,
`ASSUMED_FOR_EXECUTION`, `EMPIRICAL_CALIBRATION_EVIDENCE`,
`INDEPENDENT_VALIDATION_EVIDENCE`, `COMPARATOR_ONLY`, or
`REJECTED_SOURCE_BEHAVIOR`.

## Included Scope

- Read-only inspection of pinned `/workdir/RHESSysEastCoast` and
  `/workdir/GIS2RHESSys` checkouts.
- Read-only acquisition of permissible load-bearing scientific literature.
- Reference bibliography, rights, exact-byte checksum, acquisition, and
  locator records; redistributable corpus additions and local restricted cache.
- Complete selected-stack, equation, parameter, state, transaction, solver,
  guard, convergence, and reconstruction authority.
- Canonical `SC-VEGETATION-001` amendment and a new
  `SC-BIOGEOCHEM-001` unless an existing contract demonstrably owns the
  required receiving state and atomic transactions without semantic distortion.
- Minimum bounded adjacent contract amendments required for ownership/handoffs.
- Contract-derived tests, fixtures, and a package-local independently
  executable reference calculator; these are authority/oracle surfaces, not
  production code.
- Reconciliation of every `AUTH-RHEC-*` and `GAP-VEGETATION-*` row.
- Prospective replacement/amendment of the held coupled implementation package
  and lifecycle/catalog/backlog/roadmap records.

## Excluded Scope

- Production vegetation Rust, runtime selection, model activation, consumer
  cutover, deployment, publication, remote branches, pushes, and external
  messages.
- Universal parameter defaults, site calibration, validation, transferability,
  or recommended vegetation profile values.
- Temporary water-only, phenology-only, immutable-N, diagnostic-photosynthesis,
  no-material-mutation, or total-ET-scalar authorities.
- Surrogate/provisional/proxy/heuristic physics, hidden conductance floors,
  guessed constants, arbitrary clamps, fixed unbounded iteration, and
  canonicalize-and-proceed on domain violations.

## Pinned Inputs

- openWEPP base commit at intake: `669aafb60df3ac4eeed2661cc4db4ad33f3f2265`.
- RHESSysEastCoast: `375c75b1cd2202217651dff43aa113d80b9c1118`,
  MIT license, read-only.
- GIS2RHESSys: `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18`,
  MIT license, read-only.
- Canonical legacy WEPP baseline remains
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` only where an affected existing
  contract requires it; the selected forest stack is independently admitted.

## Intended Write Set

- This package tree.
- `references/annotated_bibliography.md`, tracked reference metadata and rights
  records, `references/vendorable/**`, and gitignored
  `references/copyrighted/**`.
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`, new
  `SC-BIOGEOCHEM-001.md`, the contract index, and only bounded ownership/handoff
  amendments to `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, `SC-WATBAL-001`,
  `SC-PLANT-001`, `SC-RESIDUE-001`, or `SC-SNOWFREEZE-001` when required.
- Contract-derived integration tests, package-local reference-calculation
  source/data, and bounded fixtures/authority-suite bindings needed to assert
  the admitted contracts.
- `docs/ROADMAP.md`, `docs/backlog/TRACKER.md`,
  `docs/backlog/20260806-rhessys-derived-vegetation-crate.md`,
  `docs/work-packages/README.md`, and affected lifecycle records.
- Prospective amendments to
  `docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/`
  or a newly scaffolded whole-state-machine implementation successor.

No production crate or runtime file is authorized.

## Security, Rights, And Data Impact Gate

Risk is `none` for secrets/credentials and `material` for copyright custody.
Network use is read-only literature retrieval only. Rights default to
restricted until affirmative redistribution proof. Restricted/ambiguous full
text stays under gitignored `references/copyrighted/`; no restricted bytes may
enter the Git diff. Vendored material retains source/license notices. Every
binding source requires exact local bytes, SHA-256, access date, rights class,
version/DOI, route, and page/table/equation locator.

## Dependencies And Required Reading

The exact tiered map and measured byte budget live in
`artifacts/required-reading-map.md`. Intake Core is exactly `795845` local
bytes, disposition `WARN`; the triggered Conditional set is `148873` bytes and
is tracked separately. Core, conditional, and triggered on-demand
documents listed by the kickoff are binding. Contract and test edits trigger
the science-authoring/profile/spec/unit/correctness/testing documents and local
instruction files before those edits.

## Phase Plan

1. Scaffold package/prompt/artifacts/lifecycle entry; freeze commits, licenses,
   worktree, instructions, predecessor identities, gap population, intended
   write set, implementation intent, and exact required-reading budget.
2. Acquire/verify load-bearing references, update bibliography and rights
   records before authority use, checksum reviewed bytes, and document failed
   acquisition routes.
3. Freeze supported domain/exclusions; compare candidate families and select
   exactly one coherent coupled stack, recording explicit rejections.
4. Author equation authority, parameter/configuration, state ownership,
   transaction sequencing, numerical solver, convergence, guard, conservation,
   and independent reconstruction ledgers.
5. Amend canonical contracts before tests or successor release, including the
   biogeochemistry receiving-owner boundary when existing authority is
   insufficient.
6. Add contract-derived tests, fixtures, and an independently executable
   package-local reference calculation surface with all required poison,
   limiting, multistratum, arbitration, transfer, closure, and rollback vectors.
7. Reconcile every authority/gap row and amend or replace the held successor so
   every implementation phase targets the one admitted state machine.
8. Update lifecycle records and reconcile the exact terminal diff against the
   declared intent/write set.
9. Run cheap/focused gates locally and delegate every selected heavy batch,
   authority-suite, broad, and Critical full-workspace command to the required
   `comparator_suite_runner`.
10. Delegate the two bounded independent science reviews, disposition every
    finding, fix accepted findings, and rerun invalidated gates.
11. Delegate two independent terminal verifications against final exact bytes,
    archive the kickoff prompt byte-for-byte, complete line-count governance,
    and record terminal disposition.

## Equation And Test Completeness Gate

Every in-domain equation/algorithm row must bind canonical/runtime symbols,
exact equation, units and dimensional proof, leaf/stratum/ground/layer/
transaction area basis, rate/amount cadence, forcing/state operands, constant
versus caller parameter role, mathematical and supported domains, branch order,
limiting behavior, typed failure, source locator/class, numerical method,
convergence tolerance/iteration bound, and an independent reconstruction
vector.

Required vectors cover light/LAI/photosynthetic limitations and transition;
coupled gas exchange/leaf temperature/VPD/transpiration convergence; wet/dry
canopy closure; deciduous/evergreen/overlapping strata; distinct roots with
common soil state; dry/frozen/supplied/competing water; C allocation and
respiration; N demand/retranslocation/limitation/competition; exact litter/CWD
donor-receiver C/N/dry-mass; no forest-floor donation; and invalid-state or
non-convergence byte-identical rollback.

## Validation Plan

Exact commands are refined after terminal-diff reconciliation under
`docs/standards/testing-and-gate-strategy.md`. Minimum selected families are:

    markdown-doc lint --path docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001 --format plain
    markdown-doc lint --path docs/specifications/science-contracts --format plain
    bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
    bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md
    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test vegetation_boundary_authority_contract --profile quick
    cargo nextest run --test auth11_required_suite_obligation_guards_contract
    cargo nextest run --workspace --profile quick
    cargo nextest run --workspace --profile full
    cargo fmt --all -- --check
    cargo clippy --test vegetation_boundary_authority_contract -- -D warnings
    cargo test --doc --workspace
    cargo deny check
    git diff --check

Rights/checksum integrity, bibliography path integrity, contract schema/profile,
unit governance, reference-calculator vectors, and exact owned-file/write-set
checks are also mandatory. Terminal diff reconciliation selects warnings-denied
Clippy on the only changed Rust target. The attempted workspace/all-targets
Clippy result and its base-identical land-surface-energy warning remain
diagnostic evidence, not silently reclassified closure evidence. Critical full
and broad commands are delegated.

## Review And Delegation Requirements

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one canopy-energy-gas-exchange-hydraulics science
reviewer with bounded write access only to `artifacts/review_agent_a.md`; one
carbon-nitrogen-biogeochemistry science reviewer with bounded write access only
to `artifacts/review_agent_b.md`; one `comparator_suite_runner` for all heavy
full-workspace, authority-suite, and broad validation commands with bounded
write access only to package log artifacts; and two independent terminal
verifiers with bounded write access only to their respective verification
artifacts. These roles are required. The parent must not run heavy commands
while the comparator runner is available.

Every finding is dispositioned `accepted`, `rejected`, `deferred`, or
`follow-up` with rationale. Accepted findings are fixed and affected gates are
rerun. No finding remains undispositioned. Reviews and verifications check gate
legitimacy, not merely artifact presence.

## Hard Release Criteria

The implementation successor is released only when all in-domain constitutive
families, sources, typed parameters, coupled causal paths, state-derived LAI,
C/N and material transfers, exact ledgers, non-PMET native architecture,
multistratum schema, typed exclusions, independent numerical vectors, dual
reviews, and exact-diff validation requirements pass on final bytes.

`HOLD` is allowed only for a named load-bearing source or contradictory
authority inaccessible/unresolvable after DOI, author manuscript,
institutional repository, government archive, local caches, established-model,
physical/dimensional, and explicit canonical-selection routes. Package size,
effort, absent calibration/defaults, or multiple defensible alternatives are
not hold boundaries. A hold requires a package-local legitimacy artifact naming
the exact blocked equation/specification, attempts, absent alternative, blocked
boundary, and first lift action.

## Line-Count Governance

Record exact changed `.rs` line counts. Files at or above 2000 lines are
`WARN` and require decomposition rationale/follow-up. Nonexempt files at or
above 3000 lines block closure. Generated/fixture exceptions require owner and
sunset. The package-local reference calculator remains test/oracle code but is
still dispositioned.

## Exit Criteria

- Every required artifact is complete and truthfully labeled `Static`, `Ran`,
  or combined.
- `AUTH-RHEC-001..011` and `014..016` are resolved for the selected C3 woody
  domain; `012` is deferred and `013` preserved.
- `SC-VEGETATION-001`, `SC-BIOGEOCHEM-001` when required, and bounded adjacent
  contracts contain the complete canonical authority.
- Contract-derived and independent numerical vectors pass and distinguish
  plausible wrong equations/aliases/double counts.
- The successor targets the complete admitted coupled state machine and no
  temporary independent authority.
- All selected validation commands pass on the exact terminal diff.
- Required dual reviews and terminal verifications have no unresolved material
  findings.
- Prompt archival, line-count, owned-file, lifecycle, handoff, and final
  disposition records are complete.

## Progress

- [x] (2026-08-11) User authorized the exact package, scope, complete coupled
  architecture, literature acquisition, bounded writes, required delegation,
  and end-to-end execution.
- [x] (2026-08-11) Confirmed clean openWEPP and pinned external checkout heads.
- [x] (2026-08-11) Complete scaffold, kickoff capture, lifecycle entry, instruction freeze,
  required-reading map, and exact byte budget.
- [x] (2026-08-11) Complete reference acquisition and rights/checksum custody.
- [x] (2026-08-11) Select the coherent model stack and author all authority/state/numerics
  ledgers.
- [x] (2026-08-11) Amend canonical contracts and add passing contract-derived oracle tests.
- [x] (2026-08-11) Reconcile authority gaps, release the whole-state successor, and update
  lifecycle records.
- [x] (2026-08-11) Complete dual terminal verification and final disposition;
  focused and delegated Critical validation, dual reviews, finding
  disposition, and prompt archival are complete.

## Decision Log

- Decision: the package begins with persistent coupled C/N and photosynthesis,
  not a water-only or phenology-only intermediate authority.
  Rationale: binding user direction requires every implementation phase to
  target one already-admitted state machine.
  Date/Author: 2026-08-11 / user.
- Decision: caller values/state are required typed configuration; science and
  numerical semantics remain openWEPP authority.
  Rationale: implementation authority and calibration/transferability are
  orthogonal under ADR-0042.
  Date/Author: 2026-08-11 / user.
- Decision: canopy snow remains deferred unless complete joint authority is
  admitted; C4 and other unsupported branches fail typed.
  Rationale: preserve explicit boundaries without silently narrowing the C3
  state machine.
  Date/Author: 2026-08-11 / user.

## Surprises And Discoveries

- The exact requested package path did not exist at intake; it is scaffolded
  from the active user authorization rather than treated as prior authority.
- The root work-package catalog alone is 391,787 bytes, making the user-listed
  core set necessarily exceed the normal `WARN` threshold; exact budget is
  measured after this scaffold.

## Outcomes And Retrospective

Scientific selection, canonical amendments, source custody, independent
vectors, delegated Critical gates, dual science reviews, and dual terminal
verification are complete. The package releases implementation authority for
the whole admitted coupled state machine. No production behavior or activation
changed.

## Idempotence And Recovery

All external source checkouts and literature endpoints are read-only. Preserve
unrelated user work. Repeated reference acquisition must verify exact bytes and
must not replace a binding source identity silently. If a gate or review fails,
retain evidence, correct only within the declared write set, and rerun every
invalidated requirement before disposition.
