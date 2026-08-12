# Owned File Manifest

Status: `reconciled for EXECUTED-HOLD checkpoint`

Evidence mode: `Static`

- Root registration and dependency lock: `Cargo.toml`, `Cargo.lock`.
- Typed protocol: `crates/openwepp-kernel-contract/Cargo.toml` and
  `crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs`.
- Vegetation implementation: `crates/openwepp-vegetation/**`.
- BGC arbitration/receivers: `crates/openwepp-biogeochemistry/src/lib.rs`.
- Default-off consumer: `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs`.
- Authority binding catalog: `tools/release/authority-policy/impact-map.json`.
- Authority and implementation tests: the two registered vegetation integration
  targets and three `tests/fixtures/c3_woody_v1_*` fixtures.
- Package lifecycle/evidence: this active package tree.

Protected and unchanged: canonical SC contracts, digest-bound model definition,
production runner selectors, CLI defaults, production outputs, canopy-snow and
soil-transformation paths.
