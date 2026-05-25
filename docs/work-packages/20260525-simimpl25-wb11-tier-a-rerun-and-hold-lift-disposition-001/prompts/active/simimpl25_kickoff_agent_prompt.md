# SIMIMPL25 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/soil-water-et-baseline-auth-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/artifacts/simimpl23_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/artifacts/simimpl24_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/package.md`
- `docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/artifacts/*.md`
- `docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/prompts/active/simimpl25_kickoff_agent_prompt.md`
- `tools/legacy_comparison_suite/**`
- `tests/integration/pl14*_tier_a_*`
- `tests/integration/pl15*_tier_a_*`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping; no heuristic/proxy/placeholder
publication lineage substitutions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Before production kernel/output edits, confirm SIMIMPL21/SIMIMPL22/SIMIMPL23
  and SIMIMPL24 prerequisite authority, tests, and migration artifacts remain
  authoritative.
- Canonical `SC-*` files are authoritative; package artifacts are evidence.
- Do not add silent fallback behavior; emit typed errors/guards for domain and
  non-finite violations.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
