# Gate Results

Status: complete
Evidence mode: Ran

## Final Focused Gates

Ran:

- `cargo fmt --check`: pass.
- `cargo test -p openwepp-runner hphys0289_wb13_rm_publication -- --nocapture`: pass, `5 passed; 0 failed`.
- `cargo test --test hphys0289_wb13_rm_snowwater_publication_contract -- --nocapture`: pass, `2 passed; 0 failed`.

## Final Broad Gates

Ran:

- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with pre-existing duplicate/unmatched-license warnings; final line `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: pass, `2 passed; 0 failed`.

Final broad gate log: `/tmp/hphys0289_final_broad_gates_20260605T001506Z.log`.

## Full Semantic Suite

Ran:

- `python docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/hphys0289_diagnostics.py --run-root /tmp/hphys0289_full_release_current_20260605T000159Z`
  - Runtime: pass for H1..H39.
  - Semantic pass: `0/39`.
  - Summary: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_semantic_summary.md`.

## Earlier Broad Gate Baseline

Ran:

- Pre-review-fix broad gate log: `/tmp/hphys0289_broad_gates_20260605T000815Z.log`.
- That run passed but is superseded by final broad gate log after review fixes.
