# CANOPY-CAL-07 Southern Hemisphere Robustness

Status: `hold / forcing authority incompatible`

Evidence mode: `Ran + Static`

## Objective

Evaluate the frozen CAL-04B 37-member GSI timing ensemble against two
independent Southern Hemisphere PhenoCam lanes, signed-latitude meteorology,
the existing cyclic phase-transform proof, and the real direct-production
consumer path. Do so without Northern Hemisphere retuning or fixed-date
phenology.

## Intent

This package is an observational evaluation and bounded characterization. It
does not refit, select, or rank the accepted ensemble and does not calibrate
absolute canopy amplitude. Camera greenness is a timing and relative-activity
proxy, not an absolute LAI, biomass, or canopy-cover observation.

This package cannot satisfy the roadmap's absolute-amplitude, quantitative
evergreen-floor, or phase-transformed-real-consumer cells with the admitted
evidence. It may close as a completed bounded evaluation while Order 7
advancement remains withheld. It must not be described as completed
Southern Hemisphere robustness or as a pass of Order 7.

## Frozen observational lanes

| Lane | Site / ROI | Latitude | Climate | Role |
|---|---|---:|---|---|
| `SH-DB-BEZA` | bezamahafaly / DB_1000, Madagascar | -23.6558 | tropical savanna, dry forest | deciduous timing and seasonal reversal |
| `SH-EN-ALERCE` | alercecosteroforest / EN_1000, Chile | -40.1726 | cool wet temperate forest | independent persistent-evergreen lane |

These lanes were selected before result execution. Their provisional
PhenoCam products and NASA POWER daily meteorology are frozen by source URL,
retrieval date, and SHA-256 digest in this package. Provisional status is
retained in every interpretation.

## Included scope

- Preserve all 37 CAL-04B accepted timing members.
- Drive the production GSI/canopy kernel with source-provided NASA POWER
  gridded/reanalysis daily temperature and contract-reconstructed
  vapor-pressure deficit at each signed latitude.
- Compare deciduous GSI threshold-crossing chronology with source-provided
  transition dates and compare normalized daily seasonal shape descriptively.
- Exercise a fully evergreen realization as a persistence boundary, while
  refusing to interpret camera color variation as absolute foliage mass.
- Re-run the complete cyclic producer-state phase-transform contract test and
  the separate real direct-production consumer-ordering test. Treat them as
  supporting compositional evidence, not as a phase-transformed consumer run.
- Independently reconstruct daily foliar mass closure.
- Produce accessible plot-only SVG figures with Markdown sidecars containing
  captions, interpretation, methods, limitations, and exact source bindings.

## Excluded scope

- No parameter fitting, member selection, posterior weighting, or
  Northern-Hemisphere retuning.
- No absolute LAI, biomass, canopy-cover, needle, fine-woody, decomposition,
  snow-fidelity, runoff, or erosion calibration claim.
- No assertion that POWER forcing is on-site meteorology or spatially
  representative of either camera footprint.
- No phase-invariance claim for real downstream consumer trajectories.
- No production Rust, canonical contract, fixture, or observation edits.
- No claim that provisional camera greenness is direct foliage mass.

## Authority and dependencies

