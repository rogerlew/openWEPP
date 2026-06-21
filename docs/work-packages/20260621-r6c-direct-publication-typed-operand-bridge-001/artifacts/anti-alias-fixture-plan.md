# Anti-Alias Fixture Plan

Evidence mode: Static.

Fixtures must distinguish accepted direct operands from:

- compatibility WB13 row fields;
- compatibility runtime symbols;
- writeback payloads;
- stale logical state;
- adjacent direct diagnostics;
- wrong area/volume denominators;
- metadata shortcuts.

Coverage required before accepting each output family:

- HBP `peakro`, `watdur`, detachment, deposition, sediment concentration.
- WAT runoff, ET, drainage/lateral, storage, calendar/OFE identity.
- PASS runoff volume, subsurface/lateral volume, peak, sediment, identity.
- Loss static run fields and event totals.
- Manifest provenance/checksum/output-policy fields.

## Execution Disposition

Anti-alias fixtures remain blocked because no output family can be accepted
until production direct publication producers exist. R6C added a stronger
negative fixture instead: the cutover candidate now fails before skeleton
publication capture, so skeleton zero/default operands and compatibility
wrappers cannot be used as alias-prone stand-ins.
