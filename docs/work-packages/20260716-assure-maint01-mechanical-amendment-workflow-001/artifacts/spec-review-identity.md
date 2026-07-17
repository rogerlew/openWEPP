# Specification Review — Identity And Lifecycle

Evidence class: Static.

Initial disposition: **HOLD** pending correction.

An independent coding-agent reviewer inspected the proposed specification and
package against the current v2 source, lifecycle, review, and publication
implementation. The reviewer identified:

- `IA-01` BLOCKING: a generated marker and self-consistency cannot prove lock
  provenance; define genesis, prior-generation chaining, and an independent Git
  or supplied trusted head.
- `IA-02` BLOCKING: approvals inside a root that an approval binds are circular;
  replace them with an acyclic subject, ledger, approval, realization, and
  transfer graph.
- `IA-03` BLOCKING: structural copyedit checks can permit semantic inversion;
  remove arbitrary prose from automatic editorial classification.
- `IA-04` BLOCKING: removing embedded approval roots requires immutable
  decision-bearing approval events, not regenerated authority.
- `IA-05` BLOCKING: define amendment legality and invalidation for every
  lifecycle state and published-version boundary.
- `IA-06` BLOCKING: lifecycle and recovery writers cannot remain optional after
  generated review locks become authoritative.
- `IA-07` BLOCKING: migrating the existing snow/frost `IN_REVIEW` root must
  invalidate and reenter review or receive an independent equivalence
  disposition; calculated equivalence is not human authority.
- `IA-08` HIGH: principal capability changes cannot qualify as fast attribution
  metadata.
- `IA-09` HIGH: a shared principal update conflicts with one-report mutation;
  calculate the complete consumer set or use immutable report-selected records.
- `IA-10` HIGH: the transaction must capture and reverify identity-bearing
  dependencies outside `assurance/v2`.
- `IA-11` HIGH: define the generated-file universe and eliminate lock/receipt
  self-reference.
- `IA-12` HIGH: make field/file-to-root projection executable and exhaustive.
- `IA-13` HIGH: define exact generated attribution/lifecycle regions and
  inventory every current duplicate.
- `IA-14` MEDIUM: machine gate data must use stable IDs and argv arrays, not
  shell command strings.
- `IA-15` MEDIUM: lifecycle, report, local-CI, and ADR-0038 authority must be
  core reading, not conditional.

All findings were accepted. Their corrections are recorded in
`spec-review-disposition.md`.

This was coding-agent architecture review, not scientific or publication
approval.

## Final Re-verification

Disposition: **PASS**.

The corrected graph is acyclic, the role matrix binds exact roots and
predecessor events, invalidation follows the graph, and release transfer binds
its immutable authority event. No closure blocker remains.
