# Required Reading Map

Evidence class: `Static`.

## Core

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository-wide agent and validation invariants |
| `docs/work-packages/AGENTS.md` | Work-package planning and review rules |
| `docs/standards/AGENTS.md` | Standards authoring instructions |
| `docs/work-packages/20260727-gate-planner-advisory-linter-roadmap-001/package.md` | Order-0 authority and boundaries |
| `docs/work-packages/gate-planner-advisory-linter-roadmap.md` | Proposed philosophy and ordered roadmap |
| `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md` | Original planner authority |
| `docs/decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md` | CI/execution cutover authority |
| `docs/decisions/0041-separate-testgate-from-observational-quality-ci.md` | Later TESTGATE/quality boundary |
| `docs/decisions/0042-science-implementation-and-calibration-readiness.md` | Science implementation/readiness authority that planner removal must preserve |
| `docs/standards/testing-and-gate-strategy.md` | Current execution and lifecycle authority |
| `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/artifacts/testgate-trajectory-and-value-assessment.md` | Retained cost/value and stop-loss evidence |
| `docs/specifications/science-contracts/AGENTS.md` | Science-contract and correctness-authority boundaries |
| `docs/specifications/correctness-authority-model.md` | A0-A6 correctness-authority definitions and preservation requirements |
| `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/package.md` | Existing CAL authority and scope |
| `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/execution-control-contract.md` | Exact freeze, dual-verifier, and open-once Harvard controls |
| `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/executor-schema.md` | CAL command/output and Harvard path separation |

Core reading totals `233177` bytes at scaffold time, which is `OK` under the
repository's `<=400000` byte threshold.

## Conditional

| Path | Trigger |
| --- | --- |
| `docs/standards/prompt-wording-guidance.md` | Prompt or delegation wording |
| `docs/codex_exec_plans.md` | ExecPlan or downstream-package specification |
| `tools/local_ci/README.md` | Operator contract or command migration |
| Recent gate-planner work-package artifacts | Exact historical claim or consumer disposition |

## On Demand

| Path | Trigger |
| --- | --- |
| `crates/openwepp-gate-planner/**` | Exact capability/dependency inventory |
| `gate-policy/**` | Policy/schema retention or deletion decision |
| `.github/workflows/**` | CI retirement inventory |
| CAL-04B package-local tools | CAL and Harvard boundary discovery |
