# HPHYS0235 H1 Transient Lane Diagnostic

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Diagnostic Question

Why does `H1` `Dp` remain about `7x` legacy for early days when WB13 lineage is
already flux-authoritative?

## Static Attribution

### Baseline hourly authority

1. `watbal.for` dispatches `ui_run=1` directly to `watbal_hourly`  
   (`watbal.for` lines 252-255).
2. `watbal_hourly.for` sets `ui_LFtstp=24` and executes an explicit hourly
   loop (`do ii = 1, ui_LFtstp`) with per-step infiltration distribution and
   `call purk` each substep  
   (`watbal_hourly.for` lines 434-461, 540-545).
3. `purk.for` applies `sep/ui_LFtstp` on each substep update  
   (`purk.for` lines 167-188).

### Current openWEPP hourly shape

1. Runner seeds hourly lane divisor only:
   `wb18_perc_lane_substeps = 24`  
   (`crates/openwepp-runner/src/hillslope/mod.rs` lines 1549-1555).
2. WB18 kernel executes one daily pass and applies `pei = pei_unscaled /
   lane_substeps` once; no `24`-substep recompute loop exists  
   (`03_kernel_support_01_kernel_phases.rs` lines 741-764, 934-946).

## Ran Evidence

- Hourly probe (`wepp_ui.txt` present): manifest selected lane `hourly`.
- Daily probe (`wepp_ui.txt` absent): manifest selected lane `daily`.
- `H1` day-1..7 `Dp`:
  - hourly probe mean ratio vs baseline: `7.2599`,
  - daily probe mean ratio vs baseline: `0.9417`.

## Root Cause

The persistent `~7x` early-transient `Dp` mismatch is caused by unresolved
hourly-lane process semantics migration. Baseline hourly behavior is iterative
(`24` substeps/day with per-substep recomputation), while openWEPP currently
uses a single daily WB18 pass with divisor attenuation.

## Closure Implication

Remaining closure work belongs in WB18/WB11 hourly execution-shape migration,
not WB13 publication lineage.
