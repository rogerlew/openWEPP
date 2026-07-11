# CQR Nightly Follow-Up Queue Burndown ExecPlan

Status: **active-burndown**
Dispatch surface: **main** unless the operator explicitly authorizes a branch.
Owner: maintainers.
Created: 2026-07-11 UTC.

This ExecPlan is a living document. Maintain `Progress`, `Surprises &
Discoveries`, `Decision Log`, and `Outcomes & Retrospective` as work proceeds.
It follows `docs/codex_exec_plans.md`, the Gate Evidence Non-Deferral Rule in
`docs/work-packages/AGENTS.md`, and the package-specific standards named below.

## Purpose / Big Picture

The 2026-07-11 eight-module CQR nightly batch completed three modules and
isolated four actionable follow-up tracks. Three tracks are semantic defects
that behavior-preserving CQR was not authorized to change. The fourth is a
coverage precondition that must close before safe decomposition. This plan turns
those findings into a finite, sequence-ordered queue of independent single-module
work packages.

After the queue burns down, the affected modules will either have their named
defects closed and eligible CRAP rows reduced to at most `30`, or will have a
reviewed `HOLD` at a concrete authority/environment boundary. The operator can
verify the outcome from per-package commits, focused contract tests, fresh
coverage/CRAP evidence, and a final workspace rerank.

This plan coordinates packages; it is not itself authority to mix unrelated
modules into one package. Every implementation package remains independently
scaffolded, reviewed, verified, and committed.

## Progress

- [x] (2026-07-11 UTC) Extract the follow-up findings from nightly batch 01.
- [x] (2026-07-11 UTC) Classify defect-closure, coverage/CQR, and no-action rows.
- [x] (2026-07-11 UTC) Complete initial independent authoring reviews.
- [x] (2026-07-11 UTC) Accept and edit all governance/technical findings.
- [x] (2026-07-11 UTC) Complete independent re-review: Review A `GO`, Review B `GO`.
- [x] (2026-07-11 UTC) Change this plan to `ready-queued` after both reviews permit dispatch.
- [x] (2026-07-11 UTC) Execute queue item `FQ-01` through its terminal package commit.
- [x] (2026-07-11 UTC) Execute queue item `FQ-02` through its terminal package commit.
- [x] (2026-07-11 UTC) Execute queue item `FQ-03` through its terminal package commit.
- [ ] Execute queue item `FQ-04` through its terminal package commit.
- [ ] Run the final fresh workspace LCOV/CRAP rerank and record the outcome.

## Surprises & Discoveries

- Observation: the nightly batch produced three real contract mismatches, not
  merely low coverage. Evidence: `FDIR-FINITE-VALUE-GUARD-001`,
  `CHANINP-RAW-NCHNUM-CARDINALITY`, and `CHN-E006-EXTRA-RATING-ROW` are recorded
  in their originating package dispositions.
- Observation: PMETPARA has no actionable eligible production row after the
  reviewed ADR-0021 exclusion. Evidence: its only row above `30` is
  `PmetparaParseError::fmt`, an observability-only formatter.
- Observation: package boundaries must be declared before implementation so a
  semantic defect correction is not hidden inside a mechanical CQR diff. The
  queue therefore separates semantic and behavior-preserving mechanical phases
  as review checkpoints inside each non-deferring parser DC package.
- Observation: FQ-01's public characterization closed the fixed-date parser at
  97.397% lines / 98.065% regions, but full coverage could not lower CC 39 below
  CRAP 30; extracting whole preamble and event blocks reduced maximum CRAP to
  17 without changing finite-input behavior.

## Decision Log

- Decision: treat this as one queued ExecPlan that dispatches multiple ordinary
  work packages, never as one multi-module package.
  Rationale: the queue needs shared ordering and transition gates, while package
  governance requires one coherent module/authority envelope per package.
  Date/Author: 2026-07-11 / Codex.
- Decision: classify PMETPARA as `CLOSED-NO-ACTION`, not as an implementation
  package.
  Rationale: forcing formatter tests would be coverage theater and would reverse
  the prior dual-reviewed closed-list exclusion without new evidence.
  Date/Author: 2026-07-11 / Codex.
