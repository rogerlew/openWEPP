# HPHYS0240 Contract Implementation Evidence

Status: completed
Evidence mode: Static

Static: amended canonical contracts before production code edits:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 69`
  - added `INV-WATBAL-032`
  - added HPHYS0240 hourly runoff carryover addendum
  - updated WB12/WB14 required surfaces, runoff equations, closure-delta equations, and outputs for `wb12_runoff_carryover`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `contract_version: 24`
  - added `INV-RUNOFFPART-012`
  - added HPHYS0240 hourly runoff carryover addendum
  - updated WB12/WB14 runoff partitioning equations and contract-test vector obligations
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `contract_version: 23`
  - added `INV-SUBHYD-022`
  - added HPHYS0240 carryover handoff addendum
  - updated WB12 coupling requirements to require same-pass carryover consumption before storage closure

Static: contract authority now requires same-pass `wb12_runoff_carryover` flux
to override stale `wb12_runon_input` when present, finite/non-negative domain
validation for resolved carryover, flux republication, and typed failure for
malformed present carryover fluxes.
