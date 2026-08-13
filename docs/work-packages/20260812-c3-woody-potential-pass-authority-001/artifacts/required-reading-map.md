# Required Reading Map

Status: `intake complete`

Evidence mode: `Static`

| Tier | Path or authority | Rationale |
| --- | --- | --- |
| Core | `AGENTS.md` | Repository-wide governance. |
| Core | `docs/codex_exec_plans.md` | Living ExecPlan requirements. |
| Core | `docs/work-packages/AGENTS.md` | Package lifecycle, review, and verification governance. |
| Core | `docs/work-packages/README.md` | Package catalog and active-package context. |
| Core | package-local `package.md` | Exact objective, scope, write set, gates, and protected boundaries. |
| Conditional | `docs/specifications/science-contract-authoring-procedure.md` | Canonical contract revision procedure. |
| Conditional | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Required kernel-contract schema. |
| Conditional | `docs/specifications/science-contract-spec.md` | Contract artifact, guard, alias, calibration, and binding schema. |
| Conditional | `docs/specifications/unit-governance.md` | Dimensional symbol and conversion authority. |
| Conditional | `docs/specifications/science-contracts/index.md` | Lifecycle registry. |
| Conditional | `docs/standards/testing-and-gate-strategy.md` | Critical validation selection and evidence rules. |
| On-demand | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | V6 predecessor read completely; now the V7 draft authority surface. |
| On-demand | `docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/` | V2 predecessor package, model identity, oracle, vectors, and terminal evidence. |
| On-demand | `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/` | V1/V2 canonical model definitions and numerical authority. |
| On-demand | `docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/potential-pass-hold-legitimacy-audit.md` | Exact authority gaps that triggered this package. |
| On-demand | CLM5 surface-albedo, flux, plant-hydraulic, and photosynthesis technical notes | Primary external scientific provenance for selected corrections. |
| On-demand | ESCOMP/CTSM commit `8e1309ab0db671d884b80746cbae9bbaafbe78a7`, `src/biogeophys/PhotosynthesisMod.F90` | Immutable Atkin coefficient, leaf-N unit, branch, and output-basis transcription authority. |

Local pre-read estimate before package-local files: `648494` bytes, `WARN`.
The large package catalog remains Core because repository governance requires it;
mechanism-specific authorities remain On-demand.

Instruction discovery ran for the package, canonical contract, model-definition,
production vegetation crate, and integration-test paths. Applicable instructions
are:

- `AGENTS.md`;
- `docs/work-packages/AGENTS.md`;
- `docs/specifications/science-contracts/AGENTS.md`;
- `crates/AGENTS.md` and `crates/openwepp-vegetation/AGENTS.md` only if a later
  separately authorized implementation package edits those paths;
- `tests/AGENTS.md` for contract-derived integration tests.
