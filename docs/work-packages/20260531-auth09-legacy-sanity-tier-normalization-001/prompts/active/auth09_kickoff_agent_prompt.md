Scope: local repository external-authority taxonomy normalization task; flat-file reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth09-legacy-sanity-tier-normalization-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/artifacts/claude-code-review-findings.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth09-legacy-sanity-tier-normalization-001/**`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001/*`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`

Task: execute AUTH09 end-to-end by adding a canonical Level-3 legacy/sanity
authority tier and re-tiering the WB19 `solwpv` branch suite into that tier
with coherent IDs, metadata, and test assertions.

Constraints: contract-first sequencing; canonical SC authority linkage; typed
fail-closed posture (no silent defaults); preserve branch-law physics semantics;
no production kernel algorithm changes unless required by contract-test closure.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: updated authority model/schema/registry/suite/SC references/tests and
completed AUTH09 artifacts through final disposition.
