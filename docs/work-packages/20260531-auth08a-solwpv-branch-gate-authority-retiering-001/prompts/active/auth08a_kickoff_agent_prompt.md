Scope: local repository governance/test retiering task; flat-file reads/edits
only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/package.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`

Task: execute AUTH08A end-to-end by re-tiering the WB19 branch-law suite from
blocking constitutive posture to non-blocking legacy-conformance governance and
updating contract-derived assertions accordingly.

Constraints:
- No production kernel physics edits.
- Preserve branch-law invariant text while changing governance posture.
- Truthful evidence labels required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- updated registry/suite/contract/test surfaces,
- gate evidence,
- final disposition and handoff.
