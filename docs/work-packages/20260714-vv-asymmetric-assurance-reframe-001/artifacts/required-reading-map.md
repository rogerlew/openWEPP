# Required Reading Map

Status: `ACTIVE`

## Core

| Path | Reason |
| --- | --- |
| `AGENTS.md` | Root governance and package routing. |
| `docs/work-packages/AGENTS.md` | Package evidence, dual review, and closure rules. |
| `docs/standards/AGENTS.md` | Normative standard maintenance requirements. |
| `docs/standards/prompt-wording-guidance.md` | Kickoff and delegated-review wording. |
| `package.md` | Authorized objective, write set, phases, and exit criteria. |
| `docs/governance/openwepp-verification-validation-strategy.md` | Active strategy being reframed. |
| `docs/standards/scientific-assurance-dossier.md` | Active dossier workflow and status language being reframed. |
| Prior inversion package final disposition | Dependency and historical closure boundary. |
| `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` | Comparator evidence remains non-authoritative. |
| `docs/decisions/0028-observed-data-admission-authority.md` | Forcing-robust and observed-data posture. |
| `docs/specifications/correctness-authority-model.md` | Existing authority planes and executable gates remain unchanged. |

## Conditional

| Path | Trigger |
| --- | --- |
| `references/annotated_bibliography.md`, entries `R-114` through `R-125` | Research-basis or bibliography edits. |

## On-Demand

Primary-source landing pages for NRC, EPA, Oreskes, Nearing, and Wang are
loaded only when canonical metadata is insufficient. The current task already
confirmed the Oreskes abstract, EPA environmental-decision framing, and NRC
licensing/continuing-oversight distinction.

## Budget

Ran: `wc -c` over the 12 Core files measured `114476` local bytes. This is `OK`
under the threshold of no more than 400000 bytes.

## Applicable Instruction Chain

`tools/agents/find-agents --for` reported:

- governance, bibliography, and root indexes: `AGENTS.md`;
- dossier standard: `AGENTS.md`, then `docs/standards/AGENTS.md`; and
- package/index artifacts: `AGENTS.md`, then
  `docs/work-packages/AGENTS.md` where applicable.
