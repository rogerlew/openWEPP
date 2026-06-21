# R5E Kickoff Prompt

Execute `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/package.md`.

Required constraints:

- Prove exactly one canonical 14-phase entry per direct day/lane frame.
- Keep direct sub-operation counters separate from canonical phase entries.
- Preserve no-publication, no-default-activation, and no-R6 boundaries.
- Run focused tests, H2637 evidence, full Rust gates, scoped docs lint, review,
  verification, commit, and push.

Subagent authorization: this package explicitly authorizes read-only reviewer,
verifier, and benchmark runner subagents for the scopes declared in
`package.md`.
