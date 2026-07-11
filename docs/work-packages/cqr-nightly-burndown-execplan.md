# CQR Nightly Burndown ExecPlan

Status: **active**
Dispatch surface: **main** unless the operator explicitly authorizes a branch.
Owner: maintainers.
Last updated: 2026-07-11.

This ExecPlan defines the operator shorthand:

`execute cqr nightly for <N> modules`

The command means: measure the live workspace CRAP surface, select the top `N`
eligible production modules by current complexity-risk burden, scaffold one
ordinary work package per selected module, commit each scaffold, execute each
package end-to-end, and commit either completion evidence or hold evidence before
moving to the next selected target.

Each occurrence of this imperative is a fresh operator-authorized nightly batch.
Do not treat a completed batch in the current worktree or recent Git history as
completion of a later request. Use prior batches only as context or an exclusion
signal; perform the new live measurement and create newly numbered packages.
Only an explicit request to inspect, summarize, audit, verify, or avoid rerunning
the existing batch changes this interpretation.

For a second or later batch on the same calendar date, assign the next two-digit
batch ordinal (`b02`, `b03`, ...) and use it in every package identifier:
`YYYYMMDD-cqr-nightly-bNN-<rank>-<module-slug>-001`. The rank remains local to
that batch (`1` through `N`), so a new request for `10` modules always produces
ten newly selected package targets rather than extending or inspecting the prior
batch.

## Governance

