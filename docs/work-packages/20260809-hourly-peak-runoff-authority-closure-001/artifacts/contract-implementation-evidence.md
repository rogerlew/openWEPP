# Contract Implementation Evidence

Status: `complete`

Evidence mode: `Static`

`SC-WATBAL-001` v170 adds `INV-WATBAL-102..104` and replaces the WB16
rainfall-envelope/APPMTH production rules with a closing post-partition hourly
peak, in-hour surface-return custody, exact dry-zero behavior, depth-rate
internals, exactly-once public area conversion, and rectangular-equivalent
duration. `TOL-WATBAL-009` declares the bounded 24-interval arithmetic
reconciliation and prohibits using it to absorb missing sources or timing.
Its `1e-9 m` per-interval provenance is bound to
`SC-RUNOFFPART-001#TOL-RUNOFFPART-007`; the exact 24-interval aggregate
boundary is tested, and no reconciliation residual mutates an hourly bin.

Routed melt and runon are producer-timed hourly liquid supply admitted through
WB14 exactly once. They are not appended as raw post-partition runoff limbs;
normalized hourly weights are derived only after the depth ledger closes.
Daily-only frost retention may clear the whole local series when reconciled
runoff is zero. Partial frost retention leaving positive runoff has no lawful
hourly timing in the current producer and therefore hard-fails instead of
being allocated across runoff bins.

WB14 is the sole owner of cumulative same-pass infiltration and hourly excess.
The former daily-only snow infiltration reconstruction is retired rather than
debiting earliest hourly bins without a producer clock; this prohibition also
applies to local-only supply. Every finite positive additional-supply bin enters
WB14, invalid bins fail, and an empty hourly ledger can never satisfy a positive
daily runoff scalar through `TOL-WATBAL-009`.

`SC-INFILE-HBP-001` v0.2.5 also binds EVENT calendar year and Julian day to the
selected producer row. The p61 and p102 consumers join the Parquet event by
simulation year plus Julian day before reconstructing volume and peak.

The lifecycle registry is current at `2026-08-09`.
Historical `GAP-WATBAL-005` is explicitly superseded: no active peak path
consumes `ealpha`, and retained manifest fields report
`false`/`retired_not_applicable` only for schema lineage.
