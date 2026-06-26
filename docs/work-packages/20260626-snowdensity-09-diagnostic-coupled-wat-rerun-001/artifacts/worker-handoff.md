# Worker Handoff

Evidence class: Static.

SNOWDENSITY-09 is closed
`COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`.

The coupled opt-in WAT path is no longer absent. Trace evidence proves the
default run selected `legacy_wepp` for `75,610` direct-production snow rows and
the opt-in run selected `physics_bulk_density_compaction_v1` for `75,610` rows.

The opt-in improves snow-depth residuals but still fails snow control at
Sleepers South, Sleepers W9, and GGD498 Morris. These are the only
gate-eligible paired-snow non-SNOTEL sites in this package. SCAN Mandan ND and
Reynolds Creek ID lack observed snow-depth rows and are diagnostic-only
out-of-gate evidence for this snow-control gate.

Next recommended package:

- Resolve `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.
- Keep frost attribution blocked.
- Diagnose why the CoE-bound SNOTEL improvement does not fully transfer to the
  non-SNOTEL WAT path: candidate scope includes fresh-snow density, compaction
  carry, canopy/forcing representativeness, observation pairing, and whether the
  current bulk density cap/boundary split leaves low-density bias in shallow
  field-site regimes.
- Do not tune site constants, change default activation, add parser/runfile/user
  CLI activation, rewrite WAT, or alter frost physics in that diagnostic package.
