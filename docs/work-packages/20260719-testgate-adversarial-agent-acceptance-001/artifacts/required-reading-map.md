# Required Reading Map

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11,317 | Repository invariants and test-economy rules. |
| `docs/work-packages/AGENTS.md` | 21,247 | Package, review, and closure governance. |
| `docs/standards/testing-and-gate-strategy.md` | 81,571 | Mechanical gate selection and receipt authority. |
| `tests/AGENTS.md` | 4,684 | Focused integration-test execution rules. |
| `tools/local_ci/README.md` | 4,111 | Canonical local TESTGATE invocation. |
| `package.md` | 8,440 | Package objective and bounded gate plan. |
| `prompts/active/execute.md` | 2,038 | Executor posture and handoff. |
| `artifacts/required-reading-map.md` | 1,339 | Reading applicability and budget. |

Core total: 134,747 bytes, `OK` (`<=400000`).

## Conditional

| Path | Trigger |
| --- | --- |
| Applicable `AGENTS.md` files from `tools/agents/find-agents` | Before a governed path is edited. |

## On-Demand

| Path | Trigger |
| --- | --- |
| `.github/workflows/testgate-shadow.yml` | Workflow ordering, labels, or live result interpretation. |
| `crates/openwepp-gate-planner/src/**` | A focused planner test fails. |
| `tests/integration/testgate_*.rs` | A focused integration contract fails. |

Kernel/science-contract authority is not applicable because production and
science surfaces are read-only.
