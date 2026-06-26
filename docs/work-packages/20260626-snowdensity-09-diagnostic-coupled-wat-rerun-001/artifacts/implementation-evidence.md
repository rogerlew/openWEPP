# Implementation Evidence

Evidence class: Static + Ran.

Implemented:

- `SC-SNOWFREEZE-001` v89 with `INV-SNOWFREEZE-062`,
  `OBL-SNOWFREEZE-P-037`, the SNOWDENSITY-09 addendum, and the paired-snow
  gate correction.
- `DirectProductionSnowFrostAuthority` package-bound diagnostic selector:
  `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`.
- Fail-closed selector parsing for unknown or non-UTF-8 values.
- Direct-production snow trace rows now include `snow_density_model`.
- Direct-production typed snow partition forwards the selected density model.
- `tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py` runs default
  and opt-in non-SNOTEL WAT evidence and writes the compact decision report.
- `tools/snowfreeze_observed/non_snotel_rubric_baseline.py` accepts a model id
  without changing existing default callers and publishes separate all-site,
  gate-eligible, and diagnostic-only out-of-gate snow-control status sets.
- `tests/integration/snowdensity09_coupled_wat_rerun.rs` guards contract,
  package, default isolation, diagnostic selector boundaries, script markers,
  and executed-report truthfulness.

Ran:

- SNOWDENSITY-09 evidence runner completed and wrote
  `snowdensity09_coupled_wat_rerun.json/md`.
- Default trace rows: `75,610`, all `legacy_wepp`.
- Opt-in trace rows: `75,610`, all `physics_bulk_density_compaction_v1`.
- Disposition:
  `COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`.

Observed result:

- Sleepers South mean signed snow-depth residual improved
  `0.4108 m -> 0.2964 m`; max absolute residual improved
  `1.5968 m -> 1.1601 m`.
- Sleepers W9 mean signed snow-depth residual improved
  `0.3233 m -> 0.2103 m`; max absolute residual improved
  `1.0599 m -> 0.7381 m`.
- GGD498 Morris mean signed snow-depth residual improved
  `0.0672 m -> 0.0586 m`; max absolute residual improved
  `0.3924 m -> 0.3215 m`.

Residual blocker:

- `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.
- The blocker is the three gate-eligible paired-snow sites. SCAN Mandan ND and
  Reynolds Creek ID lack observed snow-depth rows and are diagnostic-only
  out-of-gate evidence for this gate.
- Frost attribution remains unauthorized.
