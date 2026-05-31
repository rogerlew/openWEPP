Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-template.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry-template.yaml`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/tools/release/run_release_candidate_gates.sh`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/**`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/suite-template.md`
- `docs/specifications/external-authority/registry-template.yaml`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md`
- `docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `tools/release/README.md`
- `tools/release/run_release_candidate_gates.sh`
- `tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/{fixtures.sha256,fixtures.provenance.yaml}`
- `tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/{fixtures.sha256,fixtures.provenance.yaml}`
- `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/{fixtures.sha256,fixtures.provenance.yaml}`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `Cargo.toml`

Task: execute AUTH06 objective end-to-end for deterministic fixture
reproducibility, including required fixture hashes/provenance and blocking gate
enforcement.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
