# Pre-Integration CQR Campaign Execution Contract

Status: `BINDING`
Campaign: `CQR-PREINT-20260711`

This contract is incorporated by the campaign assessment and all four child
ExecPlans. A child or module package may add stricter gates but may not weaken
this contract.

## Revised Campaign Execution Model (Superseding, 2026-07-11)

The **tranche**, not each module, is the closure unit for workspace-wide
coverage, non-target ratchets, full gates, dual review, and dual verification.
This section supersedes later language in this contract and incorporated child
plans that requires those operations per module. Older sections remain useful
for command shapes and evidence fields, but their workspace-heavy cadence is
tranche-boundary-only.

This is an explicit campaign-specific aggregation authorization under
`docs/work-packages/AGENTS.md`: individual module checkpoints are staged
increments inside one active tranche and do not claim branch-head, merge, or
terminal readiness. The tranche-final record supplies the required full Rust
closure loop and independent terminal review/verification.

### Cost And Evidence Budget

For each tranche:

1. Run the exact workspace LCOV, JSON, CRAP, and filtered ranking once at
   tranche start and once at tranche final.
   At tranche start only, a predecessor JSON from the identical source commit
   may substitute for a lost report profile when the fresh LCOV and CRAP run
   completes, the mechanically filtered production census is byte-identical
   to the predecessor census, and the failed report plus exact hashes are
   archived. CRAP JSON remains CRAP authority and fresh LCOV remains coverage
   authority. This recovery is not available at tranche final.
2. For each module, run only focused tests plus focused crate/module coverage
   and CRAP sufficient to prove its tier, function floors, obligations, and
   target rows. Execute the instrumented focused suite once with
   `cargo llvm-cov ... --no-report`, then emit LCOV and JSON with two
   `cargo llvm-cov report` commands from that same profile. Do not rerun the
   suite merely to obtain the second format, and never repeat a workspace
   capture merely to make unrelated flaky coverage identical.
3. Run `cargo fmt --check`, workspace Clippy, full-profile nextest, and deny once
   at tranche final. During module work use focused tests and, after every three
   module checkpoints, `cargo nextest run --workspace --profile quick`.
4. Require one independent module review. Require a second module review only
   when the module proposes an `R-*`/`X-*` disposition, changes production
   control flow or public/serialization/conservation behavior, or discovers a
   semantic defect. Dual review and dual verification remain mandatory once at
   tranche final.
5. Do not require module-level verification agents. The parent records focused
   command evidence and checkpoint legitimacy; tranche-final verifiers audit
   every module record and the aggregate source state.

### Module Checkpoint Shape

Each actionable module receives a compact record under
`cqr-pre-integration-campaign-evidence/<slug>/modules/<target-id>.md`. Existing
scaffolded module packages may be retained, but new module scaffolds, placeholder
artifact sets, scaffold commits, and package-local full-gate loops are not
required. A module record contains:

- source hash, tier, raw/actionable rows, and exact classification;
- before/after focused coverage, function floors, CRAP, and commands;
- applicable A-H/named obligation map;
- behavior/numeric/consumer evidence proportional to the edit;
- one review verdict and finding disposition;
- line-count result and checkpoint commit.

Exactly one target module remains active at a time. Commit one reviewed module
checkpoint before editing the next target. `MODULE-PASS` means the focused
increment is eligible to enter tranche-final validation; it is not terminal
campaign or merge readiness. `MODULE-HOLD` requires a named authority or scope
boundary and blocks the tranche transition.

The default focused measurement shape is:

    cargo llvm-cov clean -p <package>
    cargo llvm-cov -p <package> --lib --no-report
    cargo llvm-cov report -p <package> --lcov --output-path <module>.lcov
    cargo llvm-cov report -p <package> --json --output-path <module>.json
    cargo crap --workspace --lcov <module>.lcov --min 0 \
      --format json --output <module>-crap.json

The LCOV and JSON hashes must therefore describe the same test execution and
source state. A package that cannot reuse one profile records the concrete tool
limitation before using a second instrumented execution.

### Final Ratchet And Flaky Coverage

The tranche-final exact workspace run is authoritative for target closure and
newly surfaced rows. Reject:

- any fixed-cohort eligible row still above 30;
- any new production row above 30;
- any increased row in a source-touched module; or
- an attributable regression in a real downstream consumer.

