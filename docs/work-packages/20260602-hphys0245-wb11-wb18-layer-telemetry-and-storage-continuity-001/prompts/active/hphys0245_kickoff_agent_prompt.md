Scope: local repository diagnostics task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001/artifacts/hphys0244-focus-recommendations.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/**`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Task: execute HPHYS0245 objective end-to-end by adding gated diagnostics-only
WB11/WB18 layer/storage/percolation telemetry, running `H1`, `H7`, and `H39`,
and publishing storage-continuity analysis.

Constraints: diagnostics-only; no production process-physics changes; no
science-contract amendments; label evidence truthfully; do not tune `Dp` or
storage formulas from telemetry symptoms alone.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
