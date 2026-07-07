# LANED-T3 Ratification And Implicit Solve-Cost Package

Status: EXECUTED-HOLD-CASE4-HYBRID-LADDER

## Objective

Execute the parent LANED-T3 package's two open ratification gates, land the
next implicit solve-cost lever authorized by the parent handoff, and scaffold
delegable Tier-1/Tier-2 follow-on packages.

Parent package:
`docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/`

## Scope

Included:
- Contract-first amendment of `SC-OFEROUTE-001` for deterministic implicit
  solve-cost diagnostics/seed rules and any ratified hybrid tolerance posture.
- Case-4 hybrid oracle ladder execution and evidence.
- Fidelity-tolerance execution against the parent I1 ladder and H2637 active
  hybrid evidence.
- Implementation of a deterministic implicit solve-cost reduction lever only if
  it preserves the rev-29 branch/determinism constraints.
- Package-local review, verification, timing/comparator evidence, disposition,
  and worker handoff.
- Scaffold-only Tier-1 and Tier-2 work packages.

Excluded:
- Default promotion / D16.
- New surrogate physics or empirical compatibility wrappers.
- Mesh-resolution policy changes beyond scaffolded Tier-2 planning.
- Production changes outside the Lane-D active hybrid implicit selector.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/gate-results.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i1-implicit-stepper-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md`
- `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/package.md`
- `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md`
- `docs/standards/prompt-wording-guidance.md`
- Package-local `artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if contract
  schema/profile repair is needed beyond the local amendment pattern.
- `docs/standards/local-ci-gate-selection.md` when selecting narrowed
  iteration gates before final closure.

On-demand:
- D10B/D15A artifacts and tests when comparator or active-lane endpoint evidence
  needs lineage back to those packages.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/*.rs` and focused
  tests for touched mechanisms.

Required-reading budget:
- local_required_bytes_total: 208462
- threshold_outcome: OK (`<=400000` bytes)
- map: `artifacts/required-reading-map.md`

## Write Set

Primary:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs` only if profile output
  schemas need the new diagnostic counter.
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/`
- `docs/work-packages/README.md`

Scaffold-only follow-on package write set:
- `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/`
- `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/`

## Phase Plan

### Phase A - Scaffold And Authority

Create package directories, active prompt, required-reading map, initial gate
table, and catalog entry. Confirm clean starting tree and current parent-package
hold state.

### Phase B - Contract-First Ratification/Coverage Amendment

Amend `SC-OFEROUTE-001` before code edits. The amendment must preserve the
rev-29 deterministic-rating and rev-30 deficit-carry constraints, define the
allowed solve-cost diagnostic/seed behavior, and specify whether hybrid selector
promotion remains blocked or is ratified by this package's evidence.

### Phase C - Implicit Solve-Cost Lever

Implement only a deterministic, branch-local cost lever. The intended first
lever is within-step warm seeding of implicit branch equilibrium solves plus
profile counters for implicit equilibrium map evaluations. Seeds must be derived
from the same step's already-owned upstream march state, must stay basin/branch
locked, and must not change ledger ownership or introduce fallback wrappers.

### Phase D - Parent Ratification Gates

Run and record:
- Case-4 hybrid oracle ladder.
- Fidelity-tolerance adjudication from the parent I1 ladder plus current H2637
  active hybrid evidence.
- H2637-class active hybrid endpoint timing with the exact release runner
  binary provenance.

If either ratification gate fails or lacks sufficient evidence, stop at
`EXECUTED-HOLD-*` and record the blocker, evidence, considered in-envelope
route, and first follow-on.

### Phase E - Delegable Tier-1/Tier-2 Scaffolds

Create package skeletons for:
- Tier-1 local numerics (friction/alpha evaluation reductions, Newton residual
  solve assessment, pow/branch-local math reductions).
- Tier-2 mesh-resolution adjudication (5-cell production mesh policy, Case-4
  and H2637 fidelity/timing evidence).

The scaffolds must be executable work-package specs, not only notes.

### Phase F - Review, Verification, And Disposition

Complete dual review, dual verification, line-count governance, gate results,
disposition, final disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` subagents for comparator/timing execution,
code correctness review, QA/maintainability review, and bounded codebase
questions. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact timing or
comparator metrics with command/log paths. Write access is read-only for
review/verification/comparator subagents unless a worker is explicitly assigned
a disjoint implementation write set.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs,
including full workspace `nextest`, release timing runs, comparator ladders, and
`cargo deny check`, unless the subagent tool is unavailable. If unavailable,
record the tool-policy block before running locally.

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks required by touched `SC-*` contracts
- Focused `ofe_routing` and Lane-D active tests
- Case-4 hybrid oracle ladder
- Fidelity-tolerance adjudication
- H2637-class active hybrid timing run with
  `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1`
- Protected-output byte identity with subsystem off if any default/off surface is
  touched
- Implicit solve-cost counter evidence before/after the lever when profiling is
  enabled
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
- Case-4 hybrid ladder passes the contract acceptance criteria.
- Fidelity tolerances are ratified in `SC-OFEROUTE-001` with current H2637 and
  I1 evidence.
- The solve-cost lever is contract-backed, tested, deterministic, and either
  improves or honestly characterizes timing/counter evidence.
- Tier-1/Tier-2 packages are scaffolded and discoverable from the catalog.
- All required gates are `PASS` or explicitly non-applicable with evidence.

Hold if:
- Case-4 hybrid ladder fails.
- Fidelity tolerances cannot be ratified without additional authority/evidence.
- The solve-cost lever changes physics, branch ownership, or publication
  semantics outside the package envelope.
- Heavy required gates cannot be run or reviewed and no equivalent evidence is
  available.
