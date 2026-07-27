# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

No touched production Rust file exceeds 3,000 lines. Existing touched
2,000-line warning surfaces were not grown:

- `executor.rs`: 2,986 lines, unchanged;
- `verifier.rs`: 2,794 lines, unchanged.

The corrected implementation uses bounded modules, all below 2,000 lines:

- `external_dag.rs`: 1,325;
- `external_dag/audit.rs`: 544;
- `external_dag/custody.rs`: 456;
- `external_dag/tests.rs`: 530;
- `external_outputs.rs`: 704;
- `publication.rs`: 1,769.

Existing `main.rs` and `pre_heavy.rs` receive narrow integration hooks and
remain below 2,000 lines.
