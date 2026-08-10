# Gate Results

Status: `PASS`

Evidence mode: `Ran`

Gate identity for production, authority, and test source:
`33831787b7029b28b0716c8458f08a11899db446`.

## Passed gates

- Owning orchestrator suite before terminal duration-guard addition: PASS,
  472/472. The added multi-scale guard vector passes focused 1/1 and is also
  included in the passing exact-head workspace gate.
- Peak authority contract: PASS, 4/4.
- H2637 routing-seam suite with ignored evidence included: PASS, 10/10.
- Real p61/p102 HBP and pass-Parquet consumers: PASS, 2/2.
- Retired-`ealpha` manifest fixture: PASS, 1/1.
- Census receipt/provenance suite: PASS, 6/6.
- Affected-crate warnings-denied Clippy: PASS.
- `cargo fmt --all -- --check`: PASS.
- Exact-head full-workspace regression: PASS, 2,346/2,346 tests with 33
  skipped, in 8,454.483 seconds. Run ID:
  `2a4b4f2c-d6c6-4bd6-a22f-e61bdb8f4576`.
- Exact-head workspace doctests: PASS; the workspace contains zero executable
  doctests.
- Source-level authority anti-evasion script: PASS.
- `auth11_required_suite_obligation_guards_contract`: PASS, 3/3.
- Documentation lint: PASS.
- `git diff --check`: PASS.
- Changed-Rust line-count governance: PASS; seven existing files remain in the
  2,000-line WARN band and none reaches the 3,000-line blocker.
- Fresh terminal-binary 1,088-trial Topanga cohort: PASS under v5 evidence.
  See `mutation-study.md`.
- Dual science, Rust correctness, and Rust QA reviews: PASS with no blocking
  findings at the gate identity above.

## Workspace scheduling record

The first default-concurrency quick run was not admitted: 163/2,296 tests ran,
161 passed, two assurance tests hit the unchanged 600-second limit, and 2,133
did not run. Log: `topanga-openwepp-census-full-v4-nextest-quick.log`.

The initial default-concurrency full run was intentionally interrupted after
192/2,345 tests because it reproduced the same host-contention trajectory; it
is preserved only as scheduling evidence in
`topanga-openwepp-census-full-v4-nextest-full.log`.

A four-thread quick retry advanced through the formerly timed-out assurance
case, but it was interrupted before completion after 90 passes and therefore
is not admitted. A four-thread full probe was likewise interrupted after 46
passes while selecting a lower-concurrency terminal schedule. These logs are
retained as non-admitted diagnostic evidence.

The first complete two-thread full run reached 2,345/2,345 and was not
admitted: 2,339 passed and six failed. Four failures were stale cached copies
of the file-reading peak contract, while focused fresh-target adjudication
passed all four. The other two exposed real consumer-test defects: EROD16 used
public `m3/s` where Wave-1 requires internal `m/s`, and H2637 let an unrelated
partial-frost guard preempt its routing seam. Commit `df41f3526` corrected both;
fresh focused consumer execution passed.

Later exact-head full attempts at `0d5fa08b2` and `ff7c91846` were deliberately
interrupted and are non-admitted because review found, respectively, stale
SC-SED peak authority and a malformed duration tolerance. Their commit-named
logs preserve the interruption receipts. SC-SED rev63 and the live named
seconds guard close both findings at the gate identity above.

The terminal full receipt used capped two-thread concurrency and a fresh
external target at exact source identity. It passed all 2,346 selected tests;
33 tests were skipped by their ordinary ignore posture. Log:
`topanga-openwepp-census-full-v4-nextest-full-clean-target-33831787b.log`.

The terminal inventories contain 2,346 full-profile tests and 2,297
quick-profile tests. The package-local identity diff reports
`quick_only_count=0` and `quick_subset_of_full=true`; every quick-selected test
is therefore contained in the admitted full receipt, which is reused for quick
correctness rather than rerunning the same cases. Exact-head workspace doctests
then passed independently in 45 seconds with zero executable doctests. Logs:
`full-vs-quick-summary.log` and `cargo-test-workspace-doc.log`.
