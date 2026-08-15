# Rust Correctness Review — `a5c2243e6`

Evidence: `Static` plus focused `Ran` evidence.

Verdict: `HOLD`.

Accepted findings:

1. `DirectSurfaceLiquidIngressCandidate::validate()` ran closure E003
   arithmetic before whole-input E001/E002 identity preflight.
2. Temporal parcel mass used ratio multiplication for every child. Five equal
   windows of a 0.1 kg parent summed to `0.10000000000000002`, while the
   independent replay duplicated the formula and derived its raw total from
   those same children.
3. Exact-head line-count evidence was stale, although every affected file was
   still below the blocking threshold.

The corrections share the identity-only ingress preflight with execution,
assign the canonical final temporal child the exact parent-mass remainder and
reconstruct raw source/OFE mass only from frozen parent operands. Tests bind
the five-window bits, reject a one-ULP child poison and exercise mixed public
E001/E002 plus closure-E003 precedence. The line-count inventory is reconciled
to the corrected worktree.

No broad or heavy gate was run by the reviewer. Fresh exact-byte review remains
required after the correction is committed.
