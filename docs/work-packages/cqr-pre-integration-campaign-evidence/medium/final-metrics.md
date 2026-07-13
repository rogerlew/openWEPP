# Medium Final Metrics

Evidence class: **Ran + Static**

Status: `PASS`

## Source And Protocol

- Repository: `/home/workdir/openWEPP`.
- Instrumented workspace run commit:
  `69822725a696d94a63d53fe36cdb74e4a02b95ad`.
- Ordinary-gate and closure commit:
  `553647f0b2577f1ab286f89e50e791bdf2a30b46`.
- Literal slug/phase: `medium` / `final`.
- The only Rust/test source delta between those commits is test-only and semantically
  neutral: `replacement.to_owned()` became `replacement.clone_into(...)` in
  `infile_climate_parser_contract.rs`, and the 112-line branch-matrix test in
  `cli03_runner_contract_derived_tests.rs` received a narrow
  `clippy::too_many_lines` allowance. No production source changed.
- The instrumented run began with thirteen documentation-only module-record
  checkpoint reconciliations in the worktree. Those edits changed no coverage
  input and were committed separately as `7d366395`; this is a recorded
  clean-worktree protocol deviation, not an assertion that the checkout was
  clean. The ordinary gates were source-bound to `553647f0`; their only
  untracked paths were the final evidence files being written by the runner.

The corrected one-run protocol retained one workspace profile and emitted both
formats without rerunning or clearing it:

```text
cargo llvm-cov clean --workspace
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-medium-final-run.time cargo llvm-cov --workspace --ignore-run-fail --no-report > /tmp/openwepp-cqr-preint-medium-final-run.log 2>&1
cargo llvm-cov report <one --package argument for each package in final/report-packages.txt> --lcov --output-path /tmp/openwepp-cqr-preint-medium-final.lcov
cargo llvm-cov report <the same package arguments> --json --output-path /tmp/openwepp-cqr-preint-medium-final.json
cargo crap --workspace --lcov /tmp/openwepp-cqr-preint-medium-final.lcov --min 0 --format json --output /tmp/openwepp-cqr-preint-medium-final-crap.json
```

A literal virtual-workspace `cargo llvm-cov report --lcov` and `--json` first
returned exit zero but emitted empty/zero-total reports. Adding `--workspace`
to the report subcommand is unsupported. The report-only correction therefore
expanded the 18 workspace members from `cargo metadata` into repeated
`--package` arguments; `final/report-packages.txt` preserves the exact list.
This correction reused the same 311 `.profraw` files and did not execute tests
again. The empty attempts and timings are retained as tooling evidence.

## Results And Integrity

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Instrumented workspace run | 0 | `36:56.65` | 830,812 KB |
| Corrected LCOV report | 0 | `0:03.27` | 163,772 KB |
| Corrected JSON report | 0 | `0:04.29` | 402,088 KB |
| CRAP report | 0 | `0:01.15` | 211,892 KB |

The coverage JSON contains 109,878 instrumented lines, 96,512 covered, for
87.835599% workspace line coverage.

| Durable artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `final/final.lcov` | 4,547,311 | `3c6037a7bcbec167e9c93ed4fded0e4b9dda1f975e6e8374811b0f9cf2fd9208` |
| `final/final.json` | 19,975,824 | `469e0550d9b2f11b03fd9f0a238f478f24c1d43e833e3b5acb44c118053429c9` |
| `final/final-crap.json` | 2,957,096 | `08d34162146f35fad8e1e09d5ff6c4a2074dea0377e8d8e97e9f2e5e103ac7b3` |
| `final/final-production-over30.json` | 2,656 | `9501960825bc75401c8bb98c2ccf353fe128f5f5baaee103a32737e00306bb93` |
| `final/final-identity-diff.json` | 2,902 | `f94a712601df698e32320c1d4979611bec58af96d93fd9b2b464553907adf6f7` |

Primary reports, run logs, report logs, time records, correction evidence, and
gate artifacts are archived below `medium/final/`. The final CRAP JSON is the
numeric CRAP authority.

## Instrumented-Failure Attribution

The ignored-run command returned zero by contract, but four tests failed only
under the parallel coverage environment:

- `h2637_active_fails_closed_without_routing_coefficients`;
- `h2637_active_and_disable_are_mutually_exclusive`;
- `h2637_active_and_shadow_are_mutually_exclusive`; and
- `r3c_lane_transfer_span_projects_multilane_topology`, whose process-global
  audit-count assertion observed four instead of one at
  `direct_runtime_r3c_r4b.rs:780`.

The first three are the unchanged shared-environment H2637 selector family
already present at Medium start. The fourth is the unchanged parallel
audit-counter family reproduced during M-10 diagnostics with other counts.
Neither source is part of a Medium production edit. The ordinary full profile
subsequently passed all 1,930 tests, including these tests, without a failure.
`boundary_case.json: FAILED` and
`compat_quoted_header_9002_policy_first.sol: FAILED` are intentional tamper-
guard subprocess text inside passing tests, not test failures.

## Census And Ratchet

The exact production filter fell from 32 rows across 25 modules at Medium
start to 13 rows across 12 modules at final: 19 identities removed, zero added,
and 13 retained. The 19 removed identities are exactly the complete Medium
fixed cohort across M-01 through M-13. Every fixed-cohort row is absent, no new
production identity surfaced, and no touched-module or downstream-consumer
regression appears.

This final workspace requery is authoritative for all 19 start
identities. It explicitly supersedes the missing M-01 and M-02 focused artifact
directories: their start identities, as well as every other Medium target, are
absent from the final mechanically filtered census. The retained 13 identities
are unchanged Low-tranche backlog and are preserved verbatim in
`final/final-identity-diff.json` and `final/final-production-over30.json`.

Disposition: the Medium final coverage and CRAP ratchets `PASS`.
