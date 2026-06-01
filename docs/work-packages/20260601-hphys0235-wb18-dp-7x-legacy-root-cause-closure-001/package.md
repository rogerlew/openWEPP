# 20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0234 by identifying why `Dp`
remains approximately `7x` legacy on early `H1` days under `ui_run=1`,
producing a contract-authoritative root-cause ledger and implementation-ready
handoff.

## Why This Package Exists
HPHYS0234 closed WB13 anti-shadow lineage but produced zero residual movement
for `Dp`. The persistent mismatch (`~1.65-1.82 mm/day` candidate vs
`~0.24 mm/day` baseline on early `H1` days) indicates unresolved WB18 hourly
lane process-physics migration, not publication drift.

## Scope
### Included
- Amend canonical WB18 hourly-lane authority in:
  - `SC-PERC-001`
  - `SC-WATBAL-001`
- Produce reproducible `H1` lane A/B probe evidence:
  - hourly lane (`wepp_ui.txt` present; `ui_run=1`),
  - daily lane (`wepp_ui.txt` absent; `ui_run=0`),
  - baseline comparator.
- Publish numeric attribution ledger for `Dp` and `Total-Soil` early-transient
  mismatch and identify the dominant missing legacy behavior.
- Publish implementation-ready follow-on handoff and stream disposition.

### Explicitly Out of Scope
- Production kernel changes in `crates/openwepp-*`.
- New heuristic or surrogate process-physics substitutions.
- Watershed routing, climate parser, or non-WB18 remediation work.

## Closure Measures (Required)
1. `MEASURE-HP235-001`: canonical contracts explicitly encode hourly WB18
   authority as `watbal_hourly` + `purk` iterative coupling semantics, not
   divisor-only approximation.
2. `MEASURE-HP235-002`: reproducible lane A/B probe on `H1` demonstrates:
   - hourly lane `Dp` day-1..7 ratio near `~7x` baseline,
   - daily lane `Dp` day-1..7 ratio near `~1x` baseline.
3. `MEASURE-HP235-003`: root-cause ledger maps mismatch to missing hourly
   iterative execution semantics in openWEPP WB18/WB11 runtime flow.
4. `MEASURE-HP235-004`: disposition and worker handoff publish concrete
   implementation next actions under contract-first sequencing.

## Deliverables
1. `artifacts/hphys0235-contract-implementation-evidence.md`
2. `artifacts/hphys0235-contract-test-implementation-evidence.md`
3. `artifacts/hphys0235-preimplementation-contract-gate.md`
4. `artifacts/hphys0235-implementation-and-test-evidence.md`
5. `artifacts/hphys0235-residual-authority-gap-matrix.md`
6. `artifacts/hphys0235-h1-transient-lane-diagnostic.md`
7. `artifacts/hphys0235-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0235_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Canonical contract amendments (`SC-*` authority update).
2. Contract-derived tests.
3. Pre-implementation contract gate evidence.
4. Diagnostic execution evidence.
5. Disposition and follow-on handoff.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/artifacts/hphys0234_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`

## Phase Plan
### Phase A - Intake and scaffolding
- Confirm immediate-next-action scope from HPHYS0234 handoff.
- Prepare package prompt/artifact scaffolding.

### Phase B - Contract-first updates
- Amend `SC-PERC-001` and `SC-WATBAL-001` for WB18 hourly-lane iterative
  authority (legacy `watbal_hourly` + `purk` semantics).
- Add contract-derived test obligation entries for hourly iterative vectors.
- Record pre-implementation contract gate evidence.

### Phase C - Diagnostic execution
- Run hourly-vs-daily lane A/B probes on `H1`.
- Compare against baseline `H1` day-1..7 and publish residual matrix.
- Attribute the dominant mismatch to concrete code-path differences.

### Phase D - Disposition + handoff
- Publish hold disposition and follow-on implementation queue.
- Capture owned-file manifest and kernel profile compliance checklist.

## Exit Criteria
- `MEASURE-HP235-001..004` satisfied and evidenced.
- Stream-level HOLD/GO decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local docs/diagnostic execution only; no credentials/network.
