# Implementation Summary

Evidence class: Ran + Static.

Order 4 removes the tracked legacy planner/TESTGATE control plane:

- the entire `openwepp-gate-planner` crate and workspace dependency;
- the planner controller, qualification, comparison/intent resolvers, and
  Python tests;
- the TESTGATE workflow and legacy planner/assurance integration tests;
- obsolete Nextest planner profiles and repository-snapshot overrides; and
- `gate-policy/v1` schemas, fixtures, registry, and execution matrices.

Surviving direct owners were migrated before deletion:

- science-contract admission reads compact authority inputs under
  `tools/release/authority-policy`;
- the immutable generation-17 registry remains unchanged and is verified
  through Git object identity directly;
- the optional quality observer accepts one exact current source SHA and
  observes only its own workflow occupancy;
- forest1 quality data uses `/quality-history`, independent of historical
  TESTGATE storage; and
- the direct hosted rollback workflow is named
  `conservative-correctness.yml`.

No advisory-linter source or workflow changed. No CAL, model, Harvard, freeze,
or holdout command ran.

The ignored legacy crate-local test-output directory was moved intact to
`target/retired-gate-planner-fixtures-20260727`; it was not deleted and remains
recoverable as local build output.
