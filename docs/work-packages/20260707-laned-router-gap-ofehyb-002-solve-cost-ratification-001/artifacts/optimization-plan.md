# Optimization Plan

Status: EXECUTED. Evidence mode: Static + Ran.

## Bottleneck

Refreshed H2637 source-memory hybrid baseline:

- User/wall: `38.39 s` / `0:38.41`.
- Implicit steps: `980804`.
- Equilibrium map evaluations: `151435969`.
- Branch evaluations: `20110816`.

This matches the `GAP-OFEHYB-001` handoff shape: the endpoint is dominated by
bare-skin implicit equilibrium fixed-point maps, not by switching, composition,
or output publication.

## Selected Lever

Implement an exact direct evaluator for the bare skin-only equilibrium rating:

- Laminar Shen-Li branch:
  `q = 8 g S h^3 / ((rain_term + k_o) nu)`.
- Turbulent Hirsch branch:
  `q = (sqrt(8 g S / 3.19) * nu^-0.225 * h^1.5)^(1/0.775)`.
- Preserve deterministic seed-side branch preference when both branch fixed
  points are in regime; otherwise migrate only to the single valid in-regime
  branch.
- Execute only after ordinary finite/non-negative cell-parameter validation and
  only when no form, wave, vegetation, or Manning addend is active.
- Record zero `implicit_equilibrium_map_evaluations` for direct branch values
  because no fixed-point map application occurs.

Authority: `SC-OFEROUTE-002` rev 4 / `INV-OFEHYB-003`; parent pointer
synchronized in `SC-OFEROUTE-001` rev 35.

## Rejected Alternatives

- Composed Newton solve over the whole cell residual: retained as future value,
  but higher blast radius and not needed once H2637 proves entirely removable
  map work on the active fixture.
- Near-zero addend thresholds: rejected; no contract authority exists for
  tolerance-based operand absence.
- Generic form/wave/vegetation direct evaluators: deferred; they require their
  own branch equations/proofs and are not needed for the current H2637 solve
  bottleneck.
- Mesh policy/tolerance changes: out of scope for this package.

## Expected Movement

Expected endpoint movement was complete removal of equilibrium map work on the
H2637 active hybrid vector and a material user-time reduction without changing
selector/default posture. The executed result met that expectation:

- Map evaluations: `151435969 -> 0`.
- User time: `38.39 s -> 33.37 s`.
- Wall time: `0:38.41 -> 0:33.43`.
