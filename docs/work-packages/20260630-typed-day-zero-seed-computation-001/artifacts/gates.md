# Gates

Evidence mode: Static/Ran.

| Gate | Status | Evidence |
|---|---|---|
| Package scaffold | PASS | Package files created. |
| Phase 1 typed sub-computations | PASS | Typed projections cover lane substeps, rainfall/hyetograph normalization, WB11 initial storage, fine-frost refresh, residue/`Ws`, WB12, ET-demand, `efflen`/`m`, WB16 default decision, static parsed inputs, full typed day input authority, and MOFE03/Wave-2. |
| Phase 1 production consumer cutover | PASS | Production direct and snowbench diagnostic construct `DirectProductionSeedAuthority::from_typed_inputs`; the five consumer groups read the typed carrier. No production caller reaches `from_day_zero_seed_surfaces`. |
| Phase 2 output identity | PASS | H2637 HBP/loss/PASS/WAT/plot byte-identical against clean `5b139058`; cli01 HBP/loss/WAT/plot byte-identical; focused multi-OFE/Wave-2 test passed. |
| Phase 3 symbol-map seed/runtime deletion | PENDING | Gate 1 checkpoint passed; deletion starts next. Transition/test-only surface-seed helpers remain and are marked for removal. |
| Phase 4 no-compatibility proof | PENDING | Requires Phase 3 deletion. H2637 and cli01 manifests already show `compatibility_edge_invocations=0`. |
| Perf / RSS re-measure | PASS | H2637 current `1:08.62`, `91692 KiB`; clean baseline `1:09.02`, `113268 KiB`. |
| Focused tests | PASS | `publication_wb11_seed publication_wb19_wb12_wb16`: `41` run, `41` passed; `cargo check -p openwepp-runner` passed. |
| Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` passed. |
| Format | PASS | `cargo fmt --check` passed. |
| Markdown lint/validate | PASS | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json`: `10` files scanned, `0` errors, `0` warnings; `markdown-doc validate ...`: `10` files, `0` errors. |
| Whitespace diff check | PASS | `git diff --check`: no findings. |
| Full nextest | PASS | `cargo nextest run --workspace --profile full`: `1879` passed, `1` skipped, `2` slow, `671.206s`. |
| Cargo deny | PASS | `cargo deny check`: advisories, bans, licenses, sources ok. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: pass. |
| Required-suite obligation guard | PASS | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: `2` passed. |
