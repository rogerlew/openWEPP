# Rust Correctness Review — `83e1ee296`

Evidence: `Static` plus focused `Ran` evidence.

Verdict: `HOLD`.

Accepted findings:

1. Closure arithmetic preflight could return partition-membership E009 before
   reaching a later projection-stage E003. Candidate validation ignored that
   result and subsequently reported E009, violating global precedence.
2. Sealed and post-ingress LSE/soil-thermal E003 failures paired the implicated
   receiver owner with the hydrology beginning digest instead of that owner's
   rollback row.

The correction extracts an arithmetic-only preflight that scans receipt and
raw-parent arithmetic before partition identity and fully scans reordered
complete partition sets. Candidate validation continues to propagate E003 only,
so lower-priority E010 joins do not preempt producer E009. Receiver failures now
use one unique matching owner/kind rollback row or typed absence; attempted
hashes remain the complete receiver-set digest.

No broad or heavy gate was run by the reviewer. Fresh exact-byte review remains
required after the correction is committed.
