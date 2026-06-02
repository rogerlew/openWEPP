# HPHYS0250 WB17 Ep Lineage Closure

Status: HOLD

This ExecPlan-style work package is a living autonomous execution document.
It follows `/workdir/openWEPP/docs/codex_exec_plans.md` and remains
self-contained enough for a new agent to inspect the completed work,
disposition, and continuation metrics without user intervention.

## Objective

Diagnose, correct, and validate the remaining WB17 `Ep` lineage defect after
HPHYS0249 by proving that daily plant growth/root-depth state activates
post-WB19 `PlantRootUptake`, that final `Ep` is the WB13 publication surface,
and that full `H1..H39` hillslope semantic metrics are available for the next
continuation package.

## Rationale

HPHYS0249 corrected WB17 `Es` layer-storage mutation and post-WB19 `swu`
ordering, but full-suite `Ep` remained unchanged (`0/39`, fail-count sum
`56834`, mean abs mean `1.739422`, max abs `7.780000`, worst `H1`). The next
highest-value closure target is the lineage from management/growth runtime
state (`lai`, `rtd`, `pltol`) through `PlantRootUptake` and WB13 `Ep`
publication.

## Included Scope

- Amend canonical `SC-*` contracts before production code changes.
- Add contract-derived tests proving scheduler/growth activation and final
  `Ep` publication lineage.
- Diagnose whether `PlantRootUptake` executes with nonzero `rtd` and whether
  `Ep` reaches the WB13 semantic comparator surface.
- Correct production code only for confirmed contract violations in the WB17
  `Ep` lineage.
- Run targeted Rust tests, workspace gates where feasible, dual agent review
  with fixes, dual verification, and the full `H1..H39` hillslope semantic
  suite.

## Excluded Scope

- Snow/runoff timing closure except as required to keep `Ep` diagnostics
  interpretable.
- Dp/Pe, lateral subsurface, watershed routing, or aggregate storage closure
  beyond incidental effects of corrected `Ep` lineage.
- Heuristic or surrogate process-physics formulas.
- Clean-room reimplementation. Baseline-authoritative WEPP provenance is the
  governing migration source.

## Deliverables

1. Canonical `SC-*` contract amendments for final `Ep` lineage authority.
2. Contract-derived tests that fail before production correction and pass after.
3. Diagnosis artifacts for `Ep` surface ingestion and `rtd`/`pltol` runtime
   projection.
4. Production fix for the confirmed lineage defect.
5. Dual review artifacts, fixes, and dual verification artifacts.
6. Targeted and workspace gate logs.
7. Full `H1..H39` semantic metrics with continuation residual ledger.
8. Final disposition and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/hphys0249_disposition.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/evap.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/06_growth_state.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/**`

## Phase Plan

1. Read required context and HPHYS0249 residual artifacts.
2. Amend canonical contracts for final `Ep` lineage and runtime PL activation.
3. Add contract-derived tests for scheduler/growth activation and WB13 `Ep`
   consumption.
4. Run pre-implementation contract gate and record expected failures.
5. Diagnose `Ep` runtime surface lineage with HPHYS0249 artifacts and targeted
   tests.
6. Implement the minimal production correction.
7. Run targeted Rust tests and full relevant gates.
8. Dispatch dual agent review, apply accepted fixes, and rerun affected gates.
9. Run the full `H1..H39` hillslope semantic suite.
10. Record metrics, residual ledger, disposition, and worker handoff.

## Contract-First Sequence

The package must execute in this order:

1. contracts,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production code edits.

Production code edits before the pre-implementation contract gate are not
authorized.

## Physics Authority

- Canonical physics authority is `SC-*` contract text, not package notes.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `Ep` closure must preserve baseline `evap`/`swu` semantics: potential
  transpiration partition (`Etp`), root-zone distribution (`UPi`), actual layer
  uptake (`Ui`), `Ep = ΣUi`, and stress ratio (`Ws = Ep / Etp` when `Etp > 0`).
- Do not invent physics. Every equation, guard, and invariant must trace to
  canonical contract text plus provenance citations.

## Exit Criteria

- Contract amendments and tests are present before production code edits.
- Pre-implementation contract gate records failing tests for the confirmed
  lineage defect.
- Targeted post-implementation tests pass.
- Full `H1..H39` hillslope runtime and semantic suite completes with metrics.
- Dual review findings are dispositioned and accepted fixes are applied.
- Evidence artifacts use truthfulness labels (`Static:` vs `Ran:`).
- Disposition is `GO` only if no known invariant, closure, or contract
  violations remain in scope; otherwise disposition remains `HOLD` with a
  focused continuation handoff.

## Security-Impact Gate

No external systems, credentials, network writes, shell interpolation, or
production deployment are in scope. Work is limited to local repository
flat-file reads/edits and local command execution.
