# 20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0227 by restoring authoritative
WB14 `ksatadj` success-lane contract coverage (`solwpv = 9001/9002/9003`)
without weakening typed guard behavior.

## Why This Package Exists
HPHYS0227 stabilized WB19 prerequisites but downgraded WB14 disturbed-soil
`ksatadj` regime tests to forced domain-failure signatures. The HPHYS stream
requires successful-lane constitutive coverage for the active `ksatadj`
regimes before further integrated residual-family closure work.

## Scope
### Included
- Contract-first review/amendment for WB14 disturbed-soil `ksatadj` assertions
  in canonical runoff/watbal contracts if wording is incomplete.
- Restoration of WB14 integration tests to assert successful-lane regime
  behavior for `solwpv=9001/9002/9003` (including `9003` `lkeff` floor).
- Seed-surface normalization for WB19 indexed FC/WP prerequisites used by WB14
  `ksatadj` vectors.
- Required workspace validation gates.
- Package disposition + handoff to next HPHYS residual-family package.

### Explicitly Out of Scope
- New WB19 production behavior changes.
- External-authority suite tiering/promotion changes.
- `unpalatable-rind` cohort rerun/readjudication.

## Closure Measures (Required)
1. `MEASURE-HP228-001`: canonical WB14 `ksatadj` authority coverage is present
   and explicit in `SC-RUNOFFPART-001`/`SC-WATBAL-001` (or amended if needed).
2. `MEASURE-HP228-002`: WB14 `ksatadj` integration vectors for
   `solwpv=9001/9002/9003` are success-lane assertions (not forced-failure
   signatures) and cover effective-conductivity equivalence behavior.
3. `MEASURE-HP228-003`: WB14 seeded prerequisites satisfy WB19 indexed FC/WP
   guards without surrogate/fallback behavior.
4. `MEASURE-HP228-004`: targeted WB14 contract tests pass.
5. `MEASURE-HP228-005`: workspace gates pass (`fmt`, `clippy`, `test`, `deny`).
6. `MEASURE-HP228-006`: package artifacts/disposition/handoff are updated with
   truthfulness labels and next-action queueing.

## Deliverables
1. `artifacts/hphys0228-residual-authority-gap-matrix.md`
2. `artifacts/hphys0228-contract-implementation-evidence.md`
3. `artifacts/hphys0228-contract-test-implementation-evidence.md`
4. `artifacts/hphys0228-preimplementation-contract-gate.md`
5. `artifacts/hphys0228-implementation-and-test-evidence.md`
6. `artifacts/hphys0228-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0228_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Review/amend canonical `SC-*` authority coverage for WB14 `ksatadj`.
2. Implement contract-derived WB14 test restoration.
3. Capture pre-implementation contract-gate evidence.
4. Apply implementation edits.
5. Run tests/gates and publish disposition.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/**`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`

## Phase Plan
### Phase A - Scope freeze and authority check
- Freeze scope to HPHYS0227 handoff item:
  restore WB14 `ksatadj` successful-lane vectors.
- Verify canonical contracts already carry needed equation/branch authority.

### Phase B - Contract-derived test restoration
- Restore and adapt WB14 `ksatadj` success-lane assertions for
  `9001/9002/9003`, including `lkeff` floor behavior.
- Normalize WB14 test seed symbols so WB19 prerequisites are valid.

### Phase C - Validation and disposition
- Run targeted WB14 tests and workspace gates.
- Publish artifacts, disposition, and next-action handoff.

## Exit Criteria
- `MEASURE-HP228-001..006` satisfied and evidenced.
- HPHYS stream remains `HOLD` for integrated residual-family closure beyond
  this package boundary.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local docs/tests only; no credentials/network.