For unchanged non-target backlog, coverage-only numeric variation caused by a
source-unchanged, independently attributed flaky test is recorded as noise and
does not force repeated 35-minute captures. Prove the implicated source is
unchanged and run the failed test focused when practical. Never rerun workspace
coverage solely to obtain a preferred flaky failure set.

### Multi-Responsibility Host Coverage

For a fixed target in a production file whose unrelated sibling authorities
make whole-file focused coverage depend on broad runner/workspace scenarios,
module acceptance uses a target slice instead of forcing unrelated
characterization into the checkpoint. This applies only when the record names
the distinct sibling authority families and the focused profile proves the
whole-host gap is outside the target's obligation closure; file length alone is
neither required nor sufficient. The target slice is
the actionable function, every helper extracted from it, and the transitive
private-helper closure whose branches implement an applicable A–H or named
obligation. It must meet the tier line/region threshold, every eligible slice
function must meet the 75% region floor or an ADR-0021 retained-exception
disposition, and no slice CRAP row may exceed 30.

For preclassified `E-PRODUCTION` CLI/parser/validation/error/glue only,
ADR-0021 `R-LOW-COMPLEXITY-PRODUCTION` may disposition the function floor when
all target and expanded-slice rows are at CRAP at most 30, A–H and error
priority map to named executed tests, a real subprocess consumer verifies
external behavior, same-source coverage does not regress, the complete raw
floor census stays in the denominator/debt ledger, and two reviewers accept
each row. It never applies to `E-SCIENCE`, conservation/numerical work,
publication scalar/schema/order arithmetic, or ambiguous mixed helpers.

For an orchestration/serialization slice whose remaining uncovered lines are
exclusively mutually exclusive diagnostic modes, the slice may close on at
least 90% production base-function region coverage plus the 75% per-function
floor while retaining line coverage as visibility evidence. This requires: an
exact uncovered-mode map; focused guard/selector coverage; a completed real
nominal consumer; one bounded real-mode attempt when an authoritative fixture
exists; recorded timeout/failure attribution; and dual review. It does not
apply to physics, arithmetic, conservation operands, schemas, or the nominal
consumer path, and it does not weaken the tranche-final workspace ratchet.

This is not a tranche-closure denominator exclusion. Record whole-file focused
metrics and all live rows for visibility; do not disposition or waive untouched
siblings. The tranche-final exact workspace capture remains authoritative for
fixed-cohort closure, new rows, touched-module regressions, and real-consumer
regressions. Any production edit outside the declared slice expands the slice
to that function and its obligations. Reviewers must verify the source-line and
function boundary and reject a slice that omits a branch moved or edited by the
package.

### Heavy-Run Delegation

`comparator_suite_runner` is required for tranche-start/final workspace
LCOV/JSON/CRAP, tranche-final Clippy/full-nextest/deny, and explicit comparator,
release, or cohort runs. It is not required for focused module tests, focused
crate coverage, formatting, Markdown, diff checks, or ordinary source analysis.

## Status And Transition Tokens

Use only these child-plan status tokens:

- `QUEUED-READY`: reviewed and eligible to become active when its predecessor
  has a terminal PASS transition.
- `WAITING-SEQUENCE`: reviewed but blocked only by its named predecessor.
- `ACTIVE`: the only child plan currently executing.
- `TERMINAL-PASS`: every fixed target has either a completed implementation
  package or accepted classification-only no-action evidence, all current-scope
  gates pass, and the transition evidence is committed.
- `TERMINAL-HOLD`: a named blocker prevents the child acceptance criteria.

Only High A begins `QUEUED-READY`. A successor moves from `WAITING-SEQUENCE` to
`ACTIVE` in the same clean commit that records its predecessor's
`TERMINAL-PASS`. A child may not use `GO`; the campaign's Low/Assessment plan
alone emits `GO-INTEGRATED-VALIDATION` or `HOLD-CQR-FOLLOWUP`.

## Durable Evidence Paths

Campaign evidence lives under
`docs/work-packages/cqr-pre-integration-campaign-evidence/`. Each child uses its
literal slug: `ha`, `hb`, `medium`, or `low`. Create and commit:

- `<slug>/start-metrics.md` with commands, source commit, timings, exit codes,
  ignored-failure attribution, hashes, sizes, and same-source checks;
