# Gate Evidence

Evidence class: Ran unless explicitly labeled Static.

## Focused implementation evidence

- `cargo check --locked -p openwepp-gate-planner --tests`: PASS.
- Focused Rust recovery, audit, combined-quality, and artifact seams: PASS. The final reviewer sample was 9/9 plus the combined artifact seam 1/1.
- `cargo nextest run --test testgate_align_authority_contract --test testgate_ci_executor_contract --profile quick`: PASS; retained evidence includes 15/15 integration cases and 5/5 CI-executor cases from the implementation loop.
- `.venv/bin/python -m unittest tests/python/test_testgate.py`: 17/17 PASS.
- `bash -n tools/ci/omarchy-runner/manage.sh`: PASS.
- `cargo fmt --all -- --check`, `git diff --check`, and targeted Markdown lint: PASS.
- Static: final Rust line counts remain below the hard limit; `planner.rs` is 2,999 lines, `executor.rs` is 2,990 lines, and `verifier.rs` is 2,664 lines. The 2,000-line warning remains dispositioned as existing decomposition debt; no 3,000-line closure blocker exists.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS after correcting all package-owned findings from exact attempt 3.
- Disposable audit-reconstruction regression: PASS after both successful and injected-failure reconstruction; the audit root was removed and remained distinct from execution `.work/cargo-target`.
- Canonical policy/digest authority regression: PASS; `testing-and-gate-strategy.md` SHA-256 `72e53b54...` matches the policy bundle.

- RTR-015 focused qualification: the isolated per-process temp-root unit seam and executor environment seam pass; the three exact socket fixtures that failed in attempt 5 pass under a short temp root. Unix paths are constrained to 40 encoded bytes and are removed after success and failure.
- RTR-016 focused qualification: the exact four `assurance_v2_publication_contract` cases that timed out concurrently passed 4/4 in 267.751 seconds with the unchanged 720-second ceiling after reducing the publication group from two simultaneous cases to one. The derived execution config fails closed if the canonical group declaration drifts.
- RTR-017 focused qualification: `.venv/bin/python -m unittest tests.python.test_testgate` passes 21/21, including retained-report/cache-prune, symlink-escape, and finalizer-failure cases. Applying the finalizer boundary to attempt 5 removed only the 33 GB disposable Cargo target/reconstruction/temp trees, retained receipts/checkpoints/logs/JUnit reports, and produced a 60-file index over the resulting 2.8 MB root in 16.93 seconds.
- Current correction checks: `cargo check -p openwepp-gate-planner --tests`, focused Nextest seams, `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`, `cargo fmt --all -- --check`, and the 21-case Python target all PASS. `executor.rs` is 2,990 lines; no 3,000-line blocker exists.

- Review correction qualification: the derived Nextest configuration now binds the complete canonical publication schedule contract (group declaration, binary filter, group assignment, and `threads-required = 2`) and fails closed on drift in any field. The finalizer now gives its one authoritative pass a distinct typed error and never recursively repeats a failed finalization from the CLI error path. The Python target passes 21/21, including primary and secondary finalizer-failure reporting.
- The exact serial qualification JUnit is retained at `artifacts/rtr-016-serial-nextest-junit.xml`; its SHA-256 is `27527b4e...`. `artifacts/rtr-016-serial-qualification.md` binds the command, unchanged inventory/timeout, configuration digest, JUnit digest, and 267.751-second result.

- RTR-018 focused qualification: the pure selector and real temporary-Git-repository package-admission regressions both pass. The latter exercises the exact two-package shape, one/zero/multiple independently admitted candidates, invalid candidate path, missing package, missing schema, invalid base, and changed-path mismatch. Against the exact attempt-6 base and 42-path diff, the recovery package validates `READY` with zero unauthorized paths while the workflow-qualification package validates `INVALID / UNDECLARED_CHANGED_PATH` with 41 unauthorized paths. The corrected pre-heavy selector therefore has one independently admitted authority and preserves typed fail-closed behavior at the actual attempt-6 seam.
## Quick-gate defect and rerun audit

