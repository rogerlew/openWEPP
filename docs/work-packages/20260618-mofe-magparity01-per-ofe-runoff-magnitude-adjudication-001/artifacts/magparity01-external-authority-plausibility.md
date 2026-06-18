# MAGPARITY01 External-Authority Plausibility

Evidence mode: **Static** (local WEPP documentation and annotated primary
references) + **Ran** (H2637 magnitude arithmetic).

## H2637 Magnitude Frame

H2637 precipitation over 34 years is `96,057.2 mm`, or `2,825 mm/yr`. openWEPP
exports:

- surface/routed `runvol`: `68,204.1 mm` basin-equivalent, `2,006 mm/yr`;
- terminal lateral `sbrunv`: `4,285.0 mm` basin-equivalent, `126 mm/yr`;
- combined: `72,489.1 mm` basin-equivalent, `2,132 mm/yr`.

This is high, but it remains below precipitation and the run manifests close the
water-balance identities.

## Authority Read

WEPP Chapter 4 describes surface hydrology as an infiltration, rainfall-excess,
depression-storage, and peak-discharge sequence. Rainfall excess occurs only
when rainfall rate exceeds infiltration rate, and depression storage can reduce
runoff before discharge begins. This supports treating local surface runoff as a
bounded partition term, not as a comparator-matching target.

WEPP Chapter 5 states the daily water balance includes precipitation, snow,
surface runoff, evapotranspiration, percolation, and subsurface lateral flow.
It also states infiltrated water enters the upper soil layer and is routed
through lower layers, where percolation and lateral flow are separate process
families. This supports adjudicating the H2637 delta as a coupled lateral/
subsurface magnitude question after transfer closure passes.

The repository's annotated primary references record WEPP forest-hydrology
authority:

- Dun et al. (2009) documents WEPP forest adaptations for deep percolation,
  lateral flow, and hillslope-to-channel transfer, including explicit subsurface
  flow transfer to channel flow. See
  `references/annotated_bibliography.md#r-21-dun-et-al-2009-wepp-forest-application-adaptation`.
- Srivastava (2013) records WEPP watershed runoff assembly as combining
  hillslope surface runoff and subsurface lateral flow as channel inflow; Priest
  River simulations can have no simulated surface runoff while outlet streamflow
  originates from subsurface lateral flow/baseflow. See
  `references/annotated_bibliography.md#r-22-srivastava-2013-dissertation-on-wepp-groundwater-baseflow-integration`.

## Plausibility Verdict

The external authority supports large subsurface/lateral contributions in forest
WEPP applications. It does **not** provide a hard coefficient saying H2637 must
be 55.5%, 71.0%, or any other exact fraction. Therefore:

- openWEPP's bounded 71.0% `runvol` is not physically impossible from magnitude
  alone;
- the comparator delta remains a valid investigation signal;
- the owner is Stage-2 lateral/subsurface magnitude authority, not INV-028
  transfer closure or export scaling.
