# Rejected Numerical Experiments

Status: `DIAGNOSTIC ONLY / not authority`

The WIP checkpoint contains zero-PAR-specific experiments: beta-one anchors,
inactive-coordinate anchors, row scaling, a 200-iteration ceiling, and a
50,000 mm hydraulic trust cap. These experiments did not establish an accepted
root and contradict the provisional LSE-V2 claim that V1 numerics are imported
unchanged. They are preserved only to diagnose conditioning and must not be
used as acceptance evidence.

The exact source delta is preserved verbatim in
`rejected-wip-solver.diff`. It is the tracked diff of
`aa8f55d93d58df6c62b5ae4eebb78245b5469fd6` against parent
`7bda42a5614feb3027b51dfced783eb39e7c37ad` for
`crates/openwepp-land-surface-energy/src/solver.rs`.

An additional uncommitted diagnostic explored authorization-scaled continuity
rows and a nested thermal/hydraulic reduction. It was fully reverted before
authority selection after independent review found that it applied
`beta*g0` during the uncapped potential pass, used provisional tolerances and
brackets, and had no typed infeasibility payload. No result from that
experiment is an accepted fixture or contract value.
