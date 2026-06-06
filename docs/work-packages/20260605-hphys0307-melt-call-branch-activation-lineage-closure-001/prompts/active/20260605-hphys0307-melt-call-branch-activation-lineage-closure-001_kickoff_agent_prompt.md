# HPHYS0307 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/branch-active-melt-term-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

Files:

- `Cargo.toml`
- `tests/integration/hphys0307_melt_call_branch_activation_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/**`

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical `SC-*` authority; fixed
baseline provenance from `/workdir/wepp-forest_260430_baseline`; typed guards;
no silent defaults; no heuristic/proxy process-physics substitutions; no
WB13/WB17/WB18/WB19/WB12 compensation; no production kernel edits unless
source-line provenance identifies an openWEPP branch-activation defect.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
