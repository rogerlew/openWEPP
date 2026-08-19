# API redesign disposition

Evidence class: `Static design`; implementation not started.

The closure-eligible scheduler callback must ultimately return only
`PreparedSnowFreeGsiDayV1`. `DirectV10ShadowDayInput` and all nested physical
forcing types must disappear from the public V10 seam. The shadow must construct
a private, digest-bound interval projection from current owners at every
interval, execute on a candidate, and replace the complete shadow only after all
fallible work and buffered publication succeed.

This design is blocked before implementation by the missing matric-potential
and explicit root path/gravity owner described in `live-owner-hold-intake.md`.
The API must not be changed to imply completeness while those fields remain
caller-owned, fixture-owned, silently defaulted, or derived by an unreviewed
equation.
