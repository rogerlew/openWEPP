# Owned File Manifest

## Historical Retained Checkpoint — 2026-08-14

The retained checkpoint owns:

- workspace registration in `Cargo.toml` and `Cargo.lock`;
- `crates/openwepp-land-surface-energy/**`;
- the default-off orchestrator dependency, export and
  `land_surface_energy_shadow/**`;
- narrowly scoped read-only accessors in
  `vegetation_real_hydrology_shadow.rs`;
- `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` and
  its root test registration; and
- this Child-3 package lifecycle/evidence tree.

No runner, production selector, default, publication or production scheduler
call site is in the diff.

Resumed ownership additionally includes the typed root-owner addition in
`transaction.rs`, `land_surface_energy_shadow/covered_forest.rs`, the shadow
module exports and `covered_forest_tests.rs`.

Current line-count evidence: `solver.rs` 2,802; `transaction.rs` 1,674;
`physics.rs` 665; `closure.rs` 484; orchestrator shadow `mod.rs` 2,943;
`covered_forest.rs` 158; integration root 2,757; covered tests 677. The two
2,000+ production/test files remain WARN with active decomposition through
submodules. Every file remains below the 3,000-line closure threshold.

## Active Review-Remediation Write Set — 2026-08-15

Evidence class: `Static`

Snapshot: `2026-08-15T19:31:28-07:00`, committed HEAD
`dfc7cf971284d772246f147382f4bb8a2292ee4c`, with the live worktree included.
The resumed-delta baseline is the terminal dependency-lift commit
`a7d692da4`. The historical checkpoint inventory and counts above remain dated
evidence; they are not current terminal counts.

The exact active Child-3 ownership envelope is:

- workspace membership and dependency binding in `Cargo.toml`, `Cargo.lock`,
  `crates/openwepp-land-surface-energy/Cargo.toml`, and the narrowly required
  orchestrator/vegetation crate manifests;
- the complete dependency-light runtime under
  `crates/openwepp-land-surface-energy/**`, including the resumed
  `covered_liquid.rs` and `covered_output.rs` modules;
- the default-off adapter under
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/**`,
  including `covered_forest.rs`, `covered_v8_owner.rs`,
  `covered_v8_transaction.rs`, `v8_projection.rs`, and the active remediation
  modules `covered_derived_ingress.rs`, `v8_input_projection.rs`, and
  `v8_rollback.rs`;
- the retained narrow adapter/export surfaces in
  `crates/openwepp-hillslope-orchestrator/src/vegetation_real_hydrology_shadow.rs`
  and `crates/openwepp-hillslope-orchestrator/src/lib.rs`;
- the V8 definition projection and uncommitted-owner additions in
  `crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v8_definition.json`,
  `crates/openwepp-vegetation/Cargo.toml`, and
  `crates/openwepp-vegetation/src/{carbon_phase.rs,config.rs,lib.rs,persistent_phase.rs,transaction.rs,vegetation_candidate.rs,v8_candidate.rs,v8_persistent.rs,v8_state.rs}`
  plus `crates/openwepp-vegetation/src/transaction/state_canonical.rs`;
- `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`
  and its `land_surface_energy_real_hydrology_shadow_contract/**` submodules;
  and
- this Child-3 package tree, `docs/ROADMAP.md`, the work-package catalog, and
  the three Child-3 rows in the campaign package's package/stage/release
  lifecycle artifacts.

The package owns no production runner selection, default, scheduler call site,
output publication, state commit API, deployment, or consumer cutover.
Surface-liquid custody implementation and evidence from the closed dependency
package remain dependency-owned even where they share an orchestrator module
directory; they are not reassigned to Child 3 by this manifest.

### Current Line-Count Governance

Every Rust/test file touched by either the retained checkpoint or resumed
Child-3 delta and currently at or above 2,000 lines is listed exactly:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-land-surface-energy/src/solver.rs` | 3,204 | `BLOCKING`: above the nonexempt 3,000-line closure threshold; decompose before closure |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs` | 2,964 | `WARN`: decomposition modules are active; remain below 3,000 |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` | 2,757 | `WARN`: focused cases are split into named test submodules |
| `crates/openwepp-land-surface-energy/src/transaction.rs` | 2,677 | `WARN`: keep solver/closure/output ownership in separate modules |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_real_hydrology_shadow.rs` | 2,157 | `WARN`: Child-3 ownership is limited to the retained narrow adapter/accessor surface in this dependency-shared file |
| `crates/openwepp-vegetation/src/transaction.rs` | 2,082 | `WARN`: V8 state/candidate/persistence work is split into dedicated modules |

The active new/decomposition modules are also counted so the vegetation and
orchestrator additions are not hidden by aggregate directory ownership:

| File | Lines |
|---|---:|
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/covered_forest_tests.rs` | 1,634 |
| `crates/openwepp-vegetation/src/v8_state.rs` | 1,243 |
| `crates/openwepp-vegetation/src/v8_candidate.rs` | 898 |
| `crates/openwepp-vegetation/src/v8_persistent.rs` | 647 |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs` | 559 |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_rollback.rs` | 357 |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_derived_ingress.rs` | 328 |

This is an executing-remediation snapshot, not terminal reconciliation. The
3,204-line solver prevents line-count closure until decomposition returns it
below 3,000 and a fresh exact recount is recorded.
