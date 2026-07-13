# Medium-Risk CQR Tranche: Contracts, Configuration, And Authoritative Tools

Status: `ACTIVE`
Roadmap ID: `CQR-PREINT-20260711-M`
Campaign ledger: `docs/work-packages/cqr-pre-integration-campaign-assessment.md`
Binding execution contract: `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md`
Predecessor: `docs/work-packages/cqr-high-risk-b-execplan.md`
Owner: maintainers

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` throughout execution.

## Purpose / Big Picture

Remove eligible CRAP above 30 from thirteen medium-risk contract, parser,
configuration, validation, oracle, and migration modules after both high-risk
tranches close. These modules are not the main production integration spine,
but they can admit invalid state, alter authority selection, or make diagnostic
and migration behavior unreliable. Completion leaves each fixed module with a
reviewed symbol classification and no unresolved eligible row above 30.

## Revised Execution Cadence

The 2026-07-11 revised campaign execution model is binding and supersedes this
plan's retained per-module scaffold, full-gate, dual-review, and dual-
verification language. Medium uses compact serialized module records, focused
module tests/coverage/CRAP, one review by default, quick-profile checkpoints
after every three modules, and one tranche-final workspace metric/full-gate/
dual-review/dual-verification closure. Second module review is conditional on
the triggers named in the binding contract.

This plan coordinates thirteen fixed-module terminal records. Actionable
modules receive independent packages; reviewed no-action modules receive
committed classification records. It does not authorize a combined
implementation diff.

## Progress

- [x] (2026-07-12 UTC) Confirm High A and High B terminal PASS transitions;
  activate Medium from the authoritative 32-row/25-module residual census.
- [x] (2026-07-13 UTC) Refresh workspace LCOV/CRAP and reconcile all thirteen
  fixed modules: 19 live rows, with the stale M-02 formatter row removed.
- [x] (2026-07-13 UTC) Complete dual target-selection/eligibility review;
  reviewer disagreement on three formatter rows defaults them to
  `E-PRODUCTION`, leaving 19 actionable rows and no no-action module.
- [x] (2026-07-13 UTC) Execute M-01 climate parser through `MODULE-PASS`:
  31 focused consumer/parser tests pass, file line coverage is 95.659%, and
  all target/extracted helpers are above the floor with CRAP at most 21.000.
- [ ] Execute or disposition M-02 through its terminal record.
- [ ] Execute or disposition M-03 through its terminal record.
- [ ] Execute or disposition M-04 through its terminal record.
- [ ] Execute or disposition M-05 through its terminal record.
- [ ] Execute or disposition M-06 through its terminal record.
- [ ] Execute or disposition M-07 through its terminal record.
- [ ] Execute or disposition M-08 through its terminal record.
- [ ] Execute or disposition M-09 through its terminal record.
- [ ] Execute or disposition M-10 through its terminal record.
- [ ] Execute or disposition M-11 through its terminal record.
- [ ] Execute or disposition M-12 through its terminal record.
- [ ] Execute or disposition M-13 through its terminal record.
- [ ] Run the tranche-final fresh rerank, dual verification, and transition Low/Assessment.

## Surprises & Discoveries

- High-B final rerank supplies 32 production-over-30 rows across 25 modules;
  Medium start measurement must reconcile its thirteen fixed modules against
  that authority. Record source drift, parser/validation authority discoveries,
  numerical sensitivity, defects, and gate timing here with direct evidence.
- Medium start reproduced the same 32-row/25-module filtered census byte for
  byte. A stale two-run command cleared the profile before JSON reporting; the
  start-recovery rule avoids a redundant 35-minute rerun while retaining fresh
  LCOV and CRAP authority. Medium final must use one `--no-report` run.

## Decision Log

- Decision: input contracts and shared schema/units validation precede runner,
  oracle, and migration tooling.
  Rationale: later packages consume or interpret those boundaries and should be
  characterized against their terminal behavior.
  Date/Author: 2026-07-11 / Codex.
- Decision: formatter rows do not make a whole module non-actionable.
  Rationale: several modules contain both observation-only formatting and
  eligible invariant or authority logic; classification is symbol-specific.
  Date/Author: 2026-07-11 / Codex.
- Decision: resolve the two selection reviewers' formatter disagreement to
  `E-PRODUCTION` for Snow, management YAML, and landuse migration error text.
  Rationale: the binding contract defaults disagreement to eligibility, and
  stable CLI/error text is externally observable behavior.
  Date/Author: 2026-07-13 / Codex.

## Outcomes & Retrospective

Queued behind High B. At completion record package IDs/commits, before/after
raw and actionable CRAP, exact accepted dispositions, defect escalations,
full-gate evidence, and the Low/Assessment transition commit.

## Milestones

Milestone 1 records fresh `medium/start` metrics, classification, and two
selection reviews. Milestone 2 closes M-01 through M-06 at input/schema/units
boundaries. Milestone 3 closes M-07 through M-10 at runner/oracle boundaries.
Milestone 4 closes the dependency-ordered landuse set M-11 through M-13.
Milestone 5 records `medium/final` metrics, full gates, dual review and
verification, and the terminal transition. Each milestone uses the binding
contract's exact durable evidence paths.

## Context And Target Ledger

The campaign assessment records the 2026-07-11 artifact hashes and source
identity. `Rows` is the deduplicated raw count above 30; `Max` is discovery
evidence, not an eligibility disposition.

| ID | Module | Rows | Max CRAP / function |
| --- | --- | ---: | --- |
| M-01 | `crates/openwepp-input-contract/src/parsers/climate.rs` | 3 | 70.293 / `parse_climate_from_str` |
| M-02 | `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` | 2 | 90.000 / `GwcoeffParseError::fmt` |
| M-03 | `crates/openwepp-input-contract/src/parsers/snow.rs` | 2 | 110.000 / `SnowParseError::fmt` |
| M-04 | `crates/openwepp-input-contract/src/parsers/hbp/error.rs` | 1 | 75.956 / `HbpFormatErrorCode::as_str` |
| M-05 | `crates/openwepp-management-schema/src/lib.rs` | 3 | 45.868 / `ManagementYamlError::fmt` |
| M-06 | `crates/openwepp-sim-contract/src/units_mod/registries.rs` | 1 | 62.474 / `validate_entry` |
| M-07 | `crates/openwepp-runner/src/hillslope/intake_lane_setup/runfile_helpers.rs` | 1 | 31.357 / `parse_runfile_execution_config` |
| M-08 | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | 1 | 37.332 / `build_laned_shadow_lane_day_operands` |
| M-09 | `crates/openwepp-runner/src/release.rs` | 1 | 31.459 / `validate_release_sidecar_unlocked` |
| M-10 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs` | 1 | 30.093 / `run_oracle` |
| M-11 | `crates/openwepp-landuse-migrate/src/lib.rs` | 2 | 65.672 / `LanduseMigrationError::fmt` |
| M-12 | `crates/openwepp-landuse-migrate/src/convert.rs` | 1 | 42.000 / `yearly_extension_to_yaml` |
| M-13 | `crates/openwepp-landuse-migrate/src/cli.rs` | 1 | 84.437 / `run_cli_args` |

