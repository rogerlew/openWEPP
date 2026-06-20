# Process Span Contract

Static:

R5A is not a process-physics port. Its contract is a lifecycle contract:

1. Build one direct run frame for the requested run identity.
2. Execute existing run-level direct spans once.
3. For each day in `0..day_count` and each lane in `0..lane_count`, seed a
   direct day frame from persistent lane state.
4. Execute existing day-level direct spans without compatibility storage or
   request surfaces.
5. Construct canonical phase views for every `DirectPhaseKind::ORDERED` phase.
6. Record phase lifecycle status counts for each canonical phase.
7. Commit typed day state back to the lane frame.

R5A does not authorize public output cutover, default activation, new phase
equations, or scheduler phase-order changes.
