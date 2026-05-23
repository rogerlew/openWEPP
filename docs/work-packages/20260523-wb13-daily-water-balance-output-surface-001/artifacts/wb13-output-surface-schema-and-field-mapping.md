# WB13 Output Surface Schema and Field Mapping

Status: `completed`
Evidence mode: `Static`

## Canonical Schema
WB13 authority row schema is fixed at 25 columns in this order:

1. `OFE`
2. `J`
3. `Y`
4. `P`
5. `RM`
6. `Q`
7. `Ep`
8. `Es`
9. `Er`
10. `Dp`
11. `UpStrmQ`
12. `SubRIn`
13. `latqcc`
14. `Total-Soil`
15. `frozwt`
16. `Snow-Water`
17. `QOFE`
18. `Tile`
19. `Irr`
20. `Area`
21. `SoilWaterTotal`
22. `ProfileDepth`
23. `ProfilePorosityCap`
24. `ProfileFCStore`
25. `ProfileWPStore`

## Deterministic Mapping Notes
- `QOFE` is constrained to equal `Q` for single-OFE WB13 rows.
- `SoilWaterTotal` is constrained to `Total-Soil + frozwt` (within tolerance).
- Profile storage ordering is constrained to:
  `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.
- Row order key is constrained monotonic by `(Y, J, OFE)`.

## Units
- `OFE`, `J`, `Y`: integer keys.
- All remaining columns are serialized as daily scalar magnitudes in WB13
  output-surface depth/storage units used by legacy H5.wat parity surfaces.
