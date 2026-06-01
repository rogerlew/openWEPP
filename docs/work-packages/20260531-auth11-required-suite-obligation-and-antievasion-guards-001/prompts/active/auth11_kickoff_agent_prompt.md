Scope: local repository authority-governance hardening task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth11-required-suite-obligation-and-antievasion-guards-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth10-fc-authority-gate-and-suite-consistency-001/artifacts/claude-code-review-findings.md`

Files:
- `AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth11-required-suite-obligation-and-antievasion-guards-001/**`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/promotion-protocol.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
- `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/*`
- `tools/release/check_authority_suite_antievasion.sh`
- `tools/release/README.md`

Task: execute AUTH11 end-to-end to prevent suite evidence-set manipulation by
adding required-case obligations, diff-based anti-evasion checks, promotion
protocol controls, and in-test anchor guards.

Constraints:
- contract-first sequencing;
- canonical `SC-*` authority updates before test updates;
- no silent defaults/clamping for domain violations;
- no production kernel algorithm rewrites in this package.

Autonomy: execute package phases end-to-end and update required artifacts
without asking for additional user direction unless hard-blocked.

Outputs: updated authority/protocol/test/fixture/guard-tool surfaces and
completed AUTH11 artifacts through disposition.
