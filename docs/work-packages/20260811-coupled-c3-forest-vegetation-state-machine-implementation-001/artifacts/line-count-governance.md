# Line-Count Governance

Status: `WARN / Milestone 5 focused-closed; retained files remain below 3,000-line blocker`

## 2026-08-13 V7 Focused Recount

Ran after formatting: `carbon_nitrogen.rs` 2,020 lines; `migration.rs` 2,866
lines; `migration/v6_to_v7.rs` 598 lines; `transaction.rs` 1,843 lines. The
existing `migration.rs` WARN remains below the 3,000-line blocker, and the V7
migration was placed in a separate cohesive module. `carbon_nitrogen.rs` now
enters WARN solely because the independent V7 fixture and poison tests remain
module-local; split those tests into a dedicated file before package closure.
Neither file reaches the 3,000-line closure blocker.

Evidence mode: `Ran`

Ran: `transaction.rs` is 1,069 lines and the dedicated `column.rs` is 1,479
lines including its controlled routing tests. Both remain below the 2,000-line
WARN threshold; no exception is requested. Column execution is decomposed into
topology build, per-occupancy solve/accept, column finish, identity validation,
and independent closure functions.

Responsibilities are separated into radiation, photosynthesis, energy,
hydraulics, interception, C/N, numerics, ledger reconstruction, typed resource
protocol, BGC receiving state, and diagnostic orchestration modules.

## 2026-08-12 Increment 2B HOLD Checkpoint

`occupancy_solver/resources.rs` is below the 2,000-line warning threshold.
No radiation, potential, capped, input, or diagnostics solver module was
retained because canonical authority is incomplete. Existing Increment 2A and
package-wide line-count dispositions remain unchanged.

## 2026-08-13 V4 Runtime Audit

Ran: `find crates/openwepp-vegetation/src -name '*.rs' -print0 | xargs -0
wc -l | sort -nr`.

| Rust file | Lines | Disposition |
|---|---:|---|
| `migration.rs` | 2,589 | WARN: V4 added strict historical DTOs, structural deserializers, exhaustive validation, and migration tests; below mandatory 3,000-line refactor threshold |
| `occupancy_solver/constitutive.rs` | 2,541 | WARN retained from the V3 constitutive increment; below mandatory 3,000-line refactor threshold |
| `transaction.rs` | 1,743 | PASS; V4 structural encoding and recursive shape work has been split into `transaction/state_canonical.rs` (553) and `transaction/state_shape.rs` (168) |
| `column.rs` | 1,585 | PASS |

Decomposition plan before terminal closure: move historical V1/V2/V3 DTO and
serde adapters from `migration.rs` into versioned migration submodules; move
V3-to-V4 source validation/report construction into its own module; move the
large migration test population into module-local test files. Preserve exact
wire shapes, issue ordering, and candidate revalidation. Independently split
constitutive input preparation, nested solve evaluation, and test-only failure
injection without changing numerical order. The accepted Medium review item
also calls for a shared structural-validation component while retaining
independent boundary revalidation.

No touched Rust file reaches 3,000 lines, so this WARN does not by itself block
the bounded V4 increment. It remains open package work and must be reconciled
again against terminal exact bytes.

## 2026-08-13 V5 Pre-Implementation Baseline

Ran at authority predecessor commit `b7e6f08b6` before V5 production edits:
`find crates/openwepp-vegetation/src -name '*.rs' -print0 | xargs -0 wc -l |
sort -nr`.

| Rust file | Lines | In-progress disposition |
|---|---:|---|
| `migration.rs` | 2,589 | WARN retained; V5 migration work must not cross 3,000 without decomposition |
| `occupancy_solver/constitutive.rs` | 2,541 | WARN retained; capped work should remain outside this file where practical |
| `transaction.rs` | 1,743 | PASS baseline |
| `column.rs` | 1,585 | PASS baseline |
| `occupancy_solver/potential.rs` | 1,318 | PASS baseline |
| `occupancy_solver/evaluator.rs` | 1,211 | PASS baseline |
| `occupancy_solver/capped_pass.rs` | 766 | PASS baseline; V5 implementation active |

