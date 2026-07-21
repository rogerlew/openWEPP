# Execute TESTGATE-RECOVERY-TRUST-01

Scope: local repository gate-workflow engineering; flat-file reads and edits
only. No external dispatch, runner deployment, push, or publication.

Execution mode: package-end-to-end.

Read `package.md`, `artifacts/required-reading-map.md`, root and applicable
nested `AGENTS.md`, `docs/standards/testing-and-gate-strategy.md`, and the
predecessor review findings before edits. Execute every package phase through
disposition. Treat the preserved working-tree prototype as untrusted input.

This prompt explicitly authorizes subagent spawning/delegation to two
independent read-only reviewers, two independent read-only terminal verifiers,
and the required `comparator_suite_runner` for all selected HEAVY gates. The
parent must not duplicate delegated HEAVY work. Outputs are compact findings,
metrics, timings, and artifact paths; no role may push, dispatch, or deploy.

Use the canonical pre-heavy audit. Fix every in-envelope defect rather than
holding while implementation remains possible. Preserve failed evidence and
never rerun an unchanged expensive gate for reassurance.
