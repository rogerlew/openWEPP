# Required Reading Map

Status: `active`.

## Core

| Path | Rationale |
| --- | --- |
| `AGENTS.md` | Repository invariants and routing. |
| `docs/codex_exec_plans.md` | Living autonomous ExecPlan rules. |
| `docs/work-packages/AGENTS.md` | Package execution, evidence, and closure rules. |
| `docs/work-packages/README.md` | Mandatory catalog and campaign context. |
| `package.md` | EB-04V objective, write set, phases, and gates. |

## Conditional — Triggered

| Path | Trigger and rationale |
| --- | --- |
| `docs/specifications/science-contracts/AGENTS.md` | Canonical contract amendment. |
| `docs/specifications/science-contract-authoring-procedure.md` | Contract cycle and review gates. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Kernel-adjacent runtime authority. |
| `docs/specifications/science-contracts/index.md` | Contract lifecycle registry. |
| `docs/specifications/unit-governance.md` | New dimensional trace fields. |
| `docs/decisions/0042-science-implementation-and-calibration-readiness.md` | Diagnostic/calibration claim boundary. |
| `crates/AGENTS.md` | Rust production rules. |
| `tests/AGENTS.md` | Contract and consumer test rules. |

## On Demand — Triggered By Mechanism

| Path | Trigger and rationale |
| --- | --- |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | Existing density equations, active default/cap, variables, aliases, tests, gaps, and history. |
| `docs/work-packages/20260801-snow-surface-eb-04u-mechanistic-failure-partition-001/` | Frozen population, phase, evidence-role, and admission rules. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | Density process producer. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | Real JSONL consumer. |
| EB-04S/04R harness and provenance artifacts | Exact B/L/S/LS runner identities and sanitized execution pattern. |

The currently triggered local corpus is approximately `1.0 MB`, which is
`REQUIRES_JUSTIFICATION` under the `800000`-byte threshold. The contract is
large because it contains the active layered history of density authority; its
relevant binding residues and aliases cannot be safely inferred from one
addendum. The catalog is mandatory Core reading and supplies package lifecycle
context. Source and retained run files remain on-demand and are read only as
needed for the touched mechanism.
