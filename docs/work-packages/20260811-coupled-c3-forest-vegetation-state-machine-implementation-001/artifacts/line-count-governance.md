# Line-Count Governance

Status: `WARN / HOLD checkpoint; decomposition required after authority lift`

Evidence mode: `Ran`

The largest touched production Rust files are `transaction.rs` (2,079 lines
at HOLD reconciliation) and `carbon_nitrogen.rs` (832 lines).
`transaction.rs` crossed the 2,000-line WARN threshold but remains below the
3,000-line required-refactor threshold in
`docs/standards/rust-scientific-coding-standard.md`.

The temporary concentration is deliberate while the public transaction's
ordering and rollback interfaces stabilize. Before package closure,
validation, topology/radiation aggregation, and candidate-ledger construction
will move into focused modules, retaining `transaction.rs` as the public
orchestration surface. No threshold exception is requested.

Responsibilities are separated into radiation, photosynthesis, energy,
hydraulics, interception, C/N, numerics, ledger reconstruction, typed resource
protocol, BGC receiving state, and diagnostic orchestration modules.