The raw maxima obscure mixed-role modules. For example, `gwcoeff.rs` and
`snow.rs` also contain eligible invariant enforcement above 30, while migration
and schema modules combine formatting with authority or validation behavior.
Record every raw row and its exact ADR-0021 category; never suppress a file
because its maximum happens to be a formatter.

## Execution Protocol

Work from `/home/workdir/openWEPP` on the current branch. Require a clean
worktree and the committed terminal High B transition before activation. Run
the binding contract's exact measurement protocol with slug `medium` and phase
`start`; commit the named `medium/` start evidence. Reconcile the fixed ledger
against current source without silently adding or dropping a target.

Classify every raw row at exact symbol/line granularity. Preserve raw and
actionable counts. Two independent read-only reviewers must accept each
proposed `R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` disposition.
Disagreement defaults to `E-PRODUCTION`; `R-INFRASTRUCTURE` never waives CRAP
above 30. Validation order, error codes/messages, authority selection, CLI
dispatch, and release fail-closed behavior are production behavior unless a
reviewed contract proves otherwise.

For each actionable ID, scaffold
`docs/work-packages/YYYYMMDD-cqr-preint-m-<NN>-<slug>-001/` from the nightly CQR
package and kickoff templates with the filled prompt, reading budget/map,
campaign-specific replacements, required directories, and placeholder audit in
the binding execution contract. Commit before Rust/test edits. A fully reviewed
no-action module records classification evidence instead. Its write set is the
single target module, focused tests/fixtures, package artifacts, and necessary
catalog/transition documentation. Only one package may be active.

