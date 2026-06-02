# HPHYS0241 Contract Implementation Evidence

Status: complete
Evidence mode: static

Static: canonical `SC-*` contracts were amended before production-code edits.

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`:
  contract version `70`; added `REF-WATBAL-LEGACY-HOURLY-CARRY`,
  `INV-WATBAL-033`, alias rows for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`,
  `ui_LfCrf`, and the HPHYS0241 MOFE hourly carry-array addendum.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`:
  contract version `25`; added `REF-RUNOFFPART-LEGACY-HOURLY-CARRY`,
  `INV-RUNOFFPART-013`, alias rows for the four carry arrays, and
  array-authoritative runoff/runon anti-shadow rules.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`:
  contract version `77`; added `mofe_hourly_carry` manifest authority,
  `INV-SYSTEM-028`, alias rows, and watershed contributor intake obligations.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`:
  contract version `44`; added `REF-ROUTE-MOFE-HOURLY-CARRY`,
  `INV-ROUTE-014`, routing-admission alias rows, and fail-closed manifest
  validation requirements.

Static: provenance cites `/workdir/wepp-forest_260430_baseline/src/wathour.inc`
and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Ran: not applicable for this artifact.
