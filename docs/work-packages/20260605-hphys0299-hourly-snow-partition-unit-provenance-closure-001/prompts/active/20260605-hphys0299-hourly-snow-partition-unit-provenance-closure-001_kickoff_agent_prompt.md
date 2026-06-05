# HPHYS0299 Kickoff Prompt

Scope: local repository science-contract/kernel diagnostic task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`

Files:

- `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs` (read-only unless corrected evidence proves production defect)
- `crates/openwepp-runner/src/hillslope/mod.rs` (read-only unless corrected evidence proves production defect)

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance from `/workdir/wepp-forest_260430_baseline`; typed guards;
no silent defaults; no canonicalize-and-proceed; no downstream WB17/WB18/WB19
or WB13 compensation; no production physics migration based on a
depth-vs-water-equivalent diagnostic mismatch.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
