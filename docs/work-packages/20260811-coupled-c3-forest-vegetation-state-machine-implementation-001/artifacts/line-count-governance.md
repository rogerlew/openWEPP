# Line-Count Governance

Status: `WARN / V5 remediation in progress; no 3,000-line blocker`

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
| `occupancy_solver/v5_capped_fixture_tests.rs` | 1,070 | PASS |

No touched Rust file is at or above 3,000 lines. The retained WARN remains
decomposition debt for terminal package closure, not a blocker for this bounded
HOLD-lift increment.
