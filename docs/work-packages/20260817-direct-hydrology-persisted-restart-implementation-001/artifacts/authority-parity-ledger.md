# Authority Parity Ledger

Production crate: `openwepp-persisted-restart-v1`.

- The released reference modules were promoted without wire redesign.
- All four released vectors parse and reserialize byte-identically.
- The released reference suite passes 28/28; the production promoted suite passes 29/29.
- Canonical JSON, fixed-width wrappers, `HexF64`, `HexU128`, phase union,
  nested digests, owner joins, and all released poison categories are retained.
- The immutable released package and its eight manifest-bound artifacts were not modified.
