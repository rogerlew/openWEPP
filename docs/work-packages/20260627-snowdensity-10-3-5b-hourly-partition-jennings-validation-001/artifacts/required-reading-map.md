# Required Reading Map

Status: complete
Evidence mode: Static/Ran

## Read

Core governance and execution guidance:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

Package and science authority:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/artifacts/worker-handoff.md`
- `tests/fixtures/precip_phase_observed/jennings2018/README.md`
- `references/annotated_bibliography.md`

Implementation context:

- `crates/openwepp-meteorology/src/phase.rs`
- `crates/openwepp-meteorology/src/psychrometrics.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`

## Ran

```text
wc -c AGENTS.md docs/codex_exec_plans.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/package.md docs/defect_closure_execplans.md docs/specifications/science-contract-authoring-procedure.md docs/specifications/science-contracts/kernel-process-contract-profile.md docs/specifications/science-contracts/index.md crates/AGENTS.md tests/AGENTS.md docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md docs/planning/snow-frost-fidelity-strategy.md docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/artifacts/worker-handoff.md tests/fixtures/precip_phase_observed/jennings2018/README.md references/annotated_bibliography.md
```

Total required-reading payload: `684808` bytes.

Disposition: `WARN`, not `HOLD`. The payload exceeds the 400 kB soft budget
because `SC-SNOWFREEZE-001.md`, the work-package index, the snow/frost strategy,
and the annotated bibliography are all primary authority for this package. The
read set remains within the 800 kB hard-review band, and implementation context
was loaded selectively after the authority pass.

## Jennings Fixture Discovery

Static/Ran:

- `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv` exists locally, is gitignored, and contains `17,810,806` lines including the header.
- Installed file2 schema:
  `Station_ID,Date,Hour,Air_Temp,Dewpoint,RH,gridded_data_pres,Prec_Type,Snow_Phase,Rain_Phase`.
- Installed station-threshold file is
  `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file3_temp50_observed_by_station.csv`.
- Installed file3 schema: `Station_ID,temp50`.
