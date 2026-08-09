# RHESSys East Coast Vegetation Authority Closure

Status: `executed-hold / terminal verified`

Date: `2026-08-09`

Package ID: `20260809-rhessys-east-coast-vegetation-authority-closure-001`

Plan class: `Critical contract-first authority closure when canonical authority changes`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Objective

Close the residual `AUTH-RHEC-001..011` plus `AUTH-RHEC-014/015` authority
blockers left by the executed-hold admission package. Produce one coherent,
reviewed authority boundary for the selected East Coast deciduous and evergreen
strata, explicit mixed-stratum topology, compatible initial state, canopy
water-energy chain, and carbon-root state chain. Release the held implementation
successor only when every residual blocker is canonically admitted with direct
tests and no invented equation, value, bound, alias, initializer, or transfer
rule.

`AUTH-RHEC-016` is already admitted authority. This package must preserve that
decision but must not count its future runtime implementation/tests as an
authority blocker or as work performed here.

## User Outcome

After successful execution, the existing coupled vegetation implementation
package has an exact contract-first boundary it can implement without deciding
science or data semantics inside Rust. If the complete boundary cannot be
supported, this package closes `executed-hold` with a minimum, source-specific
authority need and keeps the implementation successor non-executable. A broad
request for more literature or data is not an acceptable handoff.

## Context And Orientation

The predecessor audit mapped licensed RHESSys East Coast code and GIS profiles
to literature. The subsequent authority-admission package admitted strict local
definition acquisition and typed schema form in `SC-VEGETATION-001` version 3,
but it did not admit complete selected field declarations, selected pine/oak
values, a compatible dated state, or any complete constitutive process family.

The strongest existing state evidence is Coweeta 2005-2006: chestnut oak in
watershed WS18 and eastern white pine in adjacent watershed WS17. Those are
separate observed stands, not one mixed stand. This package treats them as the
first candidate evidence anchors and may use them together only after admitting
an explicit transfer/synthesis rule, uncertainty, and topology. It must never
present their juxtaposition as an observed mixed stand.

The held implementation successor is
`../20260808-rhessys-east-coast-coupled-vegetation-slice-001/`. It remains
non-executable until this package reaches `complete`, explicitly releases it,
and its own contract-first gate passes.

## Authority Closure Envelope

In-scope authority IDs are `AUTH-RHEC-001..011`, `AUTH-RHEC-014`, and
`AUTH-RHEC-015`. The envelope includes primary-source and observed-data
inspection, exact source/rights custody, definition of selected schema entries
and aliases, empirical value/domain adjudication, initial-state observation or
synthesis authority, complete constitutive equations, unit/scale/ownership
rules, guards, and independent test vectors.

Canonical amendment targets are:

- `SC-VEGETATION-001` for vegetation configuration, state, canopy, carbon, and
  Stage A/C authority;
- `SC-LANDSURFACEENERGY-001` only for a complete affected available-energy or
  canopy/ground recipient family;
- `SC-EVAP-001`, `SC-WATBAL-001`, or `SC-PLANT-001` only when their existing
  ownership/compatibility boundary requires a complete reviewed amendment; and
- the corresponding lifecycle index rows and contract-derived tests.

Package evidence is not authority. A family is admitted only after the owning
canonical contract, complete contract cycle, and direct tests agree.

## Frozen Candidate Boundary

- Deciduous stratum candidate: GIS2RHESSys `chestnut.oak.bgc`, ID `805`.
- Evergreen stratum candidate: GIS2RHESSys `eastern.white.pine`, ID `807`.
- Candidate state anchors: Coweeta WS18 oak and WS17 pine, 2005-2006 evidence.
- Mixed forest: explicit separately resolved strata and topology; no averaged
  parameter record and no false co-located-observation claim.
- Vascular C3 vegetation only. C4, nonvascular, and canopy snow remain rejected
  or deferred exactly as in the predecessor.
- Existing generalized openWEPP GSI remains the phenology owner unless a
  complete canonical amendment explicitly supersedes it.
- Hydrology alone mutates soil stores. Vegetation emits layer requests;
  hydrology returns accepted withdrawals through Stage B.
- Land-surface energy owns available-energy operands and latent-energy
  conversion.
- Definitions are caller-supplied local bytes bound to repository, immutable
  commit, repository-relative path, and SHA-256.

The candidate boundary may be replaced only when the replacement is recorded
prospectively in `artifacts/target-boundary-selection.md`, has stronger complete
authority, preserves the user's deciduous/evergreen/mixed outcome, and receives
dual-review acceptance before canonical amendment.

