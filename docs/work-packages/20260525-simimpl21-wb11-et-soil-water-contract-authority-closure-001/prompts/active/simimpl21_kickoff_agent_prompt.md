# SIMIMPL21 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/package.md`
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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-wb11-soil-water-baseline-authority-path-assessment.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-ep-es-er-full-fidelity-migration-risk-register.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-contract-impact-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/soil-water-et-baseline-auth-queue.md`

Files:
- `docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/package.md`
- `docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/*.md`
- `docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/prompts/active/simimpl21_kickoff_agent_prompt.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping; no heuristic/proxy/placeholder ET
substitutions; no production kernel/runtime code edits in this contract-only
package.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- This package executes contract-authority closure (contract-first step 1) for
  declared WB11 ET/soil-water scope.
- Do not edit production kernel/runtime/output code in this package.
- Canonical `SC-*` files are authoritative; package artifacts are evidence.
- Preserve truthful gap posture updates; do not mark unresolved companion
  closure risks as closed.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
