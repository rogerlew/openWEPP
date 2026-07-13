# Medium Tranche Terminal Review And Verification A

Evidence class: **Ran + Static**

Status: `PASS`

Reviewed HEAD: `3475d41d06327ba65154f45de67645727d508251`

## Terminal Review

Verdict: **PASS**.

The thirteen module records are terminal `MODULE-PASS` records and name their
committed checkpoints. The checkpoint sequence is M-01 `3a4f7f45`, M-02
`3dcb28aa`, M-03 `ee54bdca`, M-04 `15921b1b`, M-05 `2b111ba7`, M-06
`6f690359`, M-07 `2d570697`, M-08 `b58bfb33` plus test-only lint follow-up
`43289099`, M-09 `4ba55de9`, M-10 `81efc1cd`, M-11 `d8bca733`, M-12
`d8069ee7`, and M-13 `69822725`. The documentation-only reconciliation is
`7d366395`; the two semantically neutral test-only lint fixes are `553647f0`.

The final identity diff removes exactly all 19 Medium start identities, adds
zero identities, and retains only the 13 unchanged Low-tranche identities. An
independent start/final set comparison reproduced 32 -> 13 rows, 19 removed,
zero added. A common-function comparison over every touched production file
found zero CRAP regressions. Current target-source hashes agree with the module
records; M-08's later test-only lint annotation is explicitly disclosed and
changes no production region or obligation.

All target and extracted/transitive helper floors remain at least 75% regions
and at most CRAP 30. The accepted science/glue slices meet their 90%/85%
thresholds, while mixed-purpose host-file visibility is retained in the
records. Consumer evidence is proportional and includes parser/runtime,
schema/migration, runner/manifest, release/lint, routing/science, and real
binary CLI paths. M-11 accurately distinguishes its public helper contract
from the separate executable CLI agreement path. No touched file reaches the
2,000-line warning threshold, and no unresolved review finding remains.

### Review Findings And Dispositions

| Finding | Disposition | Terminal evidence |
| --- | --- | --- |
| M-01/M-02 lacked durable focused artifact directories | `accepted-fixed` | The source-bound final LCOV/JSON/CRAP requery supersedes the missing directories, removes all five M-01/M-02 start identities, and independently reproduces their target/helper floors and CRAP values. Climate is 90.837% regions and 95.820% lines; groundwater coefficient is 86.087% regions and 87.855% lines. |
| Module records formerly contained pending checkpoint SHAs | `accepted-fixed` | `7d366395` reconciles all thirteen records to their actual checkpoint commits. No pending-checkpoint statement remains. |
| Instrumented run began with documentation-only record edits present | `accepted` | The deviation is explicit. Those edits changed no Rust/test coverage input and were committed separately as `7d366395`; production/test measurement source remained `69822725`. |
| First report-only attempts emitted empty reports | `accepted-fixed` | Both failed report shapes are archived. The corrected LCOV and JSON reports expanded the same 18 workspace packages and reused the same 311 `.profraw` files without rerunning tests or clearing the profile. |
| Four tests failed only in the parallel instrumented environment | `accepted-nonblocking` | Three are the unchanged H2637 process-environment selector family and one is the unchanged R3C process-global audit-counter assertion. No failure source was modified by Medium. The ordinary full profile passed all 1,930 tests, including these four. Intentional tamper-guard `FAILED` text is not a test failure. |
| Initial all-target Clippy found two test-only lints after measurement | `accepted-fixed` | `553647f0` changes only `to_owned` to `clone_into` in a characterization test and adds a narrow `too_many_lines` allowance to the ordered M-07 test. No production, schema, numeric, or test-selection behavior changes. The authoritative all-target rerun passes. |

No finding is deferred or left open.

## Independent Verification

Verdict: **PASS**.

Independent checks against the durable final evidence reproduced:

- primary hashes: LCOV `3c6037a7...9208`, JSON `469e0550...429c`, CRAP
  `08d34162...ac7b3`, production-over-30 `95019608...6bb93`, and identity diff
  `f94a7126...6f7`;
- nonempty corrected coverage authority with 109,878 instrumented lines,
  96,512 covered, and 87.835599% workspace line coverage;
- exact final census of 13 rows across 12 modules and an identity diff of 19
  removed, zero added, and 13 retained;
- zero common-function CRAP regressions in the thirteen touched production
  modules;
- M-01/M-02 final floors and CRAP values directly from final JSON/CRAP,
  resolving the preliminary focused-artifact concern;
- one instrumented execution followed by report-only LCOV and JSON correction
  from the retained profile; and
- source deltas after measurement limited to the two disclosed semantically
  neutral test-only lint fixes before ordinary gates.

The archived authoritative ordinary gates all have exit status zero:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run --workspace --profile quick` | PASS, 1,851/1,851 |
| `cargo nextest run --workspace --profile full` | PASS, 1,930/1,930 |
| `cargo deny check` | PASS |
| exact campaign `markdown-doc lint` | PASS, 65 files |
| `git diff --check` | PASS |

The final coverage/CRAP ratchet, ordinary gates, consumer evidence, line
governance, finding disposition, and gate non-deferral requirements are all
satisfied. Medium may transition to terminal disposition and activate the
Low/Assessment tranche.
