# Disposition

Evidence mode: Static + Ran.

Status: `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

R6E did not complete direct publication cutover.

## What Changed

- Reproduced the opt-in cutover failure and preserved fail-closed no-output
  behavior.
- Split direct-publication helpers out of the over-3000-line runner file.
- Added typed direct publication day inputs and a direct capture API that binds
  parsed precipitation/effective temperature into direct day frames.
- Changed retained cutover publication from hand-authored rows to a full
  `DirectPublicationExecution` produced by direct spans.
- Removed the compatibility-loop retained-row producer.
- Updated focused tests to prove direct input binding runs, the old B003 marker
  is absent, compatibility-edge counters remain zero, and output writes remain
  fail-closed.

## Why Held

The first remaining blocker is not absent direct input binding. The cutover
candidate now reaches HBP byte comparison and fails at
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Closing that blocker requires parity-grade direct process operands for public
HBP output. Using compatibility scheduler rows, runtime publication surfaces,
writeback payloads, stale logical state, or zero skeleton capture would violate
the R6 authority envelope. R6E therefore holds at the next direct process parity
boundary and does not write public direct outputs.

## Terminal State

`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.
