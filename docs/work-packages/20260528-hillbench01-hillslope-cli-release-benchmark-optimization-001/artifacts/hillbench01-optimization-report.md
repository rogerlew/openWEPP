# HILLBENCH01 Optimization Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Optimization Targets
1. Avoid rewriting release sidecar metadata on every hillslope CLI invocation
   when the sidecar is still valid and newer than the binary.
2. Remove avoidable allocations in the hillslope execution setup/day loop.

## Implemented Changes
- `crates/openwepp-runner/src/release.rs`
  - `write_release_sidecar_for_binary` now reuses an existing valid sidecar
    when it is fresh for `(binary_path, role)` instead of always rebuilding and
    rewriting metadata.
  - Added freshness helper:
    - `sidecar_is_fresh_for_binary_unlocked(...) -> bool`
  - Added unit tests:
    - `write_release_sidecar_reuses_fresh_sidecar_without_rewrite`
    - `write_release_sidecar_rewrites_when_binary_is_newer`
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Compute required/optional output paths once and reuse them across setup,
    sidecar exclusion, and output write phases.
  - Reuse `previous_climate_symbols` vector allocation across day-steps instead
    of allocating a new vector each day.
  - Pre-allocate `HillslopeWatRow` vector in `build_hillslope_wat_rows`.

## Measured Effect (Median Wall Time)
- `single_p111`: `0.567147 s -> 0.485230 s` (`-14.44%`)
- `multi_p324`: `0.605059 s -> 0.540308 s` (`-10.70%`)

## Baseline Comparison Context
- After optimization, openWEPP still runs slower than baseline
  `wepp_260430_hill` on these lanes:
  - `single_p111`: `4.5577x` baseline median
  - `multi_p324`: `3.1488x` baseline median
- This package closes release-benchmark + first optimization wave, not full
  parity with legacy runtime speed.
