# Implementation And Test Evidence

Status: `executing / focused implementation gates pass`

## Runtime Path

`DirectFrameExecutor::run_publication_stream_with_v9_real_consumer_shadow` is
the sole explicit attachment. It clones the production frame and the complete
shadow owner set, invokes the shadow once per complete OFE day after repository
day-input application and before native hydrology, and publishes either both
successful candidates or neither. The existing scheduler APIs call the same
core with an inert day hook; the runner has no call to the V9 attachment.

Each shadow day executes exactly 48 retained 1800 s transactions. The adapter
projects V9 to the byte-identical V8 physical payload, consumes the strict
Child-3 endpoint, and rebinds the accepted physical state to V9. Live soil
water and temperature operands are reconstructed from the immutable real
hydrology and soil-thermal beginning owners. Tile soil-thermal candidates are
validated as a complete configured set and independently aggregated into one
shared OFE ending snapshot.

## Continuation Defects Found And Corrected

- An exactly dry canopy left the inactive wet-surface temperature equation as
  a zero row. It is now canonically anchored to canopy-air temperature only
  when wet area is exactly zero; wet physics is unchanged.
- Biogeochemistry accumulated receiver material as `(begin+p1)+p2` but
  validated `begin+(p1+p2)`. Continued nonzero receivers could therefore fail
  exact closure. Construction and validation now use the same aggregate-then-
  credit operation.

These were exact continuation contradictions exposed by Child 4. They do not
change V3--V9 authority bytes, constitutive parameters, or supported physics.

## Focused Evidence

Ran in the repository Nix environment:

- V9 real-consumer unit selection: 5/5 PASS. This covers 48 intervals, a late
  interval rollback, retained half-day restart equality, explicit scheduler
  production/output equivalence, and downstream failure rollback after a
  complete shadow candidate exists.
- BGC continued receiver aggregation regression: 1/1 PASS.
- affected four-crate all-target warnings-denied Clippy: PASS.
- affected cargo check and rustfmt: PASS.

Heavy, benchmark, independent review, and terminal evidence remains pending.
