# CQR Nightly B02: TESTGATE CRAP Closeout

Status: `ACTIVE`

## Objective

Close the two actionable CRAP rows retained by TESTGATE receipt
`cea13649c3ade694ec491147b347c5ed3868b2fd6f34caecccaab5be8c8cd5ce`
using one behavior-preserving module package per source module.

## Frozen Baseline

- HEAD: `24cf156ea7ebaa678c1ced21d41fd166f835394b`
- CRAP report: `/home/workdir/testgate-recovery-trust-01-final-increment.K3IEUu/execution/.work/target/adjudicated-crap/adjudicated-crap-report.json`
- Instrumented Nextest: 2,275/2,275 PASS in 804.063 seconds.
- Actionable rows:
  - `executor.rs::validate_affected_quality_scope`: CRAP 132, CC 11,
    coverage 0.
  - `verifier_coverage_tests.rs::replace_string`: CRAP 56, CC 7, coverage
    unavailable.

## Modules

1. `20260722-cqr-nightly-b02-1-executor-001`: characterize and decompose
   `validate_affected_quality_scope` without changing validation order, typed
   errors, or gate selection.
2. `20260722-cqr-nightly-b02-2-verifier-coverage-tests-001`: characterize and
   decompose `replace_string` without changing recursive traversal or mutation
   behavior.

## Execution

Commit the aggregate scaffold first. Commit each module scaffold separately,
retain canonical aggregate-admission PASS, then execute characterization,
behavior-preserving decomposition, focused metrics, dual review, and dual
verification. Heavy global evidence is reused from the failed qualification
until code changes; after both modules complete, delegate exactly one
changed-head `INCREMENT` TESTGATE qualification.

## Exit Criteria

- Both actionable functions and every extracted helper are CRAP at most 30.
- Focused behavior and output identity pass with no error-precedence drift.
- Each module package has aggregate admission, dual review, dual verification,
  and complete disposition evidence.
- One exact changed-head recovery qualification and dual terminal verification
  pass without an unchanged retry.
