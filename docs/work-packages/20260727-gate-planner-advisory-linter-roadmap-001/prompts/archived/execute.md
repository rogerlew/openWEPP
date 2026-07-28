# Execute Gate Planner Advisory-Linter Roadmap Authoring

Archived: 2026-07-27 after planning-only completion.

Scope: local repository documentation and architecture-planning work;
flat-file reads/edits only; no external systems or network actions are
required.

Execution mode: package-end-to-end.

Phase plan: execute all planning phases in `package.md` sequentially through
review, finding disposition, documentation verification, and planning
disposition. Do not implement or scaffold downstream executable work.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/AGENTS.md`, this package,
  `docs/work-packages/gate-planner-advisory-linter-roadmap.md`,
  `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`,
  `docs/decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md`,
  `docs/decisions/0041-separate-testgate-from-observational-quality-ci.md`,
  `docs/decisions/0042-science-implementation-and-calibration-readiness.md`,
  `docs/standards/testing-and-gate-strategy.md`, and
  `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/artifacts/testgate-trajectory-and-value-assessment.md`,
  `docs/specifications/science-contracts/AGENTS.md`,
  `docs/specifications/correctness-authority-model.md`,
  `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/package.md`,
  `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/execution-control-contract.md`,
  and
  `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/executor-schema.md`.
- Conditional: `docs/standards/prompt-wording-guidance.md`,
  `docs/codex_exec_plans.md`, `tools/local_ci/README.md`, and current
  gate-planner package artifacts when authoring prompts, downstream package
  specifications, or migration dispositions.
- On-demand: `crates/openwepp-gate-planner/**`, `gate-policy/**`,
  `.github/workflows/**`, and CAL-04B package-local tooling for exact capability
  and consumer discovery only.

Required-reading budget: `233177` local bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files: only the documentation paths in `package.md` under `Declared Write Set`.

Task: author and independently review the binding advisory-linter philosophy,
ADR-0043, target interface, manual fallback, current-to-target capability map,
migration/deletion design, friction criteria, and downstream package
decomposition.

Constraints: the linter is read-only and non-authoritative. It informs agents
but never executes validation, suggested, package-declared, workflow, remote,
or user-controlled commands; only the frozen literal read-only Git inspection
allowlist may run. It never owns evidence, changes lifecycle state, integrates
with CI, controls progress, or manages CAL/Harvard. Preserve underlying
correctness, science-contract, evidence, and protected-data obligations.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to three independent read-only reviewers for philosophy and
authority, operator/interface and failure behavior, and governance/science/
Harvard-boundary review; outputs are compact findings with exact paths and
recommended dispositions; write access is read-only. No heavy gate is
selected.

Autonomy: complete the planning package without further user direction unless
an irreconcilable authority conflict prevents an unambiguous design.

Outputs: update every required planning artifact, disposition all findings,
validate documentation, and record planning-only final disposition. Do not
create an implementation package directory.
