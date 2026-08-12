# Line-Count Governance

Status: `PASS / Increment 2A`

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
