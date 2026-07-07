# Coefficient Due Diligence

Status: EXECUTED-SOURCE-AUTHORITY / D16-SENSITIVITY-HOLD. Evidence mode: Static + Ran.

Scope covered:

- Static extended lookup rows: 83 rows / 21 classes.
- Base lookup rows used by `build_extended_land_soil_lookup()`: 104 rows /
  26 classes.
- Soil textures in active table: `clay loam`, `loam`, `sand loam`, `silt loam`.
- Texture policy: `texture_invariant`. No source in this package authorizes
  texture-specific route-coefficient gradients.

Authority:

- `operator_calibration`, recorded in
  `/home/workdir/wepppy/docs/adrs/ADR-0014-disturbed-openwepp-route-coefficients.md`.
- No row is `unsupported`.
- Values are explicit Disturbed native input parameters, not mechanical
  transforms from row/ridge/random roughness/residue/cover fields.

Class matrix, ordered as `k_o`, form `C_d`, `D_r` in meters, `lambda`,
vegetation `C_d`:

| Disturbed class | Values | Directional rationale |
| --- | --- | --- |
| agriculture crops | `480.0, 0.25, 0.010, 0.050, 0.12` | Intermediate managed cover. |
| bare | `540.0, 0.00, 0.000, 0.000, 0.00` | No roughness elements or vegetation drag. |
| deciduous forest | `420.0, 0.90, 0.050, 0.180, 0.65` | Seasonal forest structure below conifer/mixed. |
| forest | `410.0, 0.95, 0.060, 0.200, 0.75` | Highest intact woody/vegetated protection. |
| forest high sev fire | `530.0, 0.18, 0.006, 0.018, 0.08` | Reduced protection after high severity fire. |
| forest low sev fire | `465.0, 0.58, 0.026, 0.085, 0.34` | Intermediate between unburned and moderate. |
| forest moderate sev fire | `490.0, 0.40, 0.016, 0.050, 0.20` | Less protection than low severity. |
| forest prescribed fire | `450.0, 0.70, 0.035, 0.110, 0.45` | Bounded prescribed disturbance. |
| grass high sev fire | `530.0, 0.08, 0.003, 0.010, 0.04` | Minimal residual grass structure. |
| grass low sev fire | `475.0, 0.27, 0.010, 0.045, 0.15` | Low severity retains some grass structure. |
| grass moderate sev fire | `500.0, 0.18, 0.007, 0.026, 0.09` | Reduced relative to low severity. |
| grass prescribed fire | `465.0, 0.32, 0.012, 0.055, 0.18` | Bounded prescribed disturbance. |
| high use skid | `575.0, 0.03, 0.000, 0.000, 0.00` | Compacted/high-use skid surface. |
| low or treated skid | `545.0, 0.12, 0.006, 0.020, 0.03` | Treated skid surface with slight residual roughness. |
| mixed forest | `415.0, 0.92, 0.055, 0.190, 0.70` | Mixed forest structure near conifer forest. |
| mulch | `420.0, 0.85, 0.040, 0.180, 0.20` | Treatment increases element roughness. |
| short grass | `460.0, 0.34, 0.014, 0.070, 0.24` | Less structure than tall grass. |
| shrub | `430.0, 0.72, 0.035, 0.120, 0.45` | Woody shrub structure. |
| shrub high sev fire | `525.0, 0.14, 0.004, 0.014, 0.06` | Strongly reduced protection. |
| shrub low sev fire | `465.0, 0.44, 0.020, 0.065, 0.24` | Low severity retains shrub structure. |
| shrub moderate sev fire | `490.0, 0.30, 0.012, 0.038, 0.14` | Reduced relative to low severity. |
| shrub prescribed fire | `450.0, 0.55, 0.026, 0.090, 0.32` | Bounded prescribed disturbance. |
| skid | `560.0, 0.05, 0.000, 0.000, 0.00` | Compacted/cleared surface with no vegetation drag. |
| tall grass | `440.0, 0.48, 0.020, 0.100, 0.35` | More structure than short grass. |
| thinning | `435.0, 0.90, 0.045, 0.160, 0.50` | Thinned forest treatment. |
| young forest | `430.0, 0.85, 0.045, 0.160, 0.60` | Forest protection below mature forest. |

Domain/coupling checks:

- `route_skin_friction_coefficient_ko > 0` for every row.
- Form drag, roughness height, and vegetation drag are nonnegative.
- Roughness concentration is within `[0, 1]`.
- Roughness height and roughness concentration are both zero or both positive.
- Vegetation drag is zero only for `bare` and `skid`.
- High-severity fire rows do not exceed corresponding unburned rows in
  roughness or vegetation protection.

Ran:

```text
cd /home/workdir/wepppy
wctl run-pytest tests/disturbed/test_route_coefficients.py -q
6 passed, 2 warnings
```

Sensitivity status:

- Numeric parse/projection stability is proven by the generated native fixture
  and openWEPP runtime projection test.
- Full H2637-class plus contrasting active hydrologic sensitivity was not
  executed in this package. That remains the D16-suite follow-on because the
  current package creates source authority and native production support rather
  than a complete executable active plain-vs-hybrid cohort.
