# Terminal Schema Verification — Round 1

Evidence class: `Static` and `Ran`

Disposition: `HOLD`

The fresh read-only verifier reported:

1. `SCHEMA-TERM-001` (`HIGH`): structurally valid PASS receipts could carry
   nonmatching nonempty inventories, failed attempts/outcomes, differing
   mutation digests, or nonzero zero-work counts.
2. `SCHEMA-TERM-002` (`HIGH`): assurance plans lacked canonical request axes;
   assurance records lacked transfer/target/revocation event operands and could
   not represent the canonical empty fold.
3. `SCHEMA-TERM-003` (`HIGH`): certified/current campaign claims retained
   locally expressible and cross-field contradictions.
4. `SCHEMA-TERM-004` (`MEDIUM`): impact-map matcher values were not
   discriminated or path-safe.
5. `EVIDENCE-TERM-005` (`MEDIUM`): the runner's tracked-diff/status hashes did
   not content-bind untracked schema, fixture, test, and package bytes.

The verifier otherwise confirmed JSON parsing, 7/7 focused tests, diff hygiene,
policy digest, heavy logs, evidence digests, schema identities, closed shapes,
Git IDs, terminal lineage, issuer/state vocabularies, typed receipt subject,
Cargo registration, one-mutation negatives, and nonblocking posture.

Disposition is recorded in `terminal-finding-disposition.md`.
