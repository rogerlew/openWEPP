# HPHYS0288 Winter Rain-On-Snow Melt Partition Magnitude Closure

Status: executed-hold

This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current as work proceeds. This
package follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, openWEPP should route rain-on-snow liquid through the same
baseline-authoritative winter/runoff seam used by WEPP: `snowd.for` first
retains rain in a low-density snowpack, `winter.for` then adds residual
rain-on-snow liquid into `hrmlt`/`wmelt`, and WB12/WB18 consume that routed melt
forcing before publishing runoff/storage metrics. The observable outcome is a
contract-derived test that fails before production edits and passes after, plus
H1/H7/H39 and full H1..H39 metrics showing whether spring `RM`/`Q`/`Snow-Water`
and `Total-Soil` residuals moved.

## Progress

- [x] (2026-06-04) Scaffolded HPHYS0288 package structure, prompt, and queued artifacts.
- [x] Read required contracts, prior HPHYS0287 handoff, and pinned baseline winter/runoff files.
- [x] Amend canonical `SC-*` contracts for residual rain-on-snow routed-melt authority.
- [x] Add contract-derived tests before production code edits.
- [x] Record pre-implementation contract gate evidence.
- [x] Implement the minimal snowmelt/rain partition correction.
- [x] Run focused tests, adjacent tests, Rust gates, and full H1..H39 metrics.
- [x] Complete dual review, review disposition, dual verification, final disposition, and handoff.

## Surprises & Discoveries

- Observation: HPHYS0287 closed fail-open projected snow-state guarding but made no valid-run magnitude progress; its handoff explicitly sends the next package to `winter.for`/`runoff.for` magnitude.
  Evidence: `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/artifacts/worker-handoff.md`.
- Observation: The HPHYS0288 correction improved storage/ET/lateral metrics but did not move `RM`, `Q`, or `Snow-Water`.
  Evidence: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/hillslope_semantic_summary.md`.

## Decision Log

- Decision: Scope HPHYS0288 to residual rain-on-snow release into routed melt forcing, not another adjacent guard-hardening pass.
  Rationale: Prior full-suite metrics left `Q`, `RM`, and `Snow-Water` unchanged; the lowest-regret continuation is the baseline seam where `snowd.for` mutates `hrrain` and `winter.for` adds remaining `hrrain` into `hrmlt`/`wmelt`.
  Date/Author: 2026-06-04 / Codex.

## Outcomes & Retrospective

Executed-hold. HPHYS0288 ported the baseline residual rain-on-snow release into routed melt forcing and passed contract/workspace gates. Full H1..H39 semantic parity remains open at 0/39; `Ep`, `Total-Soil`, `SoilWaterTotal`, and `latqcc` improved versus HPHYS0287, `Dp` slightly worsened, and `Q`/`RM`/`Snow-Water` were effectively unchanged. Continue with WB13/RM publication and winter runoff/snowpack forcing lineage rather than ET compensation.

## Objective

Diagnose, correct, and validate the baseline-authoritative winter rain-on-snow
retained-liquid release and melt/rain partition magnitude defect that remains
after HPHYS0287, anchored on H1/H7/H39 and then measured across the full H1..H39
hillslope suite.

## Rationale

HPHYS0284 through HPHYS0287 improved snow state and storage guards but did not
move the valid-run `Q`, `RM`, or `Snow-Water` residuals. Static baseline
inspection shows a specific seam not yet encoded in openWEPP: residual rain that
is not retained by `snowd.for` holding capacity is added to `hrmlt`/`wmelt` in
`winter.for`, and downstream `watbal_hourly.for`/`grna.for` consume `wmelt` as
infiltration/runoff event forcing. Current openWEPP retains rain in snowpack but
leaves residual rain-on-snow on the direct-rain path, so the runoff/melt
partition lineage can be semantically wrong even if total liquid mass is similar.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and `SC-WATBAL-001`
  for baseline residual rain-on-snow routed-melt authority.
- Add contract-derived tests before production code edits.
- Diagnose H1/H7/H39 rain-on-snow days for retained rain, residual rain,
  routed melt, `RM`, `Q`, `Snow-Water`, WB12 infiltration, and WB18 ingress.
- Implement one minimal production correction in hydrology snow/runoff coupling
  if the contract/test gate proves the defect.
- Preserve HPHYS0287 fail-closed projected snow-state validation and all typed
  domain guards.
- Run focused tests, adjacent snow/runoff/hydrology tests, Rust gates, and full
  H1..H39 runtime/semantic metrics.
- Complete dual independent reviews, finding disposition, dual verification,
  final disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep` compensation, ET tuning, or plant-growth changes.
- Heuristic runoff/infiltration multipliers.
- Reproducing the rejected pinned-baseline negative-melt sign bug.
- Loosening fail-closed SWE/depth/density/liquid-domain guards.
- Broad frost migration or agricultural/non-agricultural land-use policy changes.
- Multi-OFE routing beyond noting any carry-array implications for follow-up.

## Deliverables

- Canonical contract amendments with pinned baseline provenance citations.
- Contract-derived regression test proving residual rain-on-snow is routed with
  snowmelt forcing rather than left exclusively as direct rainfall.
- Targeted H1/H7/H39 trace evidence over material spring rain-on-snow days.
- Production hydrology correction only after contract/test/gate evidence is recorded.
- Full H1..H39 runtime and semantic metrics with comparison to HPHYS0287.
- Dual review artifacts, review-disposition artifact, dual verification
  artifacts, final disposition, and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/wshirs.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/disag.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/grna.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-runner/src/hillslope/mod.rs` only if WB13 trace/publication
  evidence proves runner-owned `RM` mapping must change.
- `tests/integration/hphys0288_winter_rain_snowmelt_partition_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required contracts, prior package evidence, and pinned baseline winter/runoff files.
3. Localize H1/H7/H39 rain-on-snow partition evidence.
4. Amend canonical contracts for residual rain-on-snow routed-melt authority.
5. Add contract-derived failing tests before production code edits.
6. Record pre-implementation contract gate evidence.
7. Implement minimal hydrology snow/runoff coupling correction.
8. Run focused tests, adjacent tests, Rust gates, and H1..H39 metrics.
9. Complete dual reviews, review disposition, dual verification, final disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel edits are prohibited before steps 1-3 are complete and
recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize the residual rain-on-snow
  `snowd.for -> winter.for -> wmelt -> fin/smrate` lineage.
- A contract-derived test fails before and passes after production correction,
  or the package remains `HOLD` with evidence that no production defect was proven.
- HPHYS0287 fail-closed runtime snow-state validation remains intact.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned. Implementation did not change subprocess
orchestration or sidecar discovery.
