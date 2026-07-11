# CQR Nightly Batch 01, Target 03 — Input Watershed Channel

Package: `20260711-cqr-nightly-03-input-watershed-channel-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `01`
Target module: `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
Target rank: `3` of `8`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible target function above CRAP `30` to `<=30`, or record a
reviewed ADR-0021 disposition. Preserve watershed-channel parsing, strict and
compatibility behavior, warnings, typed errors, field values, and ordering.

## Reading And Scope

Core: root/crate/test/work-package guidance; this package/map; nightly ExecPlan;
mechanical/CQR guides; ADR-0021; prompt guide; target; and
`tests/integration/infile_watershed_channel_parser_contract.rs`.

Conditional: science-contract guidance and relevant `SC-*` authority if a
contract-derived parser invariant would be changed; semantic changes are out of
scope. On-demand: adjacent input-contract parsers and fixtures.

Write set: target, focused integration test, package directory, and work-package
catalog. No science, threshold, tolerance, schema, error-contract, API, warning,
or accepted-input change.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner roles
for CQR metrics, behavior, gates, and hold/completion verification. Outputs are
compact verdicts, metrics, logs, and package artifacts; write access is read-only
unless a bounded write-set fix is explicitly assigned.

Subagent requirement: REQUIRED for full-workspace coverage/CRAP, clippy,
full-nextest, deny, comparator, or fixture batches. Do not run those heavy gates
on the parent model while the delegated role is available.

## Protocol And Exit

Commit scaffold before implementation. Cover first to ADR-0021 thresholds, then
decompose without changing parse order, branch order, or returned values.
Re-measure, run focused and delegated closure gates, complete dual
review/verification, and commit completion or local hold before target 04. Local
holds roll only this package's implementation/test edits back to scaffold and
name the first actionable follow-on; global holds stop the batch.
