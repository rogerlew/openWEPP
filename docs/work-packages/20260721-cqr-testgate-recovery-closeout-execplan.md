# CQR ExecPlan: TESTGATE Recovery Closeout

Status: `EXECUTING` (4 of 7 module packages complete)

Campaign dependency: `TESTGATE-RECOVERY-TRUST-01` / `TESTGATE-CLOSURE-AUDIT-01`

Baseline evidence: Attempt 15 CRAP report at
`/home/workdir/testgate-recovery-trust-01-final15.CO6SU4/execution/.work/target/adjudicated-crap/adjudicated-crap-report.json`.

## Objective

Remove the 26 actionable CRAP rows that blocked the attempt-15 adjudication.
Each module is an independent, behavior-preserving CQR work package; no package
may edit a second production module without an approved package amendment.

## Package Queue

| Rank | Planned package ID | Sole production module | Actionable rows |
| --- | --- | --- | --- |
| 1 | `20260721-cqr-testgate-recovery-01-pre-heavy-001` | `crates/openwepp-gate-planner/src/pre_heavy.rs` | 13 |
| 2 | `20260721-cqr-testgate-recovery-02-main-001` | `crates/openwepp-gate-planner/src/main.rs` | 4 |
| 3 | `20260721-cqr-testgate-recovery-03-checkpoint-mirror-001` | `crates/openwepp-gate-planner/src/checkpoint_mirror.rs` | 2 |
| 4 | `20260721-cqr-testgate-recovery-04-resume-001` | `crates/openwepp-gate-planner/src/resume.rs` | 3 |
| 5 | `20260721-cqr-testgate-recovery-05-executor-001` | `crates/openwepp-gate-planner/src/executor.rs` | 2 |
| 6 | `20260721-cqr-testgate-recovery-06-verifier-001` | `crates/openwepp-gate-planner/src/verifier.rs` | 1 |
| 7 | `20260721-cqr-testgate-recovery-07-planner-001` | `crates/openwepp-gate-planner/src/planner.rs` | 1 |

Completed: rank 1, `pre_heavy.rs`, at package disposition head `f0a665ee`.
Final production coverage is 96.08% line / 89.64% region; all 111 functions
meet the 75% region floor and no CRAP row exceeds 17.

Completed: rank 2, `main.rs`. All four owned rows and all 17 extracted helpers
are at most 30; the final module maximum is 30. Dual review, dual verification,
and the 122-test affected crate inventory pass. Coverage remains an explicitly
reported below-threshold assessment; no module-test-enhancement closure is
claimed because no module characterization test was added or materially changed.

Completed: rank 3, `checkpoint_mirror.rs`. Both owned rows and all extracted
helpers are at most CRAP 5.024; production coverage is 96.95% line / 89.08%
region, and all 15 functions meet the 75% floor. Dual corrected review and dual
terminal verification pass.

Completed: rank 4, `resume.rs`. All three owned rows and every extracted helper
are below CRAP 30; the module maximum is 25.3961. Authoritative reviewed-
isolation coverage is 92.38% line / 85.50% region, all 29 production functions
meet the 75% floor, and both formerly colliding checkpoint tests pass in the
same instrumented process. Fresh dual review and dual terminal verification
pass. Earlier failed/staled measurements remain retained and were not reused.

## Execution Rules

Use `docs/work-packages/cqr-nightly-burndown-execplan.md` and the CQR package
template for every queue item. Before any production edit, scaffold and commit
that package with its exact retained CRAP row ledger, eligibility classification,
required-reading map, active prompt, and baseline provenance. Complete or hold
one package before opening the next implementation package.

Each package must preserve runtime behavior, public output, typed guards, and
error precedence. It must add characterization only when needed, reduce every
eligible owned row to CRAP `<= 30` or record an accepted symbol-level disposition,
obtain dual review and dual verification, and delegate selected heavy gates.

The attempt-15 report has 28 raw rows. Two rows are already adjudicated and are
not present in its authoritative 26-row `.actionable` array; they are preserved
as report evidence but are not CQR package targets. Do not re-run
`TESTGATE-RECOVERY-TRUST-01` until all seven packages have either
closed their eligible rows or produced an accepted hold that leaves no actionable
CRAP debt for the adjudicated gate.

## Return To Closeout

After CQR package completion, rebuild the release planner, generate a fresh
intent/terminal plan, and delegate exactly one changed-head TESTGATE attempt.
If that receipt passes, perform the two required terminal verifications and
complete `TESTGATE-CLOSURE-AUDIT-01` disposition.