The same expensive verifier acceptance was invoked only after its source or accepted review disposition changed. No unchanged reassurance rerun occurred.

1. Dirty pre-commit broad quick feedback reached 89 passing cases before the acceptance rejected the non-exact checkout in about 71 seconds. This was iteration evidence, not an exact gate; implementation was committed before retry.
2. Exact broad quick at implementation commit `a12c4d3b` reached 89/94 and then timed out the acceptance at 720.033 seconds, canceling four cases. This opened the fixture-cost tooling defect.
3. Focused exact run after cache-only commit `f13da466` again timed out at 720.031 seconds. The changed fixture was tested once; the failure proved caching immutable construction alone did not remove the three repeated reconstructions.
4. Focused exact run after reconstruction-seam commit `02ba3e01` passed in 443.485 seconds. This timing result was retained, but reviewer A found that the test no longer entered public envelope reconstruction, so it was not accepted as final semantic closure.
5. The broad quick run at `02ba3e01` was interrupted when that review finding arrived. Its subject was obsolete; continuing would have wasted compute and could not produce admissible evidence.
6. Focused exact run after reviewer correction commit `edde1deb` passed through public `verify_receipt_envelope` in 399.504 seconds with the unchanged 720-second limit.
7. One fresh exact broad run at `edde1deb` passed 94/94 with zero failures or cancellations in 1,711.098 seconds. The corrected case took 390.124 seconds in-suite; truthful FAIL/BLOCKED verification took 559.319 seconds and also remained under the unchanged limit. This run will not be repeated for reassurance.

## Terminal evidence

Four exact attempts are retained; none was rerun unchanged.

1. `/home/workdir/testgate-recovery-trust-01.FFQVyI`: all six LIGHT nodes passed; the audit blocked before HEAVY and opened RTR-010 through RTR-012.
2. `/home/workdir/testgate-recovery-trust-01-final.9vt9qp`: all six LIGHT nodes passed and audit `5fafb85c...` was `READY`; HEAVY admission failed before node execution with `GATE-RESUME-PROVENANCE-PATH`, opening `AUTO-6ec4b6897533dd60`.
3. `/home/workdir/testgate-recovery-trust-01-final2.ALfL49`: all six LIGHT nodes passed and audit `6c0744ff...` was `READY`; 15 scheduler attempts produced 8 PASS, 4 FAIL, and 3 prerequisite BLOCKED. Three suites proved audit/execution cache contamination and Clippy exposed 18 package-owned warnings, opening RTR-013 and RTR-014. Wall times were 241,081 ms LIGHT and 287,391 ms HEAVY.

4. `/home/workdir/testgate-recovery-trust-01-final3.4vP6Es`: all six LIGHT nodes and audit `52da7670...` passed. Cargo deny, all three Canopy suites, and required authority passed, confirming RTR-013 end-to-end. Workspace Clippy alone failed on a 105-line package-owned root integration helper; doctest, full Nextest, and CRAP were prerequisite-blocked. The verified-form receipt `58af9ac8...` retained 11 PASS, 1 FAIL, 3 BLOCKED, zero retries/resumes, unchanged source, and 228,835/291,227 ms LIGHT/HEAVY time.

Attempt 4 reopened RTR-014 because package-scoped Clippy had omitted the root integration target. The chained OPEN record `775b0dbf...` is in the caller-selected canonical ledger `/home/workdir/testgate-history/recovery-trust-01-attempts.jsonl`; the attempt-local ledger copy remains an immutable finalization snapshot ending at attempt closure. The root-target Clippy command and all 5 owning integration cases pass after decomposing the long helper and correcting its stale `&ledger` source assertion. Both renewed reviewers returned PASS; no unchanged rerun is authorized.

