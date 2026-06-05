# HPHYS0302 Comparator Surface Audit Closure

## Status

Executed-HOLD.

## Objective

Audit comparator surfaces for H1/H7/H39 target-window `RM`, `Snow-Water`, and
melt-term lineage before any further openWEPP producer-defect conclusion. The
package must prove whether baseline and openWEPP cut-points are the same
physical quantity in the same units, then record which residuals may proceed to
paired `melt.for` / `snowd.for` term-state instrumentation.
Comparator-surface closure requires the same physical quantity in the same units.

## Rationale

HPHYS0301 showed that the apparent H39 first-2013 forcing defect was a
comparison-surface mismatch: baseline residual rain-on-snow was compared to
openWEPP raw rain. Claude Code review approved the no-production-edit decision
and required the next package to audit comparator surfaces before another
producer-defect claim. HPHYS0302 is that gate.

## Included Scope

- Amend canonical `SC-SNOWFREEZE-001` and `SC-WATBAL-001` to require
  comparator-surface proof for `RM`, `Snow-Water`, raw/post-raw melt, and
  melt-term surfaces before producer-defect claims.
- Add contract-derived tests proving HPHYS0302 is a surface-audit gate and
  prohibits production edits from aggregate deltas alone.
- Build and run a comparator-surface audit runner over H1/H7/H39 target
  windows using HPHYS0300/HPHYS0301 artifacts and the existing HPHYS0300 run
  root.
- Publish per-surface verdicts for:
  - `RM` daily WB13/WAT surface,
  - `Snow-Water` daily WB13/WAT surface,
  - raw hourly `hrmlt` aggregate surface,
  - post-raw `wmelt` / routed melt surface,
  - term-level `amelt`/`bmelt`/`cmelt`/`dmelt` surfaces.
- Record whether any residual is authorized for production correction or
  remains in `HOLD` pending paired baseline/openWEPP term instrumentation.
- Complete dual review/disposition and dual verification.

## Excluded Scope

- Production physics edits.
- WB17, WB18, WB19, WB13, or snow-producer compensation.
- Treating aggregate `RM`, `Snow-Water`, raw melt, or routed melt deltas as
  term-level producer authority.
- Treating observe tags absent from `/workdir/wepp-forest_260430_baseline/src`
  as source-line equation authority.
- Re-running full H1..H39 metrics when no production code is changed; carry
  forward HPHYS0301 metrics truthfully.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/target-window-lineage-schema.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/corrected-partition-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/raw-post-raw-lineage-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/claude-code-review-findings.md`
- `/tmp/hphys0300_full_20260605T155527Z`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0302_comparator_surface_audit_contract.rs`
- `Cargo.toml`

## Phase Plan

1. **Contracts**: add canonical comparator-surface audit authority.
2. **Contract-derived tests**: add tests for surface gating and no production
   edit authority from aggregate deltas.
3. **Pre-implementation gate**: run the focused HPHYS0302 contract test before
   any production-code checkpoint.
4. **Audit execution**: run the HPHYS0302 surface audit runner against the
   existing HPHYS0300 run root and artifacts.
5. **Decision checkpoint**: authorize no production edit unless a valid
   like-for-like paired surface proves a source-line producer defect.
6. **Review and verification**: complete dual review, review disposition, dual
   verification, and final handoff.

## Progress

- [x] Scaffold package and required artifacts.
- [x] Amend contracts.
- [x] Add contract-derived tests.
- [x] Record pre-implementation contract gate.
- [x] Run comparator-surface audit.
- [x] Complete decision checkpoint.
- [x] Complete dual review/disposition/verification.

## Exit Criteria

- Canonical contracts define HPHYS0302 comparator-surface gate requirements.
- Contract-derived tests pass.
- `artifacts/comparator-surface-audit-ledger.json` and
  `artifacts/comparator-surface-audit-summary.md` publish per-surface verdicts.
- The decision is explicit: production correction with valid paired surface
  authority, or `HOLD` with concrete missing comparator surfaces.
- HPHYS0301 full H1..H39 metrics are carried forward truthfully because no
  production code is changed.
- Dual review and dual verification artifacts are completed with no
  undispositioned findings.

## Security Impact Gate

No external systems, credentials, network actions, or shell interpolation are
in scope. Work is limited to local flat-file reads/edits, local Rust test
execution, and local comparator artifact reads with explicit paths.
