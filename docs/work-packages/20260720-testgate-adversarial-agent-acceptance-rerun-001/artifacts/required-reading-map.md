# Required Reading Map

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11,317 | Repository invariants and test-economy rules. |
| `docs/work-packages/AGENTS.md` | 21,247 | Package, review, and closure governance. |
| `docs/standards/testing-and-gate-strategy.md` | 81,571 | Mechanical selection, reuse, receipt, and non-deferral authority. |
| `tools/local_ci/README.md` | 4,111 | Canonical local TESTGATE invocation. |
| `package.md` | 12,781 | Package objective, boundaries, and acceptance contract. |
| `prompts/active/execute.md` | 2,650 | Executor posture and delegation contract. |
| `artifacts/required-reading-map.md` | 1,616 | Reading applicability and budget. |

Core total: 135,293 local bytes, `OK` (`<=400000`).

## Conditional

| Path | Trigger |
| --- | --- |
| Applicable `AGENTS.md` files from `tools/agents/find-agents` | Before a governed path is edited. |

## On-Demand

| Path | Trigger |
| --- | --- |
| `docs/work-packages/20260719-testgate-global-crap-output-relocation-001/artifacts/terminal-execution-and-verification.md` | Reconstruct the retained exact closure receipt and subject. |
| `docs/work-packages/20260719-testgate-global-crap-output-relocation-001/artifacts/final-disposition.md` | Confirm predecessor closure and fresh-rerun handoff. |
| `crates/openwepp-gate-planner/**` | A local planner or verifier failure needs source interpretation. |
| `tools/local_ci/testgate.py` | Local helper behavior differs from its documented contract. |

Kernel and science-contract authority are not applicable because production and
science surfaces are read-only.
