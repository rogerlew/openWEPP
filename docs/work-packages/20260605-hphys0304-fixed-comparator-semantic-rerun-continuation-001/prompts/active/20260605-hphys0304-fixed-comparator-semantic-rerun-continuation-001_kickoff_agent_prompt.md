# HPHYS0304 Kickoff Prompt

Scope: local repository science-contract/comparator continuation task;
flat-file reads/edits and local comparator execution only; no external
connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/package.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/comparator-surface-audit-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/comparator-ratification-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-baseline-parquet-manifest.json`

Files:

- `docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/**`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/**`
- `docs/work-packages/README.md`
- `tests/integration/hphys0304_fixed_comparator_semantic_rerun_contract.rs`
- `Cargo.toml`

Task: execute ADR-0016 Required Continuation Order step 1 end-to-end for the
declared scope, then scaffold step 2 as HPHYS0305. Run H1..H39 semantic
comparisons against the fixed baseline, aggregate metrics, reclassify the nine
H1/H7/H39 snow/`RM` target windows, and keep HPHYS0302 production-edit `HOLD`
unless paired term/state evidence exists.

Constraints: contract-first sequencing; canonical SC authority; fixed
comparator provenance; typed guards; no silent defaults; no production forcing,
snow, WB13, WB17, WB18, WB19, or WB12 edits; no downstream compensation from
aggregate or hourly-only comparator evidence.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including semantic rerun reports, fixed-baseline full-suite metrics,
target-window reclassification, HPHYS0305 scaffold evidence, gate results,
dual review, review disposition, dual verification, and worker handoff.
