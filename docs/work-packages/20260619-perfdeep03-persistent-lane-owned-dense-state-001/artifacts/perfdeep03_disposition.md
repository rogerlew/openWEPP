# PERFDEEP03 Disposition

Evidence class: Static + Ran.

Disposition: `NO-GO - section 7 falsification / re-profile before expanding`.

## Summary

PERFDEEP03 implemented the intended ownership correction: a lane-owned,
persistent compact dense state carried through scheduler execution, borrowed by
kernel requests, updated directly from accepted writebacks, and flushed at true
boundaries. Correctness gates passed and the opt-in H2637 outputs satisfied the
package identity requirements.

The real H2637 endpoint failed the hard performance gate:

```text
PERFDEEP01 reference: 669.97 s
PERFDEEP03 opt-in:    1147.96 s, 229580 KB
```

This is the package's falsification boundary. The lane-owned compact dense state
is correct enough to keep as gated implementation evidence, but it is not a
measured endpoint win and must not become default production behavior.

## Decision

`NO-GO - section 7 falsification / re-profile before expanding`.

## Practical Consequences

- Keep the PERFDEEP03 implementation gated behind
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1`.
- Do not revert to the PERFDEEP02 temporary mirror; that path is a verified
  negative benchmark.
- Do not activate PERFDEEP03 by default.
- Do not start Stage 2 by merely expanding the same partial-island shape.
- Re-profile the current PERFDEEP03 endpoint before the next work package.

## Why This Regressed

The dense frame itself is not enough. The simulation still crosses the
logical/indexed compatibility boundary around a partial hydrology island, and
publication/downstream surfaces still require logical/indexed materialization.
The carried dense state removes the worst PERFDEEP02 lifecycle bug, but the
remaining partial-island edge cost and fallback surface machinery still dominate
the endpoint.

## Dense Array Practicality

A dense array for the entire simulation is not practical as the next immediate
step. PERFDEEP03 proves that even a compact, lane-owned hydrology working set is
not enough while the rest of the runtime still depends on logical/indexed
surfaces. The practical path is to profile the current no-go implementation,
identify the dominant remaining edge/fallback costs, then choose either a larger
typed authoritative span that removes those costs or a different representation
strategy. Expanding blindly would repeat the same class of regression at a
larger scale.
