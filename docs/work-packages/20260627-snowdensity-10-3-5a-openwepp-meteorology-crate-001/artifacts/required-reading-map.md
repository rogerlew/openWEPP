# Required Reading Map

Status: queued
Evidence mode: Static

## Budget

Current required local pre-read byte total: 648053 bytes.
Disposition: `WARN` (`<=800000`, no heavy-read justification required).

## Core

| Path | Bytes | Rationale |
|---|---:|---|
| `AGENTS.md` | 9439 | Root repository instructions. |
| `docs/codex_exec_plans.md` | 20443 | ExecPlan behavior and closure expectations. |
| `docs/work-packages/AGENTS.md` | 11901 | Work-package governance. |
| `docs/work-packages/README.md` | 146846 | Package catalog and current queue context. |
| `docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/package.md` | 11392 | Local package authority. |

## Conditional Required

Required because this package amends canonical snow/freeze authority and adds a
candidate physics/numerics crate.

| Path | Bytes | Rationale |
|---|---:|---|
| `docs/specifications/science-contracts/AGENTS.md` | 5585 | Science-contract playbook. |
| `docs/specifications/science-contract-authoring-procedure.md` | 12423 | Contract amendment procedure. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | 5044 | Kernel contract profile. |
| `docs/specifications/science-contracts/index.md` | 7876 | Contract registry context. |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | 281884 | Canonical snow/freeze process authority. |

## On-Demand

| Path | Applicability trigger |
|---|---|
| `docs/planning/snow-frost-fidelity-strategy.md` | Strategy context and 10.3.4 disposition. |
| `references/annotated_bibliography.md` | R-53 through R-57 context and citation metadata. |
| `references/copyrighted/source_pdfs/harder2013.pdf` | Equation/source extraction for implementation and provenance. |
| `/home/workdir/MetPy` | Optional BSD-3 numeric/reference cross-check for standard primitives only. |

## Execution Notes

- Do not read CHM/GPL implementation code.
- If another large local reference becomes mandatory pre-read, update this map
  before implementation.
