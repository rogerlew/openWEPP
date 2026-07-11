# Pre-Integration CQR Campaign Execution Contract

Status: `BINDING`
Campaign: `CQR-PREINT-20260711`

This contract is incorporated by the campaign assessment and all four child
ExecPlans. A child or module package may add stricter gates but may not weaken
this contract.

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
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-lcov.time \
      cargo llvm-cov --workspace --ignore-run-fail --lcov \
      --output-path /tmp/openwepp-cqr-preint-<slug>-<phase>.lcov \
      > /tmp/openwepp-cqr-preint-<slug>-<phase>-lcov.log 2>&1
    /usr/bin/time -v -o /tmp/openwepp-cqr-preint-<slug>-<phase>-json.time \
      cargo llvm-cov --workspace --ignore-run-fail --json \
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
the separately reviewed actionable ranking. For the active target, the final
JSON must contain no actionable row above 30; also compare all non-target rows
against the child-start baseline and reject regressions. The final CRAP JSON,
not the LLVM coverage JSON, is the authority for CRAP rows.

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
This is unconditional. If current tests do not meet it, characterization tests
land and pass before production decomposition. The final campaign assessment
audits these gates for every executed module.

Classification is exact symbol/line/source-hash evidence. Review A checks
semantic and consumer eligibility; Review B checks source identity, metric
deduplication, and closed-list proof. Each finding is dispositioned
`accepted`, `rejected`, `deferred`, or `follow-up` with rationale. Accepted
findings are fixed and verified; any undispositioned finding blocks closure.

A fixed module whose current raw rows all receive jointly accepted
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` dispositions is
`DISPOSITIONED-NO-ACTION`. Do not create a fake implementation package. Commit
its source-bound classification, both reviews, public-behavior tests, and exact
disposition in the child evidence. Each fixed module therefore closes through
exactly one terminal implementation package or one committed no-action record.

## Required Module-Package Scaffold

For a module with actionable work, create the directory named by its child
ExecPlan and copy both:

- `docs/work-packages/templates/cqr-nightly-package.md` to `package.md`;
- `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md` to
  `prompts/active/<date>-codex-<target-id>-prompt.md`.

Create `prompts/archived/`, `artifacts/`, and
`artifacts/required-reading-map.md`. Before the scaffold commit, replace every
template placeholder and nightly-batch assumption. In particular, replace the
owning ExecPlan with the active campaign child, set campaign/target IDs and
write set, make a local module hold block that child rather than continue a
nightly batch, and fill the kickoff prompt's package-end-to-end mode, tiered
Core/Conditional/On-demand reading, byte budget/map, autonomy, exact gates,
and mandatory subagent wording. Run a placeholder audit:

    rg -n '<[^>]+>|cqr-nightly-burndown-execplan|EXECUTED-HOLD-CQR-NIGHTLY' \
      docs/work-packages/<active-package>

The result must be empty unless an exact reviewed campaign-specific occurrence
is documented. Commit the scaffold before Rust or test edits.

## Gate Evidence Non-Deferral And Line Counts

Every package binds the Gate Evidence Non-Deferral Rule in
`docs/work-packages/AGENTS.md`. Its gate table classifies every required gate as
`PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`. Any `FAIL`, `BLOCKED`, or unjustified
`NOT RUN` forces a package hold. Both independent reviews and both independent
verifications audit current-scope evidence legitimacy, not merely artifact
presence.

Run `wc -l` on every touched `.rs` file before and after work. A file at or
above 2,000 lines is `WARN` and requires decomposition rationale plus named
follow-up split intent. A 3,000-line-or-larger non-generated/non-fixture file
blocks closure unless an explicit exception names owner and sunset plan. Both
reviews and verifications disposition the thresholds.

## Heavy-Run Subagent Requirement

Subagent requirement: **REQUIRED**. Spawn `comparator_suite_runner` for every
workspace LCOV/JSON/CRAP run, full nextest, full Clippy, deny, comparator,
release, or cohort run. The parent must not run these heavy commands while that
subagent is available. Expected output is compact metrics, exit codes, timings,
artifact/log paths, hashes, and failure attribution; write access is read-only
except for explicitly named package evidence. If spawning is genuinely
unavailable, record the tool-policy/spawn failure with command-level evidence
before local substitution. Module implementers and independent review or
verification agents remain explicitly authorized with the bounded write sets
stated by the child plan and package prompt.

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
