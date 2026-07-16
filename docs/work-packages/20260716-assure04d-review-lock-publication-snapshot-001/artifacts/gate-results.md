# ASSURE-04D Gate Results

Status: COMPLETE — independent heavy closure and dual terminal verification
PASS

Evidence class: Ran

## Phase 1 Contract-First Failure

Command:

```text
cargo nextest run --test assurance_v2_publication_contract --profile quick
```

Result: expected `FAIL` before production edits (exit 101). Rust could not find
`V2PublicationOptions`, `V2ReleaseIdentity`, `V2TrustDomain`,
`verify_v2_release_snapshot`, `V2Repository::publish_report`, or
`V2Repository::publish_test_fixture_report`. This proves the package began from
an absent publication API rather than a passing placeholder. The failure is not
a closure result and must be replaced by current passing evidence.

## Current Focused Closure

| Gate | Result |
| --- | --- |
| Formatting | PASS — `cargo fmt --check` |
| Workspace strict Clippy | PASS after the first heavy HOLD was remediated without suppressions — `cargo clippy --workspace --all-targets -- -D warnings` |
| Focused contracts | PASS — 69/69 across source, planner, assembly, publication, and retired-v1 build contracts; nextest run `84232ff0-2f48-45cb-b647-ab1aa5d49659` |
| Synthetic retained publication | PASS — 1/1 selected contract, 24 skipped; nextest run `261aad56-3785-48ae-96c1-1432a4fd8bbc`; regenerated public/snapshot/receipt tree compares byte-for-byte equal to `synthetic-publication/` |
| Workspace quick | PASS — 1,958/1,958, 34 skipped by profile, 3 slow; nextest run `d99c842d-8397-45bc-85d9-1d316ff0b4c3` |
| Release scripts | PASS — `bash -n` for all three touched release scripts; production contract executes preflight/materialization/reverification/discovery/checksums, and test-domain contract proves rejection before release-directory creation |
| Actual usersum renderer | PASS — retained README, report, and supplement rendered with the exact WEPPcloud `cmarkgfm` function; see `usersum-renderer-proof.md` |
| Documentation | PASS — terminal closeout `markdown-doc lint` and `validate` pass 30 authored changed files with zero errors/warnings; the retained binary-evidence tree is intentionally excluded; `git diff --check` passes |
| Protected surface | PASS at focused closure — all tracked `usersum/**` and retired-v1 public-transition hashes equal intake |
| Line count | PASS — no nonexempt production Rust file reaches 3,000 lines; `v2.rs` 2,984 and `publication.rs` 2,903 are WARN |

The first independent heavy sequence stopped at strict workspace Clippy and is
preserved in `heavy-gate-runner.md`. Its eight test-only diagnostics were
remediated by splitting tests/helpers and using allocation-free formatting;
no lint suppression or production edit was used. Full/deny and adjudicated
CRAP remain unrun until the complete independent sequence restarts. Both
independent reviewers renewed PASS on the bounded remediation; see
`review-agent-a.md` and `review-agent-b.md`. The remaining gates are not
inferred from these focused results.

## Second Heavy HOLD And CRAP Remediation

The second independent sequence passed formatting, strict workspace Clippy,
full Nextest (2,043/2,043; run
`6b01827d-b022-4fb8-8a91-50745687a779`), and dependency policy. Fresh
adjudicated CRAP then returned HOLD with 9 raw rows, 2 existing adjudications,
and 7 actionable rows, all in touched assurance files. Exact evidence and
chronology remain in `heavy-gate-runner.md` and
`validation-evidence/adjudicated-crap/`.

The seven rows were remediated structurally without adjudication or lint
suppression. Current evidence is:

| Gate | Current result |
| --- | --- |
| Formatting and workspace strict Clippy | PASS |
| Assurance library tests | PASS — 17/17; nextest run `523bbed0-57d0-45a6-8ee2-8fe176ac394e` |
| Focused contracts | PASS — 69/69; nextest run `69894019-829d-4e3e-8715-776f969f6387` |
| Workspace quick | PASS — 1,961/1,961, 34 skipped, 3 slow; nextest run `175ce3e5-e1d3-42e6-b39a-33ea13759fd6` |
| Retained synthetic publication | PASS — 1/1, 24 skipped; run `ad081d04-304c-4d7a-8ed0-e22712c7b3a2`; byte-identical to retained evidence |
| Focused fresh CRAP estimate | PASS — zero rows greater than 30; every touched-file maximum at or below 30; see `crap-remediation-evidence.md` |
| Protected/public bytes and release-script syntax | PASS — intake identities unchanged |

The focused CRAP estimate is not heavy closure. Both independent reviewers
returned PASS against the production refactor and independently reproduced
zero touched-file CRAP rows above 30 on focused coverage. A new complete
five-gate independent sequence remains required.

## Third Heavy Sequence — PASS

The independent runner preserved both earlier HOLDs and the complete
second-HOLD CRAP bundle, froze the remediated tree, and restarted all five
gates in order. Every gate passed:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| Workspace/all-target strict Clippy | PASS |
| Full workspace Nextest | PASS — 2,046/2,046, 3 skipped, 4 slow; run `9438c097-eccb-4959-88df-fb860cc64fdb` |
| `cargo deny check` | PASS — advisories, bans, licenses, and sources |
| Fresh adjudicated CRAP | PASS — 2 raw, 2 established adjudications, 0 actionable, 7 touched files |

Touched-file maxima are `cli.rs` 30, `v2.rs` 23, `assembly.rs` 25,
`confined.rs` 30, `lifecycle.rs` 23.9579, and `publication.rs` 26.5737;
`lib.rs` has no measured function row and no actionable row. The production
source manifest remained exact before/after/final at 228 files under SHA-256
`16e5bcb05297d5ca73ff1617242d019ee54063bf29a4dfa12b3f4c34fe30cf02`.
All canonical and historical bundle checksums, protected hashes, aggregate
`usersum`, write-set, line-count, script, diff, and index checks passed. See
`heavy-gate-runner.md` and the canonical adjudicated-CRAP report.
