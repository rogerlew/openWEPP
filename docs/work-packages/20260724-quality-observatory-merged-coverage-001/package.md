# Quality Observatory Merged Coverage

Package ID: `20260724-quality-observatory-merged-coverage-001`

Status: `QUEUED / ORDER-3`

## Objective

Produce one identity-bound coverage and adjudicated-CRAP report from sequential
`full` and `science-manual` Nextest profiles, proving manual science coverage
is represented rather than reported as false 0% debt.

## Included Scope

- Same-head, same-toolchain, same-feature, same-instrumented-build collection
  for both profiles.
- Separate profile inventories and LLVM profile inputs followed by deterministic
  merge.
- Global adjudicated CRAP from merged LCOV.
- Snowbench coverage regression fixtures.
- Canonical quality manifest, content-derived `quality_evidence_id`,
  provenance, validation, and compact outputs.

## Excluded Scope

- Moving science-manual tests back into routine TESTGATE.
- Individual exception/adjudication of the 18 snowbench rows from failed run
  `30113946779`.
- Production snowbench behavior changes.
- CQR refactoring.

## Declared Write Set

- `.config/nextest.toml`
- `Cargo.toml`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/quality_observatory_*`
- `tests/integration/testgate_*`
- `crates/openwepp-runner/tests/**`
- `docs/work-packages/20260724-quality-observatory-merged-coverage-001/**`
- `docs/work-packages/README.md`

## Dependencies

- Order 2 complete.

## Phase Plan

1. Freeze report identity and inventory/merge contracts.
2. Characterize the failed full-only snowbench observation.
3. Implement sequential profile collection into one confined instrumented root
   and deterministic coverage merge.
4. Run adjudicated CRAP from the merged LCOV and publish compact evidence.
5. Prove profile completeness, snowbench nonzero coverage, stale-input
   rejection, and exact report validation.
6. Reconcile, review, verify, and disposition.

## Exit Criteria

- Both profile inventories are independently enumerated and exactly match
  observed JUnit execution.
- An independently enumerated canonical nonignored workspace inventory is
  bound at the execution head. `full ∪ science-manual` equals that set and
  their intersection is empty unless a later versioned overlap policy
  explicitly replaces this rule. All three set digests and counts enter the
  canonical payload.
- Profile sets are combined by test identity without pretending they are one
  Nextest profile; duplicate execution is visible.
- Current characterization records 2,263 `full` and 36 `science-manual`
  nonignored tests, a disjoint intersection, and a 2,299-test union; final
  execution must recompute rather than assume those counts.
- Merge rejects different heads, toolchains, features, source manifests,
  coverage modes, or instrumented build identities.
- `artifacts/snowbench-full-only-row-ledger.json` reconstructs the exact 18
  snowbench rows from run `30113946779`. Every row is either nonzero in merged
  coverage with a `science-manual` contribution proof or explicitly proven
  legitimately uncovered and retained as real observational debt. No row may
  remain actionable solely because full-only coverage was used.
- Global CRAP consumes only the merged LCOV and reports raw, adjudicated, and
  actionable rows without reclassifying false full-only gaps.
- The canonical payload binds ordered profiles, per-profile inventory
  hashes/counts and JUnit hashes/results, union identity, source/toolchain,
  coverage/CRAP/registry/report digests, runner, workflow, run, and attempt;
  it excludes `quality_evidence_id` and any digest of itself or its containing
  envelope. `quality_evidence_id` is SHA-256 of canonical JSON payload bytes.
  An outer envelope stores the ID, payload, and publication metadata;
  verification reserializes the payload canonically and recomputes the ID.
- Compact adjudicated evidence preserves every raw, adjudicated, and actionable
  row with exact symbol identity, metrics, classification, and source/report
  hashes so downstream filtering and ranking are independently reproducible.
- Published files are limited to `quality-envelope.json`,
  `quality-payload.json`, `run-status.json`, three inventory-summary JSON files,
  two compact JUnit XML files, `adjudicated-crap-report.json`,
  `adjudicated-crap-report.md`, and `coverage-summary.json`. The indexed total
  must be at most 100 MiB. Raw LCOV, `.profraw`, target/build/reconstruction/
  temp trees, and caches remain local-only even when compressed.
- Compact report artifacts bind all inputs and pass independent verification.
- Focused tests, profile source guards, dual review, dual verification, and
  security-impact review pass.

## Security Impact

The merge is fail-closed on identity mismatch or missing profiles. It cannot
silently fall back to full-only coverage.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only measurement reviewers and two read-only terminal verifiers;
expected outputs are profile/inventory, provenance, and report-integrity
artifacts; write access is read-only. Any selected heavy collection must be
delegated to `comparator_suite_runner`.
