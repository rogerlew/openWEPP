# High-Risk CQR Tranche A: Active Hillslope And Runner Paths

Status: `ACTIVE`
Roadmap ID: `CQR-PREINT-20260711-HA`
Campaign ledger: `docs/work-packages/cqr-pre-integration-campaign-assessment.md`
Binding execution contract: `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md`
Owner: maintainers

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` throughout execution.

## Purpose / Big Picture

Reduce complexity risk in the active hillslope routing-to-publication chain
before the repository begins broader integrated testing and validation. After
this tranche, the fixed cohort below has no eligible CRAP row above 30, its
science/contract behavior is protected by current characterization, and the
primary hillslope CLI/runner path retains exact output identity.

This plan coordinates ten fixed-module terminal records. Actionable modules
receive independent packages; reviewed no-action modules receive committed
classification records. It never authorizes one ten-module implementation diff.

## Progress

- [x] (2026-07-11 UTC) Refresh workspace LCOV/CRAP and source identity from the current clean commit.
- [x] (2026-07-11 UTC) Complete dual target-selection/eligibility review for all ten modules.
- [x] (2026-07-11 UTC) Execute HA-01 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-11 UTC) Execute HA-02 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-11 UTC) Execute HA-03 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-11 UTC) Pass the HA-01..HA-03 workspace quick-profile checkpoint (1,711/1,711).
- [x] (2026-07-11 UTC) Execute HA-04 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Execute HA-05 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Execute HA-06 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Pass the HA-04..HA-06 workspace quick-profile checkpoint (1,729/1,729).
- [x] (2026-07-12 UTC) Close `DC-CQR-HA07-001` and execute HA-07 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Close `DC-CQR-HA08-001` and execute HA-08 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Execute HA-09 through reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Pass the HA-07..HA-09 workspace quick-profile checkpoint (1,750/1,750).
- [ ] Execute or disposition HA-10 through its terminal record.
- [ ] Run the tranche-final fresh rerank, dual verification, and transition to High B.

## Surprises & Discoveries

- The fixed ledger contains 13 deduplicated rows, not the 14-row narrative count
  used during initial review arithmetic. Both independent reviews accepted the
  correction; all ten modules and every listed row remain actionable.
- Both start coverage formats reproduced only the known parallel-environment
  interference in `laned_shadow_h2637`; target and failure-test sources are
  byte-identical to the prior measurement source.
- Start timing was 34:28.70 for LCOV and 34:19.14 for JSON. Preserve delegated
  execution for later full-workspace coverage passes.

## Decision Log

- Decision: route solver/cascade work precedes active-lane projection,
  execution, publication, and CLI work.
  Rationale: downstream characterization must bind the final behavior of the
  lower-level routing path rather than an intermediate implementation.
  Date/Author: 2026-07-11 / Codex.
- Decision: all ten modules are hard pre-integration blockers, but risk priority
  does not override ADR-0021 symbol eligibility.
  Rationale: a fresh, dual-reviewed classification is required before any raw
  row is suppressed or dispositioned no-action.
  Date/Author: 2026-07-11 / Codex.
- Decision: classify all 13 refreshed rows as actionable; accept no eligibility
  exception or no-action module.
  Rationale: dual review found every row controls science, accepted-input,
  publication, serialization, consumer, or CLI behavior under ADR-0021.
  Date/Author: 2026-07-11 / Codex.
- Decision: make High A the expensive closure unit and module records focused
  checkpoints.
  Rationale: HA-01 spent more than three hours repeating workspace coverage,
  full gates, reviews, and verification for a 709-line, one-module diff. The
  duplicated cadence produced flaky-evidence retries rather than additional
  correctness assurance.
  Date/Author: 2026-07-11 / maintainers + Codex.
- Decision: a proven multi-responsibility host uses exact target-slice
  focused coverage; untouched sibling authorities remain visible and are
  governed by the tranche-final workspace ratchet.
  Rationale: HA-07 reached 100% target coverage and CRAP 27 while unrelated
  runner authorities held whole-file focused coverage at 70%. Expanding a
  single-function checkpoint into hundreds of unrelated branch tests recreates
  the untenable gate cost the revised cadence was intended to remove.
  Date/Author: 2026-07-12 / Codex.

## Outcomes & Retrospective

Queued. At completion record package IDs/commits, before/after coverage and
CRAP, accepted exclusions, defect escalations, full-gate counts, and the exact
High B transition commit.

## Milestones

Milestone 1 binds the fresh `ha/start` metrics, exact raw-to-actionable ledger,
and two target-selection reviews in the campaign evidence directory. Milestone
2 closes HA-01 through HA-04, the routing/projection foundation. Milestone 3
closes HA-05 through HA-10, the executor/publication/CLI chain. Milestone 4 runs
fresh `ha/final` metrics, full gates, two reviews, two verifications, and records
the `TERMINAL-PASS` or `TERMINAL-HOLD` transition. Each milestone is observable
through the exact committed artifacts required by the binding contract.

## Context And Target Ledger

The 2026-07-11 baseline is documented in the campaign assessment. `Rows` is the
deduplicated raw count above 30; `Max` is discovery evidence, not an eligibility
decision.

| ID | Module | Rows | Max CRAP / function |
| --- | --- | ---: | --- |
| HA-01 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | 1 | 56.000 / `interpolate_unit_discharge` |
| HA-02 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | 3 | 55.026 / `KinematicWaveSolver::step` |
| HA-03 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1 | 33.796 / `laned_active_route_lane` |
| HA-04 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | 1 | 43.124 / `DirectDayFrame::validate_r4pqz_hydrology_projection_domain` |
| HA-05 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1 | 58.160 / `DirectFrameExecutor::run_laned_active_publication_stream` |
| HA-06 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs` | 1 | 33.154 / `DirectPublicationDayRow::from_day_frame` |
| HA-07 | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 1 | 32.832 / `direct_production_typed_growth_crop_authority` |
| HA-08 | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 2 | 62.279 / `write_laned_active_trace_output` |
| HA-09 | `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | 1 | 41.389 / `run` |
| HA-10 | `crates/openwepp-runner/src/bin/open_wepp_runner.rs` | 1 | 132.000 / `run_hillslope_command` |

Expected tier is science for routing, projection, contract-bearing publication,
and authoritative day-input modules. A branch-free CLI shell may be glue tier,
but argument parsing, dispatch, fail-closed behavior, and output control are
eligible production behavior. Record the exact tier per package.

## Execution Protocol

### Revised Cadence (Superseding, 2026-07-11)

High A is one closure unit with ten serialized module checkpoints. The revised
campaign execution contract supersedes the per-module scaffold/full-gate/dual-
verification language retained below for provenance. For HA-02 through HA-10,
write one compact record under
`cqr-pre-integration-campaign-evidence/ha/modules/`, run focused tests and
focused module/crate coverage/CRAP, obtain one independent review, and commit
the checkpoint before editing the next module. Focused coverage executes the
instrumented suite once and emits LCOV plus JSON from that shared profile as
specified by the binding contract. HA-01's already-created scaffold is retained
as its module record; it does not require further workspace metric reruns or
verification agents before checkpointing.

After every third checkpoint, run the workspace quick profile. Run exact
workspace LCOV/JSON/CRAP, the non-target ratchet, formatting, workspace Clippy,
full nextest, deny, dual review, and dual verification once after HA-10. A
second module review remains required for an exception, semantic defect,
production control-flow change, or public/serialization/conservation change.

Work from `/home/workdir/openWEPP` on the current branch. Require a clean
worktree before the tranche and before each module checkpoint. Run the binding
contract's exact measurement protocol with slug `ha` and phase `start`; commit
the named `ha/` start evidence. Reconcile the fixed ledger against live source.
Source drift changes current metrics but does not silently add/remove a target.

Before final selection, classify every raw row at exact symbol/line granularity
under ADR-0021. Preserve raw and actionable counts. Two independent read-only
reviewers must accept every proposed `R-OBSERVABILITY`,
`R-IRREDUCIBLE-CRAP`, or `X-*` disposition. Disagreement defaults to
`E-PRODUCTION`; `R-INFRASTRUCTURE` never waives CRAP above 30.

For each actionable ID, create or update its compact module record under the
campaign evidence directory. Do not create a new package scaffold or scaffold
commit. The checkpoint write set is the one target module, focused
tests/fixtures, the module record, and plan transition docs. A fully reviewed
no-action module records classification evidence instead of a fake package.
Only one module is active.

Each package executes cover-first:

1. Map applicable A–H obligations and exact current behavior.
2. Before decomposition, prove complete applicable A–H/named obligation binding,
   the ADR-0021 line/region tier, and the 75% per-function region floor; add and
   pass characterization first when existing coverage is insufficient.
3. For conservation/publication paths, record operand lineage and independently
   reconstruct outputs with anti-alias fixtures and a real magnitude/closure
   check.
4. Decompose whole branches/guard clusters one extraction at a time while
   preserving float grouping, accumulation, row/order, error priority, and
   schema/API identity.
5. Re-run focused coverage/CRAP until every actionable row is at most 30.
6. Complete one independent module review and disposition every finding as
   `accepted`, `rejected`, `deferred`, or `follow-up`. Add a second review only
   for the triggers in the revised cadence. Create a reviewed checkpoint commit;
   verification is tranche-final.

If tests expose a semantic defect, stop mechanical edits and follow the binding
execution contract's explicitly authorized defect-closure transition. Remeasure
before resuming CQR. A proven external boundary legitimizes `TERMINAL-HOLD` but
still blocks High B and enters the finite follow-up queue; it never permits a
successor transition.

## Validation And Acceptance

Use focused crate tests during iteration:

    cargo nextest run -p openwepp-hillslope-orchestrator
    cargo nextest run -p openwepp-runner

Use domain profiles where applicable:

    cargo nextest run --workspace --profile quick
    cargo nextest run --workspace --profile erosion

The High-A tranche closes once, after HA-10, with:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Each module requires focused LCOV/CRAP from the same source state, the binding contract's
exact Markdown/diff commands, output/numeric identity, and consumer-path
evidence. Gate tables use `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`; both reviews
and verifications audit non-deferral. Run before/after `wc -l`: 2,000+ is WARN
with rationale/follow-up split intent, while an unexcepted 3,000+ production
file blocks closure and any exception names owner and sunset.

The tranche completes only when all ten fixed modules have either reviewed
checkpoint commits or committed no-action records, zero eligible CRAP rows
above 30, no unresolved review finding or defect, and fresh `ha/final` evidence
with dual PASS verification. Then update the campaign assessment and activate
High B in the clean `TERMINAL-PASS` transition commit.

## Delegation Authorization

Subagent requirement: **REQUIRED at tranche boundaries**. Spawn
`comparator_suite_runner` for tranche-start/final workspace LCOV/JSON/CRAP,
tranche-final nextest/Clippy/deny, comparator, release, or cohort runs. Focused
module tests/coverage do not require delegation. Expected output is compact
metrics, timings, exit codes, log/artifact paths, hashes, and failure
attribution; heavy-run write access is read-only except named evidence. This
ExecPlan explicitly authorizes subagent
spawning/delegation to target-selection reviewers, bounded module implementers,
coverage/comparator runners, independent reviewers, and verification agents.
Expected outputs are package-local classification, review, verification,
coverage/CRAP, gate, and disposition artifacts. Write access is read-only unless
an agent is explicitly assigned one target module, its focused tests, or named
package artifacts.

## Idempotence And Recovery

Never repeat an existing scaffold or reset unrelated changes. Resume from the
last terminal package commit. Roll back only current-package provisional edits
with path-scoped patches. Preserve hold evidence. A local module hold remains a
High A blocker; a global baseline/tooling/dirty-overlap hold stops the tranche.

## Revision Note

2026-07-11: initial High A plan authored from the clean 45-module rerank and the
tightened ADR-0021 eligibility taxonomy.
2026-07-11: authoring-review remediation bound durable evidence, unconditional
coverage closure, exact scaffolding, non-deferral, line counts, no-action, and
mandatory heavy-run delegation.
2026-07-11: execution review found per-module workspace metrics, full gates,
dual review, dual verification, scaffold artifacts, and retry-on-flaky-ratchet
made the campaign operationally untenable. Revised High A to compact serialized
module checkpoints with focused evidence and tranche-boundary heavy closure.
