# HPHYS0294 Post-Ingress Storage/Percolation/Lateral Retention Closure

Status: executed-hold

## Objective

Diagnose and, only if proven by canonical contract evidence, correct the
post-ingress H1/H7/H39 `Total-Soil`/`SoilWaterTotal` residual after HPHYS0293
carries snow-producer residuals as excluded comparator differences and HPHYS0292
keeps `Q` parity closed.

## Rationale

HPHYS0293 localized remaining spring storage residuals away from WB14 runoff:
target rows preserve zero trace-level snow closure residual and zero `Q`
residual, while the pinned comparator retains snowpack/melt differences tied to
corrected negative-melt state authority. The next defensible focus is therefore
post-ingress storage/percolation/lateral retention. This package must not
compensate snow producer differences in WB18/WB19/WB17; it must first prove
ownership with row-level magnitude accounting.

## Included Scope

- Amend canonical `SC-*` authority for HPHYS0294 post-ingress attribution.
- Add contract-derived tests for required WB18/WB19 trace and guard surfaces.
- Run H1/H7/H39 target diagnostics over storage, `Dp`, `latqcc`, `Q`, `RM`,
  `Snow-Water`, WB18 identities, and WB19 lateral lineage.
- Run the full H1..H39 semantic suite and preserve metrics for continuation.
- Correct production code only when diagnostics prove a baseline-authoritative
  WB18/WB19 magnitude or ordering defect not explained by excluded snow
  producer residuals.

## Excluded Scope

- Do not replicate pinned-baseline negative-melt snowpack depletion behavior
  that openWEPP intentionally corrected under HPHYS0284/HPHYS0293.
- Do not tune `Total-Soil`, `SoilWaterTotal`, `Dp`, or `latqcc` publication
  fields without trace-grade process ownership.
- Do not change WB17 `Ep` lineage unless diagnostics prove a post-ingress
  storage-retention defect is not the active owner.
- Do not promote MOFE carry/runon storage-ingress behavior in this package.

## Deliverables

- Canonical contract amendments in `SC-PERC-001` and `SC-WATBAL-001`.
- Contract-derived test `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`.
- Full H1..H39 metrics artifact.
- H1/H7/H39 target row storage/percolation/lateral localization artifacts.
- Gate evidence, owned-file manifest, dual review/verification placeholders,
  disposition, and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/full-39-suite-metrics.md`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `Cargo.toml`
- Production WB18/WB19 files only if diagnostics prove a baseline-authoritative
  defect.

## Phase Plan

1. Contracts: amend canonical `SC-*` authority for HPHYS0294 attribution.
2. Contract tests: add static/contract-derived tests for trace and identity
   surfaces.
3. Pre-implementation gate: run the HPHYS0294 contract test before production
   edits.
4. Diagnostics: run H1/H7/H39 target traces and full H1..H39 semantic suite.
5. Implementation: patch only a proven WB18/WB19 defect, then rerun focused
   and full gates.
6. Review/disposition: update dual review/verification artifacts, disposition,
   and worker handoff.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code only after the gate and diagnostics prove ownership.

## Exit Criteria

- Contract amendments and contract-derived tests are present.
- Pre-implementation contract gate is recorded with `Static:`/`Ran:` truth labels.
- H1/H7/H39 diagnostics separate snow-excluded residuals from WB18/WB19
  storage/percolation/lateral evidence.
- Full H1..H39 semantic metrics are recorded.
- Any production edit has baseline-authoritative provenance and passing focused
  regression evidence.
- Package disposition is `complete` only when all gates/reviews pass; otherwise
  `executed-hold` with explicit continuation recommendation.

## Security-Impact Gate

No external systems, credential flows, network APIs, or shell interpolation are
required. Work is local repository flat-file reads/edits plus local test and
diagnostic commands.

## Autonomous Execution

This package is intended for no-intervention execution from kickoff through
disposition. If diagnostics do not prove a production defect, leave the package
in `executed-hold` and recommend the next baseline-authoritative work package
rather than asking the user for direction.
