# Watershed Channel Routing, Orchestration, and Parquet Work-Package Queue

Status: complete

Evidence mode: static+ran

Date: 2026-05-26

## Static
- Queue objective: close watershed channel routing + orchestration + parquet
  publication gaps so watershed execution can produce non-placeholder parquet
  outputs under contract-first governance.
- Baseline parity authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Contract-first sequence is mandatory for all code-authoring packages:
  1. canonical contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.

## Proposed queue
| order | wp_id | objective | depends_on | exit signal |
|---|---|---|---|---|
| 1 | `20260527-wshedimpl01-watershed-contract-authority-and-routine-map-001` | Amend canonical `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`, and watershed dispatch/writeback contracts with explicit baseline routine-chain authority (`wshdrv/wshcqi/wshirs/wshrun/wshpek/wshchr/chrqin/wshimp/chnero/chnrt`) and symbol alias continuity tables. | `20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001` | Canonical `SC-*` files explicitly encode watershed routine authority and runtime symbol families. |
| 2 | `20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001` | Add contract-derived vectors for watershed chronology, `ipeak` branch-family behavior, channel/impoundment boundary guards, writer-output expectations, and record pre-implementation gate evidence before production edits. | `20260527-wshedimpl01-watershed-contract-authority-and-routine-map-001` | Contract-derived tests exist with expected pre-migration failures and pre-implementation gate artifact is complete. |
| 3 | `20260527-wshedimpl03-watershed-runtime-intake-and-state-topology-closure-001` | Close runtime intake/state topology seams for watershed node execution, including contributor payload surfaces, routing-state arrays/symbol families, and runner ingress consistency with typed errors only. | `20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001` | Watershed runtime surface contains all contract-required ingress symbols; no silent defaults/clamps. |
| 4 | `20260527-wshedimpl04-channel-routing-hydrology-kernel-migration-001` | Migrate baseline channel-routing hydrology families (`wshcqi/wshirs/wshrun/wshpek/wshchr/chrqin`) into watershed kernel/runtime architecture with contract-derived test activation. | `20260527-wshedimpl03-watershed-runtime-intake-and-state-topology-closure-001` | Channel routing branch families execute with passing contract vectors and typed domain guards. |
| 5 | `20260527-wshedimpl05-impoundment-and-channel-erosion-coupling-migration-001` | Migrate impoundment and channel-erosion coupling chains (`wshimp`, `chnero -> chnrt`) and enforce chronology with routed channel outputs and sediment publication surfaces. | `20260527-wshedimpl04-channel-routing-hydrology-kernel-migration-001` | Impoundment + channel erosion kernels are baseline-authoritative for declared scope; sequencing tests pass. |
| 6 | `20260527-wshedimpl06-watershed-output-row-model-and-parquet-writer-activation-001` | Implement watershed output row-model builders and activate parquet emission for all required datasets; retire `OWSOUT-E-004` placeholder guard. | `20260527-wshedimpl05-impoundment-and-channel-erosion-coupling-migration-001` | Watershed CLI emits non-placeholder parquet files for all required outputs with schema metadata checks passing. |
| 7 | `20260527-wshedimpl07-runner-python-watershed-orchestration-and-e2e-contract-closure-001` | Implement Python wrapper watershed runfile/execution surfaces and integrate end-to-end watershed orchestration with CLI parity behavior and typed error mapping. | `20260527-wshedimpl06-watershed-output-row-model-and-parquet-writer-activation-001` | Python `make_watershed_*`/`run_watershed` execute supported watershed fixtures and produce required parquet outputs. |
| 8 | `20260527-wshedimpl08-watershed-parity-rerun-and-disposition-001` | Execute watershed-focused parity reruns (channel routing + impoundment + publication) and publish explicit GO/HOLD disposition with residual ownership. | `20260527-wshedimpl07-runner-python-watershed-orchestration-and-e2e-contract-closure-001` | Final package disposition provides admissible GO/HOLD decision for watershed closure scope. |

## Sequencing constraints
1. No production watershed kernel changes before WSHEDIMPL01 + WSHEDIMPL02
   complete contract/test/gate prerequisites.
2. Output writer activation (WSHEDIMPL06) must wait for routing and
   impoundment migration packages, otherwise parquet outputs risk encoding
   placeholder physics.
3. Python wrapper closure (WSHEDIMPL07) must consume the same typed CLI
   contract/error surfaces; no duplicate orchestration behavior.
4. WSHEDIMPL08 is the only package authorized to issue final GO/HOLD watershed
   closure disposition.

## Ran
- `nl -ba docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/wshedplan01-gap-assessment.md | sed -n '1,260p'`
