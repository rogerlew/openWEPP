# Closure Summary

- **Package**: 20260809-hourly-peak-runoff-authority-closure-001
- **Run**: full frozen openWEPP Topanga mutation cohort
- **Overall status**: PASS

## Outcome

WB16 now derives hillslope peak flow from the maximum closing hourly runoff
depth instead of the legacy rainfall-envelope/APPMTH surrogate. WB14 owns
hourly infiltration and residual timing; routed melt and runon enter it once,
and WB19 surface return remains in its modeled hour. Internal peak units are
`m/s`, with hillslope area applied once at public `m3/s` publication.

## Evidence identities

- Terminal implementation/contract/test commit:
  `33831787b7029b28b0716c8458f08a11899db446`.
- Release binary SHA-256:
  `ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`.
- Frozen plan SHA-256:
  `32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756`.
- External evidence root:
  `/home/workdir/openwepp-hourly-peak-topanga-census-20260809-v5`.

## Full-cohort metrics

- `eligible_trials_in_plan`: 1,088
- `selected_trials`: 1,088
- `unique_baselines`: 280
- `event_pair_rows`: 1,913,199
- `finite_positive_peak_pairs`: 1,913,158
- `invalid_max_hour_fraction_count`: 0
- `max_abs_ratio_decomposition_relative_residual`:
  4.440892098500626e-16
- `max_hour_fraction_ratio_max`: 2.755595239734283
- `max_hour_fraction_ratio_p99`: 1.0000004332094452
- `peak_ratio_max`: 12965889426731.332 (near-zero denominator)
- `peak_ratio_p99`: 1.0000000000000002
- `volume_within_5pct_peak_at_least_2x_count`: 0
- `zero_runoff_peak_topology_mismatch_count`: 0

The complete cohort therefore found no unexplained volume-stable peak
discontinuity. The terminal authority identity also reconciles the erosion
consumer to the internal `m/s` maximum-hour operand and gives its independent
rectangular-duration custody check an explicit seconds-dimensional tolerance.
Both independent science reviewers and both Rust reviewers returned PASS at
the exact terminal implementation/contract/test identity.

The exact-head full workspace gate passed 2,346/2,346 selected tests with 33
ordinary skips in 8,454.483 seconds (run ID
`2a4b4f2c-d6c6-4bd6-a22f-e61bdb8f4576`). The quick profile's 2,297-test
inventory is a subset of that admitted full inventory, so no quick-only
execution delta exists; exact-head workspace doctests also pass.

The supported claim is a maximum hourly mean hillslope runoff flow. This
package does not claim an instantaneous/subhourly peak, legacy numerical
parity, calibration, observed-flow validation, or routed watershed/channel
flow.
