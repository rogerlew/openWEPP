# Specification Review — Mechanical Ownership

Evidence class: Static.

Initial disposition: **HOLD** pending correction.

An independent coding-agent reviewer audited current assurance commands,
reports, tests, and recent ASSURE-04/05/06 evidence for deterministic work still
being assigned to agents.

Findings:

- `MC-01` BLOCKING: the approval graph was circular; use an acyclic event and
  lock sequence.
- `MC-02` BLOCKING: lock, review-lock, and receipt membership was undefined and
  could create self-hashes or receipt accumulation inside the active root.
- `MC-03` BLOCKING: an arbitrary copyedit cannot be proven semantically neutral
  from protected tokens.
- `MC-04` BLOCKING: minimum lifecycle writers are required in the current
  package once embedded roots are removed.
- `MC-05` MAJOR: shared principal mutations need complete-consumer planning.
- `MC-06` MAJOR: every reader-facing attribution/lifecycle occurrence and the
  mutable governance part of the agent packet must derive from one structured
  source.
- `MC-07` MAJOR: mutation plus emitted commands is not a complete workflow; add
  a mechanical receipt validator/runner and prohibit fast-lane bureaucracy.
- `MC-08` MAJOR: whole-tree exchange needs a scaled-corpus performance contract
  or a bounded closure/copy-on-write strategy.
- `MC-09` MODERATE: identity canonicalization needs an exact versioned
  algorithm and generated-region boundary.
- `MC-10` MODERATE: permanent receipts should carry versioned gate IDs and argv,
  not copied shell policy.

Current-package mechanical ownership should include identity generation,
acyclic review locks, attribution/normalization, reader governance blocks,
impact/invalidation, ephemeral named build/check, lifecycle writers, receipt
running, fixture mutation APIs, and protected-surface evidence. Report
scaffolding, object ingestion, scientific reproduction execution, general gate
evidence, and generated revision catalogs belong in an ordered follow-up queue.
Scientific question selection, evidence judgment, interpretation, arbitrary
prose impact, competence, independence, findings, approvals, and release
decisions remain human work.

All findings were accepted. Their corrections are recorded in
`spec-review-disposition.md`.

This was coding-agent workflow review, not human approval.

## Final Re-verification

Disposition: **PASS**.

The corrected design isolates bibliographic attribution, orders approval events
without recombination, binds final staged bytes and transfer authority, and
retains a coherent current-package and follow-up mechanization boundary. No
closure blocker remains.