Each package executes cover-first:

1. Bind parser grammar, invariant priority, authority choice, schema/units
   meaning, oracle semantics, or migration output as applicable.
2. Before decomposition, prove complete applicable A–H/named obligations, the
   reviewed ADR-0021 line/region tier, and the 75% function floor; land and pass
   missing characterization first.
3. Add anti-alias cases for neighboring invalid states, reordered validation,
   wrong-unit values, authority fallthrough, and CLI/output identity.
4. Decompose one whole branch or guard cluster at a time without changing
   formulas, float grouping, accepted syntax, typed errors, validation order,
   serialized schema, output ordering, or exit behavior.
5. Rerun focused coverage/CRAP until every actionable row is at most 30.
6. Complete two independent reviews, disposition every finding as `accepted`,
   `rejected`, `deferred`, or `follow-up`, complete two independent
   verifications, and create a terminal completion or legitimate hold commit.

If characterization finds a semantic defect, stop mechanical work and follow
the binding execution contract's explicitly authorized defect-closure
transition. Remeasure before resuming.

## Validation And Acceptance

Use the narrow crate gate for the active package, including as applicable:

    cargo nextest run -p openwepp-input-contract
    cargo nextest run -p openwepp-management-schema
    cargo nextest run -p openwepp-sim-contract
    cargo nextest run -p openwepp-runner
    cargo nextest run -p openwepp-hillslope-orchestrator
    cargo nextest run -p openwepp-landuse-migrate

Every Rust-changing package closes with:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Also require same-source focused LCOV/CRAP, the binding contract's exact
Markdown/diff commands, and exact parser/schema/output identity evidence. Gate
tables use `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`; both reviews and
verifications audit non-deferral. Run before/after `wc -l`: 2,000+ is WARN with
rationale/follow-up split intent; unexcepted 3,000+ blocks closure, and any
exception names owner and sunset.

The tranche completes only when all thirteen modules have terminal
implementation commits or committed no-action records, zero eligible CRAP rows
above 30, no unresolved review finding or defect, and fresh `medium/final`
evidence with dual PASS verification. Then update the campaign ledger and
activate Low/Assessment in the clean `TERMINAL-PASS` transition.

## Delegation Authorization

Subagent requirement: **REQUIRED**. Spawn `comparator_suite_runner` for every
workspace LCOV/JSON/CRAP, full nextest, Clippy, deny, comparator, release, or
cohort run; local substitution requires recorded command-level unavailability.
Expected output is compact metrics, timings, exit codes, log/artifact paths,
hashes, and failure attribution; heavy-run write access is read-only except
named evidence. This ExecPlan explicitly authorizes subagent
spawning/delegation to target-selection reviewers, bounded module implementers,
coverage runners, independent reviewers, and verification agents. Expected
outputs are package-local classification, characterization, review,
verification, gate, and disposition artifacts. Write access is read-only unless
an agent is explicitly assigned one target module, focused tests, or named
package artifacts.

## Idempotence And Recovery

Never repeat an existing scaffold or reset unrelated changes. Resume from the
last terminal package commit. Roll back only current-package provisional edits
with path-scoped patches. Preserve hold evidence. A source/tooling conflict
stops the tranche; a module defect follows the defect-closure transition above.

## Revision Note

2026-07-11: initial medium-risk plan authored from the clean 45-module rerank
and tightened ADR-0021 symbol eligibility taxonomy.
2026-07-11: authoring-review remediation corrected exact symbols and bound the
shared evidence, scaffold, no-action, coverage, delegation, non-deferral, and
line-count requirements.
