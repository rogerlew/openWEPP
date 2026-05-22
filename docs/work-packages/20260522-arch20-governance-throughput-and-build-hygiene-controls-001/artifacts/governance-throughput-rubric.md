# Governance Throughput Rubric

Evidence mode: `Static`
Status: `complete`

## Source Finding Linkage

- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` (`CRF-008` required evidence: throughput rubric, WIP limits, closure SLA).
- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/claude-review-findings-register.md` (`CRF-008` cadence risk: process scaffolding can outpace integrable capability).
- [DIRECT] `docs/work-packages/README.md` (high package volume and parallel wave shape).

## Purpose

Define a measurable, auditable rubric that ties governance work to delivered
openWEPP engine capability outcomes and prevents package-count theater.

## Normative Requirements

1. Every work package closeout MUST include a throughput score using this rubric.
2. Throughput scoring MUST prioritize integrable engine outcomes over artifact
   volume.
3. A package MUST be marked `HOLD` if any required rubric dimension is missing.
4. Closure claims MUST cite direct evidence for each dimension score.

## Scoring Dimensions

| id | dimension | score range | scoring rule | fail trigger |
|---|---|---:|---|---|
| `GT-001` | Capability Outcome Delta | 0-3 | `3`: landed engine-executable capability or defect closure; `2`: landed typed contract/tests directly unblocking engine behavior; `1`: governance artifact with explicit adoption hook tied to active `CRF-*`; `0`: no capability path. | score `0` on code-touch package. |
| `GT-002` | Integration Closure Quality | 0-3 | `3`: producer-consumer ownership and closure evidence complete; `2`: representative but non-exhaustive with explicit amendment; `1`: partial mapping; `0`: no closure mapping. | score `<2` without disposition amendment. |
| `GT-003` | Gate Integrity | 0-2 | `2`: all required gates executed and recorded truthfully; `1`: partial gate execution with explicit `HOLD`; `0`: missing or misclassified gate evidence. | score `<2` on `GO` claim. |
| `GT-004` | Churn Control | 0-2 | `2`: no duplicate/reopened package churn for same objective during execution window; `1`: one justified reopen with root cause; `0`: repeated churn / false closeout pattern. | score `0` without corrective action plan. |

## Decision Thresholds

- Code-touch package:
  - MUST score `>=8/10`.
  - MUST score `>=2` on `GT-001`, `GT-002`, and `GT-003`.
- Docs-only package:
  - MUST score `>=7/10`.
  - MUST score `>=1` on `GT-001` and `2` on `GT-003`.
- Any package below threshold MUST remain `HOLD`.

## Anti-Gaming Controls

1. Package count MUST NOT be used as a throughput success metric.
2. Markdown line count MUST NOT be used as a throughput success metric.
3. Parallel-agent activity alone MUST NOT be used as evidence of capability
   delivery.

## Adoption Requirement

Starting with ARCH21 re-closeout, every remediation package listed in ARCH14
queue governance lanes (`ARCH19+`) MUST include a rubric scorecard row in its
final disposition.
