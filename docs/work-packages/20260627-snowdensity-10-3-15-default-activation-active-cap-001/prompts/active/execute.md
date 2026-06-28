# Execute SNOWDENSITY-10.3.15

Execution mode: package-end-to-end.

Autonomy: execute all phases through disposition without additional user
intervention unless a hard blocker prevents the declared current-scope gates.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`,
  `docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/package.md`,
  `docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/artifacts/worker-handoff.md`,
  `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- Conditional: `docs/standards/kernel-work-package-preparation.md`,
  `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- On-demand: `tools/snowfreeze_observed/bundle_activation_adjudication.py`,
  `tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py`.

Implement contract-first default activation under the active `522 kg m^-3` cap:

1. Amend `SC-SNOWFREEZE-001` before code changes.
2. Add contract-derived tests and package diagnostics.
3. Make absent direct-production selector envs select
   `coe_liquid_holding_capacity_v1` plus
   `physics_bulk_density_compaction_v1`.
4. Preserve explicit rollback/test envs for `legacy_coe` and `legacy_wepp`;
   reject unsupported selector values fail-closed.
5. Prove default no-env direct-production trace rows consume the activated
   models and preserve parser/runfile/user CLI/output-schema boundaries.
6. Run required gates, reviews, disposition, verification, line-count
   governance, and worker handoff.

Do not include a density-cap change, spring-densification promotion,
open-surface ablation, frost attribution, Qwet/frzftp, fixture changes, or
new public configuration surfaces.
