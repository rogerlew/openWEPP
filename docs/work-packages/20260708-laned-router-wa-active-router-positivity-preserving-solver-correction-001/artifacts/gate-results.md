# Gate Results

Status: EXECUTED-COMPLETE
Evidence mode: Ran.

## Gates Run

| Gate | Status | Evidence |
|---|---|---|
| Pre-correction WA reproduction | PASS_EXPECTED_FAIL | `run_mesh_ladder.py --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5 --expect-fail-guard laned_active_clamp_exceeds_source`; fixed10 failed day 1418, clamp/source `14.291141234409194`; `dx5` failed day 1167, clamp/source `11335.893753002358`. |
| Focused solver tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --lib final_tvd_scaling_preserves_positivity_and_total stage_flux_limiter_prevents_positive_clamp_injection` -> `2 passed`; earlier broad focused solver set with Case-1/Case-4/conservation/upstream vectors -> `5 passed`. |
| D10B / Case-4 focused tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --lib d10b case4 final_tvd_scaling_preserves_positivity_and_total stage_flux_limiter_prevents_positive_clamp_injection day_closure_enforces_cascade_and_identity_tolerances` -> `19 passed`. |
| Runner selector tests | PASS | `cargo nextest run -p openwepp-runner --lib mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx trace_selector_requires_explicit_one` -> `2 passed`. |
| Release runner build | PASS | Package-local WA harness built `cargo build --release -p openwepp-runner --bins`; final `openwepp-cli-hill` SHA256 `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`. |
| WA active fixed `10 cells/OFE` rerun | PASS | Final `run_mesh_ladder.py --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5`; fixed10 passed with total clamp `2.717124262301002e-13 m3`, max cascade rel `1.532462467225031e-14`, seam rel `3.0467009154883755e-14`, identity rel `5.933273356520313e-14`. |
| WA active `dx5` rerun | PASS | Same command; `dx5` passed with total clamp `7.305156020320419e-13 m3`, max cascade rel `4.705058001136025e-14`, seam rel `4.832475752036399e-14`, identity rel `5.933273356520313e-14`. |
| `cargo fmt --check` | PASS | Completed clean after the solver patch. |
| Contract/profile/BEI checks | PASS | `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` emitted `PASS-DEFERRED`, 8 BEI rows, 7 science-review-follow-on rows. Closure classification is `PASS` because the deferred rows are standing BEI consolidation posture, not current-scope blockers for this rev-41 solver correction. |
| SC unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS`. |
| Unit registry | PASS | `bash tools/release/check_unit_registry.sh` -> `21 passed`. |
| `git diff --check` | PASS | No output. |
| Markdown/doc lint for touched docs | PASS | Final `markdown-doc lint --path ...package --path SC-OFEROUTE-001.md --path docs/work-packages/README.md` -> `20 files validated, 0 errors, 0 warnings`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Completed clean. |
| `cargo nextest run --workspace --profile full` | PASS | Final tree: `1420 tests run: 1420 passed (4 slow), 3 skipped`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |

## Conditional Gates

Authority anti-evasion guards are not required because this package does not
change required-case bindings, cohort fixture posture, or external-authority
suite posture.

## Review/Verification

Independent review findings were dispositioned in `artifacts/disposition.md`.
Verification artifacts are recorded separately.
