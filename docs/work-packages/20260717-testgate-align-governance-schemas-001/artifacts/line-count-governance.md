# Line-Count Governance

Evidence class: `Ran` and `Static`

Ran: the only touched Rust source file is
`tests/integration/testgate_align_authority_contract.rs`, at 725 lines. It is
below the 2,000-line warning threshold and the 3,000-line nonexempt closure
threshold.

Static: the repository census contains one pre-existing Rust file at or above
3,000 lines:
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
at 3,103 lines. TESTGATE-ALIGN-01 does not modify that file, does not add to its
responsibilities, and does not claim to close its separately governed
line-count debt. It is outside this package's declared write set and objective,
so it does not block this governance/schema-only package.

Static: other pre-existing Rust files at or above 2,000 lines remain warnings
under repository policy. No touched source file crosses either threshold.
