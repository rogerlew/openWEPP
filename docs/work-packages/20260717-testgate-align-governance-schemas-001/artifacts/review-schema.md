# Independent Schema And Security Review

Evidence class: `Static` and `Ran`

Initial recommendation: `HOLD`

The read-only reviewer performed schema-valid mutation checks with the installed
JSON Schema validator. No files were modified.

## Findings

1. `SCHEMA-001` (`HIGH`): the gate plan omitted required planner/node fields and
   could not represent a governed verified empty DAG.
2. `SCHEMA-002` (`HIGH`): campaign and assurance records lacked sufficient
   discriminated fold events and accepted unanchored/impossible current or
   certified states.
3. `SCHEMA-003` (`HIGH`): receipt authority outcomes contradicted canonical
   execution/A0 semantics, the DAG snapshot was incomplete, and contradictory
   PASS records validated.
4. `SCHEMA-004` (`HIGH`): generic envelope subjects did not unambiguously bind
   the claimed receipt.
5. `SCHEMA-005` (`MEDIUM`): acceptance predicates and legacy-adapter identity
   allowed undefined combinations.
6. `SCHEMA-006` (`MEDIUM`): negative fixtures failed for unrelated omissions,
   so removal of an intended guard could remain undetected.
7. `SCHEMA-007` (`LOW`): the source guard did not independently pin schema IDs
   and version constants.

The reviewer confirmed exact policy hashing, closed object shapes and governed
state vocabularies, SHA-1/SHA-256 Git identity support, terminal intent lineage,
nonrecursive unsigned receipts, closed issuer classes, and correct Cargo
registration.

Final disposition is recorded in `review-disposition.md`.
