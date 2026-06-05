# HPHYS0290 Post-Winter Rain Publication Lineage Closure

Status: executed-hold

This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current as work proceeds. This
package follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, WB13 `RM` should consume an explicit post-winter
`rain(iplane)` equivalent rather than inferring post-winter rain from raw
`prcp`, runtime SWE, and routed melt. A user can observe the change with
contract-derived runner/kernel/unit tests, H1/H7/H39 trace rows including the
H39 2014-146 material residual row, and full H1..H39 semantic metrics.

## Progress

- [x] (2026-06-05) Scaffolded HPHYS0290 package structure, kickoff prompt, and queued artifacts.
- [x] (2026-06-05) Read required contracts, HPHYS0289 handoff, and pinned baseline rain-clearing files.
- [x] (2026-06-05) Amended canonical `SC-*` contracts for explicit post-winter rain publication authority.
- [x] (2026-06-05) Added contract-derived tests before production code edits.
- [x] (2026-06-05) Recorded pre-implementation contract-gate evidence.
- [x] (2026-06-05) Implemented minimal post-winter rain publication/consumption correction.
- [x] (2026-06-05) Ran focused tests, adjacent tests, Rust gates, and full H1..H39 metrics.
- [x] (2026-06-05) Completed dual review, review disposition, dual verification, final disposition, and handoff.

## Surprises & Discoveries

- Observation: HPHYS0289 corrected routed `wmelt` publication and reduced `RM` fail count by 765, but `Q`/`Snow-Water` did not move and `RM` mean residual worsened slightly.
  Evidence: `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/full-39-suite-metrics.md`.
- Observation: H39 2014-146 remains material because WB13 currently publishes `RM=2.62 mm` from raw `prcp` when runtime SWE and routed melt are zero, while snow diagnostics show a residual rain-release seam.
  Evidence: `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/h1-h7-h39-trace-evidence.md`.
- Observation: Pinned baseline `contin.for` clears `rain(iplane)` after `winter` and restores `warain` only for the warm-rain/no-snow branch, so H39 2014-146 `RM=2.62 mm` is baseline lineage rather than a WB13 inference defect.
  Evidence: `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880`.
- Observation: The first implementation reset daily snow publication surfaces with a state zero that could satisfy WB13 after the flux was missing; review caught this as canonicalize-and-proceed masking.
  Evidence: `artifacts/review_agent_a.md`.
- Observation: The fixed implementation requires `snow.post_winter_rain_m` from the flux surface only; state-only values now fail closed before WB13 publication.
  Evidence: `crates/openwepp-runner/src/hillslope/mod.rs` and `cargo test -p openwepp-runner hphys0290_wb13_rm_publication -- --nocapture`.

## Decision Log

- Decision: Scope HPHYS0290 to explicit post-winter rain publication and WB13 consumption, not broader snowpack, runoff, or storage correction.
  Rationale: HPHYS0289 HOLD evidence localized the next defect to the missing named `rain(iplane)` equivalent after winter processing.
  Date/Author: 2026-06-05 / Codex.
- Decision: Require `snow.post_winter_rain_m` as a same-day producer flux for WB13 instead of allowing state fallback.
  Rationale: A state fallback can mask a missing producer and violates the explicit fail-closed contract.
  Date/Author: 2026-06-05 / Codex.

## Outcomes & Retrospective

Executed and held for continuation. HPHYS0290 closes the WB13 post-winter rain
publication lineage seam and proves H39 2014-146 `RM=2.62 mm` is the baseline
warm-rain/no-snow restoration branch. Full H1..H39 semantic parity remains
`0/39`; continuation should target upstream snowpack timing/state and
runoff/storage partition residuals rather than changing WB13 `RM` publication
math.

## Objective

Diagnose, correct, and validate the WB13 `RM` post-winter rain lineage so daily
water-balance rows consume a named, fail-closed runtime surface equivalent to
baseline `rain(iplane)` after `winter.for`/`contin.for` rain clearing and
warm-rain/no-snow restoration.

## Rationale

Pinned baseline WEPP computes WB13 `RM` from `rain(iplane) + wmelt(iplane) +
irdept(iplane) + iraplo(iplane)`. HPHYS0289 corrected the routed `wmelt` side
but retained a branch that inferred post-winter rain from raw `prcp` and snow
state activity. That inference is not the baseline contract: `contin.for`
explicitly clears `rain(iplane)` after winter processing and restores it only
for the warm-rain/no-snow branch. openWEPP already computes the equivalent
post-winter liquid rainfall as the snow-partitioned hyetograph liquid input;
this package names, publishes, and consumes that surface.

## Included Scope

- Amend canonical `SC-WATBAL-001`, `SC-RUNOFFPART-001`, and
  `SC-SNOWFREEZE-001` for explicit post-winter rain publication provenance.
- Add contract-derived tests before production code edits.
- Register the new boundary symbol in the unit registry with depth units.
- Publish a named post-winter rain surface from runoff reconciliation.
- Update WB13 `RM` publication to consume the named post-winter rain surface,
  routed `wmelt`, and irrigation.
- Preserve HPHYS0287 fail-closed snow-state validation and HPHYS0288/0289
  routed-melt semantics.
- Run H1/H7/H39 targeted traces and full H1..H39 semantic metrics.
- Complete dual independent reviews, finding disposition, dual verification,
  final disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep`, WB18 storage, WB19 lateral, or runoff partition tuning.
- Heuristic rain/snow/runoff multipliers.
- Replicating rejected negative-melt behavior.
- Changing HPHYS0288 rain-on-snow release into routed melt.
- Broad frost migration or watershed/MOFE routing changes.

## Deliverables

- Canonical contract amendments with pinned baseline provenance citations.
- Contract-derived tests proving WB13 `RM` consumes explicit post-winter rain,
  not an inferred raw-precipitation branch.
- Production correction for the minimal publication seam proven by tests.
- Boundary unit registry coverage for the new surface.
- H1/H7/H39 trace evidence over current material residual days.
- Full H1..H39 runtime and semantic metrics compared to HPHYS0289.
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
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/contin.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/winter.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-sim-contract/src/units.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required contracts, HPHYS0289 evidence, and pinned baseline files.
3. Amend canonical contracts for explicit post-winter rain publication authority.
4. Add contract-derived failing tests before production code edits.
5. Record pre-implementation contract gate evidence.
6. Implement minimal kernel/runner/unit-registry publication-lineage correction.
7. Run focused tests, adjacent tests, Rust gates, and H1..H39 metrics.
8. Complete dual reviews, review disposition, dual verification, final disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel/runtime publication edits are prohibited before steps 1-3 are
complete and recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly require a named post-winter rain
  publication surface equivalent to baseline `rain(iplane)` after winter
  processing.
- A contract-derived test fails before and passes after production correction,
  or the package remains `HOLD` with evidence that no production defect was proven.
- WB13 `RM` consumes explicit post-winter rain, routed `wmelt`, and irrigation.
- HPHYS0287 fail-closed runtime snow-state validation and HPHYS0288/0289
  routed-melt semantics remain intact.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or
unsafe Rust change is planned. Implementation must not change subprocess
orchestration or sidecar discovery.
