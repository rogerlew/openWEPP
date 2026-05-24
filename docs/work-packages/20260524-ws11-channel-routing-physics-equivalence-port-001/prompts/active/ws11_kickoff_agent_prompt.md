# WS11 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/
edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
Files:
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/ws11-contract-implementation-evidence.md`
- `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/ws11-channel-routing-physics-authority-and-guard-map.md`
Task: execute WS11 end-to-end with contract-first sequencing, including
canonical authority updates, contract-derived tests, pre-implementation gate
evidence, production routing implementation, verification, and disposition
artifacts.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required WS11 artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update required WS11 artifacts/disposition for all completed phases.

Execution topology:
- Run WS11 in dedicated worktree branch
  `ws11-channel-routing-physics-equivalence-port-001`.
- WS12 may run concurrently in its own worktree branch.
- WS11 merges to `main` before WS12 merge-back.

Mandatory sequencing constraints:
- Do not modify production kernel code until:
  1. canonical contract amendments are implemented,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- WS11 migration physics authority must be canonical `SC-*` text, not
  package-local notes.
- Do not introduce silent defaults/clamping for domain violations; use typed
  errors/guards.
