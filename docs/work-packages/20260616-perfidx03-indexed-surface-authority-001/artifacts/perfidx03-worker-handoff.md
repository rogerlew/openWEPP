# PERFIDX03 Worker Handoff

Status: HOLD handoff 2026-06-17
Evidence mode: **Static** + **Ran**

## Current State

PERFIDX03 did not close. Registry completeness is improved, and inactive indexed
authority helpers are present, but production activation is disabled because the
active flip regressed OFE5 wall-clock by about 42%.

## First Follow-On

Create a blocker-closure package before `PERFIDX04-hot-symbol-id-tables-001`.
Suggested slug:

```text
PERFIDX03B-indexed-kernel-seam-or-export-cache-001
```

Objective:

- Remove the per-lane/day full `BTreeMap` export from the live indexed authority
  path.
- Preserve the `BoundarySymbol` compatibility API at external seams unless a new
  ADR amends ADR-0022.
- Keep the writeback payload shape unchanged unless separately authorized.
- Re-run the same OFE5 active-flip timing before attempting H2637.

Acceptance for the follow-on:

- Active indexed authority has no OFE5 regression against baseline/no-flip.
- Same-run-name OFE5 outputs match logically and hash-stable outputs match
  byte-for-byte.
- Then run the full PERFIDX03 H2637 both-UI + OFE1-OFE5 anchor.

## Stage 4 Status

`PERFIDX04-hot-symbol-id-tables-001` is blocked. Stage 4 resolve-once tables
should not start until Stage 3 authority can be activated without the export
regression.
