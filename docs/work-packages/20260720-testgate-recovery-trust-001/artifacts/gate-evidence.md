# Gate Evidence

Evidence class: Ran unless explicitly labeled Static.

## Focused implementation evidence

- `cargo check --locked -p openwepp-gate-planner --tests`: PASS.
- Focused Rust recovery, audit, combined-quality, and artifact seams: PASS. The final reviewer sample was 9/9 plus the combined artifact seam 1/1.
- `cargo nextest run --test testgate_align_authority_contract --test testgate_ci_executor_contract --profile quick`: PASS; retained evidence includes 15/15 integration cases and 5/5 CI-executor cases from the implementation loop.
- `.venv/bin/python -m unittest tests/python/test_testgate.py`: 17/17 PASS.
- `bash -n tools/ci/omarchy-runner/manage.sh`: PASS.
- `cargo fmt --all -- --check`, `git diff --check`, and targeted Markdown lint: PASS.
- Static: final Rust line counts remain below the hard limit; `planner.rs` is 2,999 lines, `executor.rs` is 2,994 lines, and `verifier.rs` is 2,637 lines. The 2,000-line warning remains dispositioned as existing decomposition debt; no 3,000-line closure blocker exists.

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

Not run yet. The intended closure artifacts are being assembled before one exact terminal plan, its LIGHT stage, and the canonical pre-heavy audit. Selected HEAVY nodes will be delegated once to the authorized comparator runner.
