# Required Reading Map

Evidence class: Static

Read before edits:

- `AGENTS.md`: root governance, work-package requirement, line-count gates,
  truthfulness, and Rust validation gates.
- `docs/work-packages/AGENTS.md`: work-package execution and artifact
  expectations.
- `docs/standards/mechanical-refactor-authoring-guide.md`: mechanical
  refactor package posture and closure requirements.
- `docs/specifications/science-contracts/AGENTS.md`: contract authority and
  no-surrogate/no-heuristic guardrails.
- `tests/AGENTS.md`: test-suite local playbook.

Scope interpretation:

- The requested target is an integration test file.
- No production kernel path or science-contract document is in the write set.
- Contract-first sequencing is satisfied by recording a no-op contract gate
  because this is behavior-preserving module extraction only.
