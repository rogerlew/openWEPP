Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/**`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`

Task: execute HPHYS0228 end-to-end to restore WB14 disturbed-soil conductivity
adjustment (`ksatadj`) successful-lane tests for `solwpv=9001/9002/9003`,
ensuring WB19 indexed FC/WP prerequisites are valid and typed-guard behavior
remains fail-closed.

Constraints: contract-first sequencing; canonical SC authority updates before
test/implementation edits; typed hard-fail guard posture; no silent defaults;
no heuristic/proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
