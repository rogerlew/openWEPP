# Gate Results

Status: executed
Evidence mode: Static + Ran

| Gate | Runner | Result | Evidence / log |
|---|---|---|---|
| `git diff --check` | Codex + reviewers | PASS | Silent pass. |
| Markdown lint | Codex | PASS | `markdown-doc lint --path docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001 --format plain` -> 28 files, 0 errors; SC contract -> 1 file, 0 errors; README/planning -> 2 files, 0 errors. |
| Contract/profile/BEI checks | Codex | PASS-DEFERRED / PASS | `check_sc_binding_exposure.py SC-OFEROUTE-001.md` -> PASS-DEFERRED with existing science-review-follow-on rows; `check_sc_unit_compliance.sh --path SC-OFEROUTE-001.md` -> PASS. |
| D-val Case-4 / resolution sweeps | Codex + comparator subagent | PASS-DIAGNOSTIC / HOLD | `case4-ko200-c120-s1-md05.log`, `case4-ko200-c240-s025-md025.log`, `case4-ko200-c480-s0125-md0125.log`, `d10-s0-case4-resolution-sweeps.json`; Case-1 negative guard `case1-resolution-control-rejection.log` -> expected exit 2. |
| H2637 resolution reproduction | comparator subagent | PASS-DIAGNOSTIC / HOLD | `d10-s0-h2637-shadow-evidence.json`; `cargo nextest run -p openwepp --test laned_shadow_h2637 h2637_executed_vector_shadow_on_off` |
| Focused Rust tests | Codex | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` -> 47 passed after rejected limiter trial was reverted |
| `cargo fmt --check` | Codex | PASS | Silent pass. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Codex | PASS | Finished dev profile with no warnings. |
| `cargo nextest run --workspace --profile full` | comparator subagent | PASS | `nextest-full-subagent-pass.log`: 1363 passed, 0 failed, 1 skipped; summary 596.517 s / real 597.57 s. |
| `cargo deny check` | Codex | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Anti-evasion guards, if triggered | Codex | NOT TRIGGERED | No D-val fixtures, required-case bindings, cohort fixtures, or authority-suite posture changed. |
