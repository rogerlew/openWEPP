Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`

Task: execute HPHYS0216D end-to-end:
- codify FC layer+tail authority in canonical contracts,
- add contract-derived tests for `wb13_profile_fc_tail_mm` authority/guards,
- implement runtime-input FC tail publication and WB13 FC layer+tail consumption,
- run required workspace gates and update package artifacts/disposition.

Constraints:
- contract-first sequencing is mandatory (contracts -> tests -> pre-impl gate -> code),
- no silent defaults/clamping for domain violations in proposed remediation,
- preserve typed fail-closed guard posture,
- dual review + dual verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish a concrete integrated follow-up handoff for coupled-family rerun/adjudication.
