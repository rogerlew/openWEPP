# Anti-Alias Fixture Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Handoff Item 2

Add anti-alias fixtures that distinguish accepted direct operands from
compatibility aliases.

## Required Fixture Families

- HBP `peakro`.
- HBP `watdur`.
- HBP sediment fields.
- WAT water-balance fields.
- PASS volumes and area/volume denominators.
- Loss static/report fields.
- Manifest provenance/checksum fields.

## Fixture Rules

- Expected values must differ from each rejected alias.
- Wrong aliases must include compatibility WB13 rows, runtime surfaces, stale
  logical state, diagnostic ledgers, adjacent direct diagnostics, and metadata
  shortcuts when plausible.
- A fixture that aliases the wrong and right value cannot close the gate.

## Gate

BLOCKED. Anti-alias fixtures remain current-scope acceptance, but they cannot
be meaningful while the cutover candidate has no nonzero parity-grade typed
operands to distinguish from rejected aliases.

First fixture after the hold-lift bridge: force HBP `peakro`, `watdur`, and
sediment operands to differ from WB13 `q`/`qofe`, PASS zeros, and runtime
surface aliases, then assert direct HBP/PASS consumers read only the accepted
direct fields.
