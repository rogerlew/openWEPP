# Implementation And Test Evidence

Status: `executing HOLD / focused implementation gates pass`

## Runtime Path

`DirectFrameExecutor::run_publication_stream_with_v9_real_consumer_shadow` is
the sole explicit attachment. It clones the production frame and the complete
shadow owner set. The unchanged stateful production builder runs once in its
original lane-interleaved order. Each actual input and applied pre-native day
frame is retained; after the complete set exists, the shadow executes against
its isolated hydrology. The higher-frequency callback cannot read the
post-native production frame. Rows are buffered and exposed only after every
shadow day accepts. Only a complete successful call replaces production and
shadow candidates. The runner has no call to the V9 attachment; laned-active
routing is typed unsupported rather than skipped.

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

- V9 real-consumer unit selection: 9/9 PASS. This covers 48 intervals, a late
  interval rollback, retained half-day restart equality, explicit scheduler
  production/output equivalence, repository receipt poisoning, active-routing
  rejection, shared-soil candidate poisons, mixed owner lineage, and downstream
  failure rollback after a complete shadow candidate exists.
- BGC continued receiver aggregation regression: 1/1 PASS.
- LSE fully-wet inactive sun/shade/stem node regression: PASS within the frozen
  joint covered-column test.
- V8-to-V9 wrong model/configuration/state receipt poisons: PASS.
- affected four-crate all-target warnings-denied Clippy: PASS.
- affected cargo check and rustfmt: PASS.

## Open Closure Boundary

No repository component currently provides the complete higher-frequency
snow-free receipt. Daily climate supplies precipitation, extrema, daily
radiation, wind and dew point plus rainfall breakpoints, but not the exact 48
step LSE radiation/humidity/canopy/thermal/WB14 operands. The module fixture is
not valid real-provider evidence. Deriving those operands here would introduce
unauthorized physics. The complete checkpoint also retains the nonserializable
production hydrology frame by typed clone; it is not persisted-restart proof.

Heavy, fresh benchmark, terminal review, and closure evidence is withheld.
