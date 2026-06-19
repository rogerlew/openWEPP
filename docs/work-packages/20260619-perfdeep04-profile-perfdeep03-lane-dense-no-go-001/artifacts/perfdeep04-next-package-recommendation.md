# PERFDEEP04 Next-Package Recommendation

Evidence class: Ran + Static.

## Recommendation

Open a follow-on implementation package:

```text
PERFDEEP05 - Lane-Dense Transfer Authority and Sync Removal
```

Objective: remove the measured PERFDEEP03 resynchronization hotspot by making
lane-dense state the direct authority for transfer application and hot hydrology
carryover, with cached hot slot metadata and indexed/dense writeback application.

## Why This Is The Next Cut

PERFDEEP04 measured `HillslopeLaneDenseState::sync_from_writeback_surface` at
`33.49%` inclusive in the opt-in profile. The current sequence is:

1. apply transfer input to logical/indexed surfaces;
2. resynchronize lane-dense state from those surfaces;
3. execute the dense-enabled scheduler;
4. flush dirty dense slots back to logical/indexed surfaces.

That is not true dense authority. It is dense state nested inside a
logical/indexed compatibility loop.

## Proposed PERFDEEP05 Scope

In scope:

- replace `apply_transfer_input_to_lane_surface` plus full
  `sync_from_writeback_surface` in the PERFDEEP03 opt-in path with a direct
  dense transfer update;
- keep logical/indexed transfer materialization only for non-migrated consumers,
  diagnostics, and publication boundaries;
- precompute and carry hot state/flux symbol vectors or slot-id lists instead of
  rebuilding them via `HotSymbolTables::hot_state_symbols` in the daily loop;
- prefer indexed/dense writeback application so dense updates do not call
  `SymbolRegistry::id_of`;
- remove `perfdeep02_apply_logical_frame_writeback` from the lane-dense hot path
  unless a real boundary requires it;
- preserve PERFDEEP03 output identity and default-disabled posture.

Out of scope:

- whole-simulation dense array conversion;
- erosion/growth island expansion;
- output schema changes;
- default activation before the endpoint gate is passed;
- deleting logical surfaces globally.

## Acceptance Gate

PERFDEEP05 should require:

- focused tests proving transfer input mutates lane-dense state directly;
- static proof that `sync_from_writeback_surface` is not called in the opt-in
  H2637 daily hot loop;
- H2637 identity: HBP/WAT byte identity and PASS Arrow equivalence;
- real H2637 opt-in endpoint and RSS measurement versus `669.97 s`;
- a matched short or full profile confirming the resync hotspot is gone.

## Rejected Alternatives

- **Revert PERFDEEP03 and start from scratch.** Rejected. PERFDEEP03 gave useful
  dense-read wins (`state_value_for_symbol` fell from `14.83%` inclusive in
  default to `3.80%` in opt-in). The failure is localized to sync/apply/flush
  compatibility edges.
- **Expand the hydrology island immediately.** Rejected. The current shape pays
  a measured `33.49%` inclusive resync cost before expansion. Expanding first
  risks amplifying the wrong boundary.
- **Make the entire simulation dense now.** Rejected as the next immediate step.
  The profile says the first practical cut is removing the current
  logical-to-dense-to-logical loop. A whole-simulation dense conversion should
  be considered only after this measured edge is removed or proven insufficient.
- **Optimize physics kernels first.** Rejected for this track. The profile is
  still dominated by representation and compatibility costs, not arithmetic.

## Expected Result

Removing `sync_from_writeback_surface` cannot by itself guarantee `< 669.97 s`;
the opt-in run is `459.49 s` slower than default under matched profiling and
sync is about one third of opt-in cycles. It should, however, determine whether
the lane-dense island can become endpoint-flat/positive after the measured
resync edge is gone. If not, the next profile will have a cleaner target.
