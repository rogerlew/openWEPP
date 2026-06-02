# PL Runtime Projection Diagnosis

Status: complete

Evidence mode: static + ran

Static:

- `pltol` is retained as a growth/runtime symbol with default `0.25`.
- Scheduler activation now preserves the PL schedule slot count and selects the
  active slot/crop from management-present runtime state.
- Established perennial `jdplt=0` / `jdstop=0` slots remain active across the
  simulation year, matching baseline `ptgrp` discoverability semantics.
- Initial live canopy now seeds live PL state for established perennial
  no-growth inputs: `vdmt`, `lai`, `rtd`, `rtmass`, and compatible
  `sumgdd`.

Ran:

- `hphys0250_pl_activation_keeps_zero_date_perennial_slots_active` passed.
- `hphys0250_zero_date_perennial_slot_remains_active_for_growth_dispatch`
  passed.
- `management_runtime_projection_assimilates_initial_perennial_live_canopy`
  passed.
- H1 trace confirmed nonzero `pl_lai`, `pl_vdmt`, and `pl_rtd` during ET
  phases.

Open risk:

- This package did not complete baseline-authoritative crop-growth dynamics for
  all management classes. It only closed the H1-style established-perennial
  initial live-canopy activation needed to move Ep from missing to nonzero.
