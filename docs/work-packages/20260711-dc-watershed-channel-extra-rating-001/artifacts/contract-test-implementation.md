# Contract-test implementation

Status: complete
Evidence mode: Ran

Added final-channel two-/three-/four-token and invalid-domain residual vectors,
a multi-channel inserted rating record, and a valid multi-channel file whose
next `comment_1` is exactly the same valid three-number triple and whose
`comment_2` is numeric-leading prose. Both modes are bound.

Ran: focused parser suite 26 total, 24 pass and 2 intended red. The final and
multi-channel valid-triple cases do not yet return exact `CHN-E-006`; all
generic residual, neither-layout, duplicate-enabled-rating, and numeric-comment
non-regressions already pass. Production parser and network-frame diffs are
empty. Latest run: `ed839f47-be0c-41cc-923f-3c0411494e3a`.

Correction design is predeclared: extract one shared channel-block/rating
validator used by the ordinary parser and a side-effect-free suffix probe; keep
probe warnings/output local; check canonical retained closure first; consider
only the immediate single candidate; memoize suffix states by physical cursor
and remaining channel ID so validation is bounded by reachable layout states;
and return the untouched ordinary parser error when recognition fails.

After production correction and test-only science-tier expansion, the parser
suite passes 38/38 and WSHED-W5 passes 20/20. Exact E006, generic residual,
retained-comment, neither-layout, duplicate-enabled-rating, all real non-finite
families, error surfaces, and real frame optional-rating projection are green.
