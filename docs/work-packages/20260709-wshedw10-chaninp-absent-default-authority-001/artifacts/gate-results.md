# Gate Results

Status: `EXECUTED-COMPLETE`
Evidence: `Ran`

## Focused Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Parser/frame focused tests | PASS | Final run after review fixes: `cargo nextest run --test infile_chaninp_parser_contract --test wshedw5_typed_watershed_runtime_contract`: 31 tests run, 31 passed. |
| Initial inline CLI W10 test | PASS | Ran `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw10_watershed_cli_absent_chaninp_uses_typed_legacy_defaults`: 1 test run, 1 passed. |
| Standalone CLI W10 test, first attempt | FAIL then fixed | Ran `cargo nextest run -p openwepp-runner --test wshedw10_chaninp_absent_default_contract`: failed because the standalone fixture copied a two-row structure while only one hillslope block existed (`CLIWAT-E-007`/`STR-E-006`). Corrected the fixture to the one-hillslope channel-only structure used by existing output-guard tests. |
| Standalone CLI W10 test, final | PASS | Final run after review fixes: `cargo nextest run -p openwepp-runner --test wshedw10_chaninp_absent_default_contract`: 1 test run, 1 passed. |

## Closure Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo check --workspace` | PASS | Final run after review fixes: finished successfully. |
| `cargo fmt --check` | PASS | Final run after review fixes: no diff. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | First run failed on `clippy::float_cmp` in the new frame test; after tolerance assertions, reran and passed. Final rerun after review fixes passed. |
| `cargo nextest run --workspace --profile quick` | PASS | Final rerun after review fixes: 1399 tests run, 1399 passed, 26 skipped. |
| `cargo nextest run --workspace --profile full` | PASS | Final rerun after review fixes: 1474 tests run, 1474 passed, 3 skipped. |
| `cargo deny check` | PASS | Final run after review fixes: advisories, bans, licenses, sources all ok. |
| `markdown-doc lint --path ...` scoped package/docs/contracts/spec | PASS | Final run after review fixes on package, roadmap, work-package README, `chaninp` spec, `SC-INFILE-CHANINP-001`, and `SC-SYSTEM-001`: 12 files validated, 0 errors, 0 warnings. |
| `git diff --check` | PASS | Final run after review fixes and artifact updates: no whitespace errors. |

## Line Count Governance

Ran `wc -l` after moving the W10 CLI regression to a standalone file:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-input-contract/src/parsers/chaninp.rs` | 920 | PASS |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 890 | PASS |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2263 | WARN band; existing watershed CLI size, no new hard-blocking 3000+ production file. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 2971 | WARN band but below 3000 after moving W10 test out. |
| `crates/openwepp-runner/tests/wshedw10_chaninp_absent_default_contract.rs` | 231 | PASS |
| `tests/integration/infile_chaninp_parser_contract.rs` | 375 | PASS |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | 823 | PASS |

## Review-Response Gates

Kuhn's post-implementation review found that `NotApplicable` could have become
a hidden fallback if a mismatched `ChaninpFile` reached `WatershedNetworkFrame`.
The accepted correction adds an `ipeak` consistency guard and regression test:
`wshedw10_not_applicable_chaninp_cannot_mask_required_channel_sidecar`.

The same review noted flattened line provenance for CLI parser warnings. The
accepted correction formats line-bearing `ChaninpWarning` values as
`chan.inp CODE line N message`.
