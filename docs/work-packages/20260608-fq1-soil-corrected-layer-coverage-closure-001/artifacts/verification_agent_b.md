# Verification Agent B

Evidence mode: `Ran:` and `Static:`.

Verified:

- `cargo fmt --check` passed.
- `cargo deny check` passed with existing warnings.
- p11 rerun reproduced deterministic downstream
  `HKERNEL-WB11-PERC-E-003` at `1990-162`, confirming it is not the original soil
  coverage guard.
- The remaining failure sits outside the package protected boundaries.

Result: package evidence supports executed hold at the downstream percolation
boundary, not further soil edits.