- `<slug>/raw-to-actionable-ledger.md` with every live raw row and exact
  classification;
- `<slug>/target-selection-review-a.md` and
  `<slug>/target-selection-review-b.md`;
- `<slug>/final-metrics.md` with the same fields as the start report and the
  before/after/disposition comparison;
- `<slug>/gate-results.md`, `<slug>/verification-a.md`,
  `<slug>/verification-b.md`, and `<slug>/transition.md`.

Low additionally creates `low/campaign-final-assessment.md` containing the full
original-67-row comparison, newly surfaced rows, terminal package/commit
ledger, and exact campaign recommendation. No `/tmp` artifact is durable
evidence; copy compact results into the named Markdown files before cleanup.

## Exact Measurement Protocol

Run from `/home/workdir/openWEPP`. Replace `<slug>` with one literal allowed
slug and `<phase>` with `start` or `final`; record the literal expanded commands
in evidence. The `comparator_suite_runner` subagent runs these heavy commands:

    cargo llvm-cov clean --workspace
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-run.time \
      cargo llvm-cov --workspace --ignore-run-fail --no-report \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-run.log 2>&1
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-lcov.time \
      cargo llvm-cov report --lcov \
      --output-path /tmp/openwepp-cqr-preint-<slug>-<phase>.lcov \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-lcov.log 2>&1
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-json.time \
      cargo llvm-cov report --json \
      --output-path /tmp/openwepp-cqr-preint-<slug>-<phase>.json \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-json.log 2>&1
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.time \
      cargo crap --workspace \
      --lcov /tmp/openwepp-cqr-preint-<slug>-<phase>.lcov \
      --min 0 --format json \
      --output /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.json \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.log 2>&1
    sha256sum /tmp/openwepp-cqr-preint-<slug>-<phase>.lcov \
      /tmp/openwepp-cqr-preint-<slug>-<phase>.json \
      /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.json
    wc -c /tmp/openwepp-cqr-preint-<slug>-<phase>.lcov \
      /tmp/openwepp-cqr-preint-<slug>-<phase>.json \
      /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.json

`--ignore-run-fail` permits report creation; it does not make a failed test
acceptable. Attribute every underlying failure and prove the active package did
not introduce it. Unattributed, target-related, or newly regressed failures are
blocking.

Apply the production filter and exact deduplication key from
`cqr-pre-integration-campaign-baseline.md`. Preserve the full raw ranking and
the separately reviewed actionable ranking. At tranche final, the fixed cohort
must contain no actionable row above 30. Compare the workspace ranking to the
child-start baseline using the revised ratchet: reject new rows, touched-module
regressions, and attributable consumer regressions; record source-unchanged
flaky-coverage variation in untouched backlog without retrying for a preferred
failure set. The final CRAP JSON, not LLVM coverage JSON, is CRAP authority.

Materialize the mechanically filtered rows with this exact shape, replacing the
literal slug and phase:

    jq '[.entries[]
      | select(.file | startswith("/home/workdir/openWEPP/crates/"))
      | select(.file | contains("/src/"))
      | select((.file | contains("/src/tests/")) | not)
      | select(.crap > 30)
      | {file:(.file | sub("^/home/workdir/openWEPP/"; "")),
         function, line, cyclomatic, coverage, crap}]
      | unique_by([.file,.function,.line,.cyclomatic,.coverage,.crap])
      | sort_by(.file,.line,.function)' \
      /tmp/openwepp-cqr-preint-<slug>-<phase>-crap.json \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-production-over30.json

Record the output hash/size beside the three primary artifacts. Reviewers must
reproduce its module/row counts from the primary CRAP JSON.

The exact documentation gate for the campaign and active package is:

    markdown-doc lint \
      --path docs/ROADMAP.md \
      --path docs/work-packages/README.md \
      --path docs/work-packages/cqr-pre-integration-campaign-assessment.md \
      --path docs/work-packages/cqr-pre-integration-campaign-baseline.md \
      --path docs/work-packages/cqr-pre-integration-campaign-execution-contract.md \
      --path docs/work-packages/cqr-high-risk-a-execplan.md \
      --path docs/work-packages/cqr-high-risk-b-execplan.md \
      --path docs/work-packages/cqr-medium-risk-execplan.md \
      --path docs/work-packages/cqr-low-priority-assessment-execplan.md \
      --path docs/work-packages/cqr-pre-integration-campaign-evidence \
      --path docs/work-packages/<active-package> \
      --format json

