# Worker Handoff

Status: executed-hold
Evidence mode: Static + Ran

Summary:
- HPHYS0296 added snow/`RM` producer acceptance authority and a static
  contract-derived test.
- Full H1..H39 metrics remain semantically open (`0/39`), while `Q` remains
  closed (`39/39`).
- Six target windows classify as corrected-negative-melt candidates.
- H1/H7/H39 spring-2016 windows remain producer-magnitude/timing holds.
- No production kernel/runtime patch was made.

Run root:
- `/tmp/hphys0296_full_20260605T070000Z`

Important reports:
- `/tmp/hphys0296_full_20260605T070000Z/reports/hillslope_semantic_summary.md`
- `/tmp/hphys0296_full_20260605T070000Z/reports/hphys0296_snow_rm_acceptance.md`
- `/tmp/hphys0296_full_20260605T070000Z/reports/hphys0296_snow_rm_windows.json`
- `/tmp/hphys0296_full_20260605T070000Z/reports/hphys0296_first_divergence_rows.json`

Validation already run:
- `cargo fmt --check`
- `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
- `wctl doc-lint --path docs/work-packages/README.md`
- `wctl doc-lint --path docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001`

Next package:
- HPHYS0297 should focus on spring-2016 snow/winter producer magnitude/timing.
- Do not compensate in WB17, WB18, WB19, or WB13.
- The core question is why candidate spring-2016 `RM` and `Snow-Water` are lower
  than baseline when negative raw melt is immaterial and candidate publication
  identity is internally closed.
