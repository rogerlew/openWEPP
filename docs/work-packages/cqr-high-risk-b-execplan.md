# High-Risk CQR Tranche B: Erosion, Watershed, And Publication

Status: `ACTIVE`
Roadmap ID: `CQR-PREINT-20260711-HB`
Campaign ledger: `docs/work-packages/cqr-pre-integration-campaign-assessment.md`
Binding execution contract: `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md`
Predecessor: `docs/work-packages/cqr-high-risk-a-execplan.md`
Owner: maintainers

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` throughout execution.

## Purpose / Big Picture

Close complexity risk in erosion continuity, watershed/channel routing,
water-balance accumulation, and final watershed publication after High A has
stabilized the active hillslope path. These ten modules are hard blockers for a
broader integrated validation campaign because they own conservation operands,
network behavior, or final public outputs.

## Revised Execution Cadence

The 2026-07-11 revised campaign execution model is binding and supersedes this
plan's retained per-module scaffold, full-gate, dual-review, and dual-
verification language. High B uses compact serialized module records, focused
module tests/coverage/CRAP, one review by default, quick-profile checkpoints
after every three modules, and one tranche-final workspace metric/full-gate/
dual-review/dual-verification closure. Second module review is conditional on
the triggers named in the binding contract.

This plan coordinates ten serialized fixed-module terminal records. Actionable
modules receive packages; reviewed no-action modules receive committed
classification records. It does not permit a multi-module implementation diff.

## Progress

- [x] (2026-07-12 UTC) Verify High A `TERMINAL-PASS` and activate High B in
  the clean predecessor transition commit.
- [x] (2026-07-12 UTC) Refresh `hb/start` workspace metrics and dual-review all
  21 live rows: 21 actionable, zero retained exception/exclusion.
- [x] (2026-07-12 UTC) Close `DC-CQR-HB01-001` and execute HB-01 through
  reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Close `DC-CQR-HB02-001` and execute HB-02 through
  reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Close `DC-CQR-HB03-001` and execute HB-03 through
  reviewed `MODULE-PASS` checkpoint.
- [x] (2026-07-12 UTC) Pass the HB-01..HB-03 workspace quick-profile checkpoint
  (`1,772/1,772`, 28 skipped, three slow).
- [ ] Execute or disposition HB-04 through its terminal record.
- [ ] Execute or disposition HB-05 through its terminal record.
- [ ] Execute or disposition HB-06 through its terminal record.
- [ ] Execute or disposition HB-07 through its terminal record.
- [ ] Execute or disposition HB-08 through its terminal record.
- [ ] Execute or disposition HB-09 through its terminal record.
- [ ] Execute or disposition HB-10 through its terminal record.
- [ ] Run tranche-final rerank, dual verification, and transition to Medium.

## Surprises & Discoveries

- Fresh start measurement reproduces 54 production rows across 35 modules and
  all 21 fixed High-B rows across ten files. All 21 are actionable; no
  exclusion or no-action module is accepted.
- The two selection reviewers differed only on the taxonomy label for two
  eligible parser/area symbols. The binding disagreement rule records both as
  `E-PRODUCTION`; actionability and science-tier module treatment were never in
  dispute.

## Decision Log

- Decision: erosion modules execute before watershed aggregation/routing and
  final output writers.
  Rationale: downstream evidence must consume the terminal erosion/publication
  behavior rather than an intermediate refactor state.
  Date/Author: 2026-07-11 / Codex.
- Decision: conservation/publication acceptance remains current scope for every
  package that owns or maps a water/sediment operand.
  Rationale: coverage and self-consistency cannot prove that a real downstream
  consumer reads the correct authoritative field.
  Date/Author: 2026-07-11 / Codex.

## Outcomes & Retrospective

Queued. At completion record package commits, before/after metrics, operand and
consumer proof, defect escalations, full gates, and the Medium transition.

## Milestones

Milestone 1 records fresh `hb/start` metrics, classification, and two selection
reviews. Milestone 2 closes HB-01 through HB-04 for erosion continuity.
Milestone 3 closes HB-05 through HB-10 for watershed parsing, aggregation,
routing, writers, and CLI publication. Milestone 4 records fresh `hb/final`
metrics, full gates, two reviews, two verifications, and the exact terminal
transition. Each milestone uses the binding contract's durable evidence paths.

## Context And Target Ledger

| ID | Module | Rows | Max CRAP / function |
| --- | --- | ---: | --- |
| HB-01 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs` | 1 | 41.280 / `validate_erod13_inputs` |
| HB-02 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs` | 1 | 65.929 / `erosion_particle_composition` |
| HB-03 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs` | 1 | 34.808 / `assemble_wave1_continuity_inputs_quantum` |
| HB-04 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | 5 | 90.561 / `wave1_erod` |
| HB-05 | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` | 1 | 43.388 / `parse_watershed_structure_from_str` |
| HB-06 | `crates/openwepp-summary-accumulator/src/lib.rs` | 1 | 44.951 / `Wb13DailyWaterBalanceRow::from_surface` |
| HB-07 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | 2 | 58.410 / `Ws10ChannelImpoundmentKernel::compute_variable_muskingum_cunge_state` |
| HB-08 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` | 1 | 50.396 / `Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series` |
| HB-09 | `crates/openwepp-watershed-output/src/writers.rs` | 2 | 69.000 / `float64_value` |
| HB-10 | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 6 | 191.186 / `parse_watershed_runfile` |

