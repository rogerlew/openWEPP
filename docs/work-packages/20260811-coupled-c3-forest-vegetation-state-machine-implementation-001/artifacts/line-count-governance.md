# Line-Count Governance

Status: `WARN / V4 runtime increment; no 3,000-line blocker`

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
