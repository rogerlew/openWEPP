# Low-Priority CQR Disposition And Campaign Assessment

Status: `WAITING-SEQUENCE`
Roadmap ID: `CQR-PREINT-20260711-L`
Campaign ledger: `docs/work-packages/cqr-pre-integration-campaign-assessment.md`
Binding execution contract: `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md`
Predecessor: `docs/work-packages/cqr-medium-risk-execplan.md`
Owner: maintainers

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` throughout execution.

## Purpose / Big Picture

Classify and disposition the twelve provisional low-priority modules, execute
all eligible remaining work, and publish the campaign's fresh closing
assessment. Low priority means lower integration risk, not exemption. The plan
ends with an exact `GO-INTEGRATED-VALIDATION` or `HOLD-CQR-FOLLOWUP`
recommendation backed by raw/actionable rankings and full validation evidence.

## Revised Execution Cadence

The 2026-07-11 revised campaign execution model is binding and supersedes this
plan's retained per-module scaffold, full-gate, dual-review, and dual-
verification language. Low/Assessment uses compact serialized module records,
focused module tests/coverage/CRAP, one review by default, quick-profile
checkpoints after every three implementation modules, and one campaign-final
workspace metric/full-gate/dual-review/dual-verification closure. Second module
review is conditional on the triggers named in the binding contract.

## Progress

- [ ] Confirm High A, High B, and Medium have terminal PASS transitions.
- [ ] Refresh workspace LCOV/CRAP and reconcile the twelve fixed modules.
- [ ] Complete dual classification of every raw row.
- [ ] Execute one module package for each module with eligible work.
- [ ] Commit exact dual-reviewed no-action evidence for modules with no eligible work.
- [ ] Run the campaign-final workspace rerank and full validation gates.
- [ ] Update the campaign ledger and `docs/ROADMAP.md` with the exact GO/HOLD recommendation.

## Surprises & Discoveries

- None yet. Record source drift, mixed diagnostic/production roles, newly
  eligible rows, defects, and final-gate findings here with direct evidence.

## Decision Log

- Decision: classification precedes implementation-package creation.
  Rationale: most fixed modules appear formatter- or diagnostic-heavy, while
  at least one contains hand-authored parsing behavior; packages must follow
  exact symbol eligibility rather than filenames or raw maxima.
  Date/Author: 2026-07-11 / Codex.
- Decision: this plan owns the campaign closing recommendation and finite
  follow-up queue.
  Rationale: it observes the terminal source state of all preceding tranches
  and can detect newly surfaced or regressed rows without expanding them
  silently into prior plans.
  Date/Author: 2026-07-11 / Codex.

## Outcomes & Retrospective

Queued behind Medium. At completion record exact module dispositions, package
IDs/commits, before/after raw and actionable rankings, new-row assessment,
full-gate evidence, and one exact GO/HOLD recommendation.

## Milestones

Milestone 1 records fresh `low/start` metrics and dual classification for all 12
fixed modules. Milestone 2 commits exact no-action records and executes each
actionable module package. Milestone 3 records `low/final` metrics, full gates,
two reviews, and two verifications. Milestone 4 publishes
`low/campaign-final-assessment.md`, updates the campaign ledger and roadmap, and
emits the exact GO/HOLD recommendation. Each milestone is independently
observable through the binding contract's durable paths.

## Context And Target Ledger

The campaign assessment records the 2026-07-11 artifact hashes and source
identity. `Rows` is the deduplicated raw count above 30; `Max` is discovery
evidence, not an accepted eligibility disposition.

| ID | Module | Rows | Max CRAP / function |
| --- | --- | ---: | --- |
| L-01 | `crates/openwepp-input-contract/src/parsers/frost.rs` | 1 | 56.000 / `FrostParseError::fmt` |
| L-02 | `crates/openwepp-input-contract/src/parsers/phosphorus.rs` | 1 | 90.000 / `PhosphorusParseError::fmt` |
| L-03 | `crates/openwepp-input-contract/src/parsers/pmetpara.rs` | 1 | 156.000 / `PmetparaParseError::fmt` |
| L-04 | `crates/openwepp-input-contract/src/parsers/tcr.rs` | 1 | 110.000 / `TcrParseError::fmt` |
| L-05 | `crates/openwepp-input-contract/src/parsers/wepp_ui.rs` | 1 | 42.000 / `WeppUiParseError::fmt` |
| L-06 | `crates/openwepp-legacy-bridge/src/hbp.rs` | 1 | 56.000 / `HbpAdapterError::fmt` |
| L-07 | `crates/openwepp-legacy-bridge/src/sidecar.rs` | 1 | 43.172 / `SidecarAdapterError::fmt` |
| L-08 | `crates/openwepp-meteorology/src/error.rs` | 1 | 56.000 / `MeteorologyError::fmt` |
| L-09 | `crates/openwepp-runner/src/hillslope/snowbench.rs` | 1 | 32.619 / `SnowbenchError::fmt` |
| L-10 | `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | 2 | 50.643 / `parse_forcing_line` |
| L-11 | `crates/openwepp-sim-contract/src/symbols.rs` | 1 | 90.000 / `SymbolAliasRegistryError::fmt` |
| L-12 | `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 1 | 90.246 / `WatershedNetworkFrameError::fmt` |

The formatter labels are provisional descriptions only. A formatter or
`as_str` implementation that carries machine-read codes, state, error priority,
control behavior, or publication identity remains eligible. In particular,
`snowbench_coe_melt.rs` contains hand-authored forcing/canopy parsing and must
not inherit a no-action disposition from neighboring diagnostic surfaces.

## Classification And Package Protocol

Work from `/home/workdir/openWEPP` on the current branch. Require a clean
worktree and the committed terminal Medium transition. Run the binding
contract's exact measurement protocol with slug `low` and phase `start`; commit
the named `low/` start evidence. Reconcile all original 45 modules and
separately list newly surfaced rows; do not mutate the fixed baseline to hide
drift.

For each L-ID, produce exact symbol/line classification with raw and actionable
counts. Two independent read-only reviewers must accept every proposed
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` disposition. Disagreement
defaults to `E-PRODUCTION`; `R-INFRASTRUCTURE` never waives CRAP above 30.
Prior PMET or formatter evidence is historical only and must be rebound to the
current source hash.