- Decision: run contract defects before the pure coverage/CQR track.
  Rationale: accepted-input and typed-error correctness outrank maintainability
  debt, and the three parser tracks share the input-contract crate, making
  sequential execution safer than overlapping edits.
  Date/Author: 2026-07-11 / Codex.
- Decision: each parser DC package owns semantic correction, coverage and
  obligation closure, and any necessary post-correction behavior-preserving
  decomposition through final CRAP closure.
  Rationale: authoring review found that material test changes activate the
  ADR-0021 CRAP gate in the same package. Deferring ordinary in-envelope
  decomposition would violate Gate Evidence Non-Deferral. Semantic and
  mechanical phases remain separate review checkpoints inside one DC envelope.
  Date/Author: 2026-07-11 / Codex.
- Decision: retain structurally impossible defensive parser arms in the counted
  coverage denominator rather than annotating exclusions.
  Rationale: the module clears science-tier and per-function gates without
  denominator reduction, so retaining defensive fail-closed code is clearer
  than exclusion or deletion.
  Date/Author: 2026-07-11 / Codex.

## Outcomes & Retrospective

Authoring outcome: both independent re-reviews returned `GO` after every initial
finding was accepted and edited. The plan is ready to dispatch FQ-01 from a clean
plan commit. During execution, update this section after every queue item with
package IDs, scaffold/terminal commit SHAs, defect disposition, coverage/CRAP
before and after, and any legitimate boundary. At queue completion, record the
final rerank and whether another fresh nightly batch is warranted.

FQ-01 outcome: package `20260711-dc-fdir-finite-value-guard-001` completed.
`INV-FDIR-015` now rejects `NaN`/infinities for all eight typed real fields with
`FDIR-E-005` in strict and compatibility modes. Final focused tests are 27/27,
coverage is 97.397% lines / 98.065% regions, minimum function coverage is
85.366%, maximum CRAP is 17, full workspace is 1,730/1,730 with 3 configured
skips, and dual review/verification are GO/PASS. Closure is parser-boundary only.

## Context And Orientation

The source queue is the 2026-07-11 CQR nightly batch recorded in
`docs/work-packages/README.md`. CQR means code-quality refactoring driven by the
CRAP metric, which combines cyclomatic complexity and coverage. Eligible
functions must score at most `30`. CQR is behavior-preserving: it cannot change
accepted inputs, typed errors, contracts, scientific formulas, thresholds,
serialization, or public output meaning.

A Defect-Closure package, or DC package, closes an observed semantic violation
inside a declared Correction Authority Envelope. It may amend canonical
contracts, add contract-derived tests, and edit production behavior in that
order. It cannot stop at `HOLD` while an authority-backed correction remains
possible inside its envelope. See `docs/defect_closure_execplans.md`.

A module-test/CQR package raises the target to ADR-0021 coverage and obligation
closure before any decomposition. Science or contract-bearing modules require
at least `90%` line and region coverage; glue modules require at least `85%`.
Every eligible function needs at least `75%` region coverage or a reviewed
closed-list exclusion. Applicable A-H test families must be bound to named tests.

The original evidence packages remain immutable history:

- `docs/work-packages/20260711-cqr-nightly-01-runner-totalwatsed3-001/`
- `docs/work-packages/20260711-cqr-nightly-03-input-watershed-channel-001/`
- `docs/work-packages/20260711-cqr-nightly-05-input-pmetpara-001/`
- `docs/work-packages/20260711-cqr-nightly-06-input-irrigation-fixeddate-001/`
- `docs/work-packages/20260711-cqr-nightly-07-input-chaninp-001/`

## Governance And Dispatch Authorization

