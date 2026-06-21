# Data Path Proof

Status: executed-hold.
Evidence mode: Static + Ran.

## Required Proof

Before closure, record the accepted path for each output family:

- producer phase/source;
- typed direct operand;
- `DirectRunPublicationFrame` field;
- runner handoff;
- direct projection consumer;
- public output writer;
- negative proof that compatibility WB13 rows/runtime surfaces are not read.

## Current Data Path

Static:

1. `build_direct_publication_artifacts` constructs `DirectRunIdentity`.
2. It calls `DirectRunFrame::skeleton(identity)`.
3. It calls `seed_direct_publication_lane_geometry`, which only fills lane area
   and slope geometry.
4. It derives calendar days and metadata.
5. It calls `DirectFrameExecutor::run_publication_capture`.
6. `DirectRunPublicationFrame` rows are built from `DirectDayFrame` skeleton
   state through `DirectPublicationDayRow::from_day_frame`.
7. Direct HBP/WAT/PASS/loss helpers consume that skeleton-derived publication
   frame.
8. Cutover parity compares direct artifacts against compatibility HBP/loss/WAT
   and PASS artifacts, then stops before public writes.

## Negative Proof

Static: There is no accepted production bridge from parsed inputs, R4/R5 direct
state, or compatibility execution typed operands into the cutover
`DirectRunFrame` before `run_publication_capture`.

Ran: The CLI cutover candidate fails closed before writing public outputs with
`R6-DIRECT-PUBLICATION-PARITY
R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`.

## Gate

FAIL. R6B cannot prove an accepted direct-publication data path because the
direct frame remains skeleton-populated at the production cutover boundary.
