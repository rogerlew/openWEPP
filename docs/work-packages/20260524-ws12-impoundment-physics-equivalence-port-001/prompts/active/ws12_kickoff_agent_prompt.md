# WS12 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/
edits only; no external connectivity.
Phase: A only.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-ws12-impoundment-physics-equivalence-port-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
Files:
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260524-ws12-impoundment-physics-equivalence-port-001/artifacts/ws12-contract-implementation-evidence.md`
- `docs/work-packages/20260524-ws12-impoundment-physics-equivalence-port-001/artifacts/ws12-impoundment-physics-authority-and-guard-map.md`
Task: implement canonical WS12 impoundment-physics authority amendments and
record contract-implementation/authority-map evidence for Phase A only.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.
Autonomy: execute this phase end-to-end and update listed WS12 phase artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update listed WS12 artifacts for this phase only.

Execution topology:
- Run WS12 in dedicated worktree branch
  `ws12-impoundment-physics-equivalence-port-001`.
- WS11 may run concurrently in its own worktree branch.
- WS12 must rebase onto post-WS11 `main` and rerun required gates before
  merge-back.

Mandatory sequencing constraints:
- Do not modify production kernel code until:
  1. canonical contract amendments are implemented,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- WS12 migration physics authority must be canonical `SC-*` text, not
  package-local notes.
- Do not introduce silent defaults/clamping for domain violations; use typed
  errors/guards.
