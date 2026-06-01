# 20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0233 by closing WB13 subsurface
publication lineage shadow risk: make `latqcc`/`Tile`/`Qd` publication and
coupling checks flux-authoritative (`q`/`Qdd`/`Qd`), then rerun
`unpalatable-rind` (`H1..H39`) and publish residual readjudication deltas.

## Why This Package Exists
HPHYS0233 materially improved `Dp` and storage residuals but regressed
`latqcc`. WB13 publication already enforces flux-preferred lineage for `D`,
but still resolves `q`, `Qdd`, and `Qd` state-first, allowing stale state
symbols to shadow same-name flux symbols from WB19.

## Scope
### Included
- Amend canonical WB13 subsurface publication authority in:
  - `SC-WATBAL-001`
  - `SC-SUBHYD-001`
- Add/adjust contract-derived WB13 stale-state-vs-flux conflict vectors for
  `q`, `Qdd`, `Qd`.
- Implement WB13 flux-preferred symbol resolution for `q`, `Qdd`, and `Qd` in
  `crates/openwepp-runner/src/hillslope/mod.rs`.
- Rerun `H1..H39` semantic comparison and publish monitored-column deltas vs
  HPHYS0233 baseline summary.
- Execute required workspace gates and publish disposition/handoff.

### Explicitly Out of Scope
- WB18/WB19 constitutive equation changes (no new process-physics formulas).
- Frozen-soil, climate, routing, or watershed-kernel behavior changes.
- Comparator schema/policy rewrites unrelated to WB13 subsurface publication.

## Closure Measures (Required)
1. `MEASURE-HP234-001`: canonical contracts explicitly require flux-authoritative
   WB13 subsurface publication/coupling (`q`, `Qdd`, `Qd`) under state/flux
   symbol conflicts.
2. `MEASURE-HP234-002`: contract-derived stale-state-vs-flux conflict vector is
   implemented and passing.
3. `MEASURE-HP234-003`: production WB13 row assembly uses flux-preferred
   lookup for `q`, `Qdd`, and `Qd` with existing typed guard posture.
4. `MEASURE-HP234-004`: `H1..H39` rerun and semantic reports regenerate with
   full coverage (`39/39` execution + comparator coverage).
5. `MEASURE-HP234-005`: monitored residual matrix vs HPHYS0233 is published for
   `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`.
6. `MEASURE-HP234-006`: required gates pass (`fmt`, `clippy`, `test`, `deny`)
   and disposition/handoff are published.

## Deliverables
1. `artifacts/hphys0234-contract-implementation-evidence.md`
2. `artifacts/hphys0234-contract-test-implementation-evidence.md`
3. `artifacts/hphys0234-preimplementation-contract-gate.md`
4. `artifacts/hphys0234-implementation-and-test-evidence.md`
5. `artifacts/hphys0234-residual-authority-gap-matrix.md`
6. `artifacts/hphys0234-h1-transient-lane-diagnostic.md`
7. `artifacts/hphys0234-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0234_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Canonical contract amendments (`SC-*` authority update).
2. Contract-derived tests.
3. Pre-implementation contract gate evidence.
4. Production code implementation.
5. Validation gates + rerun/readjudication evidence + disposition.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/artifacts/hphys0233_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Intake and scaffolding
- Confirm immediate-next-action scope from HPHYS0233 handoff.
- Prepare package prompt/artifact scaffolding.

### Phase B - Contract-first updates
- Amend `SC-WATBAL-001` and `SC-SUBHYD-001` for WB13 subsurface
  flux-authoritative anti-shadow requirements.
- Add contract-derived stale-state-vs-flux conflict vectors.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation
- Apply WB13 flux-preferred symbol resolution for `q`, `Qdd`, `Qd`.
- Keep existing typed guard posture and coupling closure check.

### Phase D - Rerun + adjudication
- Execute `H1..H39` rerun and semantic comparator.
- Publish residual delta matrix and H1 day-1..7 diagnostics.
- Run workspace gates and publish disposition/handoff.

## Exit Criteria
- `MEASURE-HP234-001..006` satisfied and evidenced.
- Stream-level HOLD/GO decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runner/docs/test changes only; no credentials/network.
