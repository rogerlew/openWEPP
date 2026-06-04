# HPHYS0287 Snow Liquid Retention Runoff Infiltration Partition Closure

Status: executed-hold

## Objective

Diagnose, correct, and validate the snow liquid retention, runoff, and
infiltration partition that still drives H1/H7/H39 spring storage collapse after
HPHYS0286 corrected post-ET layer redistribution.

## Executed Scope Note

HPHYS0287 did not deliver valid-run snow liquid retention, melt-release, or
runoff-magnitude parity progress. The executed production correction is a
guard-hardening package: material invalid or partial projected runtime snow
state now fails closed before inactive snow fallback and liquid partition. The
valid-run magnitude objective remains open for the next package.

## Rationale

HPHYS0286 improved `Total-Soil`, `SoilWaterTotal`, `latqcc`, `Dp`, and `Ep`,
but did not move `Q`, `RM`, or `Snow-Water`. H1/H7/H39 top divergent rows still
cluster around spring snowmelt/runoff days. The next lowest-regret package is
therefore upstream of WB17 and WB18 layer retention: snowpack liquid storage,
rain-on-snow retention, melt release, runoff generation, and same-day water
delivery into WB12/WB18.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001`, `SC-WATBAL-001`, `SC-PERC-001`, and
  `SC-RUNOFF-*` contracts only where the diagnosis proves new authority is
  required.
- Preserve fail-closed SWE and liquid-storage guards; do not replace domain
  violations with canonicalize-and-proceed behavior.
- Add contract-derived tests before production code edits.
- Diagnose H1/H7/H39 spring 2014 and spring 2016 snow column, rain/melt
  release, runoff, infiltration, and WB18 ingress traces.
- Correct one baseline-authoritative snow liquid retention/runoff/infiltration
  partition defect if diagnosis proves one.
- Run focused tests, adjacent snow/runoff/hydrology tests, Rust gates, and
  full H1..H39 runtime/semantic metrics.
- Complete dual review, review disposition, dual verification, final
  disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep` compensation or ET tuning.
- Heuristic runoff/infiltration multipliers.
- Replacing fail-closed SWE or liquid-domain guards with silent clamping.
- Broad frost migration unless snow partition evidence proves frost-owned
  authority is directly blocking closure.
- Multi-OFE routing or watershed carry behavior beyond single-OFE H1..H39
  evidence.

## Deliverables

- Canonical contract amendments for any new snow liquid/runoff/infiltration
  invariant.
- Contract-derived regression tests that fail before and pass after any
  production correction.
- Targeted H1/H7/H39 trace evidence over spring 2014 and spring 2016.
- Production code changes only after contract and test gates are recorded.
- Full H1..H39 runtime/semantic metrics with HPHYS0286 deltas.
- Dual review, review disposition, dual verification, final disposition, and
  worker handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/perc.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `AGENTS.md` only if additional governance gaps are found.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0287_snow_liquid_partition_guard_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read snow/runoff/infiltration contracts and baseline authority.
3. Localize H1/H7/H39 snow liquid, runoff, infiltration, and WB18 ingress
   residuals.
4. Amend canonical contracts for any proven invariant.
5. Add contract-derived failing tests before production code edits.
6. Record pre-implementation contract gate evidence.
7. Implement the minimal baseline-authoritative correction.
8. Run focused tests, adjacent snow/runoff/hydrology tests, Rust gates, and
   H1..H39 metrics.
9. Complete dual review, finding disposition, dual verification, final
   disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel edits are prohibited before steps 1-3 are complete and
recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize any corrected snow
  liquid/runoff/infiltration behavior.
- Contract-derived test fails before and passes after implementation, or the
  package records why no production defect was proven and remains `HOLD`.
- No fail-closed SWE/liquid-domain guard is removed or loosened without
  contract-first proof and review disposition.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned
  findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned. Implementation must not change subprocess
orchestration or sidecar discovery.

## Final Disposition

Status: `executed-hold`

Ran:
- Added contract authority and contract-derived tests for fail-closed runtime
  snow-state validation before inactive snow fallback.
- Implemented a shared runtime snow-state domain guard in the hydrology kernel
  and routed WB12 same-pass infiltration and WB14 runoff reconciliation through
  it.
- Tightened the guard after dual review so partial projected snow-state
  vectors fail closed instead of defaulting missing runtime members to zero.
- Full H1..H39 release runtime completed `39/39`; semantic reports completed
  `39/39`; semantic pass remains `0/39` at
  `/tmp/hphys0287_full_release_after_review_20260604T221027Z`.

Static:
- HPHYS0287 is guard hardening, not valid-run snow-magnitude parity progress.
- The valid-run H1..H39 metrics are unchanged from HPHYS0286, which is expected
  for this guard-only correction because the production suite does not contain
  material invalid projected runtime snow state.
- Continuation should return to baseline-authoritative rain-on-snow liquid
  retention/release magnitude and melt/runoff partition lineage, not fail-open
  canonicalization.
