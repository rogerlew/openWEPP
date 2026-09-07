# Package author

Read docs/codex_exec_plans.md, docs/standards/prompt-wording-guidance.md and docs/standards/testing-and-gate-strategy.md before selecting acceptance/validation. Consult docs/work-packages/active.md for current work; historical catalog search only for identified dependencies. Kernel-affecting preparation additionally reads docs/standards/kernel-work-package-preparation.md and applicable authority references.

## Work-Package Authoring Requirements
- Use directory format `YYYYMMDD-<slug>-001` under `docs/work-packages/`.
- Add or update `docs/work-packages/README.md` so intent is discoverable.
- Scaffold `package.md`, `prompts/active/`, `prompts/archived/`, and `artifacts/` with queued placeholders.
- Encode status, objective, rationale, included/excluded scope, deliverables, dependencies, intended write set, phase plan, exit criteria, and security-impact gate.
- Encode exit criteria so each required gate is measurable inside the package or
  explicitly declared as a hold boundary before work starts. Do not author
  staged plans where an increment's required gate depends on a later increment's
  evidence while still allowing the earlier increment to close as complete.
- For conservation-sensitive output work, encode the Conservation /
  Publication Acceptance Rule as a current-scope gate unless the package is
  explicitly characterization-only.
- Encode explicit subagent authorization when package-required work depends on delegated reviewers, verifiers, comparator runners, or other role agents.
- Require dual reviews with finding disposition: `accepted`, `rejected`, `deferred`, or `follow-up`.
- Require `.rs` line-count governance: 2000+ lines is `WARN`; 3000+ nonexempt files require refactor before closure.
- Packages touching production numerical solvers must bind
  `docs/standards/numerical-solver-architecture.md`, inventory every
  production-reachable solver/regime, prohibit historical or recovery
  fallback chains, and require deletion of superseded production paths in the
  successor increment. A known inherited chain must be named as noncompliant
  quarantine with one removal owner and may not be expanded.


Freeze exact paths, acceptance and dependency sections/revisions before substantive edits. Commit scaffold when authorized. Do not duplicate procedures or history. Use common guide's current handoff and freshness contract.
