# Canopy Mutation Peak-Runoff Discontinuity

## Status

- `state`: **concept — high-priority suspected kernel defect**
- `date`: 2026-08-07
- `owner`: openWEPP maintainers + hillslope hydrology reviewer
- `origin`: Topanga 2025 fire investigation, Hill 106 high-ET screen
- `promotion trigger`: prioritize a defect-closure work package that vendors the
  frozen reproducer into openWEPP and instruments the legacy peak-runoff path
- `default eligibility`: no new or migrated peak-runoff implementation may be
  treated as scientifically closed while it reproduces this unexplained
  discontinuity

## Executive Summary

A controlled Hill 106 experiment produced a physically counter-intuitive and
potentially severe discontinuity in legacy WEPP peak runoff. Two runs used the
same climate, slope, soil, surface conductivity, PMET parameters, storm, and
nearly identical antecedent wetness. The only intended mutation was denser
undisturbed canopy management: maximum LAI increased from `5` to `6` and
initial canopy cover increased from `0.70` to `0.90`.

For the 1986-02-15 event, event runoff changed by only `0.58 mm` (`1.3%`), but
peak runoff increased from `3.56` to `294.42` in the legacy output units: an
approximately `82.7x` increase. Reported effective duration consequently
collapsed from `12.20 h` to `0.15 h`.

That response is not accepted as credible canopy physics. With the same storm
and nearly equivalent antecedent moisture, denser canopy would ordinarily be
expected to intercept more water, increase resistance, and reduce or delay the
peak. It must not create an 82-fold peak increase without a commensurate change
in runoff volume. Treat this as a serious suspected defect or numerical branch
discontinuity until event-level instrumentation identifies a physically
defensible cause.

## Frozen Reproducer

The exact input decks are preserved in the WEPPpy investigation:

