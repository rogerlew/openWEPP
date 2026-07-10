# CQR Nightly 08 Runner Errors Kickoff

Scope: local repository code-quality refactor task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and the nearest relevant
  `SC-*` contract only if refactoring expands into contract-derived tests or
  kernel-facing behavior beyond runner error output identity.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-runner/src/errors.rs`
- `tests/integration/cli01_runner_contract_derived_tests.rs`
- `crates/openwepp-runner/src/release.rs`
- `crates/openwepp-runner/src/launch.rs`
- adjacent modules that construct or match runner errors

Required-reading budget: `184459` bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/errors.rs`
- `tests/integration/cli01_runner_contract_derived_tests.rs`
- `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/**`

Task: execute the CQR nightly package for
`crates/openwepp-runner/src/errors.rs` end-to-end. Reduce every eligible
production function above CRAP `30` to `<= 30`, or record an ADR-0021-style
disposition and hold when behavior-preserving CQR cannot close the target
safely.

Constraints:

- Behavior-preserving CQR only.
- No error-code, CLI-visible message meaning, source-chain, release-contract,
  serialization, fail-closed, or public-output semantic changes.
- Preserve display wording, source ownership, short-circuit behavior, and public
  error APIs.
- No opportunistic cleanup outside the declared target.
- Do not edit unrelated dirty files or active-package work.
- Package-required validation overrides ambient test-skip guidance.
- Target line-count governance: `errors.rs` starts at `549` lines, below WARN
  and blocker thresholds.

Coverage closure: if characterization tests are added or materially changed,
record ADR-0021 glue-tier line/region threshold status, per-function
region-floor disposition, and obligation-to-test binding in
`artifacts/coverage-closure.md` before decomposition closes.

Hold handling: local target holds roll back only current-package
production/test implementation edits to the scaffold baseline, preserve and
commit package hold evidence, and may continue to the next selected module.
Global/process holds stop the nightly batch. Do not revert unrelated user
changes.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs, including `cargo nextest run --workspace --profile
full`, full-workspace CRAP/coverage runs after implementation, comparator
suites, and population/fixture batches. Do NOT run those heavy gates on the
parent model unless the subagent is unavailable; if unavailable, record
command-level evidence before running locally. This prompt explicitly authorizes
subagent spawning/delegation to comparator/closure-runner, review, and
verification subagents for CQR metric checks, focused/full gate execution,
behavior-identity verification, review, and verification. Outputs:
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access: read-only
unless a subagent is explicitly assigned a bounded implementation fix in the
declared source/test write set or package-local artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update all package artifacts, disposition every review finding, and
leave the package ready for its completion or hold commit.
