# Simulation Implementation Work-Package Queue

Status: phase-d-complete
Evidence mode: Static + Ran
Date: `2026-05-24`

## Static
- Queue sequencing is derived from SIMIMPL01 Phase A gap artifacts and the
  mandatory contract-first governance model.
- This queue is implementation-oriented; each code-authoring package must
  execute contract-first internally before production edits.

## Ran
- Inputs read to construct this queue:
  - `artifacts/simimpl01-hillslope-routine-gap-register.md`
  - `artifacts/simimpl01-pipeline-gap-audit.md`
  - `artifacts/simimpl01-watbal-authority-source-comparison.md`
  - `artifacts/simimpl01-watbal-consolidation-and-timestep-architecture.md`
  - `package.md`
- Supporting source checks used in Phase A and consumed here:
  - openWEPP runner/orchestrator paths
  - legacy baseline `watbal`/`watbal_hourly` branch anchors
  - consolidated candidate kernels under `/workdir/wepp-forest/fpm-src`

## Gap Catalog Driving Sequence
| gap_id | statement | source artifact |
|---|---|---|
| `GAP-SIMINV-001` | Full hillslope routine inventory and openWEPP ownership mapping is incomplete. | `simimpl01-hillslope-routine-gap-register.md` |
| `GAP-SIMPIPE-001` | Production runner path does not execute scheduler/kernel lifecycle before output emission. | `simimpl01-pipeline-gap-audit.md` |
| `GAP-SIMMODE-001` | `wepp_ui` requested/effective mode is parsed but not propagated to runtime lane selection. | `simimpl01-pipeline-gap-audit.md` |
| `GAP-SIMOUT-001` | WB13/H.wat publication authority remains projection/synthesis-first, not simulation-owned. | `simimpl01-pipeline-gap-audit.md` |
| `GAP-SIMCONS-001` | Consolidated daily/hourly watbal kernel architecture is not yet triaged and adopted under contract authority. | `simimpl01-watbal-authority-source-comparison.md` |
| `GAP-SIMCOUP-001` | Winter/soil/frsoil/hydout-equivalent coupling closure is not yet established in openWEPP production path. | `simimpl01-hillslope-routine-gap-register.md` |
| `GAP-SIMREPLAY-001` | Tier-A semantic replay remains blocked by keyset mismatch and non-simulation-owned candidate surfaces. | `simimpl01-pipeline-gap-audit.md` |

## Global Sequencing Rules
1. For every code-authoring package, internal order is mandatory:
   contract amendments -> contract-derived tests -> pre-implementation contract gate -> production edits.
