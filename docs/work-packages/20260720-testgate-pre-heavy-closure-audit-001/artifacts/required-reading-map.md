# Required Reading Map

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11,317 | Repository governance and efficiency rules. |
| `docs/work-packages/AGENTS.md` | 22,837 | Package, pre-heavy, review, and closure rules. |
| `docs/standards/testing-and-gate-strategy.md` | 87,686 | Canonical lifecycle, audit, reuse, and retry authority. |
| `docs/standards/prompt-wording-guidance.md` | 10,221 | Kickoff and delegation wording. |
| `tools/local_ci/README.md` | 4,111 | Current local TESTGATE interface. |
| `package.md` | 21,208 | Complete execution authority. |
| `prompts/active/execute.md` | 3,505 | Package kickoff. |
| `artifacts/defect-inventory.md` | 3,357 | Required defects and closure proofs. |
| `artifacts/pre-heavy-audit-contract.md` | 2,649 | Typed artifact contract. |
| `artifacts/acceptance-matrix.md` | 3,143 | End-to-end failure and recovery obligations. |

Core total: 170,034 local bytes, `OK` (`<=400000`). Recalculate after accepted
scaffold-review edits and immediately before execution.

## Conditional

| Path | Trigger |
| --- | --- |
| `crates/AGENTS.md` | Before planner Rust edits. |
| `tests/AGENTS.md` and `tests/fixtures/AGENTS.md` | Before test or fixture edits. |
| Applicable output of `tools/agents/find-agents --for <write-paths>` | Before any governed edit. |

## On-Demand

| Path | Trigger |
| --- | --- |
| `crates/openwepp-gate-planner/**` | Implementing the planner, executor, verifier, ledger, or canonical model. |
| `gate-policy/v1/**` | Adding schemas, gate definitions, or valid/invalid fixtures. |
| `tools/local_ci/testgate.py` and `tests/python/test_testgate.py` | Implementing local orchestration and bootstrap behavior. |
| `.github/workflows/testgate-*.yml` and `.github/workflows/release-gates.yml` | Integrating the pre-heavy block in trusted workflows. |
| `docs/work-packages/20260720-testgate-adversarial-agent-acceptance-rerun-001/artifacts/failure-record.md` | Reproducing package-base authorization failure. |
| `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/**` | Reconstructing the costly closure failure sequence. |

Science-contract and kernel-process authority are not applicable because
simulation science and kernel paths are excluded.
