# Worker Handoff

Evidence: Static
Date: 2026-06-10

## First Actionable Item

Close defect `SCSTRUCT04-SYSTEM-BEI-SCIENCE-REVIEW` in
`20260610-scstruct05-system-bei-science-review-adjudication-001`.

## Context

SCSTRUCT04 added a conservative Binding Exposure Index to `SC-SYSTEM-001`.
Because none of the 27 top-level addendum sections contains a same-section
`INV-SYSTEM-*` or `OBL-SYSTEM-*` reference, all rows were routed to
`science-review-follow-on`.

## Required Next Work

For each deferred row:

1. Map the addendum's binding residue to precise existing `INV-SYSTEM-*` or
   `OBL-SYSTEM-*` authority, or
2. promote missing binding authority through the science-contract review gate, or
3. prove historical/superseded residue conservation.

Do not relocate narrative until the row is resolved. Do not treat default-mode
`PASS-DEFERRED` as full consolidation.
