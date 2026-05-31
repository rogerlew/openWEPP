Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/h1_worked_example_source.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `.gitattributes`
- `Cargo.toml`
- `tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/**`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`

Task: execute AUTH07 objective end-to-end by promoting the H1 worked example
into a tracked, reproducible independent FC-authority cohort suite with
thresholded and rock-bucket-stratified classification checks.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
