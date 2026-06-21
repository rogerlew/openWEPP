# Disposition

Evidence mode: Static + Ran.

Status: executed-held.

Verdict:

`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`

R6D lifted the R6C blocker for cutover producer retention. The production
climate lifecycle now retains a direct run/lane/day publication frame for
`DirectPublicationFrameCutover`, and cutover consumes that retained frame
without constructing a skeleton direct frame or running post-hoc publication
capture.

R6D does not complete R6. The retained frame currently carries only parsed
climate/calendar/geometry authority plus zero/absent placeholders for required
hydrology, storage, subsurface, evaporation, PASS, loss, manifest, and erosion
publication families. The cutover path therefore fails before public writes with
the R6D hold marker.

Known blockers:

- parity-grade direct hydrology/storage/subsurface/evaporation publication
  producers are absent;
- direct PASS/loss/manifest authority is absent;
- independent reconstruction and anti-alias fixtures are not yet present for
  accepted output families;
- `00_runner_intake_and_lane_setup.rs` remains above the 3000-line closure
  threshold and needs a direct-publication helper split before complete R6
  closure.
