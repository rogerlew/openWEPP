# Characterization

Evidence label: Static.

Status: `SCAFFOLDED`

Planned characterization targets:

- `LanedShadowCollector::observe_row`
- `LanedShadowCollector::validate_lane_day_operands`
- `LanedShadowCollector::commit_day`
- `LanedShadowCollector::finalize` as a threshold-adjacent row

Behavior oracle:

- Unit or focused tests should prove same-day buffering, day-change commits,
  route-source reconstruction, uniform-shape counters, finite/non-negative
  operand validation, and summary finalization without changing formulas or
  selector behavior.
- Existing H2637 tests remain the higher-cost output-identity oracle when a
  unit test cannot prove the relevant behavior.

No characterization commands have been run for this package yet.
