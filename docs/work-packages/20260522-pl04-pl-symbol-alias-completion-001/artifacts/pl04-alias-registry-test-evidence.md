# PL04 Alias Registry Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL04 requires alias expansion, deterministic forward/reverse lookup coverage, and ambiguity guards.

Ran:
- Implemented PL04 registry rows and expanded integration tests.
- Executed required gate commands from `/home/workdir/openWEPP`.

## Test Coverage Added

- `canonical_wepp_registry_contains_pl04_schedule_growth_and_decomp_alias_entries`
- `reverse_lookup_resolves_pl04_aliases_to_single_canonical_symbol`
- `constructor_rejects_ambiguous_template_alias_strings`
- `reverse_lookup_reports_ambiguous_template_matches`

## Command Evidence

1. Targeted alias test pass:
- `cargo test --test sim_contract_symbol_alias_registry`
- Result: `11 passed; 0 failed`.

2. Required gates:
- `cargo fmt --check` -> `fail` (external PL03 formatting drift in `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`).
- `cargo clippy --workspace --all-targets -- -D warnings` -> `fail` (external PL03 clippy findings in `runtime_inputs.rs`).
- `cargo test --workspace` -> `pass`.
- `cargo deny check` -> `pass` (`license-not-encountered` warnings only).

3. PL04-owned hygiene checks:
- `cargo fmt --check -- crates/openwepp-sim-contract/src/symbols.rs tests/integration/sim_contract_symbol_alias_registry.rs` -> `pass`.
- `cargo clippy -p openwepp-sim-contract --all-targets -- -D warnings` -> `pass`.

## Outcome

- PL04 functional alias coverage and test expectations are closed.
- Workspace-level gate release remains blocked by concurrent PL03-owned formatting/lint drift.
