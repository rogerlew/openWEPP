Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211-residual-gap-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

Task: execute HPHYS0212 end-to-end for declared scope:
- stop daily WB11/WB18 mutable-state reseed re-initialization,
- source WB19 lateral/drain controls from authoritative runtime inputs,
- restore WB13 `latqcc`/`Tile`/`Qd` coupling visibility for deterministic
  residual decomposition and adjudication.

Constraints:
- contract-first sequencing (contracts -> contract-derived tests ->
  pre-implementation gate -> production edits),
- canonical SC authority updates when required by changed obligations,
- baseline provenance anchor
  (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`)
  for migration lineage,
- typed guards; no silent defaults/clamping for domain violations,
- no heuristic/proxy physics substitutions,
- dual review + dual verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- produce explicit residual delta summary and hold-lift posture.
