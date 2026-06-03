# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` advanced to `contract_version: 15` and adds `INV-SNOWFREEZE-017` for HPHYS0271 melt-term/hourly-forcing evidence.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` advanced to `contract_version: 99` and adds `INV-WATBAL-057` requiring WB13/WB17/storage ownership claims to consume HPHYS0271 melt-forcing evidence first.
- `docs/specifications/science-contracts/index.md` now indexes the HPHYS0271 obligations.
- Alias/evidence rows include `snow.hourly.melt_raw_m_####`, `snow.hourly.melt_m_####`, `amelt/bmelt/cmelt/dmelt`, thermal/dewpoint/wind/radiation/cloud forcing, and branch flags.

Ran: not-run; contract edits are static documentation authority.
