Scope: local repository science-contract/kernel diagnostics task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/artifacts/worker-handoff.md`
- `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.tsv`

Files:
- `docs/work-packages/20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`

Task: execute HPHYS0216 follow-up diagnostics end-to-end:
- isolate and quantify the `ProfileFCStore` regression pattern in the
  39-hillslope cohort,
- map the observed deltas to concrete symbol lineage and normalized-depth/tail
  behavior,
- publish explicit remediation handoff package guidance.

Constraints:
- no production kernel/runtime code edits in this package,
- no silent defaults/clamping for domain violations in proposed remediation,
- preserve contract-first sequencing obligations for follow-on implementation
  package(s),
- dual review + dual verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish a concrete follow-up remediation package recommendation.
