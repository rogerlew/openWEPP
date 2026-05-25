# Simimpl13 contract implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- SIMIMPL13 Phase A is assessment-only intake/baseline work; no canonical `SC-*`
  amendments and no production code edits are performed in this phase.
- Canonical authority surfaces consumed for Phase A baseline:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  - `docs/specifications/science-contracts/index.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- Replay/parity contract obligations emphasized for SIMIMPL13 closure assessment:
  - `SC-WATBAL-001`: `INV-WATBAL-017`..`INV-WATBAL-021`
  - `SC-SYSTEM-001`: `INV-SYSTEM-017`..`INV-SYSTEM-021`
- Legacy baseline provenance anchor remains pinned and readable:
  - `/workdir/wepp-forest_260430_baseline` @
    `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Selective consolidated-intake posture remains enforced (`SIMCONS` authority);
  no wholesale `/workdir/wepp-forest` intake is introduced in Phase A.

## Ran
- Authority and prerequisite intake reads executed via `sed -n` against required
  files listed in the Phase A kickoff prompt.
- Provenance verification commands:
  - `git rev-parse HEAD` (openWEPP)
  - `git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD`
  - `git -C /workdir/wepp-forest rev-parse HEAD`
- Replay evidence extraction commands over SIMIMPL11 bundle:
  - JSON probes (`python`) over:
    - `suite_dat/investigation/h5_wat_semantic_comparator.json`
    - `suite_parquet/investigation/h5_wat_semantic_comparator.json`
    - `suite_dat/investigation/h5_wat_strict_comparator.json`
    - `candidate/openwepp_hillslope_run_manifest.json`
  - Candidate parquet span probe (`duckdb`):
    - `count(*) = 1`, `year = 2000`, `julian = 1`
  - Baseline/candidate numeric-row probes (`awk`) showing:
    - baseline keyed rows: `1095`
    - candidate keyed rows: `1`

## Contract-governance conclusion
- Contract authority intake for SIMIMPL13 assessment scope is complete.
- No contract-first sequencing violation is introduced.
- Residual blockers remain open and are normalized in SIMIMPL13 artifact gap
  registers for closure-wave authoring and downstream execution.
