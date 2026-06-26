# Disposition

Evidence class: Static + Ran.

Status: complete.

Closure marker: COMPLETE-05D-OPT-IN-COE-MELT.

## Summary

SNOWDENSITY-05D scaffolded and executed the opt-in CoE melt implementation.
The package amended the snow/freeze contract, wired the selector into typed
hydrology, preserved `legacy_coe` default behavior, carried direct runtime
albedo and melt-lineage operands, and added focused reconstruction tests.

## Closure Evidence

- Contract v79 markers present and tested.
- Opt-in `amelt` independently reconstructs from radiation, albedo, and canopy.
- Raw melt, redistributed melt, routed melt, SWE loss, WB12 `S`, and WB13 liquid
  forcing are reconstructable in the focused fixture.
- Missing active opt-in albedo state fails closed.
- Direct runtime projects routed melt and albedo carry.
- Full workspace, deny, and anti-evasion gates passed.

## Deferred Work

SNOWDENSITY-05E should adjudicate the opt-in path against the snow/frost rubric.
No activation or coefficient changes should happen before that evidence exists.
