# Required Reading Map

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11,317 | Repository governance and provider safety. |
| `docs/work-packages/AGENTS.md` | 22,837 | Package, review, audit, and evidence rules. |
| `docs/standards/testing-and-gate-strategy.md` | 87,686 | Pre-heavy, receipt reuse, retry, and provider authority. |
| `docs/standards/prompt-wording-guidance.md` | 10,221 | Delegation and heavy-run wording. |
| `package.md` | 17,483 | Complete qualification authority. |
| `prompts/active/execute.md` | 3,296 | Executor and provider constraints. |
| `artifacts/qualification-matrix.md` | 2,982 | Frozen case requirements. |
| `artifacts/controller-contract.md` | 1,251 | Controller and dispatch boundaries. |
| `artifacts/evidence-contract.md` | 1,264 | Required machine evidence. |

Core total: 158,337 local bytes, `OK` (`<=400000`). Recalculate after scaffold
review and immediately before execution.

## Conditional

| Path | Trigger |
| --- | --- |
| Applicable output of `tools/agents/find-agents --for <write-paths>` | Before package, roadmap, or catalog edits. |
| `docs/work-packages/20260720-testgate-pre-heavy-closure-audit-001/artifacts/final-disposition.md` | Implementation intake reports completion. |
| Implementation terminal plans, audit, receipts, ledger, and reviews | Before subject freeze. |
| `.github/workflows/testgate-shadow.yml` | Before provider preflight or evidence acceptance. |

## On-Demand

| Path | Trigger |
| --- | --- |
| `tools/local_ci/testgate.py` and frozen qualification interface | Validate real-entry-point use or interpret a case failure. |
| `crates/openwepp-gate-planner/**` and `gate-policy/v1/**` | Anti-fabrication inspection or independent failure interpretation. |
| Retained prior adversarial acceptance artifacts | Compare historical reproducer behavior. |

Science-contract and kernel-process authority are not applicable because the
subject is gate infrastructure and no source edit is authorized.
