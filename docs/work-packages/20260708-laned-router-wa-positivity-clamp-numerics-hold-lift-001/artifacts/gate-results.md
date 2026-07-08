# Gate Results

Status: EXECUTED-HOLD-SOLVER-CORRECTION-REQUIRED
Evidence mode: Ran.

## Gates Run

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | No output. |
| Focused active guard test | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --lib day_closure_enforces_cascade_and_identity_tolerances` -> `1 passed` after the final executor-order fix. |
| Runner active selector tests | PASS | `cargo nextest run -p openwepp-runner --lib mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx trace_selector_requires_explicit_one` -> `2 passed`. |
| Release runner build | PASS | Built by package-local `run_mesh_ladder.py`: `cargo build --release -p openwepp-runner --bins`; final binary SHA256 `2331d10073cc4c0428d12b8a717d6e934e5eff14ba5fff07e56daa4a2b236579`. |
| WA active fixed10 rerun | PASS_EXPECTED_FAIL | `run_mesh_ladder.py --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5 --expect-fail-guard laned_active_clamp_exceeds_source` -> `PASS_EXPECTED_FAIL`; `baseline_fixed10` fails closed at day 1418, clamp/source `14.291141234409194`. |
| WA affected rung rerun | PASS_EXPECTED_FAIL | Same command; `dx5` fails closed at day 1167, clamp/source `11335.893753002358`. |
| `git diff --check` | PASS | No output. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path ...package --path SC-OFEROUTE-001.md --path docs/work-packages/README.md` -> `16 files validated, 0 errors, 0 warnings`. |
| Contract/profile/BEI checks | PASS-DEFERRED | `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED`, 8 BEI rows, 7 science-review-follow-on rows. |
| SC unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS`. |
| Unit registry | PASS | `bash tools/release/check_unit_registry.sh` -> `21 passed`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Completed clean after final executor-order fix. |
| `cargo nextest run --workspace --profile full` | PASS | `1418 tests run: 1418 passed (4 slow), 3 skipped`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Independent review | PASS-WITH-DISPOSITION | `review-feynman.md` and `review-hooke.md`; all findings accepted/fixed. |
| Verification | PASS | `verification-agent-a.md`. |

## Conditional Gates

Authority anti-evasion guards are not required unless this package changes
required-case bindings, cohort fixture posture, or external-authority suite
posture. The package does not do so.
