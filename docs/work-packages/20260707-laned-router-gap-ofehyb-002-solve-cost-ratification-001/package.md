# LANED Hybrid GAP-OFEHYB-002 Solve-Cost Ratification

Status: EXECUTED-COMPLETE-NO-PROMOTION

## Objective

Close, or legitimately hold, `SC-OFEROUTE-002#GAP-OFEHYB-002`: the hybrid
implicit selector is now Case-4-correct after `GAP-OFEHYB-001`, but implicit
cell-solve cost still limits endpoint value and blocks promotion/fidelity
ratification.

Execute the next contract-first solve-cost increment against the current
source-memory hybrid baseline. The package should implement only deterministic,
contract-authorized local numerical reductions, then re-run the promotion
ratification evidence needed by `SC-OFEROUTE-002#INV-OFEHYB-008`.

## Baseline At Package Start

- `GAP-OFEHYB-001` is resolved by the source-memory cooldown predicate.
- Current H2637 active hybrid evidence: `37.96 s` user, `0:37.99` wall,
  `980804` implicit steps, `151435969` implicit equilibrium map evaluations,
  `20110816` implicit branch evaluations.
- At package start, `GAP-OFEHYB-002` remained open: rev-31 H2637 had
  `274.7 M` map evaluations and the source-memory baseline still carried
  `151.4 M` map evaluations.
- The selector remains `experimental-unpromoted`; no default/D16 promotion is
  authorized by this package unless all promotion gates explicitly pass.

## Execution Outcome

`GAP-OFEHYB-002` is closed for the current H2637 source-memory hybrid
solve-cost bottleneck. The package lands and ratifies the exact bare skin-only
branch evaluator, reducing H2637 active hybrid map evaluations
`151435969 -> 0` and user time `38.39 s -> 33.37 s`. The selector remains
experimental/unpromoted; no default/D16 activation is made.

## Scope

Included:
- Contract-first amendment to `SC-OFEROUTE-002` and parent pointer rows only
  if the selected solve-cost lever changes binding algorithm text, guards,
  invariants, BEI rows, or promotion/tolerance posture.
- Baseline refresh on the source-memory hybrid path with exact release-binary
  provenance.
- One coherent solve-cost implementation increment, prioritized as:
  1. Newton or equivalent direct solve for the composed implicit cell residual.
  2. Deterministic equilibrium/friction evaluation reductions that preserve
     `INV-OFEHYB-003`.
  3. Local math reductions only when contract-authorized and tested.
- Retained Case-4 full-hybrid oracle ladder.
- H2637 active hybrid endpoint timing/profile and solve-cost counters.
- Fidelity/timing ratification audit for `INV-OFEHYB-008`.
- Review, verification, gate evidence, disposition, and follow-on handoff.

Excluded:
- Mesh-resolution policy changes; use the Tier-2 package for that.
- Default activation / D16 promotion unless every `INV-OFEHYB-008` promotion
  criterion is ratified in this package.
- Surrogate physics, empirical retuning, tolerance weakening, compatibility
  wrappers, silent fallback, or branch-history-dependent seeding.
- Code outside the Lane-D active hybrid selector unless the contract-first
  optimization plan proves the shared local numeric path is the correct owner.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/verification-h2637-timing.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/ratification-evidence.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/verification-h2637-timing.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i1-implicit-stepper-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md`
- Package-local `artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  schema/profile repair is needed beyond a local amendment pattern.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if adding or reorganizing contract invariant/guard/profile sections.
- `docs/specifications/unit-governance.md` if any runtime symbol, output,
  counter, unit, or metadata surface changes.
- `docs/standards/local-ci-gate-selection.md` when choosing narrowed iteration
  gates before final closure.

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- Runner active-lane/profile files only if timing/provenance surfaces need
  changes.

Required-reading budget:
- local_required_bytes_total: 260236
- threshold_outcome: OK (`<=400000` bytes)
- map: `artifacts/required-reading-map.md`

## Write Set

