# EROD16 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static
Date: 2026-05-26

## Static
- EROD16 is contract-authority step only.
- No contract-derived tests are implemented in this package.
- Contract text now explicitly defines route-branch authority needed for
  EROD17 test-vector implementation.

## Deferred to EROD17
1. `mshear` 1..5 branch-family test vectors.
2. Upper-end deposition (`du<0`) vs detachment (`du>=0`) routing vectors.
3. `ndep` post-detachment deposition follow-up vectors.
4. Near-zero `qostar` threshold vectors and typed guard expectations.
