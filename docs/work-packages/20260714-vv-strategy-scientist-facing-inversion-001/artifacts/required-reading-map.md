# Required Reading Map

Status: `ACTIVE`

## Core

| Path | Reason |
| --- | --- |
| `AGENTS.md` | Root governance and documentation/work-package routing. |
| `docs/work-packages/AGENTS.md` | Package execution, review, evidence, and closure rules. |
| `docs/standards/AGENTS.md` | Requirements for a new reusable dossier standard. |
| `docs/standards/prompt-wording-guidance.md` | Kickoff prompt and subagent authorization requirements. |
| `package.md` | Authorized objective, write set, phases, and exit criteria. |
| `docs/governance/openwepp-verification-validation-strategy.md` | Current strategy being refactored. |
| `docs/decisions/0028-observed-data-admission-authority.md` | Existing observed-data authority and SNOTEL example. |
| `docs/specifications/correctness-authority-model.md` | Existing authority ladder that the strategy must not duplicate or replace. |

## Conditional

None. This package does not touch kernels, contracts, executable gates, or
dataset admission.

## On-Demand

| Resource | Trigger |
| --- | --- |
| Bibliography entries `R-114` through `R-124` | Retained research-basis language needs provenance confirmation. |
| Integrated-validation package artifacts | A current-state statement needs clarification beyond the strategy's existing characterization. |

## Budget

Ran: `wc -c` over the nine Core files measured `108880` local bytes. This is
`OK` under the governing threshold of no more than 400000 bytes.

## Applicable Instruction Chain

`tools/agents/find-agents --for` reported:

- governance strategy and root indexes: `AGENTS.md`;
- dossier standard: `AGENTS.md`, then `docs/standards/AGENTS.md`; and
- package artifacts: `AGENTS.md`, then `docs/work-packages/AGENTS.md`.
