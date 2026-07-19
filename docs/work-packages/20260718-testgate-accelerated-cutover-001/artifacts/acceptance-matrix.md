# TESTGATE Accelerated Cutover Acceptance

Ran/static evidence: candidate
`7ccc61d5e405529789417f87130978f63679ded5`, 2026-07-19 PDT.

## Exact-Candidate Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Rustfmt | PASS | `cargo fmt --check`, exit 0. |
| Clippy | PASS | Workspace/all targets with warnings denied, exit 0. |
| Full Nextest | PASS | Coverage-instrumented full profile: 2,165/2,165 passed, 5 skipped. |
| Dependency policy | PASS | `cargo deny check`: advisories, bans, licenses, and sources all `ok`. |
| Global CRAP | PASS | 2 raw / 2 adjudicated / 0 actionable; current-source closure eligible. |
| Source identity | PASS | Clean exact pushed SHA and identical 249-source manifests before/report/final. |

The global CRAP acquisition supplied the exact-candidate full Nextest evidence;
the earlier successful 2,163-test full command was not repeated for
presentation.

## Event-Driven Matrix

| Case | Result | Bound evidence |
| --- | --- | --- |
| Ordinary documentation | PASS | `ordinary_documentation_is_editorial_and_mapped` passed in the exact-candidate full run. |
| Bounded component and affected quality | PASS | `component_prefix_and_git_style_glob_are_bounded`, `ordinary_rust_plan_builds_unique_per_package_outputs`, and the affected-quality integration contract passed. |
| Integrated and critical impact | PASS | Gate-policy and science-surface critical-selection tests passed; affected CRAP binds bounded/integrated risk classes. |
| Unknown-impact escalation | PASS | Normative-unmapped documentation and repository-wide assurance unknown-impact tests passed fail-closed. |
| FAIL/BLOCKED receipts | PASS | Executor and independent-verifier non-PASS receipt tests passed, including observed attempts and failure-visible CLI handling. |
| Cold writable surfaces | PASS | Empty Cargo/target/work surfaces fetched the locked base/head graph, then completed 2,165 tests and global CRAP offline inventories. |
| Source/output confinement | PASS | Exact source manifests stayed identical; live root/state writes failed; output remained on bounded external tmpfs; cleanup emptied all writable surfaces. |
| Untrusted pull requests | PASS | The only forest1 workflow admits push-to-main and manual dispatch; contract tests reject both pull-request trigger forms, credentials persistence, host binds, and a Docker socket. |
| Trusted main routing | HOLD | Workflow labels exactly match unique online/idle provider ID 23, but the real normal workflow consumer must pass on the docs-only activation push. |
| Conservative rollback | PASS | Hosted run 29692305394 passed exact-main admission and uploaded a non-qualifying smoke receipt; all six broad/reuse steps skipped. The reviewed reuse predicate accepts canonical fully adjudicated closure. |

## Cost And Cutover Decision

The operator accepted the measured 48.8% projected savings. No elapsed-time,
increment-count, 50%, or duplicate-environment gate remains. The accepted
candidate requires immediate provider activation and one docs-only
trusted-main consumer run. `release-gates` remains manually disabled; no broad
command is repeated.
