# Owned File Manifest

Status: `reconciled through V4 shared-state runtime increment`

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
- V3/V4 potential and resource foundation:
  `crates/openwepp-vegetation/src/occupancy_solver/**`, radiation/column inputs,
  typed water/N request seams, independent keyed ledgers, and retained capped
  draft. The capped draft is not a public consumer and carries no Stage-B
  acceptance claim.
- V4 executable identity and configuration:
  `crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v4_definition.json`,
  `src/model.rs`, and `src/config.rs`.
- V4 shared-state schema and digest:
  `src/transaction.rs`, `src/transaction/state_shape.rs`, and
  `src/transaction/state_canonical.rs`; plus bounded state consumers in
  column/radiation/request/capped/N/diagnostic modules.
- V4 displayed-pool ownership: bounded changes in
  `src/carbon_nitrogen.rs` and typed `MaterialTransfer.owner_id` propagation to
  the default-off hillslope diagnostic.
- V3-to-V4 migration: `src/migration.rs`, including historical V3-only DTOs,
  strict validation, exhaustive typed reports, identity rebinding, and tests.
- V4 diagnostic fixtures and provenance:
  `tests/fixtures/c3_woody_v4_diagnostic_{configuration,state}.json`, their
  manifest/checksum, and bounded implementation-contract fixture identity
  updates.
- V4 implementation evidence: this package's lifecycle, required-reading,
  equation/state, gate, review, line-count, owned-file, and in-progress terminal
  reconciliation artifacts.

Protected implementation boundaries: production runner selectors, CLI defaults,
production outputs, canopy-snow, soil-transformation, deployment, and consumer
cutover paths. The canonical SC contract and V4 authority-package bytes changed
only in their separately authorized contract-first package; this implementation
package consumes them and does not claim authorship of that authority diff.
