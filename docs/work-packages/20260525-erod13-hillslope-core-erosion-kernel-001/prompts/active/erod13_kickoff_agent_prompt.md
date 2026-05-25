# EROD13 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/artifacts/erod11-wave0-gate-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12-wave0-release-verdict.md`
- `/workdir/openWEPP/docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`

Files:
- `docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-sim-contract/**`
- `tests/integration/**`
- `docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/artifacts/*.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults or silent clamping.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not edit production kernel/runtime code until all of the following are
  complete and documented:
  1. canonical contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate evidence.
- Canonical `SC-*` files are authoritative for physics, invariants, and guard
  semantics; package-local notes are evidence only.
- Contract-derived tests must be authored from canonical authority, not backfit
  from current implementation behavior.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.
- Do not introduce fallback wrappers or silent default/clamp behavior for
  missing required inputs, invalid domains, or non-finite values; surface typed
  errors/guards.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
