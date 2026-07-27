# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

No touched production Rust file exceeds 3,000 lines. Existing touched
2,000-line warning surfaces were not grown:

- `executor.rs`: 2,986 lines, unchanged;
- `verifier.rs`: 2,794 lines, unchanged.

The implementation uses new bounded modules:

- `external_dag.rs`;
- `external_outputs.rs`;
- `publication.rs`.

Existing `main.rs` and `pre_heavy.rs` receive narrow integration hooks and
remain below 2,000 lines.