This is an in-progress baseline, not terminal reconciliation. Recount all
touched Rust files after the capped implementation stabilizes and before its
reviews; the 3,000-line rule remains mandatory.

## 2026-08-13 V5 Bounded Checkpoint Recount

Ran after capped-core stabilization:

| Rust file | Lines | Disposition |
|---|---:|---|
| `migration.rs` | 2,842 | WARN; below the 3,000-line closure threshold |
| `occupancy_solver/constitutive.rs` | 2,683 | WARN; retain decomposition debt |
| `transaction.rs` | 1,743 | PASS |
| `column.rs` | 1,593 | PASS |
| `occupancy_solver/potential.rs` | 1,581 | PASS |
| `occupancy_solver/evaluator.rs` | 1,300 | PASS |
| `occupancy_solver/capped_pass.rs` | 1,030 | PASS |
| `occupancy_solver/resources.rs` | 732 | PASS |
| `diagnostics.rs` | 233 | PASS |

No touched Rust file reaches 3,000 lines. The two WARN files remain package
decomposition debt and must be reconciled again after the authority HOLD lifts.

## 2026-08-13 V6 Remediation Recount

The first V6 correctness review found `migration.rs` at 3,130 lines, which was
a closure-blocking policy violation. V5-to-V6 snapshot and diagnostic migration
was extracted to cohesive submodules and the current exact counts are:

| Rust file | Lines | Disposition |
|---|---:|---|
| `migration.rs` | 2,890 | WARN; below the mandatory 3,000-line threshold |
| `migration/v5_to_v6.rs` | 378 | PASS |
| `migration/v5_to_v6/tests.rs` | 651 | PASS |
| `occupancy_solver/v5_capped_fixture_tests.rs` | 1,325 | PASS |

No touched Rust file is at or above 3,000 lines. The retained WARN remains
decomposition debt for terminal package closure, not a blocker for this bounded
HOLD-lift increment.

## 2026-08-13 Public Water-Phase Recount

| Rust file | Lines | Disposition |
|---|---:|---|
| `water_phase.rs` | 1,032 | PASS; cohesive uncommitted orchestration and owner validation |
| `transaction.rs` | 1,805 | PASS; full candidate remains sealed/fail-closed at E16 |
| `occupancy_solver/capped_pass.rs` | 1,204 | PASS; exact public capped operands |

No newly touched Rust file reaches 2,000 lines. The earlier `migration.rs` and
`constitutive.rs` WARN-level decomposition debt remains unchanged.

## 2026-08-13 V7 E19 Hold Recount

| Rust file | Lines | Disposition |
|---|---:|---|
| `carbon_nitrogen.rs` | 2,051 | WARN; production kernels remain below 2,000 lines and the retained focused tests require extraction before terminal closure |
| `persistent_phase.rs` | 452 | PASS; crate-private uncommitted composition |

No touched Rust file reaches 3,000 lines. The `carbon_nitrogen.rs` WARN is
explicit decomposition debt and does not justify bypassing the bounded E19
authority HOLD.

## 2026-08-13 E19 Ordering-Remediation Recount

Ran after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-vegetation/src/carbon_nitrogen.rs` | 2,214 | WARN; existing cohesive C/N/phenology test-heavy module, below mandatory 3,000-line split threshold |
| `crates/openwepp-vegetation/src/transaction.rs` | 1,965 | below WARN threshold |
| `crates/openwepp-vegetation/src/nitrogen_protocol.rs` | 910 | below WARN threshold |
| `crates/openwepp-vegetation/src/persistent_phase.rs` | 486 | below WARN threshold |
| `tests/integration/c3_vegetation_implementation_contract.rs` | 767 | below WARN threshold |

No changed non-generated Rust file reaches 3,000 lines. The
`carbon_nitrogen.rs` WARN is retained as decomposition debt and does not block
this bounded uncommitted E19 increment.

## 2026-08-13 Increment 4A Recount

Ran after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-vegetation/src/vegetation_candidate.rs` | 535 | PASS; candidate construction is isolated from transaction routing and constitutive kernels |
| `crates/openwepp-vegetation/src/vegetation_ledger.rs` | 443 | PASS; independent owner reconstruction and review poisons are isolated from producers |
| `crates/openwepp-vegetation/src/persistent_phase.rs` | 508 | PASS |
| `crates/openwepp-vegetation/src/transaction.rs` | 2,157 | WARN; production candidate construction was extracted, while the existing module-local transaction test population crosses the warning threshold |
| `crates/openwepp-vegetation/src/carbon_nitrogen.rs` | 2,214 | WARN retained; below the mandatory 3,000-line threshold |
| `crates/openwepp-vegetation/src/migration.rs` | 2,873 | WARN retained; shared derived-area helper removed a third implementation copy and the module remains below the mandatory threshold |

