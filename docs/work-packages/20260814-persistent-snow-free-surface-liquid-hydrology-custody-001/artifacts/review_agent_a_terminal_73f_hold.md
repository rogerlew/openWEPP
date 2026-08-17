# Rust Correctness Review — `73f22169a`

Evidence: `Static` plus focused `Ran` evidence.

Verdict: `HOLD`.

Accepted findings:

1. Arithmetic preflight could stop at an E010 parcel identity mismatch before
   scanning a later E003 arithmetic failure.
2. Derived post-ingress LSE/soil-thermal E003 failures still used a hydrology
   beginning digest through `ReceiverFailureScope`.
3. The winter-domain preflight covered only top-level snow fields, not nested
   snow/albedo/layer/frost/runtime-carry domains.

The correction makes identity-only parcel joins nonterminal during arithmetic
preflight, binds derived receiver failures to a unique `(OwnerKind, owner_id)`
rollback row or typed absence and delegates snow-free admission to the complete
production winter-lane domain validator.

No broad or heavy gate was run by the reviewer. Fresh exact-byte review remains
required after the correction is committed.
