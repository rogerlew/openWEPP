# Contract Implementation Evidence

Status: complete
Evidence mode: static

Static: HPHYS0291 added canonical same-day snow publication lifecycle authority
before production edits.

## Contract Amendments

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - Version advanced to `26`.
  - Added `INV-SNOWFREEZE-024`.
  - Requires `snow.post_winter_rain_m` and `snow.routed_melt_m` to be
    producer-owned same-day flux surfaces.
  - Prohibits WB13 consumers from accepting state defaults, stale state, raw
    precipitation reconstruction, or downstream canonicalization.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - Version advanced to `34`.
  - Added `INV-RUNOFFPART-021`.
  - Makes runoff reconciliation the owner of same-day publication for both
    snow publication fluxes.
  - Requires finite non-negative publication on every daily execution,
    including explicit zeroes.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Version advanced to `110`.
  - Added `INV-WATBAL-066`.
  - Requires WB13 `RM` publication to consume same-day producer fluxes and
    reject absent/non-finite/negative post-winter rain or routed melt.

## Provenance

- Static: HPHYS0290 showed WB13 post-winter rain inference was downstream of
  missing producer publication, not valid publication authority.
- Static: HPHYS0291 scopes lifecycle authority only; remaining snowpack,
  runoff, storage, and `Ep` semantic residuals remain continuation work.
- Static: The amendments intentionally encode fail-closed behavior and reject
  canonicalize-and-proceed defaults.
