# Gate Results

Status: complete

Evidence mode: Ran

Ran:

Completed gates:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract authority | `cargo test --test hphys0320_stmtim_start_time_source_line_contract hphys0320_contract_authority_is_registered` | Passed: `1` test. |
| Focused runtime regression | `cargo test -p openwepp-hillslope-orchestrator hphys0320` | Passed: `2` tests. |
| Release CLI build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Passed. |
| H1..H39 release-binary batch | local loop over `p1_openwepp.run` through `p39_openwepp.run` | Passed: `39/39` hillslopes exited `0`. |

Final closure gates:

| Gate | Command | Result |
|---|---|---|
| Formatter | `cargo fmt --check` | Passed. |
| HPHYS0320 contract/artifact suite | `cargo test --test hphys0320_stmtim_start_time_source_line_contract` | Passed: `3` tests. |
| HPHYS0319 stale version assertion rerun | `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract` | Passed: `5` tests after updating expected canonical contract versions to `22`/`51`/`144`. |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Passed. |
| Workspace tests | `cargo test --workspace` | Passed on final rerun. |
| Dependency policy | `cargo deny check` | Passed with warnings for existing duplicate crate versions and unmatched license allowances; advisories, bans, licenses, and sources passed. |
| Scoped markdown lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0320-stmtim-start-time-source-line-closure-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | Passed: `28` files, `0` errors, `0` warnings. |
| Whitespace check | `git diff --check` | Passed. |

Note:

- The first `cargo test --workspace` run failed only because the existing
  HPHYS0319 contract test still asserted prior contract versions
  `21`/`50`/`143`; HPHYS0320 legitimately bumped those canonical contracts.
  The assertion was updated to `22`/`51`/`144`, the targeted HPHYS0319 test
  passed, and the final full workspace rerun passed.
- Post-Claude-review disposition accepted `Claude-F5` and amended
  `SC-CLIMATE-001#REF-CLIMATE-WF-WNTTIM-MIN` with the 1-based storm-hour
  window rationale. The HPHYS0320 contract/artifact suite and scoped
  markdown-doc lint were rerun after that amendment.
