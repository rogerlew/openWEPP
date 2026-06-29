# Verification

Status: `PASS-WITH-NON-PROMOTION`

This artifact records commands, exits, and gate outcomes.

Commands:

| Command | Exit | Evidence |
|---|---:|---|
| `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py` | `0` | Real gradient entry-gate run; generated `paradigm2-stage2-gradient-entry-gate.*`. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | `0` | Built the real direct-production runner used by the frost corpus. |
| `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py --mode frost --hill-binary target/debug/openwepp-cli-hill` | `0` | Real paired frost corpus run; `319.1788842184469 s`, gate failed non-promotion. |
| `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py --mode all --skip-model-runs --hill-binary target/debug/openwepp-cli-hill` | `0` | Refreshed package artifacts from completed real outputs after script edits. |
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py` | `0` | Python syntax check. |
| `cargo test --test paradigm2_stage2_snow_frost_insulation_profile` | `0` | Focused Stage 2 integration contract. |
| `cargo test -p openwepp-runner --lib r7g_direct_production_frost_uses_prior_snowpack_not_same_day_projection` | `0` | Prior-day snow/frost source guard after helper refactor. |
| `cargo fmt --check` | `0` | Rust formatting. |
| `cargo clippy --workspace --all-targets -- -D warnings` | `0` | Workspace lint. |
| `cargo test --workspace` | `0` | Full workspace tests; rerun clean after one source-guard update. |
| `cargo deny check` | `0` | advisories, bans, licenses, sources all clean. |
| `markdown-doc lint --path docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `0` | `14` files, `0` errors, `0` warnings. |
| `markdown-doc validate --path docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `0` | `14` files, `0` errors. |
| `bash tools/release/check_authority_suite_antievasion.sh` | `0` | Authority suite anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | `0` | Required-suite obligation guard. |

Gate outcomes:

- Contract-first amendment: pass.
- Gradient entry gate: pass.
- Frost observation primary gate: fail/non-promotion, `3` robust fails / `49`
  score for both arms, `0` robust improvements.
- Snow no-regression: pass-static.
- Conservation/domain: pass for implemented checks; Stage 2 fails closed when
  layer SWE/depth do not reconstruct runtime snow state.
- Performance promotion gate: not decisive because the primary frost gate
  failed first.
