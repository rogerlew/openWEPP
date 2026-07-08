# Required Reading Map

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Static.

## Core Reading

| Path | Purpose |
|---|---|
| `AGENTS.md` | Repository governance and kernel validation posture. |
| `docs/work-packages/AGENTS.md` | Work-package execution, artifact, review, verification, and gate rules. |
| `docs/work-packages/README.md` | Package catalog and active/held state. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first rules for kernel-affecting work. |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Active router mesh, closure, and trace authority. |
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md` | Parent package scope and hold boundary. |
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/worker-handoff.md` | Follow-on objective and boundary. |
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.md` | Prior ladder result table. |
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.json` | Machine-readable prior ladder evidence. |
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-fidelity-adjudication.md` | Parent hold decision and follow-on statement. |

## Code Surfaces

| Path | Purpose |
|---|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | Active mesh policy, day closure hard-fails, and trace row fields. |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | OFE cascade handoff and width-aware mass books. |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | TVD-MacCormack stepper, CFL, positivity clamp, and mass ledger. |
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | Active selector and diagnostic target-`dx` env wiring. |

## Prior Evidence Inputs

| Path | Purpose |
|---|---|
| `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-runs/wa_cascades_forest_h1/` | Prior WA rung logs, traces, and outputs. |
| `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json` | Selected-cohort run-dir authority. |
