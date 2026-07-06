# Pre-Implementation Contract Gate

Status: BLOCKED
Evidence mode: Static

Gate question: does canonical authority now support wiring an active/shadow
friction operand builder for every active Lane D operand?

Result: `BLOCKED`.

Reason:

- `I` has a source candidate.
- `LAI` has a source candidate.
- `h_c` has unresolved source/timing binding.
- `k_o`, `C_d`, `D_r`, and `lambda` have no D11-ratified WEPP-runtime
  mapping/default.

In-envelope route considered:

1. Ratify a narrow bare-soil default using `k_o=500`, zero form/wave/vegetation,
   and source `I`.
2. Infer form/wave operands from residue depth, random roughness, or Chapter-10
   hydraulics symbols.
3. Wire only `I`/`LAI` and leave the missing operands zero.

Disposition: all three routes are rejected for D11 because they either ratify an
unsupported default or silently drop active roughness/vegetation physics. The
package holds before production/shadow builder edits.
