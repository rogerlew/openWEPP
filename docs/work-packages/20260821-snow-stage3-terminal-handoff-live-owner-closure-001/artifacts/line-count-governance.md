Status: complete
Evidence mode: Ran

Ran: Nonexempt changed Rust files were counted after formatting. Maximum was
2,988 lines in `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`; the repository ceiling is 3,000. The previously over-ceiling V9 implementation and test module were split into
`v9_real_consumer_shadow_forcing.rs` and
`v9_real_consumer_shadow_tests_tail.rs`.

Ran: `nix develop --command cargo fmt --all -- --check` — PASS.
