# Gate Results

Status: `PASS`

Evidence mode: `Ran`

Gate identity for production, authority, and test source:
`33831787b7029b28b0716c8458f08a11899db446`.

Reopened ADR authority/test identity:
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`.

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
- Reopened ADR-0036 authority reconciliation: PASS. The amended decision and
  source guard are bound to `669269ee4`; both independent science reviews,
  Rust correctness review, and Rust QA returned PASS.
- Fresh 2026-08-10 focused replay from the terminal lifecycle worktree:
  `peak_hourly_authority_contract` PASS 4/4 (run ID
  `4a896cea-345d-4f8d-b066-3099f980b2b2`), `cargo fmt --all -- --check` PASS,
  ADR Markdown lint PASS, and `git diff --check` PASS.
- Exact-source reopened full workspace at `a8a96498`: PASS, 2,346/2,346,
  46 slow, 33 ordinary skips, 8,193.187 seconds, run ID
  `64cd5e97-d253-4da1-a3cf-3c4e16f83d22`. Log:
  `reopen-20260810T121200-full.log`.
  The detached worktree HEAD is
  `a8a96498ee909c4305fbc0a4db562b72e45efd2b`. The warm target provenance is
  direct: the earlier `reopen-20260810T093947-full.log` compiled that same
  `CARGO_TARGET_DIR=/home/workdir/openwepp-task-a8a96498-target2/full` from
  `/tmp/openwepp-clean-a8a96498.../crates/...`; the admitted retry reused that
  target from the same detached worktree. Its 0.35-second build line denotes a
  warm exact-source target, not a shared-`c9f28a7db` build.
- Quick disposition: the retained inventories contain 2,297 quick identities,
  2,346 full identities, and zero quick-only identities. The admitted reopened
  full receipt therefore executes every quick-selected test. Quick attempts
  remain non-admitted because of `/tmp` exhaustion, a missing detached-worktree
  `.venv`, and the profile's known 600-second assurance interruption.
- Reopened exact-source workspace doctests: PASS in 44 seconds with no
  executable doctests. `cargo deny check`: PASS. Format: PASS. Authority
  anti-evasion: PASS. Required-suite obligation guard: PASS 3/3. Peak authority:
  PASS 4/4.

## Reopened Evidence Reuse Disposition

The reopened delta changes ADR-0036 prose, assertions in the source-reading
authority integration guard, and package lifecycle/review evidence. It changes
no production Rust, canonical SC-* contract, serialization schema, frozen
Topanga input, or release binary. The exact-runtime 2,346/2,346 full-workspace
receipt, workspace doctest receipt, warnings-denied production Clippy receipts,
and complete 1,088-trial cohort therefore remain applicable at their recorded
identities. They are reused evidence, not newly executed evidence. The fresh
4/4 guard replay directly validates the only executable surface changed by the
reopen.

## Non-Admitted Reopened Attempts

`nextest-full-669269ee4.log` remains an interrupted 94/2,346 attempt. The
2026-08-10 retry series also preserves and rejects: a `/tmp` linker-space
failure, a detached-worktree missing-`.venv` failure, the known quick-profile
assurance timeout, a monolithic orchestration exit-137 kill, an operator-stopped
duplicate, and a late source-adjacent fixture write failure while `/tmp` was
full. None changed source, selection, assertions, or timeouts. After removing
only task-owned abandoned `/tmp` build scratch, the same full command passed.

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
