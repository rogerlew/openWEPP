# Worker Handoff

Evidence label: Static + Ran.

## Current State

SNOWDENSITY-10.3.14 is complete with disposition
`READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`.

The current best bundle is:

- `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1`
- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1`

It passed the full workspace no-regression gate under those selectors and is
ready for a separate default-activation package under the active `522 kg m^-3`
density cap.

## Do Next

Scaffold `SNOWDENSITY-10.3.15-DEFAULT-ACTIVATION-UNDER-ACTIVE-CAP`.

Required scope:

- Contract-first default activation authority.
- Explicit rollback/default isolation.
- Decide whether the diagnostic selectors remain as rollback/test selectors or
  are replaced by canonical default policy.
- Prove default path changes only the intended snow melt/density bundle.
- Preserve fixtures, output schemas, parser/runfile/user surfaces unless the
  package explicitly authorizes otherwise.
- Carry known residuals in release notes: `498/1415` paired rows still fail snow
  control; frost attribution remains blocked.

Do not include:

- `550 kg m^-3` cap change.
- New density-rate acceleration.
- Open-surface ablation.
- Frost attribution.
- Qwet/frzftp.

Those are separate follow-ups.
