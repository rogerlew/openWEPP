# ADR-0014: Do not implement legacy snow drift routine (`sndrft.for`)

**Status:** Accepted
**Date:** 2026-05-26
**Deciders:** Roger Lew, Codex

## Context
openWEPP migration authority is anchored to
`/workdir/wepp-forest_260430_baseline` (ADR-0012) and requires
baseline-authoritative process behavior rather than speculative reactivation.

In the baseline winter driver, snow-drift invocation is explicitly disabled:
- `winter.for:313` comment: `we are not currently simulating snow drift 8/1/94`
- `winter.for:314-315` commented-out branch and `call sndrft(...)`
- `winter.for:325-328` drift terms are set to zero (`driftf`, `driftg`,
  `fdrft`, `gdrft`)

This indicates the authoritative baseline behavior is "snow drift inactive,"
not "snow drift active but unported."

## Decision
1. openWEPP will not implement `sndrft.for` process physics in production
   kernel/runtime paths.
2. Snow-drift terms remain explicitly inactive (zeroed) in parity scope unless
   a future authoritative baseline changes this posture.
3. Any future request to activate snow drift requires a new ADR that includes:
   - authoritative provenance showing active legacy behavior,
   - contract authority amendments,
   - contract-derived tests and parity evidence plan.

## Consequences
- Snow drift is not a migration blocker for current parity objectives.
- Engineering effort remains focused on active baseline process families.
- Comparator discussions should treat snow-drift differences as out-of-scope
  unless this ADR is superseded.
