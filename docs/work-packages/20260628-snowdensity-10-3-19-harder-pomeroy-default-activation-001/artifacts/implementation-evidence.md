# Implementation Evidence

Evidence class: Static/Ran.

## Contract

Static:

- `SC-SNOWFREEZE-001` version `104` adds `REF-SNOWFREEZE-SNOWDENSITY1019`,
  `INV-SNOWFREEZE-075`, `OBL-SNOWFREEZE-P-050`, a boundary-disposition row, a
  Harder-Pomeroy phase-default addendum, and revision history.
- `INV-SNOWFREEZE-075` makes the cross-SNOTEL `INV-SNOWFREEZE-050`
  forcing-robust rubric the primary Policy-B gate for the phase default.

## Runtime Selector

Static:

- `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` absent or empty now selects
  `SnowPhasePartitionModel::HarderPomeroyHourly`.
- Explicit `legacy_rst` still selects `SnowPhasePartitionModel::LegacyRst`.
- Unknown selector values fail closed.
- The direct snow consumer still receives `snow_phase_model: self.snow_phase_model`.
- Frost hourly forcing remains on the legacy phase path.

## Trace And Conservation

Static:

- The internal `OPENWEPP_R7H_SNOW_TRACE_PATH` trace now records
  `snow_phase_model`.
- The source hourly guard still enforces
  `abs(hrrain + hrsnow / 10 - active_precip) <= 1e-12`.

Ran:

- `harder-pomeroy-default-activation.json` records `159986` trace rows per model,
  expected phase counts for both rollback and no-env default, and max partition
  residual `5.551115123125783e-17 m`.

## Scope Protection

Static:

- No parser/runfile/user CLI selector or `.run` disable option was added.
- No fixture, public output-schema, density-cap, frost, Qwet/frzftp, or
  compatibility-runtime change was made.
