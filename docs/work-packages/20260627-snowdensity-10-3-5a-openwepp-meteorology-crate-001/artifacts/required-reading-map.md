# Required Reading Map

Status: complete
Evidence mode: Static

## Budget

Current required local pre-read byte total: 522218 bytes.
Disposition: `WARN` (`<=800000`, no heavy-read justification required).

## Core

| Path | Bytes | Rationale |
|---|---:|---|
| `AGENTS.md` | 9439 | Root repository instructions. |
| `docs/codex_exec_plans.md` | 20443 | ExecPlan behavior and closure expectations. |
| `docs/work-packages/AGENTS.md` | 11901 | Work-package governance. |
| `docs/work-packages/README.md` | 147382 | Package catalog and current queue context. |
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
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | 290733 | Canonical snow/freeze process authority. |

## On-Demand

| Path | Applicability trigger |
|---|---|
| `docs/planning/snow-frost-fidelity-strategy.md` | Strategy context and 10.3.4 disposition. |
| `references/annotated_bibliography.md` | R-53 through R-57 context and citation metadata. |
| `references/copyrighted/source_pdfs/harder2013.pdf` | Equation/source extraction for implementation and provenance. |
| `/home/workdir/MetPy` | Optional BSD-3 numeric/reference cross-check for standard primitives only. |

## Execution Notes

- Static: Root, work-package, science-contract, crate, and test instructions
  were consulted before/while executing the package.
- Static: `docs/codex_exec_plans.md`, `docs/work-packages/README.md`,
  `docs/specifications/science-contracts/index.md`, and
  `docs/planning/snow-frost-fidelity-strategy.md` were consulted for package
  governance and 10.3.5 route context.
- Static: `references/copyrighted/source_pdfs/harder2013.pdf` was consulted for
  equations and coefficients. No CHM/GPL implementation code was read.
- Static: `/home/workdir/MetPy/src/metpy/calc/thermo.py` was consulted only as
  a BSD-3 reference for standard liquid/solid saturation-helper shape, not as a
  source translation target.
