# Consumer-Path Audit

Status: **COMPLETE** (Static, pre-implementation).

## Current Default/Off Path

- Producer: `DirectFrameExecutor::dc01_surface_transfer_weights` recomputes
  DC01 source-shape weights from WB14 hourly excess, hourly saturation carry,
  and the D12 routed-melt limb.
- Frame state: `DirectDayFrame.wave1_hourly_weights` currently holds the
  selected Wave-1 water shape.
- Erosion substrate: `DirectDayFrame::r7d8_assemble_wave1_continuity_from_frame`
  calls `r7d8_surface_hourly_weights`, stores `wave1_hourly_weights`, and
  passes the same weights to `build_wave1_hourly_plan`.
- Downstream erosion handoff: `publish_erosion_inflow_to_downstream` forms
  prior-lane `qout_h` from `peak.q_runoff_m * wave1_hourly_weights[h]`.
- HBP EVENT: `build_hbp_output_from_direct_publication_summary` consumes
  `DirectPublicationErosionOperands.hourly_runoff_fraction` paired with
  `hourly_sediment_mass_kg`; `assemble_hbp_event_sediment_surfaces` forms
  `V_h = runvol * hourly_runoff_fraction[h]`.

## D13 Consumer-Seam Finding

The existing path is internally consistent for default/off DC01 authority, but
the same DC01 source weights currently feed the Wave-1 hourly plan and HBP
`V_h` whenever Wave-1 is enabled. That cannot carry the active-routed-water
claim after Lane D routing owns the surface-water path.

## Required Change

Add an explicit erosion hydrograph-shape authority selector. Default/off keeps
the DC01 source-shape path byte-flat. The routed-hydrograph authority requires
a supplied finite non-negative shape; positive-runoff shapes must close to
unit sum before the hourly plan and publication use them, and no-runoff shapes
must be all-zero.
