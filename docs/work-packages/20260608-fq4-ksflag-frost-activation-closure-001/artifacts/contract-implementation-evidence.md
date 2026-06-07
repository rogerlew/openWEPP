# Contract Implementation Evidence

Status: complete

Evidence mode: Static.

## Contract Edits

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `contract_version: 53`
  - `last_reviewed: 2026-06-07`
  - Updated `INV-SNOWFREEZE-009` so activation depends on runtime
    state/forcing triggers and valid parsed/default snow/frost controls, not on
    snow-sidecar or frost-sidecar presence alone.
  - Clarified that for standard `ksflag` frost,
    `frost.options.frost_file_present` is provenance only and must not suppress
    frozen-soil coupling when `frost.options.wintRed=1` and thermal/runtime
    triggers are active.
  - Added revision row `2026-06-07` version `53`.

## Authority

The fix is contract-first and follows `SC-SNOWFREEZE-001#INV-SNOWFREEZE-009`,
`INV-SNOWFREEZE-012`, and `INV-SNOWFREEZE-013`. No comparator magnitude target,
snow magnitude patch, forest `ksatadj` edit, runoff partition edit, ET edit, or
p11/MOFE edit was made.
