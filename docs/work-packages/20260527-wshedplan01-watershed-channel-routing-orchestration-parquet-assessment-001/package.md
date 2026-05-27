# 20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001

## Status
- state: package-complete
- date: 2026-05-26
- timezone: UTC
- decision: GO

## Objective
Assess the work needed to implement watershed channel routing and watershed
orchestration to the point where openWEPP has a fully scaffolded watershed
runtime path and can emit non-placeholder watershed parquet interchange
outputs.

## Why This Package Exists
The repository already contains important pieces of the watershed surface:
`SC-ROUTE-001`, watershed dispatch scheduling, watershed kernel/writeback
contracts, typed watershed input parsers, and watershed parquet schemas. The
remaining implementation gap is cross-cutting: channel routing physics,
watershed node orchestration, hillslope-pass intake, runtime publication, and
parquet emission must be assessed together before code-authoring packages are
sequenced. This package produces that assessment and a dependency-ordered queue
for implementation.

## Scope
### Included
- Static inventory of the current watershed routing/orchestration/parquet
  surfaces in openWEPP.
- Baseline-authoritative routine map for watershed channel routing and
  orchestration from the pinned WEPP baseline.
- Gap assessment covering:
  - watershed structure/channel/impoundment parser readiness,
  - HBP or hillslope contribution intake readiness,
  - dispatch scheduler and watershed kernel invocation readiness,
  - channel routing hydrology and sediment physics readiness,
  - impoundment boundary and downstream routing readiness,
  - watershed output publication and parquet writer readiness.
- Dependency-ordered follow-on work-package queue to reach executable
  watershed orchestration and non-placeholder parquet outputs.
- Required governance, review, verification, and disposition artifacts.

### Explicitly Out of Scope
- Production Rust code edits.
- Canonical `SC-*` contract amendments beyond gap recommendations.
- Contract-derived test implementation.
- Comparator reruns.
- Claiming watershed-channel parity or parquet closure as complete.

## Deliverables
1. `artifacts/wshedplan01-current-surface-inventory.md`
2. `artifacts/wshedplan01-baseline-routine-map.md`
3. `artifacts/wshedplan01-gap-assessment.md`
4. `artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
5. `artifacts/wshedplan01-contract-implementation-evidence.md`
6. `artifacts/wshedplan01-contract-test-implementation-evidence.md`
7. `artifacts/wshedplan01-preimplementation-contract-gate.md`
8. `artifacts/wshedplan01-implementation-and-test-evidence.md`
9. `artifacts/wshedplan01-kernel-profile-compliance-checklist.md`
10. `artifacts/owned-file-manifest.md`
11. `artifacts/gate-results.md`
12. `artifacts/wshedplan01_disposition.md`
13. `artifacts/worker-handoff.md`
14. `artifacts/review_agent_a.md`
15. `artifacts/review_agent_b.md`
16. `artifacts/verification_agent_a.md`
17. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For queued code-authoring packages:
1. implement canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence,
4. modify production code.

WSHEDPLAN01 performs assessment and queue authoring only. It must identify
which follow-on packages own each contract-first step.

## Autonomous Execution Intent (Required)
This package is execution-ready for assessment and queue scope. The executing
agent must proceed through all phases and update artifacts through disposition
without asking for next steps unless a hard blocker prevents static assessment.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labels. Do not
describe a validation command as run unless it was actually executed.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contract files.
- Baseline-authoritative migration source:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Binary hillslope pass serialization authority is inherited from
  `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` and
  `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`;
  record the inspected `/workdir/wepp-forest` commit SHA in assessment
  artifacts.
- No heuristic/proxy process-physics substitutions are acceptable in queued
  production implementation packages.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/watershed-dispatch-scheduler-contract.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-writeback-contract.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-output/src/contracts.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-output/src/writers.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/ws10_watershed_kernel_contract.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshirs.for`
- `/workdir/wepp-forest_260430_baseline/src/wshrun.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshimp.for`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/chrqin.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and authority freeze
- Confirm the assessment objective and freeze baseline/input authority paths.
- Record the `/workdir/wepp-forest` HEAD SHA used for HBP contract inspection.

### Phase B - Current openWEPP surface inventory
- Inventory parser, scheduler, kernel request/writeback, runtime input, runner,
  output contract, parquet writer, and integration-test surfaces.
- Identify implemented behavior, explicit stubs, and missing handoff seams.

### Phase C - Baseline routine map
- Map baseline watershed orchestration and channel-routing routines to
  openWEPP contract and crate surfaces.
- Distinguish channel hydrology, channel sediment/erosion, impoundment
  coupling, and output/reporting responsibilities.

### Phase D - Gap assessment and queue authoring
- Publish a gap assessment with severity, authority source, likely file targets,
  and validation expectations.
- Publish a dependency-ordered follow-on queue that reaches:
  1. canonical contract closure,
  2. contract-derived tests and pre-implementation gate,
  3. runtime watershed input and hillslope-pass intake,
  4. watershed scheduler/kernel orchestration,
  5. channel routing physics,
  6. output row model and parquet writer activation,
  7. validation and disposition.

### Phase E - Governance closeout
- Complete required evidence, review, verification, gate, handoff, and
  disposition artifacts.

## Exit Criteria
- Current-surface inventory names all relevant openWEPP crates/tests/docs and
  classifies each as implemented, partial, stubbed, or missing.
- Baseline routine map identifies the authoritative WEPP routines needed for
  watershed channel routing and orchestration closure.
- Gap assessment clearly states what remains before openWEPP can produce
  non-placeholder watershed parquet outputs.
- Queue artifact provides dependency-ordered, contract-first follow-on
  packages with concrete write sets and validation gates.
- Required governance artifacts are complete with truthful labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: planning/evidence package only; no runtime behavior change.

## Execution Outcome Summary
- Watershed dispatch scheduling and kernel writeback orchestration are present
  and executable in Rust, with typed guard/diagnostic behavior and integration
  coverage.
- Watershed runtime intake paths (runfile parse, `.str/.chn/.imp/.hbp`,
  runtime-surface seeding, topology validation) are scaffolded in the watershed
  CLI path, including MOFE contributor metadata checks.
- Watershed output contract validation for all required parquet output paths is
  implemented, but writer activation is intentionally blocked by
  `OWSOUT-E-004`; therefore the runtime cannot yet emit non-placeholder
  watershed parquet datasets.
- Python wrapper watershed APIs remain explicitly not implemented
  (`OPEN_RUNNER-E-101/102`), so orchestration closure is incomplete outside the
  Rust CLI surface.
- A dependency-ordered, contract-first follow-on queue is published to close
  routing physics migration, orchestration chronology, writer activation, and
  end-to-end parity/disposition gates.
