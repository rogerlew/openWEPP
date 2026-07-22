# Coverage Closure

Status: `PASS / RENEWED REVIEW PENDING`

Static: Review A and Review B found that the earlier zero-below-floor statement
incorrectly used cargo-crap's LCOV line-coverage field. The retained earlier
LLVM export still passed the production-only aggregate floor but left
`build_audit`, `validate_audit_for_execution`, and `reconstruct_exact_plan`
below the binding 75% per-function region floor.

Commits `1c7dfa94` and `5c1cc1c1` add direct public construction, successful
execution admission, and canonical committed-plan reconstruction coverage.

Ran: the corrected exact-head measurement at clean `68e9b747` uses production
lines 1-1,743 and reports:

- production line coverage: 1,324/1,378 (96.08%);
- production region coverage: 1,886/2,104 (89.64%);
- production functions matched: 111/111;
- production functions below 75% region coverage: zero;
- minimum per-function region coverage: 80.00%;
- CRAP rows above 30: zero; maximum: 17.0.

The single instrumented traversal passed 117/117 tests in 234.36 seconds
(244.96 seconds wall). LCOV and LLVM JSON were exported from the same profile
data. The repository index was
`d34c9cbf70713fc735c93c191ee128d85922415b` before and after. No retained
exception or denominator exclusion is used.

Evidence root: `/tmp/cqr-pre-heavy-final-region-ORwL2Q`.

| Artifact | SHA-256 |
| --- | --- |
| target source | `b8ed9863410ab9695b0820f4959ec6cd03509c3b64ea8ed7ab991d8c88ca0be3` |
| LLVM export JSON | `6597a19e8010d47e4cd834364990804c2afb74c2377734772bec0cb202fbc614` |
| LCOV | `23599310ee40dd5e02c7a2c30257af976eec4dc38ffe2bb9794cb0bb6896192a` |
| CRAP JSON | `100f9ff7d79e0c2137e3e591077bf2e76055224782a25b3c4bc4fcf41ac8adde` |
| instrumented-run log | `00b01eb80af021794ea57a710e666f109297a577321c0e224117a06cb54cf293` |

The durable compact extraction record is `function-region-summary.json`; its
exact 111-row TSV source has SHA-256
`9dc0bf13209f6a6642905ffbfb1a86e962ad4555a0fcc84b8e0333cc45b8fcd6`.
