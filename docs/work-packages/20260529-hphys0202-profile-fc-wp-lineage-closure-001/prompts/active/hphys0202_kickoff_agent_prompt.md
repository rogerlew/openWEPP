Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0201-physics-first-gate-reframe-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

Task: execute HPHYS0202 objective end-to-end for declared scope:
close `ProfileFCStore`/`ProfileWPStore` process lineage on contract authority,
and publish semantic parity diagnostics as investigation evidence.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no
silent defaults; no heuristic/proxy physics substitutions in production code.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
