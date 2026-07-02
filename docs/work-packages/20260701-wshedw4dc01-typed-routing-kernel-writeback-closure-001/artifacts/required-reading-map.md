# Required Reading Map

Status: `EXECUTED`

## Core Authority

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository governance, validation, and truthfulness rules. |
| `crates/AGENTS.md` | Rust crate authoring, typed guards, kernel behavior, and closure gates. |
| `tests/AGENTS.md` | Test authoring and integration-test conventions. |
| `docs/codex_exec_plans.md` | Base ExecPlan expectations. |
| `docs/defect_closure_execplans.md` | DC-ExecPlan envelope, conversion rule, seven-gate bar, and hold legitimacy. |
| `docs/work-packages/AGENTS.md` | Work-package execution, consumer-path, conservation, review, and subagent rules. |
| `docs/work-packages/README.md` | Package catalog and current held-package state. |
| `docs/standards/prompt-wording-guidance.md` | Standing subagent authorization and prompt wording. |
| `docs/standards/kernel-work-package-preparation.md` | Kernel package preparation and conservation acceptance requirements. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first rules for kernel-affecting routing/publication work. |
| `docs/architecture/watershed-runtime-architecture-specification.md` | Typed watershed runtime target architecture. |
| `docs/decisions/0032-watershed-runtime-ratification.md` | Public watershed runtime entrypoint and benchmark mode. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md` | W4 package scope and hold context. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/disposition.md` | Named hold blocker and first actionable follow-up. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/old-surface-inventory.md` | Remaining old-surface inventory. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/consumer-path-evidence.md` | W4 consumer-path proof and unsatisfied routing gate. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/source-guard-evidence.md` | Current source guards and remaining old-surface paths. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/operand-lineage.md` | Publication operand lineage carried forward. |
| `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/protected-output-evidence.md` | Partial output evidence and complete-closure gap. |
| `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/package.md` | Active package contract. |
| `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/correction-authority-envelope.md` | Active correction envelope. |

## Conditional Authority

| Trigger | Read |
| --- | --- |
| Any canonical contract amendment | `docs/specifications/science-contract-authoring-procedure.md`, `docs/specifications/science-contracts/kernel-process-contract-profile.md`, and `docs/specifications/science-contracts/index.md`. |
| Routing, impoundment, runoff-partition, sediment, water-balance, or watershed-input surface touched | Relevant `SC-*` contract: `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-RUNOFFPART-001`, `SC-SED-001`, `SC-WATBAL-001`, `SC-INFILE-WATERSHED-STRUCTURE-001`, `SC-INFILE-WATERSHED-CHANNEL-001`, or `SC-INFILE-WATERSHED-IMPOUNDMENT-001`. |

## On-Demand Implementation Context

| Path | Purpose |
| --- | --- |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | Current typed frame and compatibility projection. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs` | Current production dispatch and writeback application. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**` | Current symbol-map request/writeback routing helpers. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs` | Current writeback/request/report types. |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | Public CLI routed-stage handoff. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | Public CLI behavior and W4 source guard tests. |
| `tests/integration/**` | Orchestrator and source-guard integration tests if needed. |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | Committed fixture context for protected output checks. |

## Budget

The core set is acceptable for a W4 hold-lift defect-closure package. Use
targeted on-demand reads for implementation modules and load `SC-*` contracts
only for touched surfaces.
