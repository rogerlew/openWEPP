# SIMIMPL35 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/artifacts/simimpl31_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/artifacts/simimpl32_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001/artifacts/simimpl33_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001/artifacts/simimpl34_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

Files:
- `docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/package.md`
- `docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/artifacts/*.md`
- `docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/prompts/active/simimpl35_kickoff_agent_prompt.md`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping for domain violations; no heuristic
or surrogate frost-process equations in production paths.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not modify kernel code until prerequisite contract/test/gate evidence
  from SIMIMPL31/SIMIMPL32/SIMIMPL33/SIMIMPL34 is confirmed in package
  artifacts.
- Canonical `SC-*` files remain authority; package artifacts are evidence.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.
- Prohibit silent defaults/clamping for domain violations; require explicit
  typed guards.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
