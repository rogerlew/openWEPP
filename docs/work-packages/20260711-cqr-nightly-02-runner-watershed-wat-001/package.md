# CQR Nightly Batch 01, Target 02 — Runner Watershed WAT

Package: `20260711-cqr-nightly-02-runner-watershed-wat-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `01`
Target module: `crates/openwepp-runner/src/watershed_wat.rs`
Target rank: `2` of `8`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible target function above CRAP `30` to `<=30`, or record a
reviewed ADR-0021 disposition. Preserve WAT discovery, parsing, aggregation,
typed errors, schema, units, and numeric output exactly.

## Reading And Scope

Core: root/crate/test/work-package guidance; this package/map; nightly ExecPlan;
mechanical/CQR guides; ADR-0021; prompt guide; target; and
`crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`.

Conditional: science-contract guidance and relevant `SC-*` authority if any
formula/publication behavior would be touched; such semantic edits are out of
scope. On-demand: adjacent runner/output modules.

Write set: target, focused test, package directory, and work-package catalog.
No science, threshold, schema, unit, error-contract, API, or output change.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner roles
for CQR metrics, behavior, gates, and hold/completion verification. Outputs are
compact verdicts, metrics, logs, and package artifacts; write access is
read-only unless a bounded write-set fix is explicitly assigned.

Subagent requirement: REQUIRED for full-workspace coverage/CRAP, clippy,
full-nextest, deny, comparator, or fixture batches. Do not run those heavy gates
on the parent model while the delegated role is available.

## Protocol And Exit

Commit scaffold before implementation. Cover first to ADR-0021 thresholds, then
decompose without changing ordering or numeric expressions. Re-measure, run
focused and delegated closure gates, complete dual review/verification, and
commit completion or local hold before target 03. Local holds roll only this
package's implementation/test edits back to scaffold and name the first
actionable follow-on; global holds stop the batch.