Primary:
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` only for
  parent-pointer synchronization if `SC-OFEROUTE-002` posture changes.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- Focused tests under the same crate.

Secondary, only if required by diagnostics:
- `crates/openwepp-runner/src/hillslope/`

Protected:
- Do not edit Tier-2 mesh policy files except to reference this package in a
  handoff if needed.
- Do not modify default/off selector behavior except to prove byte identity.

## Phase Plan

### Phase A - Scaffold And Baseline

Confirm clean starting tree, package status, and current `GAP-OFEHYB-002`
contract text. Rebuild the exact release runner, record binary provenance, and
rerun the source-memory active hybrid H2637 timing/profile as the package
baseline.

### Phase B - Optimization Plan

Identify the dominant current implicit solve-cost terms from counters and code.
Choose one coherent lever set that can be defended under `INV-OFEHYB-003`
determinism and `INV-OFEHYB-001` ledger exactness. Record rejected alternatives
and why they are out-of-envelope, lower value, or require a later package.

### Phase C - Contract-First Amendment

If the selected lever changes normative algorithm text, branch/guard behavior,
acceptance posture, counters, or BEI surfaces, amend `SC-OFEROUTE-002` before
code. Preserve:
- branch preference and double-collapse fail-closed behavior,
- finite/positive/branch-side seed acceptance,
- basin-locked acceleration,
- exact ledger guards,
- source-memory switching predicate,
- selector-off/default non-perturbation.

### Phase D - Implementation

Implement the selected solve-cost lever with typed guards and no silent
fallback. Add contract-derived tests before or with production code. Keep
changes local to the hybrid/implicit/friction owner unless the optimization
plan proves a shared path must change.

### Phase E - Timing, Fidelity, And Ratification

Run:
- focused solve/branch/friction vectors,
- source-memory hybrid vectors,
- retained Case-4 full-hybrid oracle ladder,
- H2637 active hybrid release-binary timing/profile,
- fidelity/timing ratification audit for `INV-OFEHYB-008`.

If promotion criteria are not fully met, close as `EXECUTED-HOLD-*` or
`EXECUTED-COMPLETE-NO-PROMOTION` with exact evidence and first follow-on. Do
not partially promote the selector.

### Phase F - Review, Verification, And Disposition

Complete dual review, finding disposition, dual verification, line-count
governance, gate results, final disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` subagents for
timing/comparator execution, release-binary H2637 runs, code correctness
review, QA/maintainability review, bounded codebase questions, and disjoint
implementation subtasks when assigned. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, timing/comparator
artifacts, compact metrics, command lines, and log paths. Write access is
read-only for review/verification/comparator/explorer agents; worker write
access must be explicitly bounded to named files or modules.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs,
including full workspace `nextest`, release timing runs, comparator ladders,
and `cargo deny check`, unless the subagent tool is unavailable. If
unavailable, record command-level evidence before running locally.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/baseline-profile.md`
- `artifacts/optimization-plan.md`
- `artifacts/contract-amendment.md`
- `artifacts/implementation.md`
- `artifacts/timing-and-fidelity.md`
- `artifacts/ratification-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks required by touched `SC-*` contracts
- SC unit compliance checks for touched contracts
- Focused implicit/friction/source-memory hybrid tests
- Focused Lane-D / `ofe_routing` tests
- Retained Case-4 full-hybrid oracle ladder
- H2637-class active hybrid timing/profile with
  `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1`
- Exact release-binary provenance for timing/comparator evidence
- Implicit solve-cost counter evidence before/after the lever
- Fidelity/timing ratification audit for `INV-OFEHYB-008`
- Protected-output byte identity with subsystem off if any default/off surface
  is touched
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `.rs` line-count governance

Authority anti-evasion guard is required if any required-case binding, cohort
fixture, or external-authority suite posture is touched:
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Exit Criteria

Complete only if:
- The selected solve-cost lever is contract-backed or contract-neutral,
  deterministic, branch-safe, and tested.
- Case-4 full-hybrid ladder remains passing at the parent tolerances.
- H2637 active hybrid timing/profile and counters are recorded before/after.
- Fidelity/timing ratification is either explicitly achieved in
  `SC-OFEROUTE-002` or explicitly left unpromoted with a truthful disposition.
- All required gates are `PASS` or explicitly non-applicable with evidence.

Hold if:
- The selected lever changes physics, branch ownership, or publication
  semantics outside contract authority.
- Solve-cost improvement requires mesh-resolution policy, tolerance weakening,
  default activation, or surrogate/proxy physics.
- Fidelity/timing ratification cannot be completed with current authority or
  evidence.
- Heavy required gates cannot be run or reviewed and no equivalent evidence is
  available.
