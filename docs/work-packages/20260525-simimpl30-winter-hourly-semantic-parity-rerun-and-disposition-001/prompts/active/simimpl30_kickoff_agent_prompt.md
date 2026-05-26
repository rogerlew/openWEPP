# SIMIMPL30 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/artifacts/simimpl29_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/package.md`
- `docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/*.md`
- `docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/prompts/active/simimpl30_kickoff_agent_prompt.md`
- `docs/work-packages/README.md`

Task: execute SIMIMPL30 end-to-end by running winter-hourly parity reruns,
classifying residuals, and publishing explicit GO/HOLD disposition with
contract-governed evidence.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not modify production kernel/runtime code before contract/test/gate
  requirements are satisfied if corrective edits are required.
- Keep canonical `SC-*` authority as source of truth; package artifacts are
  evidence, not replacement authority.
- Prohibit silent defaults/clamping for domain violations; require typed
  failures/guards.
- Record `Static:` vs `Ran:` labels in all evidence artifacts.
