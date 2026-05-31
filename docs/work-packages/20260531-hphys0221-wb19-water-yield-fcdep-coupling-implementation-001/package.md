# 20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Implement baseline-authoritative WB19 coupled water-yield/saturated-depth
behavior in openWEPP (`solwpv` branch semantics, `avpora/avfca/avcoca`,
`watyld`, `fcdep`, `unsdep`) under contract-first sequencing, then run closure
gates and 39-hillslope parity adjudication.

## Why This Package Exists
HPHYS0220 identified missing WB19 process-physics lineage. This package is the
implementation closure attempt for that missing lineage.

## Scope
### Included
- Contract authority updates in canonical `SC-*` files for WB19 coupling.
- Contract-derived integration tests for WB19 branch/coupling behavior.
- Production WB19 lateral-transfer implementation updates.
- Runner/fixture updates required by newly mandatory WB19 symbols.
- 39-hillslope rerun (`unpalatable-rind`) and semantic comparison readjudication.
- Full workspace validation gates.

### Explicitly Out of Scope
- Watershed/channel-process remediations unrelated to WB19 hillslope coupling.
- Comparator-threshold policy changes.

## Closure Measures (Required)
1. `MEASURE-HP221-001`: canonical contracts publish WB19 coupling authority and
   guard semantics.
2. `MEASURE-HP221-002`: WB19 coupling/branch tests enforce runtime behavior and
   typed hard-fail paths.
3. `MEASURE-HP221-003`: production WB19 implementation publishes
   `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld` with `solwpv`-specific behavior.
4. `MEASURE-HP221-004`: validation gates pass (`fmt`, `clippy`, `test`,
   `deny`) and rerun evidence is captured with disposition.

## Deliverables
1. `artifacts/hphys0221-contract-implementation-evidence.md`
2. `artifacts/hphys0221-contract-test-implementation-evidence.md`
3. `artifacts/hphys0221-preimplementation-contract-gate.md`
4. `artifacts/hphys0221-implementation-and-test-evidence.md`
5. `artifacts/hphys0221-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0221-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0221_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequencing (Required)
1. Amend canonical `SC-*` contracts.
2. Add contract-derived WB19 tests.
3. Record contract gate evidence.
4. Modify production code and rerun closure gates.

## Autonomous Execution Intent (Required)
Execute end-to-end without additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Artifacts must label evidence as `Static:` or `Ran:`.

## Provenance and Authority Posture
- Canonical authority: `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline migration authority:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Binary pass format authority remains `/workdir/wepp-forest` contract docs.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/artifacts/worker-handoff.md`

## Intended Write Set
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/*.rs` (WB19-required fixture/test updates plus HPHYS0221 test)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/**`

## Phase Plan
### Phase A - Contract authority
- Amend SC contracts for WB19 `solwpv` branch semantics and coupled state outputs.

### Phase B - Contract-derived tests
- Add/adjust WB19 integration tests and required fixture symbols.

### Phase C - Production implementation
- Implement WB19 coupling in lateral transfer with typed guard behavior.

### Phase D - Validation and disposition
- Run workspace gates and 39-hillslope rerun; publish HOLD/GO disposition.

## Exit Criteria
- `MEASURE-HP221-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: deterministic kernel logic + tests + documentation only.