2. Baseline physics/comparator authority remains
   `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
3. Consolidated architecture intake from `/workdir/wepp-forest/fpm-src` is
   selective and must be provenance-triaged; no wholesale import.
4. Daily production execution closure precedes hourly branch closure.
5. No silent fallback, no silent clamping, and no untyped domain masking in
   production paths.

## Proposed Queue
| order | wp_id | primary gaps | objective | depends_on | exit signal |
|---|---|---|---|---|---|
| 1 | `20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001` | `GAP-SIMINV-001` | Complete baseline hillslope routine inventory and map each routine to openWEPP owner surface (`runner`, `orchestrator`, `kernel`, `output`, `contract`). | SIMIMPL01 Phase A artifacts | Routine register is complete and evidence-linked. |
| 2 | `20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001` | `GAP-SIMPIPE-001`, `GAP-SIMMODE-001`, `GAP-SIMOUT-001`, `GAP-SIMCONS-001` | Amend canonical contracts (`SC-WATBAL-001`, `SC-SYSTEM-001`, `SC-INFILE-WEPPUI-001` as needed) to encode production execution ownership, mode-propagation invariants, output provenance rules, and consolidation intake guardrails. | `simimpl02` | Contract amendments dispositioned with dual review + dual verification. |
| 3 | `20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001` | `GAP-SIMPIPE-001`, `GAP-SIMMODE-001`, `GAP-SIMOUT-001` | Add contract-derived integration tests that prove runner-to-scheduler execution, mode closure, and simulation-owned WB13 publication requirements; record pre-implementation gate evidence before code changes. | `simimpl03` | New tests exist with explicit expected-fail/expected-pass rationale and gate artifact complete. |
| 4 | `20260524-simimpl05-runner-orchestrator-daily-execution-integration-001` | `GAP-SIMPIPE-001` | Wire production runner path to execute hillslope scheduler/kernel lifecycle (daily lane) with typed error propagation and writeback governance. | `simimpl04` | Runner emits results from executed scheduler/kernel path for daily lane. |
| 5 | `20260524-simimpl06-simulation-owned-wb13-output-publication-001` | `GAP-SIMOUT-001`, `GAP-SIMREPLAY-001` | Replace projection-first WB13/H.wat emission with simulation-owned output assembly and provenance-complete manifest/reporting surfaces. | `simimpl05` | WB13/H.wat outputs are simulation-derived and contract-valid. |
| 6 | `20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001` | `GAP-SIMMODE-001` | Propagate parsed `wepp_ui` requested/effective mode into runtime lane selection and enforce strict typed closure for branch mismatch. | `simimpl05` | `wepp_ui` mode deterministically controls daily/hourly lane selection with typed guard behavior. |
| 7 | `20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001` | `GAP-SIMCONS-001` | Build per-kernel intake map (`wbk*` family) from candidate consolidated sources to baseline + contract authority; classify `adopt`, `defer`, `reject` with rationale. | `simimpl03` | Triage map approved and adoption set is explicitly bounded. |
| 8 | `20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001` | `GAP-SIMMODE-001`, `GAP-SIMCONS-001` | Implement hourly lane foundation with typed timestep policy surface (`daily`, `hourly`, future sub-hourly representable) and adapter-boundary closure. | `simimpl07`, `simimpl08` | Hourly lane executes through typed policy surface; sub-hourly remains scaffolded, not physics-enabled. |
| 9 | `20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001` | `GAP-SIMCOUP-001` | Close legacy coupling gaps for winter/frozen-soil/storage/output-boundary behavior in production execution path, with typed invariants and no silent fallback. | `simimpl09` | Coupling vectors pass and unresolved coupling gaps are explicitly dispositioned. |
| 10 | `20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001` | `GAP-SIMREPLAY-001` | Re-run strict + semantic replay after simulation-owned publication and branch closure; classify residuals and promote/hold by confidence-tier governance. | `simimpl06`, `simimpl10` | Replay evidence updated and blocker set is current, scoped, and justified. |
| 11 | `20260525-simimpl12-disposition-hold-lift-and-next-wave-queue-001` | all remaining | Final SIMIMPL01 closeout: gate results, disposition, hold-lift decision, and next-wave queue for unresolved residuals. | `simimpl11` | SIMIMPL01 disposition is `GO` or explicit `HOLD` with owned residual queue. |

## Parallelization Plan
- Parallel lane A after `simimpl03`:
  - `simimpl04 -> simimpl05 -> simimpl06`
- Parallel lane B after `simimpl03`:
  - `simimpl08` (triage/docs) can run while lane A executes.
- Convergence points:
  - `simimpl09` waits for `simimpl07` and `simimpl08`.
  - `simimpl11` waits for `simimpl06` and `simimpl10`.

## Package Authoring Guardrails for Queue Execution
1. Each queued package must be scaffolded under `docs/work-packages/<wp_id>/`
   with required prompts/artifacts before execution.
2. Kernel-affecting packages must include kernel-profile compliance checklist
   artifacts and remain `HOLD` if profile requirements are incomplete.
3. Every queued package that touches production behavior must preserve explicit
   evidence labeling (`Static:` / `Ran:`) and dual review + dual verification.