This plan is subordinate to:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`

Subagent authorization: this ExecPlan explicitly authorizes subagent
spawning/delegation to independent authoring reviewers, per-package reviewers,
verifiers, comparator/coverage runners, and closure runners. Expected outputs
are compact findings, finding dispositions, metrics, log paths, and the standard
package review/verification artifacts. Authoring and implementation reviewers
are read-only. A subagent receives bounded write access only when explicitly
assigned a named package file or implementation fix.

Subagent requirement: every package must require `comparator_suite_runner` for
heavy full-workspace coverage/CRAP, comparator, fixture-batch, clippy,
full-nextest, and deny runs. The parent must not run those heavy gates while the
delegated runner is available. Package-local `package.md` and kickoff prompts
must repeat this authorization and requirement.

## Queue Ledger

| Queue ID | Required track | Package shape | Initial state |
|---|---|---|---|
| `FQ-01` | Close `FDIR-FINITE-VALUE-GUARD-001` | One contract-first DC package through science-tier coverage, obligations, and CRAP closure | `COMPLETE` — package `20260711-dc-fdir-finite-value-guard-001`, terminal transition commit |
| `FQ-02` | Close `CHANINP-RAW-NCHNUM-CARDINALITY` | One contract-first DC package through science-tier coverage, obligations, and CRAP closure | `COMPLETE` — package `20260711-dc-chaninp-raw-cardinality-001`, terminal transition commit |
| `FQ-03` | Close `CHN-E006-EXTRA-RATING-ROW` | One contract-first DC package through science-tier coverage, obligations, and CRAP closure | `COMPLETE` — package `20260711-dc-watershed-channel-extra-rating-001`, terminal transition commit |
| `FQ-04` | Close `totalwatsed3.rs` coverage and CRAP debt | One science-tier cover-then-decompose CQR package | `ACTIVE` — package `20260711-cqr-followup-totalwatsed3-001`, 2026-07-11 UTC |
| `FQ-05` | PMETPARA formatter disposition | No package; preserve originating hold history | `DISPOSITIONED-NO-ACTION / NOT-QUEUED` |

Package IDs use the scaffold date and these stable slugs:

- `YYYYMMDD-dc-fdir-finite-value-guard-001`
- `YYYYMMDD-dc-chaninp-raw-cardinality-001`
- `YYYYMMDD-dc-watershed-channel-extra-rating-001`
- `YYYYMMDD-cqr-followup-totalwatsed3-001`

Only one queue package may be active. `WAITING-SEQUENCE` means serialized to
avoid overlapping write sets and heavy runs; it is not a technical dependency
on the preceding defect's outcome. A legitimate local hold updates this plan
and permits the next independent row to become active. A global/process hold
stops the queue.

## Queue-Wide Package Protocol

For each triggered package, scaffold the complete directory from the relevant
template/procedure, including `package.md`, `prompts/active/`,
`prompts/archived/`, `artifacts/required-reading-map.md`, review and verification
artifacts, gate evidence, disposition, and handoff. Run
`tools/agents/find-agents --for` every declared write path before edits.
Register the queued package in `docs/work-packages/README.md` and include that
registration in its scaffold commit.

Commit the scaffold before any contract, production, test, or fixture edit.
Execute the package end-to-end. Commit `EXECUTED-COMPLETE-*` or a legitimate
`EXECUTED-HOLD-*` before changing the next ledger row to active. A hold must
include `artifacts/hold-legitimacy-audit.md`, name the boundary, prove why the
in-envelope correction cannot close, and lead its handoff with `close defect
<id>`. Effort, low coverage, draft contract status, or remaining source reading
are not hold boundaries.

Every package is science tier under ADR-0021 Decision 4 because FQ-01 through
FQ-03 implement canonical contract invariants and FQ-04 aggregates
conservation-sensitive water/sediment outputs. Material test changes require at
least `90%` eligible line and region coverage, every eligible function at least
`75%` region coverage or reviewed closed-list exclusion, complete applicable
A-H obligation binding, and every eligible CRAP row at most `30`.
Before/after coverage evidence must record the exact source commit/worktree
state, commands, exit codes, timings, LCOV/JSON/CRAP SHA-256 hashes and sizes,
eligible-surface filters, and attribution of any ignored test failure.

Every package must record `.rs` line-count governance. A file at or above
`2,000` lines is `WARN` and needs decomposition rationale plus split intent; a
non-exempt file at or above `3,000` lines blocks closure. Both independent
reviews must disposition the result.

Every package must complete dual independent review, disposition every finding
as accepted/rejected/deferred/follow-up, fix and verify accepted findings, and
complete dual verification. No queue item advances with an undispositioned
finding or a current-scope failed/deferred gate.

For defect tracks, one DC package owns the complete sequence: targeted canonical
contract provenance amendment/confirmation; contract-derived tests that fail on
the defect; `artifacts/pre-implementation-contract-gate.md` with independent
review PASS; production correction; focused parser/consumer validation;
science-tier coverage and obligation closure; then any necessary
behavior-preserving decomposition through CRAP at most `30`. Semantic and
mechanical diffs must be separate phases with explicit checkpoint evidence and
review, but neither phase may terminally defer an in-envelope gate.

Every DC package must restate the conversion rule and the seven-factor
HOLD-to-fix bar from `docs/defect_closure_execplans.md`. It must run applicable
contract schema/profile checks. A `HOLD` requires the legitimacy audit and a
missing/contradictory authority, out-of-envelope mechanism, invalid upstream
input, unavailable evidence, or different contract-family boundary. Draft
status, implementation effort, low coverage, or remaining decomposition are not
boundaries.

For mechanical decomposition, preserve exact statement, expression,
accumulation, and short-circuit order. Extract whole branches or guard clusters
one at a time, remeasure after each cohesive change, and stop only when every
eligible target function is at most `30` or has a reviewed closed-list
disposition.

## FQ-01 — Fixed-Date Irrigation Finite-Value Closure

The observed defect is that Rust `f64` parsing accepts `NaN` and infinities,
while inequality-only guards can allow `NaN` into typed sprinkler/furrow output.
The input specification requires finite `irint`, `irdept`, `qspply`, `tstart`,
and `tend`. The original package also requires an audit of `nozzle` and `tdepl`.

The DC Correction Authority Envelope includes
`docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md`,
`docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md`,
`crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs`, focused
test `tests/integration/infile_irrigation_fixeddate_parser_contract.rs`, and
`tests/fixtures/infile/irrigation_fixeddate/`. Legacy evidence is limited to
`/workdir/wepp-forest_260430_baseline/src/{infile.for,irinpt.for,irrig.for,inidat.for,cdat.inc,cirfixd.inc}`
at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Before production edits,
targeted contract/spec clauses must replace unpinned `/workdir/wepp-forest`
anchors with that baseline or record a reviewed superseding justification. The
envelope authorizes contract amendment,
explicit A-H/test-vector obligations, typed `FDIR-E-005` rejection tests, and
finite-value production guards. It protects unresolved `iryr` runtime meaning
and downstream contour/non-cropland cross-file policy unless the package amends
its envelope before implementation with cited authority and review.

Acceptance requires finite valid fixtures to retain exact typed output; each
owned real field to reject `NaN`, positive infinity, and negative infinity with
the ratified typed error; negative/zero boundaries to follow field authority;
`nozzle`/`tdepl` to have explicit audited dispositions; the applicable contract
obligations, science-tier coverage, and CRAP gates to close. This is explicitly
parser-boundary closure: current production has no downstream
`FixedDateIrrigationFile` consumer. Require negative proof that non-finite input
cannot produce typed parser output, and make no runtime-handoff, activation, or
readiness claim. Focused command:

    cargo nextest run --test infile_irrigation_fixeddate_parser_contract

The pre-fix non-finite vector must fail the new expectation; after correction,
finite fixtures remain identical and all owned non-finite vectors return the
ratified typed error.

## FQ-02 — CHAN.INP Raw Cardinality Closure

The observed defect is that compatibility input with raw `nchnum=99` and two IDs
is normalized before record-cardinality closure and exposed as
`nchnum_input=2`. The canonical source model distinguishes raw input from
`nchnum_norm`, while pinned legacy provenance reads the raw-count ID list before
normalization.

The DC Correction Authority Envelope includes
`docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`,
`docs/specifications/wepp-input-files/specs/chaninp.spec.md`, pinned
`/workdir/wepp-forest_260430_baseline/src/{wshinp.for,cchrt.inc,pmxchr.inc,chnrt.for}`
provenance at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`,
`crates/openwepp-input-contract/src/parsers/chaninp.rs`,
`tests/integration/infile_chaninp_parser_contract.rs`,
`tests/fixtures/infile/chaninp/`,
`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`, and
`tests/integration/wshedw5_typed_watershed_runtime_contract.rs`. It authorizes
ratifying raw retention/cardinality,
adding explicit test-vector obligations, correcting parser order/fields, and
updating fixtures and tests. It does not authorize channel-routing physics or
unrelated writer-policy changes.

