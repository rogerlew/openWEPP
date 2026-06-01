Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/artifacts/hphys0223_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/wc1/runs/un/unpalatable-rind`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/suites/*.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/**/*hphys0224*.rs`
- `tests/integration/**/*auth*.rs`
- `tools/legacy_comparison_suite/**`

Task: execute the HPHYS0224 objective end-to-end for declared scope, closing
CAM-mandated `A0/A1/A3` authority/gate gaps for unresolved WB19/soil-water
families and publishing post-change cohort readjudication.

Constraints: contract-first sequencing; canonical SC authority updates before
kernel edits; baseline provenance
(`wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`);
typed guards and explicit failures; no silent defaults; no heuristic/proxy
process-physics substitutions in production code.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
