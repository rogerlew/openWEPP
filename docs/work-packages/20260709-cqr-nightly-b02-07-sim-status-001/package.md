# CQR Nightly Batch 02, Target 07 — Simulation Status

Package: `20260709-cqr-nightly-b02-07-sim-status-001`
Status: `SCAFFOLDED-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-sim-contract/src/status.rs`
Target rank: `7` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce eligible target CRAP above `30` behavior-preservingly. Preserve every
public status/boundary/clamp string, severity/classification mapping, typed
status construction guard, and error display.

## Scope and Write Set

In scope: target-local exhaustive mapping tests, private display/mapping
decomposition, and package artifacts. Out of scope: public names, status
semantics, failure/advisory policy, API, or contract changes.

Intended write set:

- `crates/openwepp-sim-contract/src/status.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-07-sim-status-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading and Gates

Read root/crate/work-package guidance, CQR ExecPlan/guides, ADR-0021, target
module/tests, and relevant status consumers. Commit this scaffold before source
or test edits. Add exhaustive real API characterisation first; preserve all
public strings and mappings; use delegated runner for heavy metrics/workspace
gates; complete dual review/verification and completion or local hold commit
before target 08.
