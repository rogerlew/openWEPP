# PL14S Erosion Exclusion Note

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- PL14S scope is hillslope WB13 semantic parity and comparator diagnostics only.
- Erosion/sediment surfaces are explicitly excluded from PL14S acceptance gates.
- No erosion kernel assertions are made by PL14S semantic report artifacts.

## Ran
- Executed replay/comparator lane produced only water-balance artifacts:
  - `h5_wat_semantic_comparator.json`
  - `h5_wat_strict_comparator.json` (skip sentinel)
  - `pl14s_provenance_manifest.json`
- No erosion parity artifacts were generated or claimed in this package.
