# Required Reading Map

Status: `EXECUTED`

Read during W2 execution:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/decisions/0004-subprocess-hillslope-orchestration.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/prompts/active/kickoff.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional disposition:

- No `SC-*` contract was amended.
- No canonical `NoEvent` authority was admitted. Missing latest-event payloads
  therefore fail closed with `CLIWAT-E-045`.

## Core

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Root governance, validation truthfulness, and production Rust gates. |
| `docs/work-packages/AGENTS.md` | Work-package execution, review, and closure rules. |
| `docs/standards/prompt-wording-guidance.md` | Prompt/subagent authorization wording. |
| `docs/decisions/0004-subprocess-hillslope-orchestration.md` | Subprocess-per-hillslope boundary and no shell interpolation. |
| `docs/decisions/0032-watershed-runtime-ratification.md` | Public entrypoint, `--jobs` default, and benchmark mode. |
| `docs/architecture/watershed-runtime-architecture-specification.md` | W2 architecture and acceptance authority. |
| `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md` | Committed carnivorous-adobo fixture adoption boundary. |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | Fixture provenance and intended scope. |
| `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md` | W2 scope and gates. |

## Conditional

| Path | Trigger |
| --- | --- |
| `docs/specifications/science-contracts/AGENTS.md` | Required before any science-contract or kernel-affecting edit. |
| Relevant `docs/specifications/science-contracts/contracts/SC-*.md` | Required if admitting `NoEvent` as valid instead of hard-erroring missing latest-event payloads. |

## On-Demand

| Path | Trigger |
| --- | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | Primary current watershed CLI implementation. |
| `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | Child hillslope CLI command surface. |
| `crates/openwepp-runner/src/bin/open_wepp_runner.rs` | Existing runner/subprocess orchestration patterns. |
| `crates/openwepp-runner/src/launch.rs` | Existing `Command` launch helper patterns. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | Existing watershed CLI contract tests. |
| `tests/integration/infile_watershed_structure_parser_contract.rs` | Committed fixture parser/contract precedent. |
| Adjacent runner/orchestrator modules | Read only if touched by implementation. |
