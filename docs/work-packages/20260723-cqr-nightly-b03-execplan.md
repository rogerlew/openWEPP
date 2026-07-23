# CQR Nightly B03: Sequential Authority CRAP Closeout

Status: `EXECUTED-HOLD-REPOSITORY-ATTESTATION`

## Objective

Close the four actionable CRAP rows retained by TESTGATE receipt
`64a6f2926d41f2805f1f94fb83ad90d95940a3603e6f2ea8207b5f9bfe026b44`
using one behavior-preserving module package per source module.

## Frozen Baseline

- HEAD: `21ac2fdf2fe33e855ed440e7e9bb05554434e32e`
- CRAP report:
  `/home/workdir/testgate-recovery-trust-01-final-rtr044.IYxJPd/execution/.work/target/adjudicated-crap/adjudicated-crap-report.json`
- Ordinary Nextest: 2,290/2,290 PASS in 1,014.144 seconds.
- Instrumented Nextest: 2,290/2,290 PASS in 806.407 seconds.
- Actionable rows:
  - `main.rs::validate_package_chain_command`: CRAP 42, CC 6, coverage 0%.
  - `main.rs::plan_request`: CRAP 37.092125, CC 7, coverage 15%.
  - `main.rs::package_authority`: CRAP 56, CC 7, coverage 0%.
  - `package_validation.rs::validate_package`: CRAP 156, CC 12,
    coverage 0%.

## Modules

1. `20260723-cqr-nightly-b03s-1-main-001`: characterize and decompose the three
   `main.rs` command/request authority functions without changing CLI, typed
   error, persistence, or reconstruction behavior.
2. `20260723-cqr-nightly-b03s-2-package-validation-001`: characterize and
   decompose `validate_package` without changing Git evidence, reason-code,
   status, or artifact identity behavior.

## Execution

The first B03 aggregate was held before implementation because its immutable
heading did not satisfy canonical admission. B03R was likewise held because
its manifest omitted mandatory exact module-package paths. Commit fully
validator-shaped aggregate B03S first. Commit each corrected module scaffold separately,
retain canonical aggregate-admission PASS, then execute characterization,
behavior-preserving decomposition, focused metrics, dual review, and dual
verification. Do not rerun the failed exact-head global gates. After both
modules and the independent RTR-045 prerequisite close, delegate one new
changed-head `INCREMENT` TESTGATE qualification.

## Exit Criteria

- All four actionable functions and every extracted helper are CRAP at most 30.
- Focused behavior and output identity pass with no error-precedence drift.
- Each module package has aggregate admission, dual review, dual verification,
  and complete disposition evidence.
- One new exact-head recovery qualification passes without an unchanged retry.

## Progress

- [x] Fully validator-shaped B03S aggregate and both module scaffolds committed.
- [x] Both module aggregate-admission checks passed before implementation.
- [x] B03S-1 `main.rs` completed at `0ff8f340`; CRAP 2–4, 100% coverage,
  dual review and dual verification PASS.
- [x] B03S-2 `package_validation.rs` completed at `c85c1a15`; CRAP 4–5,
  100% coverage, dual review and dual verification PASS.
- [x] Independent observer prerequisite RTR-045 closed at `4181e914` with
  superseding ledger digest `b4ab096a...` and dual verification PASS.
- [x] Completed dual aggregate review and the sole changed-head recovery
  qualification: 15/15 PASS, zero retries, and zero actionable global CRAP
  rows at exact HEAD `eadc0145...`.
- [ ] Verify a repository-reviewed GitHub attestation envelope for exact
  receipt `c22fe3f...f06ca`; the local-untrusted receipt cannot close the
  increment boundary.