Omit only a not-yet-created evidence or package path and record why. Also run
`git diff --check`.

## Classification, Coverage, And No-Action Rule

Before decomposition, every actionable module must prove its ADR-0021 tier,
100% applicable A–H/named obligation binding, science coverage of at least 90%
line and region or glue coverage of at least 85% line and region, and no
eligible function below the 75% region floor without an accepted disposition.
For a qualifying multi-responsibility host, apply these thresholds
to the exact target slice defined above and retain whole-file metrics as
visibility evidence. Otherwise this is unconditional. If current tests do not
meet it, characterization tests land and pass before production decomposition.
The final campaign assessment audits these gates for every executed module.

Classification is exact symbol/line/source-hash evidence. One reviewer checks
semantic/consumer eligibility and source/metric identity. A second reviewer is
required for every proposed retained exception or denominator exclusion. Each finding is dispositioned
`accepted`, `rejected`, `deferred`, or `follow-up` with rationale. Accepted
findings are fixed and verified; any undispositioned finding blocks closure.

A fixed module whose current raw rows all receive jointly accepted
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` dispositions is
`DISPOSITIONED-NO-ACTION`. Do not create a fake implementation package. Commit
its source-bound classification, both required exception reviews, public-
behavior tests, and exact disposition in the child evidence. Each fixed module
therefore reaches exactly one reviewed implementation checkpoint or one
committed no-action record.

## Compact Module Record

For a module with actionable work, create
`cqr-pre-integration-campaign-evidence/<slug>/modules/<target-id>.md`. Do not
create a package scaffold, kickoff prompt, placeholder artifact set, or scaffold
commit. Record:

- source hash, tier, classification, and raw/actionable metrics;
- required reading and focused write set;
- before/after coverage, floors, CRAP, and exact focused commands;
- A-H/named obligation bindings and proportional consumer/numeric evidence;
- review findings/disposition, line count, and checkpoint status.

Existing scaffolds created before the execution-model revision may remain as
the module record. Commit the reviewed module source/tests and record together
before editing the next target.

## Gate Evidence Non-Deferral And Line Counts

Every module checkpoint binds the Gate Evidence Non-Deferral Rule for its
focused acceptance: coverage/floors, obligations, target CRAP, focused tests,
review, and line count must pass. Workspace metrics, non-target ratchet, full
gates, dual review, and dual verification are explicitly tranche-final scope;
their absence does not hold an intermediate module checkpoint. Tranche-final
reviewers and verifiers audit every module record and the aggregate gates.

Run `wc -l` on every touched `.rs` file before and after work. A file at or
above 2,000 lines is `WARN` and requires decomposition rationale plus named
follow-up split intent. A 3,000-line-or-larger non-generated/non-fixture file
blocks checkpointing unless an explicit exception names owner and sunset plan.
The module reviewer and tranche-final verifiers disposition the thresholds.

## Heavy-Run Subagent Requirement

Subagent requirement: **REQUIRED at tranche boundaries**. Spawn
`comparator_suite_runner` for tranche-start/final workspace LCOV/JSON/CRAP,
tranche-final nextest/Clippy/deny, comparator, release, or cohort runs. Focused
module tests/coverage do not require the heavy runner. Expected output is
compact metrics, exits, timings, paths, hashes, and failure attribution. If
spawning is unavailable, record the tool-policy failure before local
substitution.

## Semantic-Defect Transition

Campaign execution explicitly authorizes a defect-closure package for a
semantic defect discovered by characterization in a fixed target module. Name
it `YYYYMMDD-dc-cqr-preint-<target-id>-<defect-slug>-001`. Before production
edits, declare the Correction Authority Envelope, canonical contract/baseline
authority, protected boundaries, tests, real-consumer/conservation evidence,
dual review/verification, and terminal transition. Execute it end-to-end under
`docs/defect_closure_execplans.md`; then remeasure the original CQR target.

If correction requires a different process family, contradictory or missing
authority, an external state change, or a write set outside the fixed target's
authority envelope, record a hold-legitimacy audit and set the child to
`TERMINAL-HOLD`. Mere effort, diagnostic uncertainty, or an in-envelope
implementation is not a hold boundary.
