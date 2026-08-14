# Owned File Manifest

Status: `reconciled through V7 Milestone 6 implementation / terminal gates active`

Evidence mode: `Static`

- Root registration and dependency lock: `Cargo.toml`, `Cargo.lock`.
- Typed protocol: `crates/openwepp-kernel-contract/Cargo.toml` and
  `crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs`.
- Vegetation implementation: `crates/openwepp-vegetation/**`.
- BGC arbitration/receivers: `crates/openwepp-biogeochemistry/src/lib.rs`.
- Default-off consumer: `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs`.
- Authority binding catalog: `tools/release/authority-policy/impact-map.json`.
- Authority science-surface classifier:
  `tools/release/check_science_contract_admission.sh`.
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
- V5 read-only predecessor authority: `SC-VEGETATION-001` v9 and exact V5
  definition/vector/generator/review/gate/verifier artifacts released at
  commit `b7e6f08b655452c5c59a498ac9becd1439dd21ef`. This implementation
  package consumes but does not edit or claim authorship of those bytes.
- V5 implementation scope in progress: model registry/runtime identity,
  V4-to-V5 migration, capped coupled evaluator, exact fixture consumption,
  fixed-authorization/finalized-use protocol, focused tests, and this package's
  evidence artifacts. No completed solver/public-path claim is made by this
  manifest entry.
- V6 public water integration: `src/water_phase.rs`, bounded public-stage
  registration and transaction wiring, exact capped diagnostic operands, and
  `WaterArbiter` receiving-owner candidate construction in the default-off
  hillslope diagnostic. This surface is explicitly uncommittable and does not
  authorize E16--E22 or a partial owner commit.
- V7 Increment 4A vegetation candidate:
  `src/vegetation_candidate.rs`, `src/vegetation_ledger.rs`, bounded phase
  accessors in `src/persistent_phase.rs`, public-path containment wiring and
  focused tests in `src/transaction.rs`, plus module registration. The sealed
  type has no commit method; its public re-export exposes candidate inspection,
  not mutation or commit.
- Increment 4B BGC receiving-owner slice:
  `crates/openwepp-biogeochemistry/src/lib.rs` owns proposal-input validation,
  BGC-constructed receipts, exact mineral debit, receiver operands, ending
  state, and focused poisons. The default-off adapter update in
  `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` consumes
  that owner candidate.
- Increment 4B component-energy receiving-owner slice:
  `crates/openwepp-vegetation/src/energy_proposal.rs`, bounded accepted-result
  projection in `column.rs` and `occupancy_solver/evaluator.rs`, and
  `crates/openwepp-hillslope-orchestrator/src/vegetation_energy_owner.rs` own
  immutable physical proposals, independent occupancy/radiation/stand
  reconstruction, typed equilibrium-zero storage, receipts, and focused
  poisons. The default-off diagnostic aggregate seam is removed.
- Milestone 5 all-owner connection:
  `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` owns the
  sealed vegetation/water/BGC/energy envelope, exact cross-owner protocol and
  receipt validation, 27-point rollback/malformed-owner matrix, and one non-fallible whole-
  state replacement. `crates/openwepp-biogeochemistry/src/lib.rs` retains the
  exact nitrogen request/authorization/final-use protocol for cross-owner
  validation. `crates/openwepp-vegetation/src/transaction.rs` exposes the
  sealed candidate and contains no vegetation-only commit API.
- Milestone 6 resource-boundary remediation:
  `crates/openwepp-kernel-contract/src/lib_mod/resource_transaction.rs`
  centralizes deterministic compensated proportional arbitration by projected
  owner-supply identity while preserving the full request key;
  `crates/openwepp-vegetation/src/error.rs`, water/N boundary modules, the
  diagnostic, and BGC owner preserve the distinct canonical outer VEGTXN and
  inner BGC error families.
- Milestone 6 selection and performance evidence:
  `tests/integration/c3_vegetation_implementation_contract.rs` recursively
  proves the diagnostic has no production runner/direct-runtime selector;
  `artifacts/m6-benchmark-final-20260814-20260814004247/` retains the final
  corrected five-surface release benchmark matrix. The rejected first matrix
  and its incomplete invalid-filter metadata remain separately visible.

Protected implementation boundaries: production runner selectors, CLI defaults,
production outputs, canopy-snow, soil-transformation, deployment, and consumer
cutover paths. The canonical SC contract and V5 authority-package bytes changed
only in their separately authorized contract-first package; this implementation
package consumes them and does not claim authorship of that authority diff.
