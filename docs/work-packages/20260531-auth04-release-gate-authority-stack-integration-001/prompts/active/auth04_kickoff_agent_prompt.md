Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/package.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `.github/workflows/release-gates.yml`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/README.md`

Task: execute package objective end-to-end for declared scope by integrating
authority-stack suite classes into release/CI gates with explicit fail policy.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards/no silent defaults in any proposed runtime policy.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
