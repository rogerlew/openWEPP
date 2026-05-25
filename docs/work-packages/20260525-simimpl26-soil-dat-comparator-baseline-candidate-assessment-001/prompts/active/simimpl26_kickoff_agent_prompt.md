# SIMIMPL26 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/artifacts/simimpl25_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/pl14r-comparator-run-provenance-manifest.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-run-provenance-manifest.md`

Files:
- `docs/work-packages/20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/package.md`
- `docs/work-packages/20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/artifacts/*.md`
- `docs/work-packages/20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/prompts/active/simimpl26_kickoff_agent_prompt.md`
- `tools/legacy_comparison_suite/**`
- `tests/integration/infile_soil_parser_contract.rs`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping; no heuristic/proxy substitutions.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Before production kernel/output edits, confirm SIMIMPL25 prerequisite
  artifacts remain authoritative for this follow-on comparison scope.
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
