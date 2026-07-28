# Execute CAL-04B Legacy Integration Removal

Status: `ARCHIVED / COMPLETE`

Executed roadmap Order 2 end-to-end under
`20260727-gate-planner-cal04b-legacy-integration-removal-001/package.md`.

CAL's planner/external-transaction dependency was replaced with direct package
execution, durable primary-failure capture, bounded publication, and a minimal
Harvard custody owner. All ADR-0043 Decision 10 invariants were preserved. No
CAL, Harvard, or model workflow ran, and Order 3 was not scaffolded.
