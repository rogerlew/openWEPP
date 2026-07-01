# Required Reading Map

Status: `QUEUED`

## Core Authority

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository governance, validation, and truthfulness rules. |
| `crates/AGENTS.md` | Rust crate authoring, typed guards, and closure gates. |
| `tests/AGENTS.md` | Test authoring and integration-test conventions. |
| `docs/work-packages/AGENTS.md` | Work-package execution, review, gate, and subagent rules. |
| `docs/standards/prompt-wording-guidance.md` | Standing subagent authorization and prompt wording. |
| `docs/standards/kernel-work-package-preparation.md` | Kernel/runtime package preparation and conservation acceptance requirements. |
| `docs/architecture/watershed-runtime-architecture-specification.md` | W4 target architecture and acceptance. |
| `docs/decisions/0032-watershed-runtime-ratification.md` | Public entrypoint, `--jobs` default, and benchmark mode. |
| `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/package.md` | Completed W3 worker-pool baseline and carry-forward boundaries. |
| `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/consumer-path-evidence.md` | Current W3 consumer-path proof and W4 old-surface boundary. |
| `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/line-count-governance.md` | Current line-count warning state for W4 governance. |
| `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/scaling-evidence.md` | Current committed-fixture scaling surface and timing baseline. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md` | Active W4 package contract. |

## Conditional Authority

| Trigger | Read |
| --- | --- |
| Any kernel-affecting, routing-semantic, publication-semantic, unit, guard, or canonical contract edit | `docs/specifications/science-contracts/AGENTS.md`. |
| Any canonical contract amendment | `docs/specifications/science-contract-authoring-procedure.md`, `docs/specifications/science-contracts/kernel-process-contract-profile.md`, and `docs/specifications/science-contracts/index.md`. |
| Routing, impoundment, runoff/sediment, water balance, or watershed-input surface touched | Relevant contracts: `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-RUNOFFPART-001`, `SC-SED-001`, `SC-WATBAL-001`, `SC-INFILE-WATERSHED-STRUCTURE-001`, `SC-INFILE-WATERSHED-CHANNEL-001`, and `SC-INFILE-WATERSHED-IMPOUNDMENT-001`. |

## On-Demand Implementation Context

| Path | Purpose |
| --- | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | Current public watershed CLI, old runtime-surface construction, and publication seed. |
| `crates/openwepp-runner/src/watershed_supervisor.rs` | W2/W3 plan, worker-pool, and pass inventory handoff. |
| `crates/openwepp-watershed-orchestrator/src/**` | Current watershed routing orchestrator and runtime surface code. |
| `crates/openwepp-watershed-output/src/**` | Current watershed output publication helpers. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | Public CLI behavior and output identity coverage. |
| `tests/integration/**` | Integration tests and source-guard tests if needed. |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | Committed 32-hillslope fixture context. |

## Budget

The core set is intentionally broader than W2/W3 because W4 owns a production
routing/publication cutover with conservation-sensitive outputs. Use targeted
section reads for on-demand context and load `SC-*` contracts only for touched
surfaces.
