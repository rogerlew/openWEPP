# Symbol Alias Registry Implementation Evidence (SR04)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Extended `SymbolAliasRegistry` to support both exact aliases and template aliases for indexed runtime symbol families.
- Added template validation with typed erroring for malformed/unsupported alias templates.
- Expanded canonical registry rows to include SR02 slope and SR03 soil runtime symbol surfaces.
- Expanded integration tests to cover SR04 canonical rows, indexed reverse lookup, and template validation failure behavior.

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Code Changes

Primary implementation:
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`
  - added `InvalidBoundaryAliasTemplate` error: `:43`
  - added template alias storage and registry branching: `:147`, `:168`, `:197`
  - expanded `canonical_wepp_registry` SR04 rows: `:255` onward
  - reverse lookup template matching branch: `:346`
  - template parsing/matching helpers: `:390`, `:394`, `:435`

Primary tests:
- `/home/workdir/openWEPP/tests/integration/sim_contract_symbol_alias_registry.rs`
  - SR04 canonical-row coverage: `:22`
  - indexed alias reverse lookup assertions: `:61`, `:81`
  - invalid template-token rejection test: `:149`

## Behavioral Evidence

1. Exact alias behavior preserved:
- Existing ARCH03 baseline alias resolution tests continue to pass.

2. SR04 indexed family resolution added:
- `ofe2_xinput_0003 -> xinput`
- `slpinp_0002 -> slpinp`
- `ofe5_ssc_0002 -> ssc`

3. Strict template validation:
- Unsupported token aliases (e.g. `ofe{bad}_xinput_{idx4}`) fail construction with typed `InvalidBoundaryAliasTemplate`.

## Closure Notes

Static:
- SR04 closes alias continuity for slope/soil runtime seam surfaces without changing parser/runtime ownership boundaries.
- No silent fallback path was introduced; unmatched aliases remain typed `BoundaryAliasNotFound` failures.

Ran:
- Full workspace validation remained green with SR04 changes.
