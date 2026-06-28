# ADR Consistency Check

Evidence class: Static.

## Checked ADRs

- ADR-0011: Accepted; establishes architecture-first, contract-first science
  authority and comparator distrust (`docs/decisions/0011-architecture-first-top-down-science-contracts.md:16-54`).
- ADR-0017: Accepted; comparator is a flag, not target
  (`docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md:63-95`).
- ADR-0025: Accepted; array-native hot-path runtime authority with workspace,
  performance, and markdown gates (`docs/decisions/0025-array-native-hillslope-day-frame.md:24-80`).
- ADR-0026: Accepted; winter-column sub-solver is the authorized snow/frost
  stateful exception (`docs/decisions/0026-stateful-winter-column-sub-solver.md:30-59`).
- ADR-0028: Proposed before this package; observed-data admission tier below
  derivable science and above comparators (`docs/decisions/0028-observed-data-admission-authority.md:44-87`).

## Result

No conflict found.

ADR-0028 extends ADR-0011 and reaffirms ADR-0017 rather than replacing either.
It admits only physically defensible, conserving, no-fit candidates when
established authority is insufficient and a forcing-robust observed-data rubric
exists.

ADR-0029 uses ADR-0028 only as admission framing for a staged, opt-in program.
It does not activate a default, change physics, change public schemas, or relax
ADR-0025 performance constraints. It homes future snow-layer state in the
ADR-0026 winter-column exception and preserves the bulk default as rollback.
