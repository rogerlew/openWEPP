# Kickoff

Execute `20260630-typed-day-zero-seed-computation-001`.

Build the typed day-zero seed computation from typed parse state and day-one
climate directly. Do not call `seed_wb11_runtime_surface_inputs` and copy its
surface results into a typed carrier except as a shadow comparator. Implement
sub-computations incrementally, shadow-prove each one against the current
day-zero surface, and only cut over consumers after full value identity exists.

If a typed sub-computation cannot be implemented without first factoring a
deeper parsed-input projection API, close HOLD with that exact missing API and
the first actionable factor step. Do not create a typed wrapper around
`HillslopeWritebackSurface` and call it single-authority.
