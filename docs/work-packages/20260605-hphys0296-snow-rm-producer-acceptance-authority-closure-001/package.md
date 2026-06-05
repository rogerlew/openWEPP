# HPHYS0296 Snow/RM Producer Acceptance and Authority Closure

Status: executed-hold

This work package is an autonomous execution plan and must remain a living
document during execution. It follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Objective

Classify the remaining H1/H7/H39 snow/`RM` producer residuals after HPHYS0295
as either accepted corrected-negative-melt semantic divergence or unresolved
baseline-authoritative winter/snow/rain/melt producer migration debt. Do not
compensate the residual downstream in WB17, WB18, WB19, or WB13.

## Rationale

HPHYS0295 showed that cumulative storage-collapse windows are dominated by
snow/`RM` residuals, while `Q`, WB18 local identity, and WB19 trace identities
remain internally closed. The next low-regret package must decide whether the
snow/`RM` producer is acceptable under the corrected `/workdir/wepp-forest`
negative-melt authority, or whether additional baseline-authoritative producer
work is required before downstream hydrology residuals can be trusted.

## Progress

- [x] Scaffold package and autonomous prompt.
- [x] Amend canonical `SC-*` authority.
- [x] Add contract-derived test.
- [x] Run pre-implementation contract gate.
- [x] Run H1..H39 and targeted H1/H7/H39 snow/`RM` diagnostics.
- [x] Classify acceptance versus producer-migration hold.
- [x] Run validation gates.
- [x] Update review, verification, disposition, and handoff artifacts.

## Surprises & Discoveries

- Observation: H1/H7/H39 first-2013 and spring-2014 windows show material
  negative raw hourly melt and internally closed `RM` publication identity.
  Evidence: HPHYS0296 diagnostics classified six windows as
  `corrected-negative-melt-candidate`.
- Observation: H1/H7/H39 spring-2016 windows do not have material negative raw
  melt sufficient to explain the residual.
  Evidence: spring-2016 negative raw melt is only `-0.224814` to `-0.255930 mm`
  while cumulative `ΔRM` is `-15.276407` to `-16.885426 mm`.
- Observation: Candidate WB13 `RM` publication identity is closed in all
  targeted windows.
  Evidence: `RM identity abs` is `0.000000 mm` for all nine diagnostic windows.

## Decision Log

- Decision: Do not patch WB17/WB18/WB19/WB13 in HPHYS0296.
  Rationale: HPHYS0296 is a snow/`RM` producer acceptance package; diagnostics
  did not prove a downstream hydrology defect.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: Leave the package in `executed-hold`.
  Rationale: six windows can be carried as corrected-negative-melt candidates,
  but the spring-2016 H1/H7/H39 windows remain producer-magnitude/timing holds
  and dual review/verification was not dispatched.
  Date/Author: `2026-06-05` / `Codex`.

## Outcomes & Retrospective

HPHYS0296 established contract authority and diagnostics for snow/`RM`
acceptance. Six H1/H7/H39 windows are consistent with corrected negative-melt
semantic divergence candidates, while all three spring-2016 windows remain
producer-magnitude/timing holds. No production kernel/runtime patch was made.
Continuation should focus on spring-2016 snow/winter producer magnitude/timing,
not downstream ET, percolation, lateral flow, or WB13 aggregate compensation.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and
  `SC-WATBAL-001` for HPHYS0296 snow/`RM` acceptance authority.
- Add a static contract-derived HPHYS0296 test.
- Run full H1..H39 semantic metrics.
- Run H1/H7/H39 targeted traces and classify:
  - corrected-negative-melt evidence,
  - snowpack state closure,
  - `RM = post_winter_rain + routed_melt + irrigation` publication identity,
  - retained/released rain-on-snow contribution,
  - `Snow-Water` state residual magnitude,
  - whether residuals are producer-migration debt or accepted semantic
    divergence.
- Patch production code only when diagnostics prove a baseline-authoritative
  producer defect with a concrete source-line lineage.

## Excluded Scope

- Do not patch WB17 `Ep`/`Es`.
- Do not patch WB18 percolation or aggregate `watcon`.
- Do not patch WB19 lateral flow.
- Do not patch WB13 aggregate storage or `RM` publication as compensation.
- Do not recreate the pinned-baseline negative-melt sign/scale bug rejected by
  corrected `/workdir/wepp-forest` authority.

## Deliverables

- Canonical contract amendments and revision-history entries.
- Contract-derived test
  `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`.
- H1/H7/H39 snow/`RM` classification artifacts.
- Full H1..H39 metrics artifact.
- Gate evidence, owned-file manifest, review/verification artifacts,
  disposition, and worker handoff.

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
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/artifacts/h1-h7-h39-budget-trace-evidence.md`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `Cargo.toml`
- Production snow/winter producer files only if baseline-authoritative producer
  defect evidence is proven.

## Phase Plan

1. Contracts: amend canonical snow, runoff, and water-balance authority for
   HPHYS0296 acceptance classification.
2. Contract tests: add static guards for authority, observability, and
   downstream-compensation prohibition.
3. Pre-implementation gate: run the HPHYS0296 contract test before production
   edits.
4. Diagnostics: run full H1..H39 metrics and H1/H7/H39 snow/`RM` producer
   classification.
5. Implementation: patch only a proven producer defect; otherwise record a
   no-production-patch hold.
6. Review/disposition: update dual review/verification placeholders,
   disposition, and handoff.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code only after diagnostic ownership is proven.

## Exit Criteria

- Contracts and contract-derived tests exist and pass.
- H1/H7/H39 snow/`RM` windows are classified as accepted corrected-negative
  semantic divergence or producer-migration hold.
- Full H1..H39 metrics are recorded.
- No downstream compensation patch is applied.
- Package remains `executed-hold` unless semantic parity and dual
  review/verification gates close.

## Security-Impact Gate

No external systems, credentials, network calls, or shell interpolation are
required. Work is local flat-file reads/edits plus local test and diagnostic
commands.

## Autonomous Execution

This package is intended for end-to-end autonomous execution. If diagnostics do
not prove a production producer defect, leave the package in `executed-hold`
with a specific continuation recommendation.