This plan is subordinate to:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`

CQR nightly work is behavior-preserving maintenance. It may decompose functions,
add characterization tests, remove local lint debt, simplify control flow, and
record CRAP dispositions. It must not change scientific formulas, thresholds,
contracts, fail-closed behavior, public output semantics, serialization formats,
or production route ownership. If a target cannot be improved without such a
change, that target closes in `EXECUTED-HOLD-*` and routes to the appropriate
science, contract, defect-closure, or feature package.

## Target Selection

Run the live measurement from the current worktree before selecting targets:

1. `cargo llvm-cov clean --workspace`
2. `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov`
3. `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json`

The initial live CRAP/LCOV measurement is a heavy batch run. Delegate it to
`comparator_suite_runner` when available. If the subagent is unavailable, record
the spawn/tool-policy failure before running the measurement locally.

Candidate files are production Rust modules under `crates/**/src/`, excluding
test-only modules. Preserve the raw CRAP table, then classify every unique row
above 30 with the ADR-0021/module-test-enhancement §3 symbol-level taxonomy.
Do not infer eligibility from a filename such as `bin`, `error`, `parser`,
`adapter`, or `writer`.

Before ranking, produce a raw-to-actionable ledger with: exact file/function/
line, source SHA-256, CRAP/coverage/CC, classification, aggregate/floor/CRAP
treatment, evidence, and proposed reviewer disposition. Apply these rules:

- `E-SCIENCE` and `E-PRODUCTION` rows are actionable.
- `R-OBSERVABILITY` and `R-IRREDUCIBLE-CRAP` do not leave the actionable set
  until independent review accepts the exact disposition.
- `R-INFRASTRUCTURE` can waive only the 75% coverage floor; a CRAP row above 30
  remains actionable and must be decomposed or reclassified under another
  accepted category.
- `X-GENERATED`, `X-NONDEFAULT-CFG`, `X-DELEGATING-MAIN`, and `X-IMPOSSIBLE`
  require the exact closed-list proof; never exclude a whole module or glob.
- parser grammar/cardinality, guards, error precedence, state/order/key logic,
  numerical boundaries, serialization/publication, and consumer handoffs are
  always eligible when hand-authored.
- prior dispositions are evidence only; revalidate them when source identity,
  semantic role, complexity, or public behavior changed.
- files modified by unrelated work or owned by an active package are selection
  conflicts, not eligibility exclusions; record them separately and stop or
  skip according to the package-overlap rule.

Publish both raw and actionable module counts. Group accepted actionable rows by
module path and rank modules by:

1. total excess CRAP: `sum(max(crap - 30, 0))`;
2. number of unique production functions above `30`;
3. maximum CRAP in the module;
4. review judgment about output-risk and available characterization coverage.

Before finalizing the selected `N`, obtain two independent read-only reviews of
the proposed ledger: Review A checks semantic/consumer eligibility and closed-
list fit; Review B checks source identity, metric deduplication, and evidence.
Only jointly accepted `R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` rows
leave the actionable set. Any disagreement defaults to `E-PRODUCTION`.
Preserve both reviews with the first selected package as
`artifacts/target-selection-review-a.md` and
`artifacts/target-selection-review-b.md`.

Continue down the raw ranking until the requested `N` actionable modules are
selected. A module containing only accepted `R-OBSERVABILITY`,
`R-IRREDUCIBLE-CRAP`, or `X-*` rows is `DISPOSITIONED-NO-ACTION`, not one of the
`N`; record it and continue scanning.
Record the raw ranking, actionable ranking, classification ledger, conflicts,
and every accepted disposition in the first selected package's
`artifacts/target-selection.md`; repeat the selected target row and relevant
symbol ledger in each module package.

## Per-Module Package Shape

Each selected module is executed as its own normal work package:

`docs/work-packages/YYYYMMDD-cqr-nightly-<batch-prefix><rank>-<module-slug>-001/`

Use an empty `<batch-prefix>` for the first batch on a date and `bNN-` for a
second or later batch that day.

Use `docs/work-packages/templates/cqr-nightly-package.md` as the package-local
starting point and `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md`
as the active prompt starting point. The package must include:

- `package.md`
- `artifacts/`
- `prompts/active/`
- `prompts/archived/`
- package-local review, verification, gate, disposition, and handoff artifacts.
- `prompts/active/<date>-codex-cqr-nightly-<module-slug>_prompt.md`
- `artifacts/required-reading-map.md`

The scaffolded package and active kickoff prompt must explicitly authorize
subagent spawning/delegation for review, verification, comparator, and
closure-runner work, with bounded write access. The active kickoff prompt must
also include `Execution mode: package-end-to-end`, tiered required reading, a
required-reading budget, `Autonomy:`, and the required heavy-run subagent
directive from `docs/standards/prompt-wording-guidance.md`.

## Required Commit Boundaries

Every per-module package has two rollback points.

### Scaffold Commit

After scaffolding a per-module CQR package, commit the scaffold before any
production/test implementation edits for that target. The scaffold commit
contains:

- package directory and prompt skeletons;
- selected module row and intended write set;
- live CRAP/LCOV baseline summary or command provenance;
- package gates and hold/rollback rules.

If the operator has forbidden commits for the session, stop after scaffolding and
report that the ExecPlan-required scaffold commit is blocked by operator
direction.

### Completion Or Hold Commit

After executing a target package, commit one of:

- `EXECUTED-COMPLETE-*`: implementation, tests, after-metrics, review,
  verification, gates, disposition, and handoff;
- `EXECUTED-HOLD-*`: hold evidence, rollback proof, disposition, and first
  actionable follow-on.

Do not leave a completed or held package uncommitted before moving to the next
selected target unless the operator explicitly forbids commits.

## Execution Protocol

For each selected module:

1. Scaffold the package from the template.
2. Commit the scaffold.
3. Capture baseline metrics inside the package artifacts.
4. Confirm or create characterization coverage before refactoring.
5. If characterization tests are added or materially changed, record ADR-0021
   coverage closure: tier assignment, line/region threshold status,
   per-function 75% region-floor disposition, and obligation-to-test binding.
6. Refactor one cohesive extraction or local simplification at a time.
7. Preserve floating-point expression order and accumulation order.
8. Re-run focused tests and target CRAP after meaningful changes.
9. Stop editing when every owned production function is `<= 30` or explicitly
   dispositioned.
10. Run final gates and record exact commands, exit codes, and evidence.
11. Complete dual review, disposition findings, and dual verification.
12. Commit completion or hold evidence.
13. Continue to the next selected target only when the current target is
    complete or has a local hold.

## Hold And Rollback Rules

Implementation edits are provisional until the package reaches completion or a
hold disposition. Hold handling is split by blocker scope.

### Local Target Hold

A local target hold is confined to the selected module and does not undermine the
batch or the measurement baseline. Examples:

- the module needs a contract/science decision before decomposition can continue;
- no behavior identity fixture exists and characterization cannot be safely added
  inside the package;
- the target's CRAP rows are formatting/error-display variants that should be
  dispositioned rather than tested or decomposed;
- a refactor would require public API, serialization, or output changes.

For a local target hold:

1. Roll back only the in-progress production/test implementation edits for that
   package to the scaffold baseline.
2. Preserve package docs and artifacts proving the hold.
3. Record exact blocker, evidence, attempted in-envelope route, why it cannot be
   safely closed as CQR, and the first actionable follow-on.
4. Mark the package `EXECUTED-HOLD-*`.
5. Commit the hold package and continue to the next selected target.

Do not revert unrelated user changes or active-package work.

### Global Or Process Hold

A global hold stops the nightly batch. Examples:

- the workspace baseline gates are red before CQR edits;
- CRAP/coverage tooling is unavailable or produces unusable output;
- the selected target overlaps dirty active science or feature work;
- output identity breaks in a way that might implicate shared behavior;
- full closure gates fail outside the target package's control;
- the worktree cannot create required scaffold/hold/completion commits.

For a global hold:

1. Roll back in-progress implementation edits from the current package.
2. Preserve package docs and artifacts proving the global blocker.
3. Do not revert unrelated user changes, unrelated active-package work, or
   package evidence needed to explain the blocker.
4. Mark the current package `EXECUTED-HOLD-*` or leave it queued only if no
   package-specific evidence exists yet.
5. Stop the batch and report the blocker.

## Required Artifacts

Each per-module package records:

- `artifacts/target-selection.md`
- `artifacts/eligibility-classification.md` with raw/actionable counts and the
  exact symbol-level ledger
- `artifacts/target-selection-review-a.md`
- `artifacts/target-selection-review-b.md`
- `artifacts/crap-before.md`
- `artifacts/coverage-before.md`
- `artifacts/coverage-closure.md` when characterization tests are added or
  materially changed
- `artifacts/characterization.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/crap-after.md` when implementation lands
- `artifacts/coverage-after.md` when implementation lands
- `artifacts/numeric-equivalence.md`
- `artifacts/line-count-governance.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

