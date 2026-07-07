# Disposition

Status: EXECUTED-COMPLETE

Outcome:

- Closed the selected-cohort active-suite row-crop canopy-height blocker.
- `SC-PLANT-001` rev 19 now makes daily `canhgt/Hc` a PL16 growth-state
  publication using the legacy WEPP equation.
- `SC-OFEROUTE-001` rev 36 now binds Lane D dynamic `h_c` to post-growth plant
  state, not stale static typed-management height.
- Active and shadow Lane D routing operand builders consume the same
  post-growth day-frame canopy-height surface.
- The former `mn_corn_h4` active plain day-136 failure is gone, and the full
  selected cohort completed in active plain and active explicit hybrid modes.

Not claimed:

- No hybrid default selector promotion landed.
- No route-coefficient or disturbed-management parameter changes landed.
- No Wave-1 erosion consumer change landed; an attempted broader move failed
  the p61 sediment gate and was reverted.
