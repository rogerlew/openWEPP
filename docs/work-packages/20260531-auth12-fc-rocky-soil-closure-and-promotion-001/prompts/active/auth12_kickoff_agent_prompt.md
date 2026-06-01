Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth12-fc-rocky-soil-closure-and-promotion-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/work-packages/20260531-auth11-required-suite-obligation-and-antievasion-guards-001/artifacts/claude-code-auth11-review.md`

Files:
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp_hillslope_orchestrator/**`
- `docs/work-packages/20260531-auth12-fc-rocky-soil-closure-and-promotion-001/**`

Task: execute AUTH12 objective end-to-end for declared scope, including
contract-first FC rocky-soil closure, anchor transition to `within`,
and posture promotion readiness evidence.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline`
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults;
no heuristic/proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases with
truthfulness labels (`Static:` vs `Ran:`).
