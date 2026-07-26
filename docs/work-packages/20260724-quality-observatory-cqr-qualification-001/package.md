# Quality Observatory And CQR End-To-End Qualification

Package ID: `20260724-quality-observatory-cqr-qualification-001`

Status: `COMPLETE`

## Pre-Execution Intent

Risk: `CRITICAL`.

The package will first run source, schema, inventory, report-contract, CQR
intake, occupancy, documentation, and authority-chain checks. Live execution
is limited to one QA attempt per corrected stable head. The qualified source
must have exact successful TESTGATE evidence; QA never overlaps current
TESTGATE work.

Initial static preflight found that the workflow requires `source_sha` to equal
both the workflow revision and current `main`, while this package requires the
Order-6 qualified subject
`955358449381ab38378d28dac93ba7b21b496d14` and permits later evidence-only
documentation commits without relabeling that subject. No QA dispatch is
authorized until this contradiction is corrected or an exact current subject
is freshly TESTGATE-qualified. Any workflow/tooling correction must be
committed, pushed, and pass exact-head TESTGATE before QA.

The terminal diff will be mechanically reconciled against the declared write
set. The package will retain every live attempt, run CQR selection-only intake
without collection, reject stale/tampered evidence, complete dual reviews and
verifications, then close the roadmap.

## Objective

After TESTGATE succeeds, qualify the optional QA observatory on forest1 and
prove CQR Nightly consumes that exact report without recollecting coverage.

## Included Scope

- Operator-authorized QA dispatch after TESTGATE is idle.
- Sequential `full` then `science-manual`, merged coverage, global adjudicated
  CRAP, compact artifacts, and observational status.
- Live-occupancy typed-deferral characterization without manufacturing a
  second concurrent heavy run.
- CQR selection-only intake using the exact QA evidence identity.
- Iterative correction of in-scope QA/CQR tooling defects.

## Excluded Scope

- TESTGATE redesign or a second reassurance dispatch.
- Science implementation or selected CQR module refactors.
- Canceling or waiting for defunct Omarchy records.

## Declared Write Set

- `.github/workflows/quality-observatory.yml`
- `.config/nextest.toml`
- `Cargo.toml`
- `gate-policy/v1/**`
- `crates/openwepp-gate-planner/**`
- `tools/local_ci/**`
- `tools/release/**`
- `crates/openwepp-runner/tests/**`
- `tests/integration/testgate_*`
- `tests/integration/quality_observatory_*`
- `tests/integration/cqr_*`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/20260724-quality-observatory-cqr-qualification-001/**`
- `docs/work-packages/20260724-cqr-testgate-coverage-reconstruction-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/testgate-quality-observatory-roadmap.md` (Order-7
  closeout routing only)
- `docs/ROADMAP.md`

## Dependencies

- Orders 5 and 6 complete on the current head lineage.
- Forest1 idle for QA; defunct Omarchy queue records are ignored.

## Execution Protocol

1. Run all cheap QA source/schema/inventory/artifact-contract gates.
2. Commit and push one stable current head if qualification corrections exist.
3. Freeze `qualification_subject_sha`, prove it has a passing Order-6 TESTGATE
   qualification, prove relevant TESTGATE idle, and dispatch the initial QA
   attempt against that exact SHA.
4. Verify ordered profiles, exact inventories/JUnits, one merged coverage
   identity, snowbench measurement, CRAP input digest, and compact artifacts.
5. Verify actionable debt does not fail execution integrity.
6. Characterize live-TESTGATE occupancy through retained provider state or an
   exact deterministic fixture; do not create concurrent heavy work.
7. Run CQR selection-only intake against the exact `quality_evidence_id` and
   prove the collector is not invoked.
8. If an in-scope defect appears, retain the attempt, correct it, commit/push a
   changed head, return to Order 6 for TESTGATE qualification of that exact
   head, then make one new QA attempt per correction. Never rerun unchanged
   failure; permit one infrastructure-only retry under canonical policy.
9. Reject a stale/tampered report fixture, then review, verify, disposition,
   close the roadmap, commit, and push.

## Exit Criteria

- `workflow_dispatch` is the sole QA trigger and forest1 performs heavy work.
- `full` completes before `science-manual` in one source-frozen instrumented
  root without an intervening clean.
- Both JUnit inventories equal independent inventory enumeration.
- The independently enumerated canonical nonignored workspace inventory equals
  the union of `full` and `science-manual`; their intersection is empty; all
  three set identities/counts match the canonical payload.
- The reconstructed 18-row ledger across
  `crates/openwepp-runner/src/hillslope/snowbench.rs`,
  `snowbench_coe_density.rs`, `snowbench_coe_melt.rs`, and
  `snowbench_physics_bulk.rs` is fully dispositioned: every row has nonzero
  merged coverage with `science-manual` contribution proof or is retained as
  legitimate uncovered debt, and none is actionable solely because of
  full-only collection.
- CRAP consumes the merged LCOV digest.
- A valid report with actionable debt is execution-successful with
  `debt_status=FAIL` and `closure_eligible=false`.
- Uploads match the exact Order-3 allowlist, exclude raw LCOV, targets,
  reconstruction roots, `.profraw`, caches, and temp trees even when
  compressed, and pass the indexed 100 MiB total ceiling before publication.
- QA report source head equals the final successful TESTGATE
  `qualification_subject_sha`. Later evidence-only documentation commits are
  recorded separately and do not relabel the qualified subject.
- Live forest1 TESTGATE occupancy yields typed successful deferral before
  acquisition; retired Omarchy records remain ignored.
- CQR consumes the exact QA report with no recollection; stale/tampered evidence
  fails closed.
- Dual result review, dual verification, security review, and roadmap/catalog
  closeout pass.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to `comparator_suite_runner` for heavy workflow monitoring/evidence collection,
two read-only result reviewers, and two read-only terminal verifiers; expected
outputs are compact metrics, log/artifact paths, and package artifacts; write
access is read-only. Only the parent may commit, push, or dispatch.
