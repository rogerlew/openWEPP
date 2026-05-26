# SIMIMPL32 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/artifacts/simimpl31_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

Files:
- `docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/artifacts/*.md`
- `docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/prompts/active/simimpl32_kickoff_agent_prompt.md`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping; no production kernel/runtime edits
in this package.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- SIMIMPL32 executes contract-first steps 2 and 3 for declared frost scope.
- Do not edit production kernel/runtime code in this package.
- Canonical `SC-*` files are authoritative; package artifacts are evidence.
- Preserve truthful gate posture updates; do not claim runtime closure.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
