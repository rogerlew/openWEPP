# Independent Reconstruction Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Handoff Item 3

Add independent reconstruction for accepted HBP/WAT/PASS/loss operands.

## Required Evidence

- Reconstruction inputs and output fields named per family.
- Reconstruction implementation does not call the production direct projection
  builder under test.
- Declared tolerances for floating-point values and byte/JSON normalization.
- Real closure or magnitude audit for conservation-sensitive outputs.

## Gate

BLOCKED. Reconstruction remains current-scope acceptance, but no accepted direct
publication operands exist yet.

First reconstruction after the hold-lift bridge:

- parse HBP bytes independently and compare peak, duration, detachment,
  deposition, and sediment concentration;
- rebuild WAT rows from accepted direct day-row operands without calling the
  production direct WAT builder;
- recompute PASS volumes from accepted depth and area operands;
- rebuild loss JSON from accepted direct run metadata/calendar/static operands.
