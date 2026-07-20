# Prospective Gate Plan

Status: pre-implementation expectation; the authenticated planner owns final
selection from the exact diff.

## Cheap increment nodes

- Rustfmt and focused Clippy for the gate-planner crate.
- Planner unit tests, canonical/schema fixtures, local-helper Python tests, and
  TESTGATE integration/failure-injection tests.
- Workflow, package-schema, gate-policy-schema, documentation, prompt-state,
  diff-hygiene, and path-integrity checks.
- `.rs` line-count governance and applicable anti-evasion source guards.
- The new pre-heavy audit itself over the intended closure state.

## Heavy nodes

The terminal plan is expected to classify planner/executor/verifier, policy,
workflow, cache, and anti-evasion changes as critical. After `READY`, dispatch
the required heavy-run subagent for selected full workspace, global CRAP,
authority/anti-evasion, or release-equivalent contract nodes. Run each node
once. When combined full/coverage parity passes, one instrumented full Nextest
execution must feed both functional and LCOV/CRAP receipts. Otherwise the audit
must state the typed reason that separate executions remain necessary.

`LIGHT` and `HEAVY` are policy-owned gate-definition fields. The executor must
finish the light stage and bind its receipts into a `READY` audit before the
heavy stage exists as an executable transition. A recovery attempt imports
verified current, target-reusable per-node receipts and does not rerun an
eligible successful prefix. Ineligible receipts retain their exact §10.4
trust/reuse/context rejection reason.

## Reuse and invalidation

Evidence-only package and documentation edits after heavy execution invalidate
only bound documentation/path nodes. Production, planner, policy, schema, test,
workflow, dependency, toolchain, fixture, feature, environment, or inventory
changes invalidate the affected executable audit and receipts. Accepted review
fixes rerun only nodes whose bound roots changed.
