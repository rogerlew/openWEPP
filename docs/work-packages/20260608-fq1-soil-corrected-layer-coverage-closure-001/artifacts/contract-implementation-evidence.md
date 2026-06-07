# Contract Implementation Evidence

Evidence mode: `Static:`.

Updated `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`:

- Raised `contract_version` to `23`.
- Added `INV-SOIL-017`, requiring valid parser-layer corrected diagnostics to
  cover parser profile bottom by extending the deepest normalized corrected
  interval when the parser profile is deeper than the normalized grid.
- Added the FQ1 corrected parser-layer coverage addendum.
- Preserved `INV-SOIL-015`: normalized WB11/WB18/WB19 hydrology seed aliases
  remain on the baseline-normalized corrected-layer grid.
- Preserved fail-closed behavior for missing lineage, non-finite depths,
  nonpositive layer thickness, and nonmonotone parser layers.

No `SC-WATBAL-001` amendment was needed because the correction is diagnostic/
constitutive parser-layer coverage, not hydrology seed-grid or water-balance
publication authority.
