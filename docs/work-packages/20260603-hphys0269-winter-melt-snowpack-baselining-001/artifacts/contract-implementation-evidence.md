# Contract Implementation Evidence

Status: completed/HOLD
Evidence mode: static

Static: canonical contract amendments were authored before the HPHYS0269 production patch.

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` moved to `contract_version: 13`; `INV-SNOWFREEZE-015` now accepts corrected `/workdir/wepp-forest` commit `03fee455` as superseding authority for daily negative-melt redistribution while retaining pinned `snowd.for`/`melt.for` lineage for rain retention and raw signed melt.
- `SC-SNOWFREEZE-001` now records source anchors for pinned `winter.for` negative-melt comparator context, corrected `wepp-forest` negative-melt target authority, and `snowd.for` rain-on-snow retention.
- `SC-SNOWFREEZE-001` now defines aliases for `snow.hourly.melt_raw_m` and `snow.hourly.rain_retained_m`.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` moved to `contract_version: 97`; `INV-WATBAL-055` now rejects pinned-baseline negative-melt bug compatibility as target water-balance behavior while preserving retained-rain/signed-melt closure requirements.

Ran: `git diff --check` passed after contract and implementation edits.
