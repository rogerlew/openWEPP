# CQR Nightly 07 Input Management Parser Kickoff

Scope: local repository code-quality refactor task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/wepp-input-files/parser-contract-requirements.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and the nearest relevant
  `SC-*` contract only if refactoring expands into contract-derived tests or
  kernel-facing behavior beyond parser output identity.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-input-contract/src/parsers/management.rs`
- `tests/integration/infile_management_parser_contract.rs`
- `tests/integration/infile_management_yaml_contract.rs`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- adjacent parser modules only if imported helper behavior needs comparison

Required-reading budget: `296616` bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-input-contract/src/parsers/management.rs`
- `tests/integration/infile_management_parser_contract.rs`
- `tests/integration/infile_management_yaml_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/**`

Task: execute the CQR nightly package for
`crates/openwepp-input-contract/src/parsers/management.rs` end-to-end. Reduce
every eligible production function above CRAP `30` to `<= 30`, or record an
ADR-0021-style disposition and hold when behavior-preserving CQR cannot close
the target safely.

Constraints:

- Behavior-preserving CQR only.
- No parser contract, plant-file spec, `lanuse` authority, datver, threshold,
  tolerance, serialization, fail-closed, stable error ID, or public-output
  semantic changes.
- Preserve parsed field order, statement order, floating-point expression
  grouping, accumulation order, and short-circuit behavior.
- No opportunistic cleanup outside the declared target.
- Do not edit unrelated dirty files or active-package work.
- Package-required validation overrides ambient test-skip guidance.
- Keep target line-count governance in view: `management.rs` starts at `2851`
  lines, and closure is blocked if it crosses `3000` without a valid refactor.

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
