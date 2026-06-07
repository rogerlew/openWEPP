# Corn ET Engagement Localization

Status: complete

Evidence mode: Static + Ran.

## Symptom

Ran: Pre-fix p8 Corn output reproduced the FQ-3 symptom: `Ep=0`,
`Interception=0`, and plant growth state remained zero through growing-season
days. The p1 perennial Tah path had nonzero plant state and nonzero `Ep`, proving
the hydrology/ET consumer path was reachable in openWEPP.

Static: Upstream FQ-3 classification showed annual Corn prefixes had zero
openWEPP `Ep` while pinned legacy produced material Corn transpiration on the
same forcing (`p8` legacy `Ep=1830.76 mm`). The same upstream classification
showed `Er=0` for both legacy and openWEPP on p8/p1 and classified that term as
`expected-config-zero`.

## Root Cause

The in-envelope root cause had two required parts:

1. `prepare_pl_runtime_activation_for_scheduler` removed
   `pl_schedule_slot_count` during annual pre-plant days. That turned a
   day-local pre-plant skip into permanent loss of the PL activation sentinel,
   so annual Corn never activated later in the rotation. Perennial p1 uses a
   zero-date active slot and was unaffected.
2. The scheduler-facing runtime calendar symbol `day` came from generic climate
   projection as day-of-month. Annual PL scheduling compares against Julian
   planting day (`jdplt`), so the annual activation predicate saw `1..31`
   instead of `1..366` and remained pre-plant.

After fixing those, Corn activation exposed a downstream WB15 guard mismatch:
plant state could legitimately exceed `vdmt=0.8 kg m^-2`, but the interception
consumer rejected it. Static pinned-baseline inspection of
`/workdir/wepp-forest_260430_baseline/src/idat.for:286-291` showed the legacy
interception equation caps only the equation input (`livems/deadms`) at
`8000 kg ha^-1`; it does not cap plant live-mass state. The production fix
therefore allows finite non-negative `vdmt` and caps only the Chapter 5
interception biomass input.

## Ownership

Ownership is openWEPP annual PL runtime activation and WB15 plant-state consumer
guarding:

- Legacy transpires Corn on the same climate and run window.
- openWEPP perennial p1 path already transpires and was not the failing branch.
- The fix is contract-backed by `SC-PLANT-001`, `SC-EVAP-001`,
  `SC-WATBAL-001`, `SC-RUNOFFPART-001`, and pinned-baseline `idat.for`
  provenance for the interception biomass cap.

No comparator magnitude tuning, runoff partition edit, p11 percolation edit,
snow magnitude edit, or MOFE edit was used.
