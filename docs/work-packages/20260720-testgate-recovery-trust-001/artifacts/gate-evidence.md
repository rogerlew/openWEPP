# Gate Evidence

Evidence class: Ran unless explicitly labeled Static.

## Focused implementation evidence

- `cargo check --locked -p openwepp-gate-planner --tests`: PASS.
- Focused Rust recovery, audit, combined-quality, and artifact seams: PASS. The final reviewer sample was 9/9 plus the combined artifact seam 1/1.
- `cargo nextest run --test testgate_align_authority_contract --test testgate_ci_executor_contract --profile quick`: PASS; retained evidence includes 15/15 integration cases and 5/5 CI-executor cases from the implementation loop.
- `.venv/bin/python -m unittest tests/python/test_testgate.py`: 17/17 PASS.
- `bash -n tools/ci/omarchy-runner/manage.sh`: PASS.
- `cargo fmt --all -- --check`, `git diff --check`, and targeted Markdown lint: PASS.
- Static: final Rust line counts remain below the hard limit; `planner.rs` and `executor.rs` are each 2,999 lines, and `verifier.rs` is 2,664 lines. The 2,000-line warning remains dispositioned as existing decomposition debt; no 3,000-line closure blocker exists.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS after correcting all package-owned findings from exact attempt 3.
- Disposable audit-reconstruction regression: PASS after both successful and injected-failure reconstruction; the audit root was removed and remained distinct from execution `.work/cargo-target`.
- Canonical policy/digest authority regression: PASS; `testing-and-gate-strategy.md` SHA-256 `bb69884b...` matches the policy bundle.

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
