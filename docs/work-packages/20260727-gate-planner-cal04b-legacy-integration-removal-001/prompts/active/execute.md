# Execute CAL-04B Legacy Integration Removal

Execute roadmap Order 2 end-to-end under
`20260727-gate-planner-cal04b-legacy-integration-removal-001/package.md`.

Replace CAL's planner/external-transaction dependency with direct package
execution, durable primary-failure capture, bounded publication, and a minimal
Harvard custody owner. Preserve all ADR-0043 Decision 10 invariants. Do not run
CAL, open Harvard, change science/model behavior, or scaffold Order 3.

Subagent authorization: this prompt explicitly authorizes spawning/delegating
to two independent read-only reviewers and two independent read-only verifiers
for the scopes and outputs in `package.md`.