No changed non-generated Rust file reaches 3,000 lines. Before terminal
closure, split the transaction test population into its existing module path
without changing public transaction bytes or test semantics; retain the prior
carbon/nitrogen test-split obligation.

Increment 4B BGC slice recount after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-biogeochemistry/src/lib.rs` | 672 | PASS; owner construction and focused tests remain below WARN threshold |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` | 378 | PASS; default-off adapter only |

Increment 4B energy-owner recount after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/vegetation_energy_owner.rs` | 968 | PASS; independent component owner and focused poison fixtures remain isolated from vegetation production equations |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` | 366 | PASS; aggregate energy reconstruction removed |
| `crates/openwepp-vegetation/src/energy_proposal.rs` | 172 | PASS; immutable identity/boundary batch only |
| `crates/openwepp-vegetation/src/occupancy_solver/evaluator.rs` | 1,409 | PASS; bounded proposal projection added to the existing constitutive adapter |
| `crates/openwepp-vegetation/src/transaction.rs` | 2,168 | WARN retained; module-local transaction test extraction remains a terminal-closure obligation |
| `tests/integration/c3_vegetation_implementation_contract.rs` | 784 | PASS |

No changed non-generated Rust file reaches the mandatory 3,000-line split
threshold. The energy owner is already isolated in its own module; its focused
fixtures may be extracted during terminal cleanup if subsequent owner-envelope
work would push the production module past the warning threshold.

## 2026-08-13 Milestone 5 Recount

Ran after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-vegetation/src/transaction.rs` | 2,082 | WARN retained; module-local transaction test extraction remains required before terminal closure |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_energy_owner.rs` | 1,232 | PASS; independent owner and poison tests remain isolated |
| `crates/openwepp-vegetation/src/vegetation_candidate.rs` | 568 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` | 503 | PASS; complete owner envelope, typed mismatch poisons, and atomic replacement |
| `crates/openwepp-vegetation/src/energy_proposal.rs` | 185 | PASS |

No changed non-generated Rust file reaches the mandatory 3,000-line split
threshold. The retained transaction and carbon/nitrogen WARN obligations remain
terminal cleanup work; they do not block this focused Milestone 5 closure.

## 2026-08-13 Milestone 6 Exact Recount

Ran after formatting:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-vegetation/src/migration.rs` | 2,873 | WARN; versioned migration responsibilities remain explicit and below the mandatory 3,000-line threshold |
| `crates/openwepp-vegetation/src/occupancy_solver/constitutive.rs` | 2,790 | WARN; exact solver and its authority-vector tests remain cohesive and below the mandatory threshold |
| `crates/openwepp-vegetation/src/carbon_nitrogen.rs` | 2,214 | WARN; exact six-tissue kernels and tests remain cohesive and below the mandatory threshold |
| `crates/openwepp-vegetation/src/transaction.rs` | 2,082 | WARN; production code ends before the module-local identity/state test population; below the mandatory threshold |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_energy_owner.rs` | 1,232 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs` | 503 | PASS |
| `tests/integration/c3_vegetation_implementation_contract.rs` | 1,216 | PASS |
| `crates/openwepp-biogeochemistry/src/lib.rs` | 827 | PASS |

No changed non-generated Rust file reaches 3,000 lines. The WARN modules are
accepted test-heavy cohesion debt rather than closure blockers; no public API,
numerical ordering, or canonical serialization is changed solely to reduce
line count at stable terminal bytes.