Raw LCOV and full CRAP JSON may stay in `/tmp` when large. Commit compact,
package-local summaries, command provenance, hashes or paths, and the filtered
target rows needed to reproduce the decision.

## Required Gates

At minimum:

- `git diff --check`
- markdown/doc lint for touched docs
- focused tests for the touched module/crate
- target-module `cargo llvm-cov` / `cargo crap` before and after, or a recorded
  reason an after measurement is not possible on hold
- output identity or API/fixture identity appropriate to the touched surface
- `.rs` line-count governance
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If the package touches external-authority suite posture, cohort fixtures, or
required-case bindings, also run:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

Heavy closure/comparator runs are required to be delegated to the
`comparator_suite_runner` subagent when available, per
`docs/standards/prompt-wording-guidance.md`. The parent agent must not run those
heavy gates locally unless the subagent is unavailable and that unavailability is
recorded with command-level evidence.

## Completion

A nightly batch is complete when all selected targets have either:

- a completion commit proving target CRAP closure and behavior preservation; or
- a local hold commit with implementation edits rolled back and a concrete
  follow-on.

The final response reports:

- selected modules and ranking basis;
- packages created;
- completion/hold commit SHAs when committed;
- CRAP before/after or hold status per target;
- gates run;
- any global blocker that stopped the batch.