## Included Scope

- Select and bind an exact stand/plot/date/topology/area/age-size domain.
- Complete the selected consumed-field manifest, aliases, units, cadence,
  area/leaf basis, classification, finite domain, sentinel/missing rules,
  ecosystem domain, evidence locator, and prohibited extrapolations.
- Admit or replace every consumed pine/oak value without using source defaults,
  generic biome tables, or numerical resemblance as authority.
- Construct a complete compatible LAI/geometry/root/C/N initial state from
  observations or an independently authoritative synthesis with uncertainty
  and reconstructible operators.
- Close the radiation, interception, aerodynamic, conductance,
  Penman-Monteith, and available-energy chain.
- Close C3 photosynthesis, retained/superseded phenology ownership,
  layer-resolved root demand, respiration, allocation, turnover, and minimum
  persistent carbon-state transitions.
- Amend canonical contracts and contract-derived tests only for complete
  admitted families.
- Prospectively amend and release the held implementation successor only after
  every residual authority ID closes.
- Complete dual independent science review, finding disposition, dual
  verification, direct validation, prompt archival, and lifecycle disposition.

## Excluded Scope And Protected Boundaries

- No production Rust, Cargo, runtime selector, management schema, output,
  activation, deployment, release, publication, calibration claim, independent
  validation claim, or cutover.
- No implementation of admitted `AUTH-RHEC-016`; that remains successor work.
- No canopy snow, C4, nonvascular physiology, nitrogen competition, mortality,
  disturbance, or broad soil-biogeochemistry process family.
- No mutable-network acquisition, hidden parser defaults, source sentinels,
  nonzero floors, arbitrary clamps, nonfinite-to-zero behavior, unbounded
  iteration, direct soil mutation, or defective source equations.
- No transfer of WS17/WS18 state across species, watersheds, dates, or topology
  without explicit authority, uncertainty, and prohibited extrapolation.
- No package-local provisional equation or `ASSUMED_FOR_EXECUTION` value may
  enter canonical production authority.

## Closure Lanes

### Lane 0: Exact Target And Evidence Custody

Freeze one exact target domain before value admission. Record site, plot or
watershed, observation date/interval, horizontal area, topology, age/size
structure, profile identity, custody, rights, checksums, and intended evidence
role. If the Coweeta dual-anchor route cannot support an explicit mixed state,
evaluate a named co-located replacement route rather than broad searching.

### Lane 1: Selected Schema, Aliases, And Values

Start from the predecessor's exact 71-by-two ledger. Produce a complete consumed
manifest including the 53 parser-only dependencies. For each field, bind the
canonical symbol, exact raw key, accepted/rejected aliases, units, cadence,
scale and area basis, parameter class, domain, missing/sentinel policy, primary
locator, selected value or replacement, uncertainty, ecosystem applicability,
and prohibited extrapolations. Every unresolved consumed field blocks Lane 1.

### Lane 2: Compatible Initial State

Admit a dated state with independently reconstructible LAI, height/geometry,
root profile, and required live/dead carbon and nitrogen pools. Distinguish
observed quantities from transformations and synthesis. State transfer,
allometry, stoichiometry, carbon-fraction conversion, and cross-site composition
each require explicit authority and uncertainty. A species profile label is not
an initial state.

### Lane 3: Canopy Water And Energy

Close `AUTH-RHEC-003..007/014` as one operand and ownership chain. Require
direct/diffuse multistratum radiation with optical closure and recipient
lineage; finite-store interception with named releases; aerodynamic and
leaf/canopy conductance scales; complete Penman-Monteith units and constants;
and authoritative canopy/ground available-energy ownership. Locked vectors must
make wrong operand aliases numerically distinct.

### Lane 4: Carbon, Phenology, And Roots

Close `AUTH-RHEC-008..011` with a complete C3 route, selected capacity and
temperature parameters, bounded coupled solve, respiration/allocation/turnover
state transitions, explicit GSI ownership, layer-resolved root requests,
dry/frozen/zero-participation branches, and Stage A/B/C closure. No forced-C3,
fixed-pass, single-depth, direct-store-mutation, or hidden-allocation behavior
is admissible.

## Authority Decision Rules

Each in-scope `AUTH-RHEC-*` ID must finish `ADMITTED` or `BLOCKED` with exact
evidence and a named lift condition. `complete` requires every in-scope ID to be
`ADMITTED`; partial admission closes `executed-hold` and does not release the
successor. Missing implementation effort, package size, or a desire to defer
coupling is not an authority boundary.

