# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

No touched production Rust file exceeds 3,000 lines. Existing touched
2,000-line warning surfaces were not grown:

- `executor.rs`: 2,986 lines, unchanged;
- `verifier.rs`: 2,794 lines, unchanged.

The corrected external-DAG implementation uses bounded modules, all below
2,000 lines:

- `external_dag.rs`: 1,641;
- `external_dag/audit.rs`: 706;
- `external_dag/custody.rs`: 726;
- `external_dag/tests.rs`: 1,057;
- `external_outputs.rs`: 761.

`publication.rs` is 2,547 lines and therefore receives a `WARN`, not a closure
block. Its growth is attributable to descriptor-relative source, staging,
backup, journal, and destination custody plus the real committed-authority and
root-replacement fixtures required to verify those paths. It remains below the
3,000-line mandatory-refactor threshold. Split intent: move the publication
fixture and adversarial tests into `publication/tests.rs` in the next
non-critical structural increment; do not change the verified descriptor
custody path during that mechanical split.

Existing `main.rs` (1,823) and `pre_heavy.rs` (1,946) receive integration hooks
and remain below 2,000 lines.
