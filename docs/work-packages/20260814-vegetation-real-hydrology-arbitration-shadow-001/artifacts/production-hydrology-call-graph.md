# Production Hydrology Call Graph

Evidence class: `Static`

The executable production owner is `DirectFrameExecutor` operating on a
seeded `DirectDayFrame`, with persistent state in `DirectLaneFrame`.

```text
DirectLaneFrame::seed_day_frame
  -> DirectFrameExecutor::run_day_spans_hydrology
     -> normalization and storage bounds
     -> decomposition, residue, growth and storage input
     -> frost ingress
     -> R4I liquid input
     -> R4J runon
     -> R4K infiltration/depression storage
     -> R4M percolation
     -> native R4N surface ET
     -> R4O drainage/lateral flow
     -> native R4N root uptake
     -> snow coupling and saturation addback
     -> runoff partition, peak runoff and WAT5
     -> storage reconciliation and hydrology projection
  -> DirectLaneFrame::commit_day
```

The exact immutable arbitration snapshot is the freshly seeded day frame
before `run_day_spans_hydrology`. Current rain, runon, infiltration, runoff,
percolation and lateral flow have not yet occurred at that point.
