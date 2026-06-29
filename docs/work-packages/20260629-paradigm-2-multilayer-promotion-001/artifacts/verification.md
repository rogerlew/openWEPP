# Verification

Status: `PASS`

Evidence class: Ran + Static.

This artifact records package, Rust, observed, performance, dependency, and
Markdown gates.

## Gate Matrix

| Gate | Result |
| --- | --- |
| Contract-first amendment | PASS: `SC-SNOWFREEZE-001` v112, `INV-SNOWFREEZE-082`, `OBL-SNOWFREEZE-P-057`. |
| Focused promotion tests | PASS: `cargo test --test paradigm2_multilayer_promotion -- --nocapture`. |
| Adjacent Stage 3 tests | PASS: `cargo test --test paradigm2_stage3_decouple_water_temperature -- --nocapture`; `cargo test --test paradigm2_stage3_liquid_routing_meltwater_temperature -- --nocapture`. |
| Unit registry/schema tests | PASS: WAT schema and output/boundary registry focused tests passed. |
| Observed guardrail | PASS: current default `15` / `179`; promoted opt-in `15` / `179`; `0` worse robust cells; runoff/timing `0` worse cells. |
| Supported output | PASS: observed candidate WAT files contain nullable `MeltwaterTemperature`; `27965` non-null values, all `0.0 degC`; default files have `0` non-null values. |
| H2637 perf | PASS: `70.65 s`, `1153680 KiB`, exit `0`; ADR-0025 `<=91.2 s` budget. |
| `cargo fmt --check` | PASS. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS. |
| `cargo test --workspace` | PASS. |
| `cargo deny check` | PASS: advisories, bans, licenses, sources OK. |
| Markdown lint | PASS: `markdown-doc lint ... --format json`, `16` files, `0` errors, `0` warnings. |
| Markdown validate | PASS: `markdown-doc validate ...`, `16` files, `0` errors. |
| Diff whitespace | PASS: `git diff --check`. |
| Authority anti-evasion | PASS: `bash tools/release/check_authority_suite_antievasion.sh`. |
| AUTH11 obligation guard | PASS: `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`. |

## Notes

The first observed-harness attempt used system Python and failed before scoring
because `pyarrow` was unavailable. The successful gate used `.venv/bin/python`.

The observed guardrail harness is still named for Stage 3-Decouple because the
physics arm is intentionally unchanged; this package promotes the same
snow-neutral arm and adds supported WAT output exposure.
