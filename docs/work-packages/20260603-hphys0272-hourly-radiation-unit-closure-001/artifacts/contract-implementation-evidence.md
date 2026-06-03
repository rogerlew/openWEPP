# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  version `16` adds `INV-CLIMATE-013`, `radly`/`radmj`/`hradmj` variables,
  guard mapping, alias mapping, invalid-state language, producer obligation
  `OBL-CLIMATE-P-008`, SIMIMPL28 unit requirements, and revision history.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  version `16` links `INV-SNOWFREEZE-017` to
  `SC-CLIMATE-001#INV-CLIMATE-013` so day-36 melt-forcing evidence cannot use
  Langley-scale radiation or heuristic radiation clipping.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  version `100` links `INV-WATBAL-057` to climate radiation-unit closure and
  prohibits WB13/WB17/storage compensation for a radiation-unit artifact.
- `docs/specifications/science-contracts/index.md` notes HPHYS0272 authority in
  the climate, snow/freeze, and water-balance registry rows.

Ran: not applicable.
