# Line Count Governance

Status: executed-hold.
Evidence mode: Static.

## Requirement

Before closure, measure touched `.rs` files. Files at or above 2000 lines require
WARN disposition. Non-exempt files at or above 3000 lines require refactor or a
package-authorized blocker before completion.

## Current Disposition

PASS for executed-hold scope. No Rust files were touched under resumed R6
execution. Documentation files touched:

- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/**`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
