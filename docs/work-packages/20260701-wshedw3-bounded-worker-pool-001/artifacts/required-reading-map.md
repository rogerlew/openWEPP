# Required Reading Map

Status: `EXECUTED`

## Core Authority

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository governance, validation, and truthfulness rules. |
| `docs/work-packages/AGENTS.md` | Work-package execution, review, gate, and subagent rules. |
| `docs/standards/prompt-wording-guidance.md` | Standing subagent authorization and prompt wording. |
| `docs/decisions/0004-subprocess-hillslope-orchestration.md` | Subprocess-per-hillslope orchestration authority. |
| `docs/decisions/0032-watershed-runtime-ratification.md` | Public entrypoint, `--jobs` default, and benchmark mode. |
| `docs/architecture/watershed-runtime-architecture-specification.md` | W3 worker-pool target architecture and acceptance. |
| `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md` | Completed W2 baseline and carry-forward boundaries. |
| `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/consumer-path-evidence.md` | Current public runner consumer-path proof to extend. |
| `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/line-count-governance.md` | Current line-count warning state for W3 governance. |
| `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md` | Fixture adoption contract and auditability boundary. |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | Committed 32-hillslope canonical W3 fixture context. |
| `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/package.md` | Active W3 package contract. |
| `docs/specifications/science-contracts/AGENTS.md` | Science-contract guard posture for the user-authorized fixture-only `radly` clamp. |
| `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | `INV-CLIMATE-013` daily `radly` guard and no-production-clipping authority. |

## Conditional Authority

| Trigger | Read |
| --- | --- |
| Any kernel-affecting or science-contract edit | `docs/specifications/science-contracts/AGENTS.md` plus relevant `SC-*` contracts. |
| Any change to latest-event payload semantics, `NoEvent`, routing physics, or publication meaning | `docs/specifications/science-contracts/AGENTS.md` plus relevant `SC-*` contracts. |

## On-Demand Implementation Context

| Path | Purpose |
| --- | --- |
| `crates/openwepp-runner/src/watershed_supervisor.rs` | W2 plan, job, pass inventory, and serial execution implementation. |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | Public watershed CLI parser and routing handoff. |
| `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | Hillslope child command behavior. |
| `crates/openwepp-runner/src/launch.rs` | Explicit hillslope argv construction. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | W2 public CLI behavior coverage to extend. |
| `tests/integration/infile_watershed_structure_parser_contract.rs` | Committed fixture parser guard if fixture metadata changes. |

## Budget

The required-reading set is acceptable for an implementation package that edits
the W2 supervisor and public CLI. Use targeted `rg` and section reads for
on-demand context rather than loading unrelated architecture or contract text.

Execution note: no production kernel science-contract, latest-event semantic,
routing physics, or publication-schema edits were made. Conditional
science-contract authority was triggered only by the user-authorized
fixture-data `radly` clamp; production runtime guard behavior remains
fail-closed and unmodified.
