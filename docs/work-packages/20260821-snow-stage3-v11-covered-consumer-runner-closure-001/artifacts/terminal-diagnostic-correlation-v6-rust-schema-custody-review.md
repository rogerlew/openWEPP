# Terminal diagnostic correlation V6 Rust/schema/custody/privacy review

Recommendation: **HOLD / NO GO-to-evidence**

Evidence class: `Static`. Independent read-only review; no edits and no contact
with the numerical reviewer. All 13 frozen manifest hashes matched exactly.

## Findings

1. **Critical:** page/anchor strings are not compiler item IDs/DefIds, and a
   5000-byte HTML-window hash is not a resolved type ID. Visibility is never
   parsed or checked; base type checking is only substring search and expanded
   bindings omit even that.
2. **Critical:** core evidence has no source selector: carrier keys, coupling,
   selected-trial assembly, pair/error math, floor admission, ingress zeros,
   outer error and caller-local/before-after locations are missing.
3. **Critical:** method paths, conversions, stage, owner and access are trusted
   prose. The privacy negative fixture proves only that an access string is
   nonempty, not that private access is legal.
4. **High:** carrier coverage is top-level only and does not resolve nested leaf
   expressions or prove digests exclude whole values.
5. **High:** DTO-name closure does not model sequence cardinality/framing;
   `Digest32V6` does not mechanically enforce 32 bytes; constraints are inert.
6. **High:** snapshot checking proves a member-name subset, not exact types,
   canonicalizers, before/after timing, nested locations or caller-local state.
7. **High:** the zero report follows a shallow, incomplete universe. Claim-path
   validation checks only the first two components rather than nested paths.

The purpose-built DTO direction, 20/20 name closure, non-null nested names,
13/13 top-level carrier disposition and narrow inner outcome selector are
accepted but insufficient.
