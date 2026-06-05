# HPHYS0301 H39 Forcing/Melt-Term Producer Closure

## Status

Executed-HOLD.

## Objective

Resolve the H39 first-2013 `corrected-depth-hourly-forcing` lane from
HPHYS0300 by proving whether the apparent `hrrain`/`hrsnow` forcing residual is
a production forcing defect, a rain-on-snow retention/release lineage mapping
defect, or a hard blocker. Preserve the raw/post-raw melt-term evidence
requirement for every row that remains snow-producer-owned.

## Rationale

HPHYS0300 correctly refused raw/post-raw production edits from aggregate melt
deltas. Its continuation required a forcing-function package: fix H39
first-2013 hourly forcing if source-line proof exists, and instrument the
paired `melt.for`/`snowd.for` term/state lineage needed for raw/post-raw
closure. The H39 first-2013 row is the lowest-regret next boundary because it
is the only HPHYS0300 row still labeled as corrected-depth hourly forcing.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001` and `SC-WATBAL-001` to require HPHYS0301
  H39 first-2013 forcing/release reconciliation before production forcing edits.
- Add contract-derived tests proving HPHYS0301 is an implementation-or-blocker
  package, not an unbounded diagnostic-only continuation.
- Build and run an HPHYS0301 lineage runner that compares:
  - HPHYS0300 baseline aggregate `hrrain`/`hrsnow` evidence,
  - baseline observe rain-release aggregate evidence available in the pinned
    comparator run,
  - openWEPP raw rain, retained rain, released rain, post-winter rain, raw melt,
    and routed melt trace surfaces, and
  - source-line static authority for `stmtim.for`, `snowd.for`, `winter.for`,
    and openWEPP forcing/snow trace publication.
- Record an implementation decision: production forcing correction, production
  snow-retention/melt correction, or concrete blocker with continuation scope.
- Run focused contract and artifact gates plus dual review/disposition and
  dual verification.

## Excluded Scope

- WB17, WB18, WB19, or WB13 compensation edits.
- Production physics edits from aggregate-only `RM`, `Snow-Water`, storage, or
  raw rain deltas.
- Reproducing the pinned-baseline negative-melt sign/branch bug.
- Silent defaults, canonicalize-and-proceed paths, or weakened typed
  fail-closed snow-state guards.
- Treating baseline observe tags as source-line authority when the tag site is
  absent from `/workdir/wepp-forest_260430_baseline/src`.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/raw-post-raw-lineage-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0301_h39_forcing_melt_term_producer_contract.rs`
- `Cargo.toml`
- Production forcing or snow-kernel files only if HPHYS0301 evidence identifies
  a source-line producer defect:
  - `crates/openwepp-climate-runtime-adapter/src/lib.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

## Phase Plan

1. **Contracts**: add HPHYS0301 canonical authority for H39 first-2013
   forcing/release reconciliation and implementation-or-blocker closure.
2. **Contract-derived tests**: add tests that enforce package autonomy,
   H39-specific release reconciliation, and prohibition of raw-rain aggregate
   production edits.
3. **Pre-implementation contract gate**: run the focused HPHYS0301 test before
   diagnostic runner execution and record truthfully labeled evidence.
4. **Lineage execution**: run the HPHYS0301 runner against the existing
   HPHYS0300 run root and package artifacts.
5. **Correction checkpoint**: edit production code only if source-line evidence
   identifies an openWEPP producer defect; otherwise record the concrete
   blocking invariant and continuation scope.
6. **Review and verification**: complete dual review, review disposition, dual
   verification, and final handoff.

## Progress

- [x] Scaffold package and required artifacts.
- [x] Amend contracts.
- [x] Add contract-derived tests.
- [x] Record pre-implementation contract gate.
- [x] Run lineage execution.
- [x] Complete correction checkpoint.
- [x] Complete dual review/disposition/verification.

## Disposition Summary

HPHYS0301 completed as `executed-hold`. The H39 first-2013 apparent raw-rain
forcing residual is not production forcing authority: baseline evidence is
residual rain-on-snow after retention/release, while the HPHYS0300 openWEPP
comparison used raw hourly rain. The valid comparison is baseline residual
rain-on-snow against openWEPP released plus post-winter rain, which reduces the
aggregate residual from `-16.476986 mm` to `-0.237193 mm`.

No production forcing, snow-producer, WB17, WB18, WB19, or WB13 edit is
authorized by this package. The independent Claude Code review approves that
refusal and withdraws the prior HPHYS0300 "fix H39 now" recommendation.
Continuation must first audit comparator surfaces for `RM`, `Snow-Water`, and
melt-term lineage so baseline and openWEPP cut-points are the same physical
quantity in the same units before any producer-defect conclusion or paired
`melt.for` / `snowd.for` implementation package.

## Exit Criteria

- Canonical contracts define HPHYS0301 H39 forcing/release evidence
  requirements.
- Contract-derived tests pass.
- `artifacts/h39-forcing-release-lineage-ledger.json` and
  `artifacts/h39-forcing-release-lineage-summary.md` publish the H39
  first-2013 source-bounded reconciliation.
- The implementation decision is explicit: production correction with
  source-line authority, or `HOLD` with named blocker and continuation scope.
- HPHYS0300 full H1..H39 metrics are carried forward or rerun truthfully with
  evidence labels.
- Dual review and dual verification artifacts are completed with no
  undispositioned findings.

## Security Impact Gate

No external systems, credentials, network actions, or shell-interpolation
changes are in scope. Work is limited to local flat-file reads/edits, local
Rust test execution, and local comparator artifact reads with explicit paths.
