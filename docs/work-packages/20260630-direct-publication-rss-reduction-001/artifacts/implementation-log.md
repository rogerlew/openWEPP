# Implementation Log

Evidence class: Static + Ran

## Code Changes

- Added `DirectLaneConstructorInputs::from_topology_with_dynamic_day_inputs`.
  This constructs topology-only lane inputs with an empty `day_inputs` vector.
- Relaxed direct lane constructor validation to accept either:
  - an empty `day_inputs` vector for dynamic day-input construction, or
  - the historical full-length vector when tests or non-production callers
    intentionally provide prebuilt day inputs.
- Changed the production direct run-frame builder to use the dynamic
  day-input constructor, avoiding the H2637-scale zero-filled
  `DirectDayConstructorInputs` allocation.
- Changed `build_direct_publication_artifacts` to take the retained direct
  publication execution with `.take()` instead of cloning it.
- Changed direct publication artifacts so `wat_rows` and
  `pass_projection_rows` are optional and are built only when the runfile
  requests those outputs.
- Updated direct publication validation and optional-output writers to
  fail-closed if a requested optional output has no projection rows.

## False Start

An initial implementation cleared each lane's preallocated `day_inputs` vector
after construction. That preserved identity but only reduced H2637 full RSS from
`1159672 KiB` to `1110524 KiB`, because the peak allocation had already
occurred. The final implementation avoids the allocation at construction time.

## Behavior Boundary

The change is representation-only:

- No physics path was edited.
- No output schema was edited.
- No runtime-selection policy was edited.
- No compatibility runtime path was made reachable.
- Unknown/missing requested WAT/PASS rows fail closed.

The production direct executor still constructs per-day inputs dynamically and
still publishes the same HBP/WAT/PASS/loss/plot bytes for the measured H2637
full-output fixture.
