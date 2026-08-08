# Required Reading Map

Status: `queued`

Evidence mode: `Static scaffold`

## Core

| Path | Rationale | Local bytes |
| --- | --- | ---: |
| `AGENTS.md` | Repository invariants and kernel authority | 11927 |
| `docs/codex_exec_plans.md` | ExecPlan lifecycle | 20921 |
| `docs/work-packages/AGENTS.md` | Package gates and artifact rules | 26013 |
| `docs/work-packages/README.md` | Current package catalog and lifecycle | 385930 |
| package-local `package.md` | Authorized objective, write set, and gates | 23170 |
| predecessor `artifacts/worker-handoff.md` | Binding successor boundary | 1851 |
| `docs/specifications/science-contracts/AGENTS.md` | Contract governance | 5599 |
| `SC-VEGETATION-001.md` | Canonical vegetation boundary and open gaps | 34832 |
| `docs/standards/testing-and-gate-strategy.md` | Validation lifecycle | 22200 |
| `crates/AGENTS.md` | Rust crate rules | 5165 |
| `tests/AGENTS.md` | Test authority and conventions | 4723 |

## Conditional

| Path or family | Trigger | Local bytes |
| --- | --- | ---: |
| `docs/specifications/science-contract-authoring-procedure.md` | Contract edits; applicable before edits | 13715 |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Constitutive kernel authority; applicable before edits | 5792 |
| `docs/specifications/science-contracts/index.md` | Contract lifecycle edit; applicable before edits | 10500 |
| `docs/specifications/science-contract-spec.md` | Contract schema/readiness; applicable before edits | 13749 |
| `docs/specifications/unit-governance.md` | Unit-bearing constitutive work; applicable before edits | 12896 |
| `docs/specifications/correctness-authority-model.md` | A0/A1/A3 selection; applicable before result-bearing work | 11159 |
| `docs/specifications/external-authority/README.md` | A3 suite admission; applicable before suite edits | 3620 |
| `docs/specifications/external-authority/suite-schema.md` | A3 suite schema; applicable before suite edits | 5692 |
| `docs/specifications/external-authority/promotion-protocol.md` | Required-lane posture; applicable before suite edits | 2147 |
| `docs/decisions/0011-architecture-first-top-down-science-contracts.md` | Architecture-first decision | 2473 |
| `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` | Comparator/source posture | 8943 |
| `docs/decisions/0042-science-implementation-and-calibration-readiness.md` | Readiness claim limits | 4890 |

## On Demand

| Path or family | Trigger |
| --- | --- |
| `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, `SC-WATBAL-001` | Exact ownership/lineage amendment |
| External-authority templates, registry, obligations, and fixture guide | A3 suite authoring |
| Backlog and predecessor artifacts beyond the handoff | Named boundary/provenance question |
| Selected peer-reviewed literature | Constitutive authority admission |
| Adjacent crate/test sources | Exact implementation seam |

## Budget

Core subtotal: `542331` bytes.

Applicable Conditional subtotal: `95576` bytes.

Local required-reading total: `637907` bytes.

Disposition: `WARN` (`>400000` and `<=800000`). The mandatory
`docs/work-packages/README.md` contributes `385930` bytes; large adjacent
contracts and implementation sources remain trigger-loaded On Demand.

On-demand files and external literature are excluded from the local mandatory
pre-read total until their trigger applies. Recompute and record the actual set
before implementation edits.