Acceptance requires the canonical contract to state raw-vs-normalized ownership
and record-4 cardinality unambiguously. The original raw `nchnum=99` plus two-ID
fixture is invalid and must fail exact `CHN-E-002`; it must not be normalized
into validity. A separate valid compatibility fixture must provide raw-count
closure (for example, `99` IDs), preserve `nchnum_input=99`, and normalize
`nchnum_norm` according to ratified topology policy. The network-frame consumer
must read normalized fields while raw/observability export preserves source
input. Science-tier coverage, obligations, and CRAP must close. Focused commands:

    cargo nextest run --test infile_chaninp_parser_contract
    cargo nextest run --test wshedw5_typed_watershed_runtime_contract

## FQ-03 — Watershed-Channel Extra Rating Row Closure

The observed defect is that a three-float rating row following a channel with
`icntrl != 4` is classified as generic extra input `CHN-E-002`, while canonical
`G-CHN-013` currently requires rating-curve closure `CHN-E-006`. Recognition is
ambiguous across multi-channel input and numeric-leading comment text, so the
contract must ratify the recognition rule before production changes.

The DC Correction Authority Envelope includes
`docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`,
`docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md`,
`/workdir/wepp-forest_260430_baseline/src/{infile.for,wshinp.for,inidat.for,verchk.for}`
at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`,
`crates/openwepp-input-contract/src/parsers/watershed_channel.rs`, focused tests,
`tests/integration/infile_watershed_channel_parser_contract.rs`,
`tests/fixtures/infile/watershed_channel/`,
`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`, and
`tests/integration/wshedw5_typed_watershed_runtime_contract.rs`. Before
production edits, targeted contract/spec clauses must re-anchor unpinned legacy
paths or record reviewed superseding justification. The envelope authorizes the
precise extra-rating recognition rule,
typed error mapping, parser correction, and one-/multi-channel regression
fixtures. It does not authorize changing rating-curve physics or accepting
previously invalid input as valid.

Acceptance requires one-channel EOF two-/three-/four-token extra-row cases,
multi-channel boundary cases, and numeric comment-like text to be classified by
an explicit contract rule. A lexical heuristic such as “three floats means
rating” is prohibited. Valid numeric comments and next-channel records must not
be reclassified. Only contract-recognized extra rating records emit the ratified
`CHN-E-006` variant; valid parses remain identical. Science-tier A-H, coverage,
and CRAP gates close. Focused commands:

    cargo nextest run --test infile_watershed_channel_parser_contract
    cargo nextest run --test wshedw5_typed_watershed_runtime_contract

## FQ-04 — Totalwatsed3 Cover-Then-Decompose Closure

This is behavior-preserving work, not defect closure. The old module baseline was
below its threshold and lacked region/per-function evidence. The package
must therefore cover first and decompose only after the safety net closes.

The single CQR package write set includes
`crates/openwepp-runner/src/totalwatsed3.rs`,
`crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`, additional focused
fixtures required for optional soil/element inputs, and package artifacts. It
must bind fallible column/type/null/value ordering, optional soil/element
aggregation, typed error-code behavior, date/OFE keys, and valid output rows.

Because totalwatsed3 aggregates conservation-sensitive water/sediment output,
the package must record operand lineage and use fixtures where plausible wrong
columns, areas, row keys, or per-OFE sums differ from the expected result.
Output identity must be reconstructed from independent source rows; producer
self-consistency and schema-only checks are insufficient.

After the ADR-0021 coverage threshold, per-function floor, and obligation map
pass at science tier (`>=90%` eligible line and region, every eligible function
`>=75%` region), decompose eligible high-CRAP functions by whole branches while preserving
row-read order, typed-error precedence, floating-point grouping, and
accumulation order. Acceptance requires every eligible row at most `30`, exact
valid-output identity, unchanged public schema/API, and all focused/full gates.
Focused command:

    cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract

## FQ-05 — PMETPARA No-Action Disposition

No package is queued, and the originating package remains historical
`EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-FORMATTER-DISPOSITION`. This queue records a
new `DISPOSITIONED-NO-ACTION` decision; it does not rewrite that history. The
originating dual review granted the ADR-0021 observability-only exclusion for
`PmetparaParseError::fmt` (CC `12`, coverage `0%`, CRAP `156`); every other
logical row is at or below `30`. Current source SHA-256 is
`f4b09d61143b080bfecbaa37fe0b7a75099400069d378c06e50864a2b932dbb1`.
The raw module coverage below threshold is not independently actionable because
no test or production change is required by this queue.

If an operator later requests comprehensive PMETPARA coverage for its own value,
create a fresh module-test-enhancement package under a new authorization. Do not
describe that optional work as necessary to close this queue. If source identity
or the metric surface changes before queue completion, remeasure and independently
review this no-action disposition.

## Concrete Execution Steps

Work from `/home/workdir/openWEPP` on the current branch. Before the first queue
package, require a clean worktree and record the plan commit.

For each queue item:

1. Update its ledger state to `ACTIVE` and record the UTC timestamp.
2. Run `tools/agents/find-agents --for` every intended write path.
3. Scaffold the dated package and active kickoff prompt with
   `Execution mode: package-end-to-end`, `Autonomy:`, tiered required reading,
   reading budget/map, DC conversion/hold wording where applicable, and required
   heavy-run delegation.
4. Commit the scaffold before implementation/test edits.
5. For DC work, execute targeted provenance/contract amendment or confirmation,
   contract-derived failing tests, independent PASS of
   `artifacts/pre-implementation-contract-gate.md`, production correction,
   parser/consumer validation, coverage/obligation closure, and necessary
   mechanical decomposition in that order. Record the seven-factor conversion
   audit and applicable contract schema/profile checks.
6. For FQ-04, close characterization and the science-tier safety net before any
   behavior-preserving decomposition.
7. Complete current evidence, dual review, dual verification, and terminal
   disposition. Update this plan's ledger, progress, discoveries, decisions, and
   outcomes in the same terminal commit. If that is impossible, create and
   record a dedicated clean plan-transition commit before scaffolding the next
   package.

At the end, run a fresh workspace measurement through the delegated runner:

    cargo llvm-cov clean --workspace
    cargo llvm-cov --workspace --ignore-run-fail --lcov \
      --output-path /tmp/openwepp-cqr-followup-final.lcov
    cargo llvm-cov --workspace --ignore-run-fail --json \
      --output-path /tmp/openwepp-cqr-followup-final.json
    cargo crap --workspace \
      --lcov /tmp/openwepp-cqr-followup-final.lcov \
      --min 0 --format json \
      --output /tmp/openwepp-cqr-followup-final-crap.json

Record the source commit, exact commands, exit codes, timings, SHA-256 hashes,
and byte sizes for LCOV, JSON, and CRAP outputs. Attribute every known ignored
test failure and prove it was not introduced by the queue. Compare the four
target modules against their queue-entry and post-package baselines, then record
the new repository ranking without automatically starting another nightly batch.

## Validation And Acceptance

Every implementation package must run focused contract/fixture tests and target
LCOV/CRAP from the same source state. Material test changes must satisfy the
science-tier ADR-0021 `90%` line/region threshold, per-function floor, eligible-surface
exclusions, and complete obligation-to-test mapping.

Every package that changes Rust must finish with delegated current-source:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check

Also run `git diff --check` and scoped Markdown lint. Run source-level
anti-evasion gates when a package changes external-authority suite posture,
cohort fixtures, or required-case bindings. Defect packages must prove the real
downstream consumer reads corrected typed state when the package makes a
consumer/handoff claim. FQ-01 is intentionally parser-boundary-only and instead
proves rejected non-finite input cannot produce typed parser output. The
totalwatsed3 package must add independent operand/output reconstruction because
it publishes conservation-sensitive aggregates.

The plan is complete only when `FQ-01` through `FQ-04` have terminal queue-item
states, `FQ-05` remains no-action unless separately authorized, all triggered
packages have scaffold and terminal commits, every finding is dispositioned,
the worktree is clean, and the final rerank is recorded.

## Idempotence And Recovery

Scaffolding is additive and must not be repeated when the dated package already
exists. Resume from the last committed package/ledger transition. Never use a
destructive reset to recover a failed package. Revert only current-package
provisional edits with path-scoped patches, preserve evidence, and do not touch
unrelated user changes.

A failed focused or closure gate keeps the current package active until fixed.
A legitimate `HOLD` commits its evidence and boundary, then advances only if the
hold is local to that queue item. A global red baseline, unavailable evidence
environment, dirty overlapping target, or inability to create required commits
stops the whole queue.

## Authoring Review And Finding Disposition

This section is completed before changing plan status to `ready`. Review A must
audit process/governance, package boundaries, non-deferral, DC envelopes, commit
discipline, and queue transitions. Review B must audit technical findings,
canonical authority paths, acceptance tests, ordering, coverage/CRAP gates,
consumer/output proof, and PMETPARA no-action classification.

Disposition every finding as accepted, rejected, deferred, or follow-up.
Accepted findings are edited into this plan and reverified. A deferred/follow-up
authoring finding makes the recommendation `HOLD` unless it is explicitly
outside dispatch readiness. The final recommendation must be exactly `GO` or
`HOLD`, with rationale recorded here.

Initial reviews both recommended `HOLD`. All findings are accepted:

| Finding | Disposition and edit |
|---|---|
| `A1` same-package ADR/CRAP non-deferral | Accepted: each DC now owns correction through CRAP `<=30`; conditional successors removed. |
| `A2/B4` science-tier assignment | Accepted: all four actionable modules explicitly require `90%` line/region. |
| `A3` DC conversion sequence | Accepted: contract amendment, failing tests, pre-implementation gate, correction, validation, coverage/CRAP sequence is mandatory. |
| `A4` line-count governance | Accepted: `2000` WARN and `3000` closure block added. |
| `A5/B5` exact paths and commands | Accepted: contracts, specs, tests, fixtures, consumers, pinned files, and focused commands are named. |
| `A6` clean plan transitions | Accepted: plan/ledger updates land in terminal commits or a dedicated clean transition commit. |
| `A7` PMET history/identity | Accepted: originating hold is preserved; no-action is identity/metric conditional. |
| `B1` unpinned legacy anchors | Accepted: FQ-01/FQ-03 must re-anchor targeted clauses to baseline commit before production. |
| `B2` nonexistent fixed-date consumer | Accepted: FQ-01 is parser-boundary-only and prohibits runtime-readiness claims. |
| `B3` CHAN.INP invalid/valid cardinality | Accepted: `99+2` rejects; a distinct raw-count-closed fixture proves normalization. |
| `B6` sequence versus dependency | Accepted: rows use `WAITING-SEQUENCE`; local holds do not contaminate later rows. |
| `B7` rating-row heuristic risk | Accepted: lexical three-float heuristics are prohibited; EOF/channel/comment vectors govern recognition. |
| `B8` final measurement provenance | Accepted: source commit, hashes, sizes, timings, and ignored-failure attribution are required. |

No finding is rejected, deferred, or routed to a later authoring package.

Final re-review:

- Review A (governance/non-deferral): `GO`; A1-A7 and consumer/provenance
  clarifications verified with no remaining self-containment blocker.
- Review B (technical/authority): `GO`; B1-B8, PMET no-action, README, and
  ROADMAP consistency verified with no technical blocker.

Current authoring recommendation: **GO**.

## Plan Revision Note

2026-07-11: initial plan authored from the five nightly hold records. It creates
four actionable queue tracks, classifies PMETPARA as reviewed no-action, and
prevents both multi-module diffs and diagnostic relay packages.

2026-07-11: initial Review A/B findings accepted. Parser tracks now use one
non-deferring DC package through CRAP closure; all targets are science tier;
provenance, consumer boundaries, cardinality cases, exact paths/commands,
line-count governance, clean transitions, and final evidence provenance were
made explicit. Status remains HOLD until independent re-review passes.

2026-07-11: independent re-review A/B returned `GO`. Plan status changed to
`ready-queued`; no authoring finding is deferred or left undispositioned.

2026-07-11: FQ-01 completed parser-boundary finite-value defect, science-tier
coverage/A-H binding, and CRAP closure with dual PASS verification. FQ-02 is the
next serialized row.

2026-07-11: FQ-02 completed raw-before-normalized CHAN.INP record-cardinality
closure, exact diagnostic priority, normalized-count consumer proof,
science-tier coverage, and CRAP decomposition with dual PASS verification.
FQ-03 is the next serialized row.

2026-07-11: FQ-03 completed structural extra-rating recognition without lexical
heuristics, exact error precedence, real frame projection proof, science-tier
coverage, and CRAP closure with dual PASS verification. FQ-04 is the next row.
