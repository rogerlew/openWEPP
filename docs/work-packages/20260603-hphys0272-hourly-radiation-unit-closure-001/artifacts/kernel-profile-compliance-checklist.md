# Kernel Profile Compliance Checklist

Status: completed/HOLD
Evidence mode: static + ran

Static:

- Contract-first sequence followed:
  `SC-*` amendments -> contract-derived tests -> red gate -> production edit.
- Canonical authority used:
  `/workdir/wepp-forest_260430_baseline` radiation-unit lineage from
  `stmget.for`, `winter.for`, `sunmap.for`, `radcur.for`, and `hr_tmp.for`.
- No provisional physics, empirical calibration, radiation clipping, or
  WB13/WB17/storage compensation was introduced.
- Variable continuity preserved: `radly`, `radmj`, and `hradmj` are named in
  canonical contracts and mapped to openWEPP runtime symbols.
- Typed hard-fail posture for missing/non-finite active forcing inputs remains
  unchanged.

Ran:

- Contract-derived tests pass after the implementation.
- Full H1..H39 runtime/comparator metrics were recorded.
- Package status remains `completed/HOLD` because semantic parity residuals
  remain outside the scoped radiation-unit closure.
