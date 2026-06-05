# HPHYS0303 Kickoff Prompt

Scope: local repository science-contract/comparator-governance task; flat-file
reads/edits and local git/build operations only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/package.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/wepp-forest_260430_baseline/AGENTS.md`

Files:

- `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/**`
- `docs/work-packages/README.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0303_adr0016_comparator_ratification_contract.rs`
- `Cargo.toml`

Task: execute package objective end-to-end for declared scope. Create local
fixed-comparator evidence if feasible; otherwise record exact blockers and keep
ADR-0016 in Proposed-HOLD. Do not push refs or reset remote default branches.

Constraints: contract-first sequencing; canonical SC authority; fixed
comparator provenance is not production kernel authority; typed guards; no
silent defaults; no production forcing, snow, WB17, WB18, WB19, or WB13 edits
before paired term/state instrumentation and disposition complete.
Guard phrase: no silent defaults; no production forcing, snow, WB17, WB18,
WB19, or WB13 edits.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including comparator-ratification ledger, ADR update evidence, gate results,
dual review, review disposition, dual verification, and worker handoff.