When a module has eligible work, scaffold
`docs/work-packages/YYYYMMDD-cqr-preint-l-<NN>-<slug>-001/` from both nightly
CQR templates with the filled prompt, reading budget/map, campaign replacements,
directories, and placeholder audit required by the binding execution contract.
Commit the scaffold and execute that module alone. Before decomposition, prove
the reviewed ADR-0021 line/region tier, 75% function floor, and complete
applicable A–H/named obligations; land missing characterization first.
Decompose whole branches/guard clusters without semantic change, and reduce
each actionable row to at most 30. Parser packages require anti-alias
invalid-input and exact error/output cases. Complete two independent reviews,
disposition every finding as `accepted`, `rejected`, `deferred`, or `follow-up`,
and complete two independent verifications before the terminal package commit.

When every raw row in a module receives an accepted no-action disposition, do
not create a fake implementation package. Commit the classification, two
reviews, source binding, and disposition in this plan's campaign evidence.

If characterization finds a semantic defect, stop mechanical work and follow
the binding execution contract's explicitly authorized defect-closure
transition. Any unresolved production defect forces `HOLD-CQR-FOLLOWUP`.

## Campaign-Final Assessment

From the terminal commit of all eligible low-priority packages, run the binding
contract's exact measurement protocol with slug `low` and phase `final`. Commit
the named final evidence and publish:

- artifact hashes/sizes and exact source commit;
- the complete raw ranking and actionable ranking;
- before/after/disposition for all original 45 modules and 67 rows;
- classification and risk of every newly surfaced row;
- coverage/CRAP regressions, unresolved findings, defects, dirty overlaps, and
  conservation or consumer-path evidence gaps;
- package and terminal commit ledger for all four tranches.

The recommendation must be exactly `GO-INTEGRATED-VALIDATION` when every high
and medium module has zero eligible row above 30, each low row is closed or has
a current dual-reviewed disposition, every executed module has current tier,
line/region threshold, 75% floor, and complete applicable A–H/named obligation
evidence, no blocker remains, all full gates pass, the assessment is committed,
and the worktree is clean.

Otherwise recommend exactly `HOLD-CQR-FOLLOWUP`. Add a finite roadmap queue
grouped by authority and non-overlapping write set, with named blockers,
acceptance evidence, and ordering. Do not begin broader integrated validation
or silently append the work to a completed tranche.

## Validation And Acceptance

Use focused crate gates for Rust-changing packages:

    cargo nextest run -p openwepp-input-contract
    cargo nextest run -p openwepp-legacy-bridge
    cargo nextest run -p openwepp-meteorology
    cargo nextest run -p openwepp-runner
    cargo nextest run -p openwepp-sim-contract
    cargo nextest run -p openwepp-watershed-orchestrator

Every Rust-changing package and the campaign-final source state require:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Also require same-source LCOV/CRAP, the binding contract's exact Markdown/diff
commands, dual PASS verification, and exact output/error identity where
applicable. Gate tables use `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`; both reviews
and verifications audit non-deferral. Run before/after `wc -l`: 2,000+ is WARN
with rationale/follow-up split intent; unexcepted 3,000+ blocks closure, and any
exception names owner and sunset. A classification-only terminal state still
requires current source hashes, two reviews, scoped docs lint, and a clean diff;
it does not pretend to have run Rust gates that no source change required.

## Delegation Authorization

Subagent requirement: **REQUIRED**. Spawn `comparator_suite_runner` for every
workspace LCOV/JSON/CRAP, full nextest, Clippy, deny, comparator, release, or
cohort run; local substitution requires recorded command-level unavailability.
Expected output is compact metrics, timings, exit codes, log/artifact paths,
hashes, and failure attribution; heavy-run write access is read-only except
named evidence. This ExecPlan explicitly authorizes subagent
spawning/delegation to classification reviewers, bounded module implementers,
coverage/rerank runners, independent reviewers, campaign assessors, and
verification agents. Expected outputs are exact classification, package-local
review/verification, final raw/actionable rankings, gate evidence, and the
GO/HOLD recommendation. Write access is read-only unless an agent is assigned
one eligible module, named package artifacts, or the final campaign assessment.

## Idempotence And Recovery

Never repeat an existing scaffold or reset unrelated changes. Resume from the
last terminal package or classification commit. Preserve all no-action and hold
evidence. Reuse no artifact across source changes. A failed final gate remains
a recorded blocker and forces the finite follow-up queue.

## Revision Note

2026-07-11: initial low-priority disposition and campaign-assessment plan
authored from the clean 45-module rerank and tightened ADR-0021 taxonomy.
2026-07-11: authoring-review remediation replaced generic symbols and bound the
shared evidence, scaffold, coverage, delegation, non-deferral, line-count, and
defect-transition requirements.