- [`openwepp-hill106-effective-duration-reproducer`](https://github.com/rogerlew/wepppy/tree/master/docs/investigations/2026-08-07-topanga-2025-fire-peak-flow-analysis/artifacts/openwepp-hill106-effective-duration-reproducer)
- [reproducer instructions and checksums](https://github.com/rogerlew/wepppy/blob/master/docs/investigations/2026-08-07-topanga-2025-fire-peak-flow-analysis/artifacts/openwepp-hill106-effective-duration-reproducer/README.md)
- [Topanga investigation](https://github.com/rogerlew/wepppy/blob/master/docs/investigations/2026-08-07-topanga-2025-fire-peak-flow-analysis/README.md)

Each case contains `p106.cli`, `p106.man`, `p106.run`, `p106.slp`, `p106.sol`,
`pmetpara.txt`, `wepp_ui.txt`, `gwcoeff.txt`, and `snow.txt`. The baseline and
dense-canopy input directories differ only in `p106.man`. Promotion to a work
package must vendor a frozen copy into that package's `artifacts/` directory so
openWEPP validation does not depend on another checkout or a moving branch.

The discovery binary was:

- path: `/workdir/wepppy/wepp_runner/bin/wepp_260803`
- SHA-256:
  `4a5158e224c175ac06c760f1006cc19f7691a9bd28911d94788af2622ba178a5`

## Observed Event

| Metric | Baseline canopy | Dense canopy | Difference / ratio |
|---|---:|---:|---:|
| Maximum LAI input | 5 | 6 | +20% |
| Initial canopy-cover input | 0.70 | 0.90 | +0.20 |
| Event-day LAI | 2.506 | 4.241 | +69.2% |
| Event-day canopy height | 0.455 | 0.779 | +71.2% |
| Pre-event total soil water | 211.10 mm | 206.02 mm | -5.08 mm |
| Surface saturation | 0.99 | 0.98 | -0.01 |
| Effective rainfall intensity | 39.12 | 39.12 | identical |
| Event runoff | 43.47 mm | 44.05 mm | +0.58 mm |
| Peak runoff | 3.56 | 294.42 | **82.7x** |
| Reported effective duration | 12.20 h | 0.15 h | 0.0123x |

The antecedent states are not bit-identical, but they are too similar to make
the peak response self-explanatory. The dense case is also slightly drier in
both reported measures. Any claim that antecedent moisture caused the reversal
must demonstrate the relevant threshold and branch using internal state; the
aggregate values above do not support that explanation on their own.

## Important Interpretation of `EffDur`

The output field called effective duration is not an independently diagnosed
duration of rainfall excess. In the inspected legacy path it is derived after
peak calculation as runoff volume divided by peak runoff rate, with an upper
cap of 86,400 seconds. Therefore the `12.20 h` to `0.15 h` collapse does not
explain the peak-flow jump. It is a mathematical consequence of the jump.

Documentation and future output schemas must distinguish:

- physical rainfall-excess duration and its start/end times;
- hydrograph duration or time to peak;
- the volume/peak rectangular-equivalent duration; and
- any capped or fallback value.

Ambiguous publication of these quantities makes a kernel discontinuity appear
to be a meteorological response and is unsafe for ordinary model users.

## Defect Hypothesis

The leading hypothesis is a threshold-triggered branch or numerical regime
change in the legacy peak-runoff hydraulics. The management mutation alters
canopy and biomass state and also changes derived surface hydraulic quantities,
including observed rill width (`0.305` to `0.336` in the event diagnostics).
One or more of these changes may cross a branch boundary or destabilize a
peak-flow approximation while leaving integrated runoff nearly unchanged.

This is deliberately not yet attributed to one formula. Candidate mechanisms
include:

1. a branch change in infiltration-excess or rainfall-excess timing;
2. a discontinuity in hydraulic geometry, composite friction, or rill width;
3. a threshold in the approximate peak-flow solution (`appmth` or equivalent);
4. an unstable or poorly conditioned use of runoff duration, time variables,
   or peak-shape parameters;
5. stale, overwritten, unit-inconsistent, or order-dependent event state; or
6. a reporting/unit defect that affects peak rate but not integrated volume.

Interactions matter. The investigation must trace the full management-to-peak
dependency chain rather than varying LAI, cover, roughness, or geometry in
isolation and assuming additivity.

## Required Investigation

### 1. Establish deterministic reproduction

- Run both frozen decks with the pinned discovery binary.
- Confirm the event-level values above and record raw outputs and hashes.
- Repeat each case to exclude uninitialized state or run-order dependence.
- Reverse case order and run each case in a clean process.
- Run with bounds checking, floating-point traps, and uninitialized-value
  diagnostics where supported.

### 2. Instrument the event computation

Capture every term needed to reconstruct the 1986-02-15 peak calculation,
including at minimum:

- rainfall and infiltration time series, excess start/end, and `drlast`;
- runoff volume before and after any infiltration adjustment;
- `remax`, `runtmp`, `peakro`, `ealpha`, and all duration variables;
- surface saturation and layer water contents before the storm;
- rill/interrill cover, canopy cover, LAI, biomass, residue, random roughness,
  rill width, and hydraulic radius;
- component and composite friction/roughness values;
- slope, flow length, driving head, `tstar`, `vstar`, and related iterates;
- the selected peak-flow method and every conditional branch controlling it;
- iteration counts, convergence residuals, clamps, caps, fallback values, and
  unit conversions; and
- operands used to publish effective duration.

The trace must make branch selection explicit. A final scalar dump is
insufficient because it cannot reveal a threshold crossing or stale state.

### 3. Locate the discontinuity

Perform deterministic one-dimensional and paired sweeps between the two
management files. Use sufficiently fine increments to bracket the first jump:

- maximum LAI from `5` through `6`;
- initial canopy cover from `0.70` through `0.90`;
- paired LAI and canopy-cover interpolation;
- directly derived event-state inputs such as rill width and composite
  friction, while holding the rest of the event state fixed; and
- antecedent layer water contents around both observed states.

Near the transition, use binary search or an equivalently reproducible bracket
to identify the smallest input perturbation that changes the selected branch
or causes a non-smooth peak response. Do not use these sweeps to calibrate away
the symptom.

### 4. Reconstruct and test the peak independently

- Recalculate peak runoff from the captured operands outside the kernel.
- Check dimensional consistency and conversion factors.
- Verify that integrating the represented hydrograph closes to event runoff.
- Compare the approximation against a stable reference hydrograph or numerical
  solution using the same rainfall-excess forcing and surface hydraulics.
- Determine whether either `3.56` or `294.42` is physically plausible; legacy
  agreement alone is not acceptance authority.

### 5. Determine scope

After Hill 106 is explained, test whether the defect generalizes:

- nearby events with similar runoff and antecedent saturation;
- dry, intermediate, and saturated antecedent states;
- additional Topanga hillslopes spanning slope and flow length;
- burned and undisturbed management states with matched surface conductivity;
- the Palisades timing-inversion fixture; and
- watershed routing using unchanged hillslope hydrographs except for the
  corrected peak representation.

Hillslope diagnosis comes first. Watershed routing can amplify, attenuate, or
temporally combine a bad hillslope signal; it cannot establish that the source
peak was valid.

## Required Behavioral Invariants

The work package must define exact tolerances, but at minimum:

1. Event runoff volume must close to the generated hydrograph integral.
2. Effective duration must be reconstructible from explicitly named operands
   and labeled as derived when that is its meaning.
3. Small continuous changes in canopy inputs must not cause an unreported
   discontinuous peak response.
4. Any legitimate regime transition must expose its branch, threshold,
   physical rationale, and continuity behavior.
5. With identical storm forcing and controlled antecedent state, added canopy
   interception or resistance must not increase peak runoff absent a traced,
   conservative physical mechanism.
6. A large peak ratio with nearly unchanged runoff volume must produce a
   diagnostic warning until its hydrograph shape and branch provenance are
   demonstrated valid.
7. No repair may tune canopy, Ksat, `kslast`, anisotropy, or PMET coefficients
   merely to suppress the observed peak.

## Acceptance and Closure Criteria

This item is not closed by reproducing legacy output. Closure requires all of
the following:

- the exact branch or numerical mechanism causing the 82.7x jump is identified;
- the explanation is supported by an operand-complete event trace;
- the behavior is classified as valid physics, implementation defect, numeric
  defect, state-lifecycle defect, or output/reporting defect;
- if valid, independent physical authority and hydrograph closure justify the
  magnitude and direction;
- if defective, the corrected implementation passes the controlled mutation
  sweep without an unexplained discontinuity;
- regression tests preserve the Hill 106 pair and the bracketed transition;
- peak, volume, timing, and effective-duration outputs carry unambiguous units
  and derivation provenance;
- broader hillslope and watershed checks show the correction does not merely
  move the anomaly elsewhere; and
- the applicable peak-runoff science contract records the governing equations,
  branches, invariants, provenance, and known applicability limits.

## Promotion Requirements

When prioritized, create a dedicated defect-closure work package. It must:

1. vendor the frozen input decks and discovery outputs into its own artifacts;
2. pin the authoritative legacy source revision and binary hashes;
3. map the peak-runoff call chain before implementation edits;
4. declare whether the work changes migrated legacy behavior, native openWEPP
   behavior, or both;
5. identify and amend the controlling science contract before accepting changed
   production physics; and
6. include an independent hydrology/science review because the defect directly
   affects peak-flow prediction and post-fire hazard interpretation.

## Explicit Non-Solutions

The following do not resolve this defect:

- explaining the result with the derived `EffDur` value;
- assuming nearly saturated soil makes an 82.7x peak reversal reasonable;
- calibrating ET or soil conductivity until the event disappears;
- suppressing the output or clipping the peak without physical authority;
- matching a different WEPP binary without explaining the computation; or
- testing only annual runoff totals.

Annual water balance and ET remain important context, but this defect is an
event-scale peak-generation failure and must be investigated at event scale.
