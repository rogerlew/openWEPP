# ADR-0030: R7 terminal contract and compatibility runtime deletion

**Status:** Accepted
**Date:** 2026-06-30 UTC
**Deciders:** Roger Lew, Codex
**Amends:** [ADR-0025](0025-array-native-hillslope-day-frame.md),
[ADR-0026](0026-stateful-winter-column-sub-solver.md)
**Evidence:** [FROST RATIFICATION AND DEFAULT ACTIVATION](../work-packages/20260629-frost-ratification-default-activation-001/),
[FROST DIRECT CUTOVER CORRECTION](../work-packages/20260629-frost-direct-cutover-correction-001/),
[COMPATIBILITY RUNTIME DELETION](../work-packages/20260630-compatibility-runtime-deletion-001/)

## Context

R7 originally required public-output bit/Arrow identity against the legacy
compatibility runtime before default activation or compatibility deletion. That
was correct while the direct runtime was still a mechanical rewrite and the
compatibility frost path was treated as an output oracle.

The frost validation arc changed the authority. `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-047/048/050` are accepted, `GAP-SNOWFREEZE-002` is
open-but-attributed/bounded, and the production no-env hillslope default now
selects direct production for all supported surfaces. The legacy compatibility
frost solver is no longer the acceptance target; it is an ADR-0017 comparator
flag.

The remaining symbol-map runtime therefore survives only as an explicit
diagnostic/replay seam. Keeping its obsolete skeleton, shadow, and cutover
transition modes in production code adds confusion and carries a stale fallback
risk without providing default-path authority.

## Decision

Amend the R7 terminal contract:

- Frost-influenced bit/Arrow divergence from the compatibility runtime is not a
  blocker after observed-data frost ratification, default-direct activation, and
  full no-regression gates pass.
- Production direct mode is the normal hillslope execution path. It must remain
  selected once at run setup and must not silently fall back to compatibility
  because of input shape, multi-OFE/Wave-2 topology, sidecar discovery, or snow/
  frost activation.
- Delete obsolete production-transition modes and hot-loop symbol-map machinery
  once static call-graph proof, zero compatibility-edge counters, and output
  identity/no-regression gates show direct behavior is unchanged.
- Retain the explicit `--compatibility-runtime` replay/comparator seam for now.
  It is diagnostic and deprecated, not a production fallback. Full removal of
  the seam requires a separate package.

## Consequences

Positive:

- Removes the stale premise that compatibility frost bit-parity is required for
  direct default activation.
- Makes accidental compatibility fallback a defect instead of a tolerated
  runtime policy.
- Keeps a cheap replay oracle while the direct runtime and RSS work settle.

Negative / costs:

- The compatibility seam remains as deprecated code until a later full-deletion
  package removes replay needs or replaces them with direct-native diagnostics.
- Setup-time carriers that still reuse compatibility-shaped types need separate
  typed-setup work before every symbol-map type can be deleted.

