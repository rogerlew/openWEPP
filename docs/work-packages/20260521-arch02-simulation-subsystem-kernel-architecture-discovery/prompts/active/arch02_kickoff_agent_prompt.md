# ARCH02 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/package.md


You are executing 20260521-arch02-simulation-subsystem-kernel-architecture-discovery.

Objectives:
1. Extract subsystem ownership and kernel requirement patterns from
   `wepp-forest` static analysis.
2. Assess `/workdir/rancor` simulation architecture and explicitly disposition
   transferability to Rust/openWEPP.
3. Survey Rust simulation architecture exemplars and best-practice patterns
   relevant to process-based environmental simulation.
4. Compare all three tracks in one evidence-tagged decision matrix.
5. Produce an actionable openWEPP subsystem/kernel architecture proposal with
   explicit ownership, dependency direction, and orchestration boundaries.
6. Queue follow-on implementation work-packages based on the proposal.
7. Run dual-agent review/disposition/verification gates.

Constraints:
- Evidence mode must be labeled per claim (`Static:` vs `Ran:`).
- Use `[DIRECT]` and `[INFERENCE]` tags per assertion.
- Preserve architecture-first/top-down science-contract posture from ADR-0011.
- Preserve canonical WEPP/wepp-forest symbol continuity in variable and state
  tables where applicable.
- Correctness over completion: unresolved high-severity architecture risks
  remain `HOLD`.

Required outputs:
- `artifacts/wepp-forest-subsystem-ownership-patterns.md`
- `artifacts/wepp-forest-kernel-requirements-extract.md`
- `artifacts/rancor-simulation-architecture-pattern-assessment.md`
- `artifacts/rust-simulation-architecture-exemplar-survey.md`
- `artifacts/cross-system-architecture-comparison-matrix.md`
- `artifacts/openwepp-simulation-architecture-requirements.md`
- `artifacts/openwepp-subsystem-and-kernel-ownership-proposal.md`
- `docs/architecture/simulation-subsystem-kernel-architecture.md`
- `artifacts/follow-on-architecture-implementation-wp-queue.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/arch02_disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
