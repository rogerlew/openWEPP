# CQR Nightly 10 Runner Laned Shadow Kickoff

Scope: local repository code-quality refactor task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/artifacts/required-reading-map.md`

Conditional:

- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.
- Adjacent direct-publication source files only if characterization needs to
  prove a live consumer/producer path beyond the existing source guards.

On-demand:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
- `tests/integration/laned_shadow_h2637.rs`
- adjacent modules imported by `laned_shadow.rs`

Required-reading budget: `354434`, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs` only if needed
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
  only if needed
- `tests/integration/laned_shadow_h2637.rs` only if needed
- `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/**`

Task: execute the CQR nightly package for
`crates/openwepp-runner/src/hillslope/laned_shadow.rs` end-to-end. Reduce every
eligible production function above CRAP `30` to `<= 30`, or record an
ADR-0021-style disposition and hold when behavior-preserving CQR cannot close
the target safely.

Constraints:

- Behavior-preserving CQR only.
- No science-formula, threshold, tolerance, contract-authority, serialization,
  fail-closed, selector-policy, manifest-semantic, or public-output changes.
- Preserve floating-point statement order, expression grouping, accumulation
  order, and short-circuit behavior.
- Preserve Lane D shadow diagnostics, active/shadow mutual exclusion,
  coefficient authority, dynamic operand validation, protected output identity,
  and profile-slot accounting.
- No opportunistic cleanup outside the declared target.
- Do not edit unrelated dirty files or active-package work.
- Package-required validation overrides ambient test-skip guidance.

Coverage closure: if characterization tests are added or materially changed,
record ADR-0021 tier assignment, line/region threshold status, per-function
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
target module, named focused tests, or package-local artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update all package artifacts, disposition every review finding, and
leave the package ready for its completion or hold commit.
