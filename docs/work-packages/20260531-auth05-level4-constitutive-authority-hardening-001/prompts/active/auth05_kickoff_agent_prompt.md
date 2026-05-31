Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/artifacts/claude-code-review-findings.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/**`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md`
- `docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md`
- `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/*.json`
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`

Task: execute AUTH05 objective end-to-end for declared scope, hardening Level-4
constitutive gates to use model-to-authority checks on real soils and removing
legacy-as-authority citation posture.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance where applicable; typed guards; no silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