Observed data are prospectively `DIAGNOSTIC_ONLY` unless the package explicitly
assigns them to initial-state authority or a future calibration/validation role
before use. No calibration or independent-validation claim is in scope.

## Contract-First Sequence

1. Freeze intent, target candidates, source identities, reading map, and exact
   write set.
2. Complete Lane 0 and obtain a reviewable exact target boundary.
3. Execute Lanes 1-4 without stopping merely because another lane is blocked;
   parameter-independent authority work remains required.
4. Draft canonical contract amendments only for complete families.
5. Add contract-derived tests and independent locked vectors.
6. Complete the package-local contract cycle and pre-implementation contract
   gate.
7. Amend the implementation successor only to the exact admitted boundary and
   release it only if all residual IDs close.
8. Run exact-diff validation, dual science review, finding disposition, dual
   post-fix verification, prompt archival, and terminal lifecycle disposition.

Production code is outside this sequence.

## Milestones

Milestone 1 freezes the selected domain and source custody. It is accepted only
when a reviewer can reproduce every identity and distinguish observed state,
transformed state, external configuration, and candidate synthesis.

Milestone 2 closes the selected schema/value and compatible-state lanes. It is
accepted only when every consumed field and every required initial-state pool
has a terminal authority decision with units, basis, uncertainty, and
prohibited extrapolation.

Milestone 3 closes both coupled constitutive lanes. It is accepted only when all
equations, operands, ownership boundaries, domains, guards, convergence rules,
and independent vectors form complete chains.

Milestone 4 integrates admitted authority into canonical contracts and tests,
completes review/verification, reconciles the exact diff, and either releases
the implementation successor or records a precise executed hold.

## Intended Write Set

- This package tree.
- `docs/work-packages/README.md`, `docs/ROADMAP.md`,
  `docs/backlog/TRACKER.md`, and
  `docs/backlog/20260806-rhessys-derived-vegetation-crate.md`.
- Dependency/status and proven-boundary edits in
  `docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/`.
- `SC-VEGETATION-001`, its index row, and only the minimum adjacent canonical
  contracts required by a complete admitted family.
- Bounded contract-derived tests/fixtures after their canonical authority is
  admitted.
- Rights-compliant bibliography, checksums, and vendorable or ignored source
  custody required by actual acquisition.

No production crate/module, Cargo file, runtime consumer, observed-data
publication, external-authority required-suite registry, or deployment path is
in scope.

## Validation Plan

Execution must declare intent before canonical edits and reconcile the exact
terminal diff before disposition. Minimum scaffold/document checks are:

    markdown-doc lint --path docs/work-packages/20260809-rhessys-east-coast-vegetation-authority-closure-001 --format plain
    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
    markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
    markdown-doc lint --path docs/backlog/TRACKER.md --format plain
    markdown-doc lint --path docs/ROADMAP.md --format plain
    markdown-doc lint --path docs/work-packages/README.md --format plain
    git diff --check

For each canonical contract amendment, run the science-contract schema/profile,
unit-governance, strict Binding Exposure, and affected contract-derived tests.
Select all other requirements directly from
`docs/standards/testing-and-gate-strategy.md` and the exact diff.

A semantic canonical-authority or contract-test change is Critical and requires
an immediate exact-byte full correctness run:

    cargo nextest run --workspace --profile full

If execution closes `executed-hold` with documentation evidence only and no
canonical contract/test/authority surface changes, Rust gates are not selected
solely because the repository contains Rust. Record that exact-diff decision.

## Review, Verification, And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/source reviewers, two
independent read-only terminal verifiers, and a read-only
`comparator_suite_runner` for any selected heavy full-workspace or comparator
run. Expected outputs are compact severity-ranked findings, verdicts, metrics,
and log/receipt paths recorded in package artifacts; the primary executor owns
all writes. Reviewer B and Verifier B must not read their A counterpart before
their initial verdict.

Subagent requirement: REQUIRED for dual science review and dual terminal
verification. If the exact diff selects full workspace, comparator, cohort, or
other heavy batch execution, spawning `comparator_suite_runner` is also
REQUIRED; the parent must not run that heavy command unless the runner is
unavailable and command-level evidence of the block is recorded.

Reviews must verify source/claim alignment, target-domain legitimacy, closure of
every consumed field and state pool, complete constitutive chains, canonical
contract-cycle compliance, rights, exact diff, validation non-deferral,
line-count governance, prompt lifecycle, and the legitimacy of any hold.

## Security, Licensing, Data, And Assurance Impact

