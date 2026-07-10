# Review Agent A

Evidence: Static, independent read-only review.

Initial findings:

1. Required reading omitted the kernel-writeback and unit-safe-boundary
   contracts.
2. Characterization needed direct dense/fallback and hot-symbol cases to support
   science-tier closure.
3. Typed invalid-value checks needed exact `BoundaryError` variants.

Disposition: accepted and fixed. The required-reading map/prompt/package now
name both contracts; tests cover dense view, legacy slots, indexed fallback, and
all hot accessors; invalid constructors assert `NonFinite`, `BelowMinimum`, and
`AboveMaximum`. Final post-fix metrics are `603 / 603` production lines and
`628 / 628` production regions, with no target CRAP row above `30`.

Re-review: PASS. The dense-view-to-legacy-slot fallback closures are directly
covered; mapping helpers preserve public API and original match order. No source
finding remains.
