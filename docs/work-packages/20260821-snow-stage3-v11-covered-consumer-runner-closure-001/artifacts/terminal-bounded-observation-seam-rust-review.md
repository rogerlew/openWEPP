# Terminal bounded observation-seam Rust/API/private-compilation review

Recommendation: **HOLD / NO GO-to-evidence**

Evidence class: `Static`. Independent read-only review; no edits and no contact
with the numerical reviewer. All four frozen hashes matched exactly.

## Findings

1. **Blocker:** six hook argument types are undefined, preventing compilation,
   custody, conversion and allocation review.
2. **Blocker:** `RejectedPrefixEvidence: Default` is invalid because its
   three-element `ZeroIngressEvidence` array has no default.
3. **Blocker:** the required three exact ingress accessors do not exist inside
   the seven-file write set.
4. **High:** proposed `PartialEq` derives embed live types that do not implement
   `PartialEq`; required derive/source changes are absent from the intent.
5. **High:** missing borrowed hook definitions prevent proof that `NoEvidence`
   avoids allocation/cloning during hook argument construction.
6. **High:** selection validation requires per-iteration convergence and later-
   record lookup not present in the validator input.
7. **High:** the coarse complete joint is currently bound as `_full_joint` and
   discarded before the proposed capture point.
8. **Medium:** first equal component winner must remain distinct from the
   existing cross-pair `>=` maximum-retention chronology.