5. `/home/workdir/testgate-recovery-trust-01-final4.3vbZS2`: all six LIGHT nodes and audit `17bd0021...` passed; HEAVY reached 13 PASS, 1 FAIL, and 1 prerequisite BLOCKED. Full Nextest ran 2,197/2,218 cases before the obsolete nonpass subject was cut off: 2,187 passed, three socket fixtures failed on `SUN_LEN`, four publication cases timed out at the unchanged 720-second ceiling, and three cases were terminated during drain. The executor had already sealed receipt `42907514...`, both stage lifecycles, and ATTEMPT-CLOSED with no source mutation. Finalizer traversal was stopped separately only after durable closure because it spent more than nine minutes hashing a 33 GB disposable target; the corrected pruning boundary retained the evidence and regenerated the index.

6. `/home/workdir/testgate-recovery-trust-01-final5.mbHSTh`: package audit `519885dd...` was `READY`, all six LIGHT nodes passed in 237,285 ms, and receipt `3cd013ab...` was sealed. Pre-heavy audit `7f7be60e...` then returned `INVALID / GATE-AUDIT-PACKAGE-AMBIGUOUS`; no HEAVY lifecycle or node started. The exact subject was clean `1e09babc...`, the binary was `69afb24e...`, and the 29-file retained root is 860 KB. The mismatch opened RTR-018 rather than authorizing an unchanged retry.

7. `/home/workdir/testgate-recovery-trust-01-final-rtr044.IYxJPd`: exact HEAD
   `21ac2fdf` passed LIGHT 6/6 and pre-HEAVY 10/10 READY. HEAVY sealed receipt
   `64a6f292...26b44` with 14 PASS, 1 FAIL, and zero retries. Ordinary Nextest
   passed 2,290/2,290 in 1,014.144 seconds; instrumented Nextest passed
   2,290/2,290 in 806.407 seconds. CRAP adjudication exposed four actionable
   rows across `main.rs` and `package_validation.rs`. Source mutation passed.
   Post-seal observer defect RTR-045 was preserved separately; the attempt and
   ledger remained authoritative.

8. `/home/workdir/testgate-recovery-trust-01-final-b03s.oJ1TCz`: exact HEAD
   `eadc01459df18e83d94362dc225219232f0a4c65` passed LIGHT 6/6 and READY audit
   `35729c88...009b8` 10/10. Receipt `c22fe3f...f06ca` sealed all 15 nodes PASS
   with zero retries and unchanged source. Ordinary and instrumented Nextest
   passed 2,293/2,293 in 1,026.563 and 795.488 seconds. Fresh global CRAP was
   closure eligible with zero actionable rows, two valid adjudications, and no
   invalid adjudication. The durable 151-record chain closed PASS at
   `2096272c...b067b`. The unsigned local receipt remains
   `LOCAL_UNTRUSTED`; a repository-reviewed GitHub attestation is still needed
   to close the `INCREMENT` boundary. The absolute-path pre-receipt rejection
   in sibling root `...Hs7tZ9` ran zero nodes and did not mutate the ledger.

9. GitHub run `29978778150` at pushed head `ba6c1e1d...` admitted the exact
   trusted main comparison, verified the pinned toolchain, bootstrapped
   dependencies, and built the planner. It then failed before TESTGATE
   execution at `Restore and verify newest durable attempt history`.
   `Execute content-verifiable increment gates` was skipped. Live read-only
   inspection proved runner `forest1-openwepp-01` used the reviewed image but
   mounted only `/runner-state`; named volume `openwepp-testgate-history` did
   not exist. RTR-046 records the unactivated runner configuration. No gate
   node, receipt, or unchanged expensive rerun occurred.

10. `/home/workdir/testgate-recovery-trust-01-final-rtr060.D7NH60`: exact HEAD
    `b114ecf50a091cc6e9fafa480d09e647149ed3b6` passed LIGHT 6/6 and pre-HEAVY
    audit `e4350142...` 10/10. Receipt `7b3c199d...` sealed all 15 nodes PASS
    with zero retries. Ordinary and instrumented Nextest each passed
    2,304/2,304 in 994.409 and 817.610 seconds. Fresh global CRAP was closure
    eligible with two valid adjudications, zero invalid adjudications, and
    zero actionable rows. Source mutation passed. Dual terminal verifiers
    independently verified the receipt, envelope, 2,322-entry inventory,
    79-file retained index, package authority, and durable ledger without
    rerunning a gate.