The listed risk is raw discovery evidence from the campaign baseline. All ten
modules are expected science/contract/conservation tier except any literal
branch-free CLI shell proven to be glue. The watershed CLI's parsing,
validation, orchestration, metadata, and publication behavior is eligible even
though the file is under `src/bin`.

## Execution Protocol

Do not start until High A records terminal PASS, its final rerank, and a clean
transition commit. Then run fresh workspace LCOV/JSON/CRAP from the current
commit using the binding contract's exact measurement protocol with slug `hb`
and phase `start`; commit the named `hb/` start evidence.

Classify each live row using ADR-0021's symbol taxonomy before implementation.
Two independent target-selection reviewers must accept every proposed
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` disposition. Preserve raw and
actionable counts; `R-INFRASTRUCTURE` cannot waive CRAP above 30. A changed row
is re-reviewed rather than inheriting the 2026-07-11 risk label.

For each actionable ID, scaffold
`docs/work-packages/YYYYMMDD-cqr-preint-hb-<NN>-<slug>-001/` and commit the
nightly package and kickoff templates with every campaign-specific replacement,
prompt, reading budget/map, directory, and placeholder audit required by the
binding execution contract. Commit before implementation/test edits. A fully
reviewed no-action module records classification evidence instead. Limit the
write set to one target module, focused tests/fixtures, package artifacts, and
catalog/plan transition docs. Only one module package may be active.

Each package must:

1. Identify canonical contract/provenance and applicable A–H obligations.
2. Before decomposition, prove complete applicable A–H/named obligations,
   ADR-0021 science line/region coverage, and the 75% function floor; land and
   pass missing characterization first.
3. Record operand lineage for water/sediment/mass outputs, including units,
   normalization, source authority, and rejected aliases.
4. Prove a real consumer reads the refactored path; producer-only, schema-only,
   and self-consistency evidence cannot close publication/routing claims.
5. Decompose whole branches without changing floating grouping, accumulation,
   temporal/key order, typed errors, schemas, or output identity.
6. Reduce every actionable row to CRAP at most 30, complete two independent
   reviews, disposition every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up`, complete two independent verifications, and create a terminal
   commit.

A semantic or conservation defect follows the binding contract's explicitly
authorized defect-closure transition. The HB plan stays active until that defect
is closed and the affected module remeasured. A comparator mismatch alone is a
flag; use independent contract, conservation, and lineage authority before
assigning a production defect.

## Validation And Acceptance

Focused iteration uses the narrowest applicable commands, including:

    cargo nextest run --workspace --profile erosion
    cargo nextest run -p openwepp-hillslope-orchestrator
    cargo nextest run -p openwepp-watershed-orchestrator
    cargo nextest run -p openwepp-watershed-output
    cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract

Every Rust-changing package closes with:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Also require same-source focused LCOV/CRAP, the binding contract's exact
Markdown/diff commands, numeric/output identity, independent closure, and
real-consumer proof. Gate tables use `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`;
both reviews and verifications audit non-deferral. Run before/after `wc -l`:
2,000+ is WARN with rationale/follow-up split intent; unexcepted 3,000+ blocks
closure, and any exception names owner and sunset.

The tranche completes only when all ten modules have terminal implementation
commits or committed no-action records, zero eligible CRAP rows above 30, no
unresolved defect/finding/gate, and fresh `hb/final` evidence with dual PASS
verification. Update the campaign assessment and activate Medium in the same
clean `TERMINAL-PASS` transition.

## Delegation Authorization

Subagent requirement: **REQUIRED**. Spawn `comparator_suite_runner` for every
workspace LCOV/JSON/CRAP, full nextest, Clippy, deny, comparator, release, or
cohort run; local substitution requires recorded command-level unavailability.
Expected output is compact metrics, timings, exit codes, log/artifact paths,
hashes, and failure attribution; heavy-run write access is read-only except
named evidence. This ExecPlan explicitly authorizes subagent
spawning/delegation to target-selection reviewers, bounded single-module
implementers, science/lineage reviewers, comparator/coverage runners, and
independent verification agents. Outputs are package-local classification,
operand-lineage, review, verification, metric, gate, disposition, and handoff
artifacts. Write access is read-only unless explicitly bounded to one module,
its focused tests, or named package artifacts.

## Idempotence And Recovery

Resume from the last terminal commit; never repeat a scaffold or reset unrelated
work. Roll back only the current package's provisional implementation with
path-scoped patches and preserve hold evidence. A genuine external boundary may
legitimize `TERMINAL-HOLD`, but it still blocks Medium and enters the finite
follow-up queue; only `TERMINAL-PASS` permits successor activation.

## Revision Note

2026-07-11: initial High B plan authored from the clean baseline, sequencing
erosion authority before watershed routing and final publication.
2026-07-11: authoring-review remediation bound the shared execution contract
and corrected coverage, evidence, scaffold, no-action, delegation, non-deferral,
and line-count requirements.
