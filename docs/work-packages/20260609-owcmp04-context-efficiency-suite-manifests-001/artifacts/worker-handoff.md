# Worker Handoff

Evidence mode: Static

Status: complete.

What changed:

- Added `tools/owcmp/suite_manifest.py`.
- Added three `cohort-inventory` suite manifests under `tools/owcmp/suites/`.
- Added `owcmp manifest list`, `owcmp manifest show`, and `owcmp env`.
- Updated `tools/owcmp` docs/spec/local AGENTS, comparator-runner config,
  reusable prompt guidance, artifact retention policy, and focused tests.

How to use:

- Discover suites:

      tools/owcmp/owcmp manifest list --json

- Preflight a suite:

      tools/owcmp/owcmp env --manifest tools/owcmp/suites/<suite>.json --json

- Delegate heavy comparator work through `comparator_suite_runner` and return
  compact metrics plus artifact paths.

Follow-up:

- Future packages can promote a `cohort-inventory` manifest to an executable
  comparison manifest by adding a complete baseline/candidate argument list and
  focused tests for that lane.