Pinned external source checkouts remain read-only. Read-only source discovery is
permitted; no external system mutation, upload, message, deployment, or
publication is authorized. Preserve MIT notices for distributed derived bytes.
Do not track restricted full text, Earthdata credentials, tokens, personal
information, or access-controlled data. Record citations, locators, licenses,
checksums, and derived facts only when redistribution rights are incomplete.

Authority and contract changes may affect future vegetation, ET, hydrology,
land-surface energy, and assurance claims, but this package creates no assurance
approval, publication, campaign transfer, release transfer, or runtime result.

## Exit Criteria

- One exact target domain and evidence-role assignment is frozen and reviewed.
- Every selected consumed field, alias, value, and initial-state pool has a
  terminal authority disposition.
- Every Lane 3 and Lane 4 equation/operand/ownership/guard/vector obligation has
  a terminal authority disposition.
- Every admitted family is canonical, contract-cycle complete, and directly
  tested; no package artifact substitutes for authority.
- Every finding is dispositioned; accepted fixes are independently verified;
  rejected findings have authority-backed rationale.
- Exact diff, reading budget, rights, source identities, validation, line-count
  governance, prompt archive, lifecycle, and assurance impact reconcile.
- Both terminal verifiers pass the claimed disposition.
- `complete` is allowed only when every in-scope residual authority ID closes
  and the successor is explicitly released. Otherwise close `executed-hold`
  with `artifacts/hold-legitimacy-audit.md` naming the minimum external boundary.

## Progress

- [x] (2026-08-09) User authorized the authority-closure package scaffold.
- [x] (2026-08-09) Scaffolded package, active prompt, evidence controls, and
  prospective lifecycle/dependency updates.
- [x] (2026-08-09) Froze pre-implementation intent, pinned source identities,
  candidate profiles, and `DIAGNOSTIC_ONLY` observation roles; the exact target-
  domain gate failed at the recorded co-located/synthesis authority boundary.
- [x] (2026-08-09) Executed Lanes 1-4 without cross-lane deferral and recorded
  terminal `BLOCKED` decisions for every residual authority ID.
- [x] (2026-08-09) Preserved contract-first sequencing: no incomplete family
  entered a canonical contract or contract-derived test.
- [x] (2026-08-09) Completed dual independent science review, accepted and
  fixed every finding, passed direct documentation validation, archived the
  prompt, and reconciled lifecycle and exact diff.
- [x] (2026-08-09) Both independent terminal verifiers passed the corrected
  exact evidence set with no residual findings; finalized executed-hold.

## Surprises And Discoveries

- The strongest existing oak and pine state observations are from adjacent
  Coweeta watersheds, not one mixed stand. Their combination therefore requires
  explicit synthesis/transfer authority rather than simple composition.
- Acquisition authority is already closed as `AUTH-RHEC-016`; repeating it as
  an authority prerequisite would deadlock implementation behind a passed gate.

## Decision Log

- Decision: scaffold one integrated closure package rather than separate
  parameter, state, water-energy, and carbon-root packages.
  Rationale: the selected state and coupled constitutive chains constrain each
  other, while internal lanes retain independent evidence and prevent one
  blocker from deferring separable authority work.
  Date/Author: 2026-08-09 / Codex.
- Decision: retain Coweeta WS18 oak and WS17 pine as candidate evidence anchors,
  not an observed mixed stand.
  Rationale: they are the strongest inspected dated evidence and expose the
  exact synthesis/transfer question without inventing co-location.
  Date/Author: 2026-08-09 / Codex.
- Decision: keep production Rust and `AUTH-RHEC-016` implementation outside the
  authority package.
  Rationale: scientific/data closure must precede implementation, and the
  acquisition authority decision is already canonical.
  Date/Author: 2026-08-09 / Codex.

## Outcomes And Retrospective

The pinned local sources and prior primary-source routes were re-evaluated
through all four lanes. They do not establish a co-located mixed target,
field-level provenance for selected executable values, compatible dated C/N/root
state, or complete water-energy and carbon-root constitutive cycles. The
minimum truthful outcome is an executed hold. No canonical contract, test, or
production file changed, and the implementation successor remains held.

## Idempotence And Recovery

All source inspection is read-only and checksum-bound. Evidence edits are
additive. Re-running selection or validation must preserve earlier failed and
counterevidence records. Never reset unrelated user work or mutate external
checkouts. If a required source cannot be acquired lawfully, record its exact
identity, attempted route, and minimum substitute evidence rather than storing
restricted bytes.

Revision note (2026-08-09): initial authority-closure scaffold created from the
terminal handoff of the executed-hold authority-admission package.