- `docs/planning/canopy-phenology-assurance-roadmap.md`, section 7.4.
- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`.
- `SC-PLANT-001`, especially INV-PLANT-028--037 and OBL-PLANT-P-013.
- CAL-04B frozen accepted ensemble.
- CAL-05 missing litter-source authority.
- CAL-06 bounded downstream characterization.
- PhenoCam V3 methods and site/ROI products.
- NASA POWER daily point meteorology.

## Intended write set

- This package directory.
- `docs/work-packages/README.md`.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.

Production code, contracts, fixtures, and prior package evidence are
protected.

## Prespecified analysis

1. Admit raw `gcc_90` only when `image_count > 0`,
   `outlierflag_gcc_90 == 0`, and the value is finite. The source-smoothed
   series is not scored.
2. Daily relative-shape scores use the two non-edge scoring years 2024 and
   2025 separately; “scoring year” does not imply gap-free daily camera
   coverage. For each site/year/member, join model and raw GCC90 on exactly
   the admitted camera dates. Require at least 180 paired dates. Independently
   min-max normalize each joined series over those dates as
   `(x - min(x)) / (max(x) - min(x))`; a zero-range series is retained as
   unscored. Report population Pearson correlation and RMSE per
   site/year/member. Only distributions across these retained scores may be
   summarized; partial 2023 and 2026 seasons are plotted but not scored. A
   site/year has directional shape agreement only when its across-member
   median Pearson correlation is strictly positive. Zero or negative median
   correlation is `CONTRADICTED`, not hidden by RMSE or pooling.
3. Use source-provided 50% transition dates as an analogous relative midpoint
   for the deciduous timing proxy; do not invent an observational uncertainty
   or pass threshold. Eligible observed events require a previous and next
   source event in chronological order, excluding edge events with an
   incomplete bracket. Detect modeled crossings at `old < 0.5 <= new` for
   rising and `old >= 0.5 > new` for falling, linearly interpolate the
   fractional day, and assign each observed event the first same-direction
   crossing inside the interval bounded by the temporal midpoints to its
   previous and next source events. Retain zero matches as missing and retain
   the count of extra same-direction crossings; never select the nearest
   crossing by residual.
4. Run every accepted member from 2022-01-01 through 2026-07-24, retaining
   2022 as meteorological warm-up and scoring only camera-supported dates.
5. Calculate VPD in Pa as
   `1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`, using the production
   saturation-vapor-pressure equation. Negative or non-finite values fail;
   they are not clipped.
6. For deciduous realization use evergreen fraction 0; for the evergreen
   boundary use fraction 1. Other canopy-scale operands reuse the native
   contract-test values solely to exercise state and closure, not as
   site-calibrated quantities.
7. Require exact inventory, finite bounded state, daily mass closure within
   `1e-12 kg m-2`, cyclic phase-test passage, and real-consumer-test passage.
8. Keep absolute amplitude, quantitative evergreen-floor agreement,
   phase-transformed real-consumer chronology, and missing
   litter/decomposition sources visibly `NOT_EVALUATED`.

## Advancement logic

The package may support signed-latitude producer-state calendar/phase handling
when both lanes, observed seasonal direction, and cyclic producer-state phase
invariance agree. Observational timing remains `BOUNDED` because the
provisional proxy lacks defensible accuracy uncertainty and the gridded
forcing differs in scale and elevation from each camera site. The real
consumer test can support correct ordering and common-state lineage only.
Absolute canopy amplitude, quantitative evergreen-floor agreement,
phase-transformed consumer chronology, and source-incomplete downstream
physics cannot advance.

Any missing member, data gap concealed as zero, negative reconstructed VPD,
mass-closure failure, phase-test failure, consumer-test failure, or
Northern-Hemisphere retuning forces `HOLD`.

## Validation requirements

- Source and result digest manifests.
- Parameter-level custody from both predecessor CAL-04B tables and complete
  37-member inventory at both sites.
- Independent VPD and mass-closure reconstruction.
- Focused native-canopy producer-phase and real-consumer ordering tests, with
  their distinct evidence ceilings preserved.
- Deterministic analyzer and renderer reruns.
- SVG XML, title/description, contrast/encoding, sidecar, and
  figure-to-table validation.
- Markdown lint and exact terminal-diff reconciliation.
- Two independent prospective reviews before result execution, then two
  independent terminal reviews/verifications with every finding dispositioned.

## Review authorization

Subagent authorization: this package explicitly authorizes spawning or
delegating to two read-only prospective scientific reviewers and two
read-only terminal review/verifier agents. Their write access is limited to
their four named package artifacts. Reviewers must assess source authority,
proxy semantics, frozen analysis, ensemble retention, phase/consumer proof,
negative evidence, figures, validation, and final claim calibration.

## Exit criteria

- Both independent lanes and all 37 members are accounted for.
- All prespecified analyses are reported without member selection.
- Phase, closure, and consumer gates actually ran and passed.
- Every observational and downstream claim carries its evidence class and
  limitations.
- Every figure has a detailed Markdown sidecar and exact data binding.
- Required reviews, finding disposition, validation, roadmap/catalog updates,
  and exact-diff reconciliation are complete.
