# 20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0212 to remediate the HPHYS0211 root-cause owners for the
coupled `Dp`/`latqcc` residual family by:
1. separating one-time WB11/WB18 state initialization from daily execution
   refresh, so mutable layer/aggregate hydrology state is not re-initialized
   each simulated day,
2. sourcing WB19 lateral/drain control symbols from authoritative runtime-input
   projection surfaces instead of hard-coded runner constants, and
3. restoring WB13 subsurface coupling visibility so `latqcc`/`Tile` publication
   is deterministic and auditable against exported `Qd` lineage.

## Why This Package Exists
HPHYS0211 identified concrete defect ownership (`HP211-RC-001..003`) blocking
hold lift. Daily reseeding currently overwrites mutable WB18/WB11 state and
runner hard-coded WB19 controls dominate lateral/drain behavior, producing
fully saturated `Dp` and `latqcc` residuals. This package lands the first
remediation wave before aggregate follow-on in HPHYS0213.

## Scope
### Included
- Contract-authority intake for WB11/WB19 lifecycle and WB13 subsurface
  coupling obligations.
- Contract-derived tests for:
  - no daily WB11/WB18 mutable-state reseed regression,
  - WB19 control-source projection and guard behavior,
  - WB13 `latqcc`/`Tile` coupling visibility with `Qd` relation checks.
- Production code changes in hillslope runner/runtime-input surfaces needed to
  close RC-001/RC-002 and expose coupling diagnostics for RC-003 adjudication.
- Required workspace gates and targeted tests with evidence logs.
- Residual rerun summary for `Dp`/`latqcc` and updated disposition/handoff.

### Explicitly Out of Scope
- Full closure of `Total-Soil` / `SoilWaterTotal` residual families
  (`HPHYS0213` owner).
- Watershed/channel kernel changes.
- Non-hydrology refactors unrelated to WB11/WB19/WB13 coupling lineage.

## Closure Measures (Required)
1. `MEASURE-HP212-001`: contract-first sequence evidence is complete
   (authority intake -> contract-derived tests -> pre-implementation gate ->
   production edits).
2. `MEASURE-HP212-002`: production no longer re-initializes mutable WB18/WB11
   storage state each simulated day.
3. `MEASURE-HP212-003`: WB19 control symbols used by hydrology kernel are
   runtime-input sourced (no hard-coded runner constants in daily seed path).
4. `MEASURE-HP212-004`: WB13 subsurface publication includes deterministic
   `latqcc`/`Tile` visibility with explicit `Qd` coupling guard evidence.
5. `MEASURE-HP212-005`: required gates pass and residual deltas are published
   with truthful `HOLD`/`GO` decisioning.

## Deliverables
1. `artifacts/hphys0212-contract-implementation-evidence.md`
2. `artifacts/hphys0212-contract-test-implementation-evidence.md`
3. `artifacts/hphys0212-preimplementation-contract-gate.md`
4. `artifacts/hphys0212-implementation-and-test-evidence.md`
5. `artifacts/hphys0212-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0212-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0212_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Ingest canonical contract authority and HPHYS0211 residual lineage.
2. Implement/adjust contract-derived tests for HPHYS0212 closure measures.
3. Record pre-implementation contract gate evidence.
4. Apply production code edits and run validation/rerun evidence.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting user
direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator baseline anchor:
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Comparator outcomes remain diagnostic evidence; process-authoritative closure
  is the promotability gate.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211-residual-gap-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/**`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Phase Plan
### Phase A - Authorization and scope freeze
- Confirm HPHYS0212 authorization from HPHYS0211 handoff.
- Freeze scope to RC-001/RC-002/RC-003 remediation lane only.

### Phase B - Contract + test intake
- Capture authoritative WB11/WB19/WB13 coupling obligations.
- Land/adjust contract-derived tests before production edits.

### Phase C - Production remediation
- Implement lifecycle carry-state correction.
- Implement runtime-input sourced WB19 controls.
- Implement WB13 coupling visibility/guard checks.

### Phase D - Validation and residual rerun
- Run required workspace gates and targeted tests.
- Re-run residual diagnostics for `Dp`/`latqcc` lanes and summarize deltas.

### Phase E - Disposition and handoff
- Publish truthful `HOLD`/`GO` decision and immediate-next queue
  (`HPHYS0213`/`HPHYS0214` as needed).

## Exit Criteria
- `MEASURE-HP212-001..005` are evidenced.
- Owned-file manifest is complete.
- Disposition and worker handoff identify any residual blockers explicitly.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: hydrology runtime/math + documentation updates only; no auth,
  network, or privilege-surface changes.
