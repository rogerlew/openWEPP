# Owned File Manifest

Status: `reconciled through Increment 2B authority hold checkpoint`

Evidence mode: `Static`

- Root registration and dependency lock: `Cargo.toml`, `Cargo.lock`.
- Typed protocol: `crates/openwepp-kernel-contract/Cargo.toml` and
  `crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs`.
- Vegetation implementation: `crates/openwepp-vegetation/**`.
- BGC arbitration/receivers: `crates/openwepp-biogeochemistry/src/lib.rs`.
- Default-off consumer: `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs`.
- Authority binding catalog: `tools/release/authority-policy/impact-map.json`.
- Authority and implementation tests: the two registered vegetation integration
  targets, retained V1 migration/oracle fixtures, and crate-local V2
  configuration/state/migration tests.
- Package lifecycle/evidence: this active package tree.
- Increment 2A: `crates/openwepp-vegetation/src/column.rs`, module registration,
  V2-named diagnostic configuration/state fixtures, and bounded fixture-identity
  tests in configuration/transaction modules.
- Increment 2B safe foundation:
  `crates/openwepp-vegetation/src/occupancy_solver/{mod.rs,resources.rs}` and
  module registration. No radiation or potential/capped solver file is present.
- Increment 2B hold evidence: this package's
  `potential-pass-hold-legitimacy-audit.md` plus bounded lifecycle, map, gate,
  and disposition updates.

Protected and unchanged: canonical SC contracts, digest-bound model definition,
production runner selectors, CLI defaults, production outputs, canopy-snow and
soil-transformation paths.
