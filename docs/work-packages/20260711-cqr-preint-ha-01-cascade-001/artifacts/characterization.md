# Characterization

Evidence class: **Ran**

Added one inline interpolation table covering empty input, singleton
before/exact/after, the degenerate-span fallback, exact endpoints, both binary-
search update directions, interior interpolation, and non-negative output
clamping. Expanded `degenerate_cascade_fails_closed` over widths `0`, `-1`,
`NaN`, positive infinity, and negative infinity at both `run_cascade` and
`route_single_ofe`, with exact `DegenerateConfiguration` matching.

`cargo nextest run -p openwepp-hillslope-orchestrator -E
'test(/cascade::tests/)'` passed 7/7 twice (implementer run and parent rerun).
The delegated full-library coverage executions each passed 341/341.
