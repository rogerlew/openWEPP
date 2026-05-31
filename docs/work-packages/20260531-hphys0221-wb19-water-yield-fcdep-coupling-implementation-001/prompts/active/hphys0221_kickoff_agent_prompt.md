Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

Files:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/work-packages/20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/**`

Task: implement HPHYS0221 end-to-end:
- codify WB19 `solwpv` branch semantics,
- implement `avpora/avfca/avcoca` and `watyld` coupling,
- publish `wb19_fcdep`/`wb19_unsdep`/`wb19_watyld`,
- add contract-derived tests and rerun parity + workspace gates.

Constraints:
- contract-first sequencing (contracts, tests, gate, production code),
- canonical SC authority and baseline provenance are mandatory,
- typed guards only; no silent defaults/clamping,
- preserve truthful `Static:`/`Ran:` evidence labels.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- updated contracts/tests/production code,
- full gate evidence (`fmt`, `clippy`, `test`, `deny`),
- rerun adjudication and disposition with immediate next actions.
