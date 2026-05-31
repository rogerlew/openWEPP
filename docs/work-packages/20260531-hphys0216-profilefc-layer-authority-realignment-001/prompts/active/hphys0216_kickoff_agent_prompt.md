Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0215-coupled-family-remediation-planning-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`

Files:
- `docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `docs/work-packages/README.md`

Task: execute HPHYS0216 end-to-end for declared scope:
- realign `ProfileFCStore` publication authority to baseline-authoritative
  layer aggregation (`Σ(thetfc_i * dg_i) * 1000`) under contract-first
  sequencing,
- preserve corrected-layer projection lineage and typed fail-closed guards,
- run required gates and 39-hillslope semantic diagnostics to evaluate closure.

Constraints:
- contract-first sequencing (contracts -> contract-derived tests ->
  pre-implementation gate -> production edits),
- canonical SC authority governs migration obligations,
- baseline provenance anchor
  (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`),
- typed guards; no silent defaults/clamping for domain violations,
- no heuristic/proxy physics substitutions,
- dual review + dual verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish explicit `ProfileFCStore` delta posture and next-package handoff.
