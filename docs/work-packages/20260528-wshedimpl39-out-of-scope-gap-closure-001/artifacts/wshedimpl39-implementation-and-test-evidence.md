# WSHEDIMPL39 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime closure edits in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`:
  - introduced `inputs.applicability` runfile table parsing with explicit
    optional bool selectors,
  - added validator `validate_watershed_runfile_applicability(...)`,
  - enforced fail-closed typed errors (`CLIWAT-E-040`) for missing or
    disallowed selector declarations,
  - wired validation before runfile path-resolution/dispatch.
- Updated watershed runfile contract file to require
  `[inputs.applicability]` selectors and explicit `CLIWAT-E-040` semantics.
- Updated canonical contracts/index and WSHEDIMPL39 artifacts for closure
  traceability.

## Ran
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> pass
- `cargo test -p openwepp --test erod11_alias_boundary_ownership_contract --test erod12_cross_domain_contract_closure_contract` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings-only output; no advisory/bans/licenses/sources failures)
