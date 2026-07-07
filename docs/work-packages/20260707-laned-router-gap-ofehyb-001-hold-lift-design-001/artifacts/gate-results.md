# Gate Results

Status: **EXECUTED-COMPLETE**. Evidence mode: **Static + Ran**.

| Gate | Result | Evidence |
|---|---|---|
| Clean starting tree | PASS | `git status --short` returned no output before scaffold edits. |
| Required reading map | PASS | `artifacts/required-reading-map.md`. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/README.md --path docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001`: final run passed, 17 files validated, 0 errors, 0 warnings. |
| SC BEI/profile checks | PASS-DEFERRED | `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`: 7 BEI rows, 6 science-review-follow-on rows not yet consolidated; `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`: 4 BEI rows, 4 science-review-follow-on rows not yet consolidated. |
| SC unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` and `--path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`: no findings. |
| `cargo check` | PASS | `cargo check -p openwepp-hillslope-orchestrator`. |
| Focused source-memory tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator hybrid_source_memory --profile quick`: final run 4 passed. |
| Focused `ofe_routing` tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick`: final run 89 passed in `153.339 s`. |
| Case-4 hybrid ladder | PASS | Included in the final focused `ofe_routing` run and final full workspace run; earlier isolated run passed 1 test in `144.342 s`. |
| H2637 active hybrid timing/profile | PASS | Final release-binary run: `37.96 s` user, `0:37.99` wall, `980804` implicit steps, `151435969` map evaluations; see `verification-h2637-timing.md`. |
| Line count | PASS | `wc -l`: `cascade.rs` 1400, `dval.rs` 744, `d10b_reconciliation_tests.rs` 524. |
| Review | PASS | `review-code.md` and `review-qa.md`; all findings dispositioned. |
| Authority anti-evasion guard | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: passed. |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: 2 passed. |
| `git diff --check` | PASS | Final run passed. |
| `cargo fmt --check` | PASS | Final run passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final run passed. |
| `cargo nextest run --workspace --profile full` | PASS | Final run 1432 passed, 4 skipped in `585.981 s`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
