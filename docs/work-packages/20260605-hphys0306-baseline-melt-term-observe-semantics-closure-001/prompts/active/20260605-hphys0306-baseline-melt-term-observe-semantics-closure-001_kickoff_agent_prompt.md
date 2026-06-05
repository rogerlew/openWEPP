# HPHYS0306 Kickoff Prompt

Scope: local repository science-contract/kernel diagnostic task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0306_baseline_melt_observe_semantics_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/**`

Task: execute the package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; fixed comparator
provenance at `47ac4c32faeea81bb99081f955a14c38b815ef4d`; typed
branch-active/missing-surface HOLD classification; no silent defaults; no
inactive-hour zero imputation; no production physics edits; no downstream
compensation.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
