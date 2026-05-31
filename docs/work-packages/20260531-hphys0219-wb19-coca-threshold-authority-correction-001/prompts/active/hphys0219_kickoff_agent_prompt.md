Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0219-wb19-coca-threshold-authority-correction-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0218-wb19-cpm-adjusted-lateral-drain-threshold-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0219-wb19-coca-threshold-authority-correction-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`

Task: execute HPHYS0219 end-to-end:
- codify WB19 `drfc` threshold authority to `coca_####` lineage
  (`wb18_perc_fc_#### + (1-coca_####)*dg_####`) in canonical contracts,
- implement contract-derived tests for `coca` threshold behavior and typed
  guard vectors,
- project and consume `coca_####` in runtime/production WB19 paths,
- run required workspace gates and fresh `unpalatable-rind` 39-hillslope
  rerun/readjudication.

Constraints:
- contract-first sequencing is mandatory (contracts -> tests -> pre-impl gate -> code),
- no silent defaults/clamping for domain violations,
- preserve typed fail-closed guard posture,
- no heuristic/parity-only substitutions for process physics.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish concrete next-package recommendation for unresolved families.
