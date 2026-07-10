# Required Reading Map

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Required-reading budget:

- Local byte total: `184459`.
- Threshold: `OK` (`<=400000` bytes).
- Source: `wc -c` over kickoff required-reading paths.

Instruction discovery:

- `tools/agents/find-agents --for crates/openwepp-runner/src/errors.rs docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/package.md`
  reported root `AGENTS.md`, `crates/AGENTS.md`, and
  `docs/work-packages/AGENTS.md`.

| Path | Tier | Budget counted | Rationale | Applicability trigger | Read status |
|---|---|---:|---|---|---|
| `AGENTS.md` | Core | yes | Root governance, CQR commit boundaries, runner posture. | Always | Read |
| `docs/work-packages/AGENTS.md` | Core | yes | Work-package execution, gate non-deferral, review/verification, CQR nightly process. | Always | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | yes | Operator shorthand, target selection, scaffold/commit protocol. | Always | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | yes | Behavior-preserving refactor procedure and closure ladder. | Always | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | yes | CRAP-specific cover-then-decompose procedure and behavior identity guard. | Always | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | yes | Glue-tier coverage and CRAP thresholds. | Always | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | yes | Required prompt shape and subagent wording. | Always | Read |
| `crates/AGENTS.md` | Core | yes | Rust crate, typed guard, API, and validation rules. | Editing crate code | Read |
| `tests/AGENTS.md` | Core | yes | Integration-test conventions and full gate expectations. | Adding/materially changing tests | Read |
| `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/package.md` | Core | yes | Package-local scope, phases, and exit criteria. | Always | Authored |
| `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/artifacts/required-reading-map.md` | Core | yes | Local instruction map and reading state. | Always | Authored |
| `crates/openwepp-runner/src/errors.rs` | On-demand | yes | Target module and source surface. | Before implementation | Inspected for scaffold sizing |
| `tests/integration/cli01_runner_contract_derived_tests.rs` | On-demand | yes | Existing runner contract test home and likely characterization surface. | Before characterization | Inspected |
| `crates/openwepp-runner/src/release.rs` | On-demand | no | Constructs release metadata/lint errors covered by target. | If release error construction behavior is touched | Pending before relevant edits |
| `crates/openwepp-runner/src/launch.rs` | On-demand | no | Constructs `RunnerError` launch variants. | If runner launch error construction behavior is touched | Pending before relevant edits |
