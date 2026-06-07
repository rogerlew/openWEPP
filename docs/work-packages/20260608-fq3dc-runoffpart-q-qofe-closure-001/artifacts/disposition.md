# Disposition

Status: complete

Evidence mode: Static + Ran.

## Decision

`FQ3-DC-RUNOFFPART-QQOFE-001` is closed.

The in-envelope root cause was WB12/WB14 same-pass over-infiltration and later
WB14/WB18 producer-consumer inconsistency. `SC-RUNOFFPART-001` v39 now defines
the required storage-limit and producer-consumer identity. Production code and
contract tests implement that authority.

## Acceptance Criteria

- Nonzero infiltration-excess `Q/QOFE` on affected population: satisfied
  (`42/42` runnable prefixes nonzero).
- Conservation closure preserved: satisfied (annual WAT max abs residual
  `2.808064891723916e-11 mm`).
- Contract-derived tests: satisfied.
- No comparator matching: satisfied.
- No protected-boundary edits: satisfied.
- Dual review and dual verification: satisfied.

## Finding Disposition

- Accepted Review A finding 1: fixed and verified.
- Accepted Review A finding 2: fixed and verified.
- Accepted Review B finding 3: fixed.
- Rejected Review B findings 1 and 2: no change required; rationale recorded.

No undispositioned findings remain.
