# Gate Results

Status: `EXECUTED-COMPLETE-WSHED-W3`

| Gate | Result | Evidence |
| --- | --- | --- |
| Handoff prompt authored | `PASS` | `prompts/active/kickoff.md` |
| Package scaffold authored | `PASS` | `package.md` |
| Scaffold docs lint | `PASS` | `markdown-doc lint` over package Markdown plus `docs/ROADMAP.md` and `docs/work-packages/README.md`: `14 files validated, 0 errors, 0 warnings` |
| Scaffold diff whitespace | `PASS` | `git diff --check` over package and touched index docs |
| `--jobs N` worker pool implemented | `PASS` | `crates/openwepp-runner/src/watershed_supervisor.rs` adds `HillslopeWorkerPool`; public CLI handoff is `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`. |
| `--jobs 1`/`--jobs N` output identity proven | `PASS` | `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw3 -- --nocapture`: `3 passed`; synthetic generated 3-hillslope routed outputs are row-equivalent for `--jobs 1` and `--jobs 3`. |
| Fail-closed child/pass behavior proven | `PASS` | Same focused W3 test run proves child nonzero stops pending jobs and skips routing; missing generated passes after successful no-op children fail at pass inventory before routing. |
| Canonical scaling evidence recorded | `PASS` | `artifacts/scaling/carnivorous-adobo-release-scaling-summary.json`: release matrix passed `1/2/4/8/16/32`, three repeats each, all row-equivalent to `jobs1-rep1`; fixture `radly` clamp evidence is `artifacts/scaling/carnivorous-adobo-radly-clamp-manifest.json`. |
| Consumer-path proof recorded | `PASS` | `artifacts/consumer-path-evidence.md`. |
| Focused and final Rust gates run | `PASS` | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo nextest run --workspace --profile full` (`1283` passed, `1` skipped); `cargo deny check`; focused W3 and full watershed CLI behavior tests. |
| Dual review and verification dispositioned | `PASS` | `artifacts/review-disposition.md` and `artifacts/verification.md`. |
| Final disposition recorded | `PASS` | `artifacts/disposition.md`. |
