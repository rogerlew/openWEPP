# Required Reading Map

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Required-reading budget:

- Local byte total: `296616`.
- Threshold: `OK` (`<=400000` bytes).
- Source: `wc -c` over kickoff required-reading paths.

Instruction discovery:

- `tools/agents/find-agents --for crates/openwepp-input-contract/src/parsers/management.rs docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/package.md`
  reported root `AGENTS.md`, `crates/AGENTS.md`, and
  `docs/work-packages/AGENTS.md`.

| Path | Tier | Budget counted | Rationale | Applicability trigger | Read status |
|---|---|---:|---|---|---|
| `AGENTS.md` | Core | yes | Root governance, CQR commit boundaries, parser/runtime posture. | Always | Read |
| `docs/work-packages/AGENTS.md` | Core | yes | Work-package execution, gate non-deferral, review/verification, CQR nightly process. | Always | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | yes | Operator shorthand, target selection, scaffold/commit protocol. | Always | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | yes | Behavior-preserving refactor procedure and closure ladder. | Always | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | yes | CRAP-specific cover-then-decompose procedure and behavior identity guard. | Always | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | yes | Glue-tier coverage and CRAP thresholds. | Always | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | yes | Required prompt shape and subagent wording. | Always | Read |
| `crates/AGENTS.md` | Core | yes | Rust crate, typed guard, API, and validation rules. | Editing crate code | Read |
| `tests/AGENTS.md` | Core | yes | Integration-test conventions and full gate expectations. | Adding/materially changing tests | Read |
| `docs/specifications/wepp-input-files/parser-contract-requirements.md` | Core | yes | Parser contract taxonomy and fail-closed surface separation. | Parser target | Read |
| `docs/contracts/openwepp-management-lanuse-authority-contract.md` | Core | yes | `lanuse`, datver, native routing extension, and fail-closed authority this parser preserves. | Parser target | Read |
| `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | On-demand | yes | Plant/management input wording and native landuse/routing coefficient context. | If refactor touches plant/native branches | Pending before relevant edits |
| `docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/package.md` | Core | yes | Package-local scope, phases, and exit criteria. | Always | Authored |
| `docs/work-packages/20260709-cqr-nightly-07-input-management-parser-001/artifacts/required-reading-map.md` | Core | yes | Local instruction map and reading state. | Always | Authored |
| `crates/openwepp-input-contract/src/parsers/management.rs` | On-demand | yes | Target module and source surface. | Before implementation | Inspected for scaffold sizing |
| `tests/integration/infile_management_parser_contract.rs` | On-demand | yes | Existing parser behavior oracle and likely characterization home. | Before characterization | Inspected |
| `tests/integration/infile_management_yaml_contract.rs` | On-demand | yes | Existing YAML dispatch/projection parser oracle. | If YAML helpers are touched | Inspected |
