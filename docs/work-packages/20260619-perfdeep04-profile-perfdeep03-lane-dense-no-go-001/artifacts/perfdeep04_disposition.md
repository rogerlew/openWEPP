# PERFDEEP04 Disposition

Evidence class: Ran + Static.

Disposition: `PROFILED - cut PERFDEEP05 at lane-dense sync removal`.

## Summary

PERFDEEP04 scaffolded and executed a matched profiling package for the
PERFDEEP03 H2637 no-go. It captured a full opt-in lane-dense profile and a
default-disabled comparison profile with the same `perf record` settings.

The result is decisive enough for the next package boundary: PERFDEEP03's
regression is dominated by `HillslopeLaneDenseState::sync_from_writeback_surface`
and related compatibility-edge work. Dense reads helped, but the code still
roundtrips through logical/indexed surfaces around the dense state.

## Decision

Open `PERFDEEP05 - Lane-Dense Transfer Authority and Sync Removal`.

Do not:

- default-activate PERFDEEP03;
- revert PERFDEEP03 wholesale;
- expand the island before removing the measured sync edge;
- start a whole-simulation dense conversion as the immediate next step.

## Closure

This package is complete as a profiling/decision package. It does not claim a
performance fix and it does not authorize production default activation. Its
output is the next implementation boundary.
