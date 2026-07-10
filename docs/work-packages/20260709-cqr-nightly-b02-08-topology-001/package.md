# CQR Nightly Batch 02, Target 08 — Topology

Package: `20260709-cqr-nightly-b02-08-topology-001`
Status: `SCAFFOLDED-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-topology/src/lib.rs`
Target rank: `8` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective and Scope

Close eligible target CRAP above 30 behavior-preservingly. Scope is private
topology fixture parsing/validation/error-display decomposition and
characterisation only. Preserve fixture grammar, node/reference validation,
cycle detection, message IDs, typed errors, graph identity, and fail-closed
pre-execution validation.

Write set: target module, its declared tests, package artifacts, and README
after closure. No public topology API, fixture grammar, routing behavior, or
validation policy changes.

## Required Reading and Gates

Read root/crate/work-package/test guidance, CQR/ADR docs, target/tests, and
topology consumer contract tests. Commit scaffold before edits; use test-first
characterisation, delegated heavy metrics/gates, dual review/verification, and
completion or hold commit before target 09.
