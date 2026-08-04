# Owned File Manifest

Status: `scaffolded`

Evidence mode: `Static`

The authorized write set is the exact set in `package.md`. The orchestrator
alone may edit it. Reviewers and verifiers are read-only; the comparator runner
may write only ordinary build/test outputs under `target/`. Terminal evidence
must enumerate actual changed files and prove every other path unchanged.
